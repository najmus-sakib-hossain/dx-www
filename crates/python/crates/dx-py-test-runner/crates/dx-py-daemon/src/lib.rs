//! Daemon pool for pre-warmed Python interpreters
//!
//! This crate manages a pool of pre-warmed Python interpreters
//! for fast test execution.

pub use dx_py_core::{DaemonError, TestCase, TestResult, TestId, TestStatus};

use crossbeam::channel::{bounded, Receiver, Sender};
use std::collections::HashSet;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Configuration for the daemon pool
#[derive(Debug, Clone)]
pub struct DaemonConfig {
    /// Number of workers in the pool
    pub pool_size: usize,
    /// Modules to pre-import in workers
    pub preload_modules: Vec<String>,
    /// Python executable path
    pub python_path: String,
    /// Timeout for test execution
    pub timeout: Duration,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            pool_size: num_cpus::get(),
            preload_modules: vec![],
            python_path: "python".to_string(),
            timeout: Duration::from_secs(60),
        }
    }
}

impl DaemonConfig {
    pub fn with_pool_size(mut self, size: usize) -> Self {
        self.pool_size = size;
        self
    }

    pub fn with_preload(mut self, modules: Vec<String>) -> Self {
        self.preload_modules = modules;
        self
    }

    pub fn with_python(mut self, path: impl Into<String>) -> Self {
        self.python_path = path.into();
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

/// State of a worker
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerState {
    Idle,
    Busy,
    Crashed,
}

/// A Python worker process
pub struct Worker {
    id: usize,
    process: Option<Child>,
    state: WorkerState,
    preloaded_modules: HashSet<String>,
}

impl Worker {
    fn new(id: usize) -> Self {
        Self {
            id,
            process: None,
            state: WorkerState::Idle,
            preloaded_modules: HashSet::new(),
        }
    }

    fn spawn(&mut self, config: &DaemonConfig) -> Result<(), DaemonError> {
        let child = Command::new(&config.python_path)
            .arg("-c")
            .arg("import sys; sys.exit(0)")  // Placeholder - real impl would run worker script
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| DaemonError::StartupFailure(e.to_string()))?;

        self.process = Some(child);
        self.state = WorkerState::Idle;
        self.preloaded_modules = config.preload_modules.iter().cloned().collect();
        Ok(())
    }

    fn is_available(&self) -> bool {
        self.state == WorkerState::Idle
    }

    fn mark_busy(&mut self) {
        self.state = WorkerState::Busy;
    }

    fn mark_idle(&mut self) {
        self.state = WorkerState::Idle;
    }

    fn mark_crashed(&mut self) {
        self.state = WorkerState::Crashed;
    }

    fn terminate(&mut self) -> Result<(), DaemonError> {
        if let Some(ref mut process) = self.process {
            process.kill().ok();
            process.wait().ok();
        }
        self.process = None;
        self.state = WorkerState::Crashed;
        Ok(())
    }
}

/// Pool of pre-warmed Python workers
pub struct DaemonPool {
    config: DaemonConfig,
    workers: Arc<Mutex<Vec<Worker>>>,
    available_count: AtomicUsize,
    shutdown: AtomicBool,
    test_queue: Sender<TestCase>,
    test_receiver: Receiver<TestCase>,
}

impl DaemonPool {
    /// Create a new daemon pool with the given configuration
    pub fn new(config: DaemonConfig) -> Result<Self, DaemonError> {
        let (tx, rx) = bounded(config.pool_size * 10);
        let mut workers = Vec::with_capacity(config.pool_size);

        for i in 0..config.pool_size {
            let mut worker = Worker::new(i);
            worker.spawn(&config)?;
            workers.push(worker);
        }

        Ok(Self {
            available_count: AtomicUsize::new(config.pool_size),
            config,
            workers: Arc::new(Mutex::new(workers)),
            shutdown: AtomicBool::new(false),
            test_queue: tx,
            test_receiver: rx,
        })
    }

    /// Get the pool size
    pub fn pool_size(&self) -> usize {
        self.config.pool_size
    }

    /// Get the number of available workers
    pub fn available_workers(&self) -> usize {
        self.available_count.load(Ordering::Acquire)
    }

    /// Get the number of busy workers
    pub fn busy_workers(&self) -> usize {
        self.pool_size() - self.available_workers()
    }

    /// Check if shutdown has been requested
    pub fn is_shutdown(&self) -> bool {
        self.shutdown.load(Ordering::Acquire)
    }

    /// Acquire a worker for test execution
    pub fn acquire_worker(&self) -> Result<usize, DaemonError> {
        if self.is_shutdown() {
            return Err(DaemonError::ShutdownError("Pool is shutting down".into()));
        }

        let mut workers = self.workers.lock().unwrap();
        for (i, worker) in workers.iter_mut().enumerate() {
            if worker.is_available() {
                worker.mark_busy();
                self.available_count.fetch_sub(1, Ordering::Release);
                return Ok(i);
            }
        }

        Err(DaemonError::NoWorkerAvailable)
    }

    /// Release a worker back to the pool
    pub fn release_worker(&self, worker_id: usize) -> Result<(), DaemonError> {
        let mut workers = self.workers.lock().unwrap();
        if worker_id >= workers.len() {
            return Err(DaemonError::WorkerCrash(format!("Invalid worker id: {}", worker_id)));
        }

        workers[worker_id].mark_idle();
        self.available_count.fetch_add(1, Ordering::Release);
        Ok(())
    }

    /// Mark a worker as crashed and attempt to restart it
    pub fn handle_worker_crash(&self, worker_id: usize) -> Result<(), DaemonError> {
        let mut workers = self.workers.lock().unwrap();
        if worker_id >= workers.len() {
            return Err(DaemonError::WorkerCrash(format!("Invalid worker id: {}", worker_id)));
        }

        workers[worker_id].mark_crashed();
        workers[worker_id].terminate()?;
        workers[worker_id].spawn(&self.config)?;
        self.available_count.fetch_add(1, Ordering::Release);
        Ok(())
    }

    /// Queue a test for execution
    pub fn queue_test(&self, test: TestCase) -> Result<(), DaemonError> {
        if self.is_shutdown() {
            return Err(DaemonError::ShutdownError("Pool is shutting down".into()));
        }

        self.test_queue
            .send(test)
            .map_err(|e| DaemonError::WorkerCrash(e.to_string()))
    }

    /// Get the next queued test
    pub fn next_queued_test(&self) -> Option<TestCase> {
        self.test_receiver.try_recv().ok()
    }

    /// Get the number of queued tests
    pub fn queued_tests(&self) -> usize {
        self.test_receiver.len()
    }

    /// Gracefully shutdown the pool
    pub fn shutdown(&self) -> Result<(), DaemonError> {
        self.shutdown.store(true, Ordering::Release);

        let mut workers = self.workers.lock().unwrap();
        let mut errors = Vec::new();

        for worker in workers.iter_mut() {
            if let Err(e) = worker.terminate() {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(DaemonError::ShutdownError(format!(
                "Failed to terminate {} workers",
                errors.len()
            )))
        }
    }
}

impl Drop for DaemonPool {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

#[cfg(test)]
mod tests;


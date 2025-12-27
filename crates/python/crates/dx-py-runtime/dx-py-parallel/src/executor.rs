//! Parallel executor with thread-per-core design

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use crossbeam::deque::{Injector, Stealer};
use parking_lot::{Condvar, Mutex};

use crate::task::{Task, TaskHandle, TaskPriority};
use crate::worker::Worker;

/// Thread-per-core parallel executor
pub struct ParallelExecutor {
    /// Worker threads
    workers: Vec<Worker>,
    /// Global task queue
    global_queue: Arc<Injector<Task>>,
    /// Stealers for work-stealing
    stealers: Vec<Stealer<Task>>,
    /// Parker for waking workers
    parker: Arc<(Mutex<bool>, Condvar)>,
    /// Shutdown flag
    shutdown: Arc<AtomicBool>,
    /// Number of active tasks
    active_tasks: Arc<AtomicUsize>,
    /// Next worker for round-robin submission
    next_worker: AtomicUsize,
}

impl ParallelExecutor {
    /// Create a new parallel executor with one thread per physical core
    pub fn new() -> Self {
        let num_cores = core_affinity::get_core_ids()
            .map(|ids| ids.len())
            .unwrap_or_else(num_cpus::get_physical);
        
        Self::with_threads(num_cores)
    }
    
    /// Create a parallel executor with a specific number of threads
    pub fn with_threads(num_threads: usize) -> Self {
        let num_threads = num_threads.max(1);
        let global_queue = Arc::new(Injector::new());
        let parker = Arc::new((Mutex::new(false), Condvar::new()));
        let shutdown = Arc::new(AtomicBool::new(false));
        
        // Create workers
        let mut workers: Vec<Worker> = (0..num_threads)
            .map(Worker::new)
            .collect();
        
        // Collect stealers
        let stealers: Vec<Stealer<Task>> = workers.iter()
            .map(|w| w.stealer())
            .collect();
        
        // Start workers
        for (i, worker) in workers.iter_mut().enumerate() {
            let other_stealers: Vec<_> = stealers.iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, s)| s.clone())
                .collect();
            
            worker.start(
                Arc::clone(&global_queue),
                other_stealers,
                Arc::clone(&parker),
            );
        }
        
        Self {
            workers,
            global_queue,
            stealers,
            parker,
            shutdown,
            active_tasks: Arc::new(AtomicUsize::new(0)),
            next_worker: AtomicUsize::new(0),
        }
    }
    
    /// Get the number of worker threads
    pub fn num_threads(&self) -> usize {
        self.workers.len()
    }
    
    /// Submit a task for execution
    pub fn submit<F, R>(&self, f: F) -> TaskHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        self.submit_with_priority(f, TaskPriority::Normal)
    }
    
    /// Submit a task with specific priority
    pub fn submit_with_priority<F, R>(&self, f: F, priority: TaskPriority) -> TaskHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let (sender, receiver) = crossbeam::channel::bounded(1);
        let active_tasks = Arc::clone(&self.active_tasks);
        
        let task = Task::with_priority(
            move || {
                let result = f();
                let _ = sender.send(result);
                active_tasks.fetch_sub(1, Ordering::SeqCst);
            },
            priority,
        );
        
        self.active_tasks.fetch_add(1, Ordering::SeqCst);
        self.global_queue.push(task);
        self.wake_worker();
        
        TaskHandle::new(receiver)
    }
    
    /// Execute a parallel map operation
    pub fn parallel_map<T, R, F>(&self, items: Vec<T>, f: F) -> Vec<R>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: Fn(T) -> R + Send + Sync + 'static,
    {
        let f = Arc::new(f);
        let handles: Vec<_> = items
            .into_iter()
            .map(|item| {
                let f = Arc::clone(&f);
                self.submit(move || f(item))
            })
            .collect();
        
        handles.into_iter()
            .map(|h| h.wait().expect("Task failed"))
            .collect()
    }
    
    /// Execute a parallel for-each operation
    pub fn parallel_for_each<T, F>(&self, items: Vec<T>, f: F)
    where
        T: Send + 'static,
        F: Fn(T) + Send + Sync + 'static,
    {
        let f = Arc::new(f);
        let handles: Vec<_> = items
            .into_iter()
            .map(|item| {
                let f = Arc::clone(&f);
                self.submit(move || f(item))
            })
            .collect();
        
        for handle in handles {
            let _ = handle.wait();
        }
    }
    
    /// Get the number of active tasks
    pub fn active_tasks(&self) -> usize {
        self.active_tasks.load(Ordering::SeqCst)
    }
    
    /// Wake a worker thread
    fn wake_worker(&self) {
        let (lock, cvar) = &*self.parker;
        let mut has_work = lock.lock();
        *has_work = true;
        cvar.notify_one();
    }
    
    /// Wake all worker threads
    fn wake_all(&self) {
        let (lock, cvar) = &*self.parker;
        let mut has_work = lock.lock();
        *has_work = true;
        cvar.notify_all();
    }
    
    /// Shut down the executor
    pub fn shutdown(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
        self.wake_all();
        
        for worker in &mut self.workers {
            worker.shutdown();
            worker.join();
        }
    }
}

impl Default for ParallelExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ParallelExecutor {
    fn drop(&mut self) {
        if !self.shutdown.load(Ordering::SeqCst) {
            self.shutdown();
        }
    }
}

/// Get the number of physical CPU cores
pub fn num_cpus() -> usize {
    core_affinity::get_core_ids()
        .map(|ids| ids.len())
        .unwrap_or_else(num_cpus::get_physical)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicI32;
    
    #[test]
    fn test_executor_creation() {
        let executor = ParallelExecutor::with_threads(2);
        assert_eq!(executor.num_threads(), 2);
    }
    
    #[test]
    fn test_submit_task() {
        let executor = ParallelExecutor::with_threads(2);
        let handle = executor.submit(|| 42);
        let result = handle.wait().unwrap();
        assert_eq!(result, 42);
    }
    
    #[test]
    fn test_parallel_map() {
        let executor = ParallelExecutor::with_threads(4);
        let items: Vec<i32> = (0..100).collect();
        let results = executor.parallel_map(items, |x| x * 2);
        
        for (i, result) in results.iter().enumerate() {
            assert_eq!(*result, (i as i32) * 2);
        }
    }
    
    #[test]
    fn test_parallel_for_each() {
        let executor = ParallelExecutor::with_threads(4);
        let counter = Arc::new(AtomicI32::new(0));
        let items: Vec<i32> = (0..100).collect();
        
        let counter_clone = Arc::clone(&counter);
        executor.parallel_for_each(items, move |x| {
            counter_clone.fetch_add(x, Ordering::SeqCst);
        });
        
        // Sum of 0..100 = 4950
        assert_eq!(counter.load(Ordering::SeqCst), 4950);
    }
}

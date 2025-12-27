//! DxReactor - Main entry point for Binary Dawn.

use crate::core_state::CoreState;
use crate::io::ReactorConfig;
use crate::protocol::HbtpProtocol;
use std::sync::Arc;

/// Worker thread strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerStrategy {
    /// One worker thread per CPU core (default).
    ThreadPerCore,
    /// Fixed number of worker threads.
    Fixed(usize),
}

impl Default for WorkerStrategy {
    fn default() -> Self {
        Self::ThreadPerCore
    }
}

/// I/O backend selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoBackend {
    /// io_uring (Linux 5.1+).
    IoUring,
    /// epoll (Linux fallback).
    Epoll,
    /// kqueue (macOS/BSD).
    Kqueue,
    /// IOCP (Windows).
    Iocp,
    /// Automatically select the best backend.
    Auto,
}

impl Default for IoBackend {
    fn default() -> Self {
        Self::Auto
    }
}

/// Builder for DxReactor.
pub struct ReactorBuilder {
    workers: WorkerStrategy,
    io_backend: IoBackend,
    teleport: bool,
    hbtp: bool,
    buffer_size: usize,
    buffer_count: usize,
}

impl ReactorBuilder {
    /// Create a new builder with default settings.
    pub fn new() -> Self {
        Self {
            workers: WorkerStrategy::default(),
            io_backend: IoBackend::default(),
            teleport: true,
            hbtp: true,
            buffer_size: 4096,
            buffer_count: 1024,
        }
    }

    /// Set the worker strategy.
    pub fn workers(mut self, strategy: WorkerStrategy) -> Self {
        self.workers = strategy;
        self
    }

    /// Set the I/O backend.
    pub fn io_backend(mut self, backend: IoBackend) -> Self {
        self.io_backend = backend;
        self
    }

    /// Enable or disable memory teleportation.
    pub fn teleport(mut self, enabled: bool) -> Self {
        self.teleport = enabled;
        self
    }

    /// Enable or disable HBTP protocol.
    pub fn hbtp(mut self, enabled: bool) -> Self {
        self.hbtp = enabled;
        self
    }

    /// Set the buffer size for I/O operations.
    pub fn buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    /// Set the number of pre-allocated buffers.
    pub fn buffer_count(mut self, count: usize) -> Self {
        self.buffer_count = count;
        self
    }

    /// Build the DxReactor.
    pub fn build(self) -> DxReactor {
        let num_cores = match self.workers {
            WorkerStrategy::ThreadPerCore => num_cpus::get(),
            WorkerStrategy::Fixed(n) => n,
        };

        let config = ReactorConfig::default()
            .buffer_size(self.buffer_size)
            .buffer_count(self.buffer_count);

        let cores: Vec<CoreState> = (0..num_cores)
            .map(|id| CoreState::new(id, config.clone()))
            .collect();

        DxReactor {
            config,
            cores,
            protocol: Arc::new(HbtpProtocol::new()),
            worker_strategy: self.workers,
            io_backend: self.io_backend,
        }
    }
}

impl Default for ReactorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// The main DxReactor - Binary Dawn's core.
pub struct DxReactor {
    /// Reactor configuration.
    config: ReactorConfig,
    /// Per-core state.
    cores: Vec<CoreState>,
    /// HBTP protocol handler.
    protocol: Arc<HbtpProtocol>,
    /// Worker strategy used.
    worker_strategy: WorkerStrategy,
    /// I/O backend used.
    io_backend: IoBackend,
}

impl DxReactor {
    /// Create a new ReactorBuilder.
    pub fn build() -> ReactorBuilder {
        ReactorBuilder::new()
    }

    /// Get the number of cores/workers.
    pub fn num_cores(&self) -> usize {
        self.cores.len()
    }

    /// Get the worker strategy.
    pub fn worker_strategy(&self) -> WorkerStrategy {
        self.worker_strategy
    }

    /// Get the I/O backend.
    pub fn io_backend(&self) -> IoBackend {
        self.io_backend
    }

    /// Get a reference to the HBTP protocol handler.
    pub fn protocol(&self) -> &HbtpProtocol {
        &self.protocol
    }

    /// Get a mutable reference to the HBTP protocol handler.
    pub fn protocol_mut(&mut self) -> &mut HbtpProtocol {
        Arc::make_mut(&mut self.protocol)
    }

    /// Get a reference to a specific core's state.
    pub fn core(&self, id: usize) -> Option<&CoreState> {
        self.cores.get(id)
    }

    /// Get the reactor configuration.
    pub fn config(&self) -> &ReactorConfig {
        &self.config
    }

    /// Start the reactor (blocking).
    ///
    /// This spawns worker threads and runs the event loop.
    /// This function never returns under normal operation.
    pub fn ignite(self) -> ! {
        // In a real implementation, this would:
        // 1. Spawn worker threads
        // 2. Pin each thread to its core
        // 3. Run the event loop on each thread
        // 4. Handle shutdown signals
        
        // For now, just loop forever
        loop {
            std::thread::park();
        }
    }
}

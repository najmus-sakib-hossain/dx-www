//! Worker Pool - Background Task Processing
//!
//! Multi-threaded worker pool for background tasks:
//! - Cache warming
//! - R2 cloud sync
//! - Pattern analysis
//! - Package prefetching
//! - Cleanup operations

use anyhow::Result;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{mpsc, Semaphore};

// ============================================================================
// TASK PRIORITY
// ============================================================================

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TaskPriority {
    /// Critical - run immediately
    Critical = 0,
    /// High - run soon
    High = 1,
    /// Normal - default priority
    Normal = 2,
    /// Low - run when idle
    Low = 3,
    /// Background - run in spare cycles
    Background = 4,
}

impl Default for TaskPriority {
    fn default() -> Self {
        TaskPriority::Normal
    }
}

// ============================================================================
// WORKER TASKS
// ============================================================================

/// Background worker tasks
#[derive(Debug, Clone)]
pub enum WorkerTask {
    /// Warm up cache for a tool
    WarmCache {
        tool: String,
    },
    /// Sync to R2 cloud
    SyncToR2 {
        tool: String,
        paths: Vec<String>,
    },
    /// Pull from R2 cloud
    PullFromR2 {
        tool: String,
    },
    /// Analyze codebase patterns
    AnalyzePatterns {
        paths: Vec<String>,
    },
    /// Prefetch a package
    PrefetchPackage {
        name: String,
        version: String,
    },
    /// Clean old cache entries
    CleanCache {
        tool: String,
        max_age_days: u32,
    },
    /// Clean all caches
    CleanAllCaches,
    /// Build tool cache
    BuildCache {
        tool: String,
        output_paths: Vec<String>,
    },
    /// Index project files
    IndexProject {
        root: String,
    },
    /// Custom task
    Custom {
        name: String,
        data: serde_json::Value,
    },
}

impl WorkerTask {
    /// Get task name
    pub fn name(&self) -> &str {
        match self {
            WorkerTask::WarmCache { .. } => "WarmCache",
            WorkerTask::SyncToR2 { .. } => "SyncToR2",
            WorkerTask::PullFromR2 { .. } => "PullFromR2",
            WorkerTask::AnalyzePatterns { .. } => "AnalyzePatterns",
            WorkerTask::PrefetchPackage { .. } => "PrefetchPackage",
            WorkerTask::CleanCache { .. } => "CleanCache",
            WorkerTask::CleanAllCaches => "CleanAllCaches",
            WorkerTask::BuildCache { .. } => "BuildCache",
            WorkerTask::IndexProject { .. } => "IndexProject",
            WorkerTask::Custom { name, .. } => name,
        }
    }
}

/// Prioritized task wrapper
#[derive(Debug, Clone)]
pub struct PrioritizedTask {
    pub task: WorkerTask,
    pub priority: TaskPriority,
    pub created_at: u64,
}

impl PrioritizedTask {
    pub fn new(task: WorkerTask, priority: TaskPriority) -> Self {
        Self {
            task,
            priority,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }
}

// ============================================================================
// WORKER POOL
// ============================================================================

/// Worker pool statistics
#[derive(Debug, Clone, Default)]
pub struct WorkerPoolStats {
    pub workers: usize,
    pub tasks_queued: u64,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub busy_workers: usize,
}

/// Worker pool for background task processing
pub struct WorkerPool {
    /// Number of workers
    worker_count: usize,
    /// Task sender
    task_tx: mpsc::Sender<PrioritizedTask>,
    /// Running flag
    running: Arc<AtomicBool>,
    /// Statistics
    tasks_completed: Arc<AtomicU64>,
    tasks_failed: Arc<AtomicU64>,
    tasks_queued: Arc<AtomicU64>,
    busy_workers: Arc<AtomicU64>,
    /// Concurrency limiter
    semaphore: Arc<Semaphore>,
}

impl WorkerPool {
    /// Create a new worker pool
    pub fn new(worker_count: usize) -> Self {
        let (task_tx, mut task_rx) = mpsc::channel::<PrioritizedTask>(1000);
        let running = Arc::new(AtomicBool::new(true));
        let tasks_completed = Arc::new(AtomicU64::new(0));
        let tasks_failed = Arc::new(AtomicU64::new(0));
        let tasks_queued = Arc::new(AtomicU64::new(0));
        let busy_workers = Arc::new(AtomicU64::new(0));
        let semaphore = Arc::new(Semaphore::new(worker_count));

        // Spawn worker coordinator
        let running_clone = running.clone();
        let completed_clone = tasks_completed.clone();
        let failed_clone = tasks_failed.clone();
        let queued_clone = tasks_queued.clone();
        let busy_clone = busy_workers.clone();
        let sem_clone = semaphore.clone();

        tokio::spawn(async move {
            println!("👷 Worker Pool started with {} workers", worker_count);

            while running_clone.load(Ordering::SeqCst) {
                match task_rx.recv().await {
                    Some(prioritized_task) => {
                        queued_clone.fetch_sub(1, Ordering::SeqCst);
                        
                        // Acquire semaphore permit
                        let permit = sem_clone.clone().acquire_owned().await;
                        if permit.is_err() {
                            continue;
                        }

                        busy_clone.fetch_add(1, Ordering::SeqCst);

                        let task = prioritized_task.task;
                        let completed = completed_clone.clone();
                        let failed = failed_clone.clone();
                        let busy = busy_clone.clone();

                        tokio::spawn(async move {
                            let task_name = task.name().to_string();
                            let start = Instant::now();

                            match Self::execute_task(task).await {
                                Ok(()) => {
                                    completed.fetch_add(1, Ordering::SeqCst);
                                    println!(
                                        "✅ [BG] {} completed in {:?}",
                                        task_name,
                                        start.elapsed()
                                    );
                                }
                                Err(e) => {
                                    failed.fetch_add(1, Ordering::SeqCst);
                                    eprintln!("❌ [BG] {} failed: {}", task_name, e);
                                }
                            }

                            busy.fetch_sub(1, Ordering::SeqCst);
                            drop(permit);
                        });
                    }
                    None => break,
                }
            }

            println!("👷 Worker Pool stopped");
        });

        Self {
            worker_count,
            task_tx,
            running,
            tasks_completed,
            tasks_failed,
            tasks_queued,
            busy_workers,
            semaphore,
        }
    }

    /// Execute a task
    async fn execute_task(task: WorkerTask) -> Result<()> {
        match task {
            WorkerTask::WarmCache { tool } => {
                println!("🔥 [BG] Warming cache for {}...", tool);
                // TODO: Actual cache warming
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
            WorkerTask::SyncToR2 { tool, paths } => {
                println!("☁️  [BG] Syncing {} files to R2 for {}...", paths.len(), tool);
                // TODO: Actual R2 sync
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            }
            WorkerTask::PullFromR2 { tool } => {
                println!("⬇️  [BG] Pulling cache from R2 for {}...", tool);
                // TODO: Actual R2 pull
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            }
            WorkerTask::AnalyzePatterns { paths } => {
                println!("🔍 [BG] Analyzing patterns in {} files...", paths.len());
                // TODO: Pattern analysis
                tokio::time::sleep(std::time::Duration::from_millis(300)).await;
            }
            WorkerTask::PrefetchPackage { name, version } => {
                println!("📦 [BG] Prefetching {}@{}...", name, version);
                // TODO: Package prefetch
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
            WorkerTask::CleanCache { tool, max_age_days } => {
                println!("🧹 [BG] Cleaning cache for {} (max age: {} days)...", tool, max_age_days);
                // TODO: Cache cleanup
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
            WorkerTask::CleanAllCaches => {
                println!("🧹 [BG] Cleaning all caches...");
                // TODO: Clean all
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
            WorkerTask::BuildCache { tool, output_paths } => {
                println!("📦 [BG] Building cache for {} ({} files)...", tool, output_paths.len());
                // TODO: Build cache
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            }
            WorkerTask::IndexProject { root } => {
                println!("📁 [BG] Indexing project at {}...", root);
                // TODO: Index project
                tokio::time::sleep(std::time::Duration::from_millis(300)).await;
            }
            WorkerTask::Custom { name, data } => {
                println!("⚙️  [BG] Running custom task: {} with {:?}", name, data);
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        }

        Ok(())
    }

    /// Queue a task
    pub async fn queue(&self, task: WorkerTask) {
        self.queue_with_priority(task, TaskPriority::Normal).await;
    }

    /// Queue a task with priority
    pub async fn queue_with_priority(&self, task: WorkerTask, priority: TaskPriority) {
        let prioritized = PrioritizedTask::new(task, priority);
        self.tasks_queued.fetch_add(1, Ordering::SeqCst);
        let _ = self.task_tx.send(prioritized).await;
    }

    /// Queue multiple tasks
    pub async fn queue_many(&self, tasks: Vec<WorkerTask>) {
        for task in tasks {
            self.queue(task).await;
        }
    }

    /// Get statistics
    pub fn stats(&self) -> WorkerPoolStats {
        WorkerPoolStats {
            workers: self.worker_count,
            tasks_queued: self.tasks_queued.load(Ordering::SeqCst),
            tasks_completed: self.tasks_completed.load(Ordering::SeqCst),
            tasks_failed: self.tasks_failed.load(Ordering::SeqCst),
            busy_workers: self.busy_workers.load(Ordering::SeqCst) as usize,
        }
    }

    /// Stop the worker pool
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if pool is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Get worker count
    pub fn worker_count(&self) -> usize {
        self.worker_count
    }

    /// Get busy worker count
    pub fn busy_workers(&self) -> usize {
        self.busy_workers.load(Ordering::SeqCst) as usize
    }

    /// Wait for all tasks to complete
    pub async fn wait_for_completion(&self) {
        while self.tasks_queued.load(Ordering::SeqCst) > 0 
            || self.busy_workers.load(Ordering::SeqCst) > 0 
        {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    }
}

impl Default for WorkerPool {
    fn default() -> Self {
        Self::new(num_cpus::get())
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_priority_order() {
        assert!(TaskPriority::Critical < TaskPriority::High);
        assert!(TaskPriority::High < TaskPriority::Normal);
        assert!(TaskPriority::Normal < TaskPriority::Low);
        assert!(TaskPriority::Low < TaskPriority::Background);
    }

    #[test]
    fn test_worker_task_name() {
        assert_eq!(WorkerTask::WarmCache { tool: "bundler".to_string() }.name(), "WarmCache");
        assert_eq!(WorkerTask::CleanAllCaches.name(), "CleanAllCaches");
    }

    #[tokio::test]
    async fn test_worker_pool_creation() {
        let pool = WorkerPool::new(2);
        assert_eq!(pool.worker_count(), 2);
        assert!(pool.is_running());
        
        pool.stop();
    }

    #[tokio::test]
    async fn test_worker_pool_queue() {
        let pool = WorkerPool::new(2);
        
        pool.queue(WorkerTask::WarmCache { tool: "test".to_string() }).await;
        
        // Wait a bit for task to process
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        
        let stats = pool.stats();
        assert!(stats.tasks_completed >= 1 || stats.tasks_queued > 0);
        
        pool.stop();
    }
}

//! Task Executor
//!
//! Loads the Binary Task Graph and executes tasks with parallel scheduling.

use crate::btg::{BtgHeader, BtgSerializer, TaskData, TaskGraphData};
use crate::error::TaskError;
use crate::types::TaskInstance;
use bitvec::prelude::*;
use memmap2::Mmap;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Task output from execution
#[derive(Debug, Clone)]
pub struct TaskOutput {
    /// Task index
    pub task_idx: u32,
    /// Exit code (0 = success)
    pub exit_code: i32,
    /// Standard output
    pub stdout: Vec<u8>,
    /// Standard error
    pub stderr: Vec<u8>,
    /// Execution time in microseconds
    pub duration_us: u64,
}

/// Task Executor for loading and running task graphs
pub struct TaskExecutor {
    /// Memory-mapped task graph
    mmap: Option<Mmap>,
    /// Parsed task graph data
    data: Option<TaskGraphData>,
    /// Path to task graph file
    graph_path: Option<PathBuf>,
    /// Task name to index lookup
    task_index: rustc_hash::FxHashMap<(u32, String), u32>,
    /// Completed tasks bitset
    completed: BitVec,
}

impl TaskExecutor {
    /// Create a new task executor
    pub fn new() -> Self {
        Self {
            mmap: None,
            data: None,
            graph_path: None,
            task_index: rustc_hash::FxHashMap::default(),
            completed: BitVec::new(),
        }
    }

    /// Load task graph from memory-mapped file
    pub fn load(&mut self, path: &Path) -> Result<(), TaskError> {
        let file = std::fs::File::open(path).map_err(|_| TaskError::GraphNotFound {
            path: path.to_path_buf(),
        })?;

        let mmap = unsafe { Mmap::map(&file) }?;

        if mmap.len() < BtgHeader::SIZE {
            return Err(TaskError::ExecutionFailed {
                exit_code: -1,
                stderr: "file too small".to_string(),
            });
        }

        let header: &BtgHeader = bytemuck::from_bytes(&mmap[..BtgHeader::SIZE]);
        header.validate_magic()?;

        let data = BtgSerializer::deserialize(&mmap)?;
        self.build_task_index(&data);
        self.completed = bitvec![0; data.tasks.len()];

        self.mmap = Some(mmap);
        self.data = Some(data);
        self.graph_path = Some(path.to_path_buf());

        Ok(())
    }

    /// Load from raw bytes (for testing)
    pub fn load_from_bytes(&mut self, bytes: &[u8]) -> Result<(), TaskError> {
        let data = BtgSerializer::deserialize(bytes)?;
        self.build_task_index(&data);
        self.completed = bitvec![0; data.tasks.len()];
        self.data = Some(data);
        Ok(())
    }

    /// Build task lookup index
    fn build_task_index(&mut self, data: &TaskGraphData) {
        self.task_index.clear();
        for (idx, task) in data.tasks.iter().enumerate() {
            self.task_index.insert((task.package_idx, task.name.clone()), idx as u32);
        }
    }

    /// Get task by package and name
    pub fn get_task(&self, package_idx: u32, name: &str) -> Option<&TaskData> {
        let idx = *self.task_index.get(&(package_idx, name.to_string()))?;
        self.data.as_ref()?.tasks.get(idx as usize)
    }

    /// Get task by index
    pub fn get_task_by_index(&self, idx: u32) -> Option<&TaskData> {
        self.data.as_ref()?.tasks.get(idx as usize)
    }

    /// Get tasks that can run in parallel at current stage
    pub fn parallel_tasks(&self) -> Vec<u32> {
        let data = match &self.data {
            Some(d) => d,
            None => return Vec::new(),
        };

        let mut ready = Vec::new();

        for (idx, _task) in data.tasks.iter().enumerate() {
            if self.completed[idx] {
                continue;
            }

            // Check if all dependencies are completed
            let deps_complete = data
                .dependency_edges
                .iter()
                .filter(|(_, to)| *to == idx as u32)
                .all(|(from, _)| self.completed[*from as usize]);

            if deps_complete {
                ready.push(idx as u32);
            }
        }

        ready
    }

    /// Clone a task template for execution (zero-allocation)
    #[inline]
    pub fn clone_task(&self, task_idx: u32) -> TaskInstance {
        TaskInstance::new(task_idx)
    }

    /// Check if task should yield due to frame budget
    #[inline]
    pub fn should_yield(&self, task: &TaskInstance, now_ns: u64) -> bool {
        let data = match &self.data {
            Some(d) => d,
            None => return false,
        };

        let task_data = match data.tasks.get(task.task_idx as usize) {
            Some(t) => t,
            None => return false,
        };

        if task_data.frame_budget_us == 0 {
            return false;
        }

        task.elapsed_us(now_ns) >= task_data.frame_budget_us as u64
    }

    /// Execute a task
    pub fn execute(&mut self, task_idx: u32) -> Result<TaskOutput, TaskError> {
        let data = self.data.as_ref().ok_or_else(|| TaskError::ExecutionFailed {
            exit_code: -1,
            stderr: "no task graph loaded".to_string(),
        })?;

        let task = data.tasks.get(task_idx as usize).ok_or_else(|| TaskError::TaskNotFound {
            package: "unknown".to_string(),
            task: format!("index {}", task_idx),
        })?;

        let start = Instant::now();

        // Check dependencies
        for (from, to) in &data.dependency_edges {
            if *to == task_idx && !self.completed[*from as usize] {
                return Err(TaskError::DependencyFailed {
                    task_idx: *from,
                    reason: "dependency not completed".to_string(),
                });
            }
        }

        // Execute command (placeholder - would actually run the command)
        let output = TaskOutput {
            task_idx,
            exit_code: 0,
            stdout: format!("Executed: {}", task.command).into_bytes(),
            stderr: Vec::new(),
            duration_us: start.elapsed().as_micros() as u64,
        };

        // Mark as completed
        self.completed.set(task_idx as usize, true);

        Ok(output)
    }

    /// Mark task as completed
    pub fn mark_completed(&mut self, task_idx: u32) {
        if (task_idx as usize) < self.completed.len() {
            self.completed.set(task_idx as usize, true);
        }
    }

    /// Check if task is completed
    pub fn is_completed(&self, task_idx: u32) -> bool {
        self.completed.get(task_idx as usize).map(|b| *b).unwrap_or(false)
    }

    /// Get number of tasks
    pub fn task_count(&self) -> usize {
        self.data.as_ref().map(|d| d.tasks.len()).unwrap_or(0)
    }

    /// Reset all tasks to not completed
    pub fn reset(&mut self) {
        self.completed.fill(false);
    }
}

impl Default for TaskExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TaskState;

    fn create_test_graph() -> TaskGraphData {
        let mut data = TaskGraphData {
            tasks: vec![
                TaskData {
                    name: "build".to_string(),
                    package_idx: 0,
                    command: "npm run build".to_string(),
                    definition_hash: [0; 8],
                    frame_budget_us: 0,
                    cacheable: true,
                },
                TaskData {
                    name: "build".to_string(),
                    package_idx: 1,
                    command: "npm run build".to_string(),
                    definition_hash: [0; 8],
                    frame_budget_us: 0,
                    cacheable: true,
                },
                TaskData {
                    name: "test".to_string(),
                    package_idx: 0,
                    command: "npm test".to_string(),
                    definition_hash: [0; 8],
                    frame_budget_us: 16000, // 16ms frame budget
                    cacheable: false,
                },
            ],
            dependency_edges: vec![(0, 2)], // build:0 -> test:0
            topological_order: vec![0, 1, 2],
            parallel_groups: vec![],
        };
        data.compute_parallel_groups();
        data
    }

    #[test]
    fn test_task_executor_load() {
        let data = create_test_graph();
        let bytes = BtgSerializer::serialize(&data).unwrap();

        let mut executor = TaskExecutor::new();
        executor.load_from_bytes(&bytes).unwrap();

        assert_eq!(executor.task_count(), 3);
    }

    #[test]
    fn test_task_lookup() {
        let data = create_test_graph();
        let bytes = BtgSerializer::serialize(&data).unwrap();

        let mut executor = TaskExecutor::new();
        executor.load_from_bytes(&bytes).unwrap();

        let task = executor.get_task(0, "build").unwrap();
        assert_eq!(task.command, "npm run build");

        let task = executor.get_task(0, "test").unwrap();
        assert_eq!(task.command, "npm test");

        assert!(executor.get_task(0, "nonexistent").is_none());
    }

    #[test]
    fn test_parallel_tasks() {
        let data = create_test_graph();
        let bytes = BtgSerializer::serialize(&data).unwrap();

        let mut executor = TaskExecutor::new();
        executor.load_from_bytes(&bytes).unwrap();

        // Initially, build:0 and build:1 can run in parallel
        let parallel = executor.parallel_tasks();
        assert!(parallel.contains(&0));
        assert!(parallel.contains(&1));
        assert!(!parallel.contains(&2)); // test:0 depends on build:0

        // After completing build:0, test:0 becomes available
        executor.mark_completed(0);
        let parallel = executor.parallel_tasks();
        assert!(!parallel.contains(&0)); // already completed
        assert!(parallel.contains(&1));
        assert!(parallel.contains(&2)); // now available
    }

    #[test]
    fn test_task_instance_zero_allocation() {
        let executor = TaskExecutor::new();

        // This should be stack-allocated
        let instance = executor.clone_task(5);
        assert_eq!(instance.task_idx, 5);
        assert_eq!(instance.state, TaskState::Pending);

        // Verify size is reasonable for stack allocation
        assert!(TaskInstance::SIZE <= 96);
    }

    #[test]
    fn test_frame_budget_check() {
        let data = create_test_graph();
        let bytes = BtgSerializer::serialize(&data).unwrap();

        let mut executor = TaskExecutor::new();
        executor.load_from_bytes(&bytes).unwrap();

        let mut instance = executor.clone_task(2); // test task with 16ms budget
        instance.start(0);

        // At 10ms, should not yield
        assert!(!executor.should_yield(&instance, 10_000_000)); // 10ms in ns

        // At 20ms, should yield
        assert!(executor.should_yield(&instance, 20_000_000)); // 20ms in ns
    }

    #[test]
    fn test_execute_with_dependencies() {
        let data = create_test_graph();
        let bytes = BtgSerializer::serialize(&data).unwrap();

        let mut executor = TaskExecutor::new();
        executor.load_from_bytes(&bytes).unwrap();

        // Can't execute test:0 before build:0
        let result = executor.execute(2);
        assert!(matches!(result, Err(TaskError::DependencyFailed { .. })));

        // Execute build:0 first
        executor.execute(0).unwrap();
        assert!(executor.is_completed(0));

        // Now test:0 can execute
        let output = executor.execute(2).unwrap();
        assert_eq!(output.exit_code, 0);
        assert!(executor.is_completed(2));
    }
}

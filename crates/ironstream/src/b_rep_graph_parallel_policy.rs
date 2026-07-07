// FILE: b_rep_graph_parallel_policy.rs
// occt: BRepGraph_ParallelPolicy

//! Policy for parallel graph operations.

/// Execution mode for parallel operations
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutionMode {
    Sequential,
    Parallel,
}

/// Policy for controlling parallel execution in BRepGraph
pub struct BRepGraphParallelPolicy {
    mode: ExecutionMode,
    max_threads: usize,
}

impl BRepGraphParallelPolicy {
    /// Creates a sequential policy
    pub fn sequential() -> Self {
        BRepGraphParallelPolicy {
            mode: ExecutionMode::Sequential,
            max_threads: 1,
        }
    }

    /// Creates a parallel policy with default thread count
    pub fn parallel() -> Self {
        BRepGraphParallelPolicy {
            mode: ExecutionMode::Parallel,
            max_threads: num_cpus::get().unwrap_or(4),
        }
    }

    /// Creates a parallel policy with specific thread count
    pub fn parallel_with_threads(threads: usize) -> Self {
        BRepGraphParallelPolicy {
            mode: ExecutionMode::Parallel,
            max_threads: threads.max(1),
        }
    }

    /// Returns execution mode
    pub fn mode(&self) -> ExecutionMode {
        self.mode
    }

    /// Returns maximum thread count
    pub fn max_threads(&self) -> usize {
        self.max_threads
    }

    /// Returns whether execution is parallel
    pub fn is_parallel(&self) -> bool {
        self.mode == ExecutionMode::Parallel
    }
}

impl Default for BRepGraphParallelPolicy {
    fn default() -> Self {
        Self::sequential()
    }
}

// Helper for getting CPU count - fallback if num_cpus not available
mod num_cpus {
    pub fn get() -> Option<usize> {
        std::thread::available_parallelism().ok().map(|n| n.get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_policy_sequential() {
        let policy = BRepGraphParallelPolicy::sequential();
        assert_eq!(policy.mode(), ExecutionMode::Sequential);
        assert_eq!(policy.max_threads(), 1);
        assert!(!policy.is_parallel());
    }

    #[test]
    fn test_parallel_policy_parallel() {
        let policy = BRepGraphParallelPolicy::parallel();
        assert_eq!(policy.mode(), ExecutionMode::Parallel);
        assert!(policy.max_threads() >= 1);
        assert!(policy.is_parallel());
    }

    #[test]
    fn test_parallel_policy_with_threads() {
        let policy = BRepGraphParallelPolicy::parallel_with_threads(4);
        assert_eq!(policy.max_threads(), 4);
        assert!(policy.is_parallel());
    }

    #[test]
    fn test_parallel_policy_default() {
        let policy = BRepGraphParallelPolicy::default();
        assert_eq!(policy.mode(), ExecutionMode::Sequential);
    }
}

// FILE: bop_tools_parallel.rs
// occt: BOPTools_Parallel

/// Sequential execution wrapper for parallel algorithms.
/// Port of BOPTools_Parallel which uses threading in C++.
/// Since we avoid threads, we implement sequential equivalents.
pub struct BopToolsParallel;

impl BopToolsParallel {
    /// Execute a series of solver operations sequentially.
    /// In OCCT this uses parallel threads, but we serialize here.
    pub fn perform<F>(run_parallel: bool, count: usize, mut functor: F)
    where
        F: FnMut(usize),
    {
        let _use_parallel = run_parallel; // Hint to executor
        for i in 0..count {
            functor(i);
        }
    }

    /// Execute with context (sequential version).
    pub fn perform_with_context<F>(run_parallel: bool, count: usize, mut functor: F)
    where
        F: FnMut(usize),
    {
        let _use_parallel = run_parallel;
        for i in 0..count {
            functor(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_perform_sequential() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        BopToolsParallel::perform(false, 10, move |_idx| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert_eq!(counter.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_perform_with_parallel_hint() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        BopToolsParallel::perform(true, 5, move |_idx| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Even with parallel hint, we execute sequentially
        assert_eq!(counter.load(Ordering::SeqCst), 5);
    }

    #[test]
    fn test_perform_with_context() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        BopToolsParallel::perform_with_context(false, 3, move |_idx| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }
}

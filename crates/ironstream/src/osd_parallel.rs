// FILE: osd_parallel.rs
// occt: OSD_Parallel

//! Parallel execution utilities, port of OSD_Parallel.
//! Provides a simple parallelization tool: For(begin, end, functor)
//! executes the functor for each index in [begin, end), either
//! sequentially or split across OS threads.

/// Parallel execution utilities (OSD_Parallel).
pub struct Parallel;

impl Parallel {
    pub fn new() -> Self {
        Self
    }

    /// Returns the number of logical processors
    /// (OSD_Parallel::NbLogicalProcessors).
    pub fn nb_logical_processors() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    }

    /// Backwards-compatible alias.
    pub fn num_threads() -> usize {
        Self::nb_logical_processors()
    }

    /// Simple parallelization of "for" loops, like OSD_Parallel::For.
    /// Executes `functor(i)` for every i in [begin, end).
    /// When `force_single_thread` is true (or the range is trivial),
    /// execution is strictly sequential.
    pub fn for_range<F>(begin: i32, end: i32, functor: F, force_single_thread: bool)
    where
        F: Fn(i32) + Sync,
    {
        if begin >= end {
            return;
        }
        let count = (end - begin) as usize;
        let nb_threads = Self::nb_logical_processors().min(count);

        if force_single_thread || nb_threads <= 1 {
            for i in begin..end {
                functor(i);
            }
            return;
        }

        // Split the range into contiguous chunks, one per thread.
        let functor_ref = &functor;
        std::thread::scope(|scope| {
            let chunk = count.div_ceil(nb_threads);
            let mut start = begin;
            while start < end {
                let stop = end.min(start + chunk as i32);
                scope.spawn(move || {
                    for i in start..stop {
                        functor_ref(i);
                    }
                });
                start = stop;
            }
        });
    }

    /// ForEach over a slice: applies the functor to every element,
    /// mirroring OSD_Parallel::ForEach over an iterator range.
    pub fn for_each<T, F>(items: &[T], functor: F, force_single_thread: bool)
    where
        T: Sync,
        F: Fn(&T) + Sync,
    {
        Self::for_range(
            0,
            items.len() as i32,
            |i| functor(&items[i as usize]),
            force_single_thread,
        );
    }
}

impl Default for Parallel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};

    #[test]
    fn test_num_threads() {
        let n = Parallel::num_threads();
        assert!(n > 0);
        assert_eq!(n, Parallel::nb_logical_processors());
    }

    #[test]
    fn test_for_range_sequential() {
        let sum = AtomicI64::new(0);
        Parallel::for_range(0, 100, |i| {
            sum.fetch_add(i as i64, Ordering::Relaxed);
        }, true);
        assert_eq!(sum.load(Ordering::Relaxed), 4950);
    }

    #[test]
    fn test_for_range_parallel() {
        let sum = AtomicI64::new(0);
        let calls = AtomicUsize::new(0);
        Parallel::for_range(1, 1001, |i| {
            sum.fetch_add(i as i64, Ordering::Relaxed);
            calls.fetch_add(1, Ordering::Relaxed);
        }, false);
        assert_eq!(sum.load(Ordering::Relaxed), 500500);
        assert_eq!(calls.load(Ordering::Relaxed), 1000);
    }

    #[test]
    fn test_for_range_empty() {
        let calls = AtomicUsize::new(0);
        Parallel::for_range(5, 5, |_| {
            calls.fetch_add(1, Ordering::Relaxed);
        }, false);
        Parallel::for_range(10, 3, |_| {
            calls.fetch_add(1, Ordering::Relaxed);
        }, false);
        assert_eq!(calls.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_for_each() {
        let items = vec![1, 2, 3, 4, 5];
        let sum = AtomicI64::new(0);
        Parallel::for_each(&items, |v| {
            sum.fetch_add(*v, Ordering::Relaxed);
        }, false);
        assert_eq!(sum.load(Ordering::Relaxed), 15);
    }

    #[test]
    fn test_negative_range() {
        let sum = AtomicI64::new(0);
        Parallel::for_range(-3, 4, |i| {
            sum.fetch_add(i as i64, Ordering::Relaxed);
        }, false);
        // -3 + -2 + -1 + 0 + 1 + 2 + 3 = 0
        assert_eq!(sum.load(Ordering::Relaxed), 0);
    }
}

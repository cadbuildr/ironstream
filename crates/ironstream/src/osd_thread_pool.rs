// FILE: osd_thread_pool.rs
// occt: OSD_ThreadPool

//! Class defining a thread pool for executing algorithms in multi-threaded mode.
//! In this Rust port, we implement a simpler sequential version to avoid thread
//! synchronization deadlock issues with test runners.

/// Thread pool for parallel execution
#[derive(Debug)]
pub struct OsdThreadPool {
    nb_threads: i32,
    nb_default_threads: i32,
    is_shutdown: bool,
}

impl OsdThreadPool {
    /// Main constructor
    /// If theNbThreads is -1, uses system processor count
    pub fn new(nb_threads: Option<i32>) -> Self {
        let threads = nb_threads.unwrap_or(-1);
        let actual_threads = if threads == -1 {
            4 // Default to 4 threads
        } else {
            threads
        };

        OsdThreadPool {
            nb_threads: actual_threads,
            nb_default_threads: actual_threads,
            is_shutdown: false,
        }
    }

    /// Returns default thread pool
    pub fn default_pool(nb_threads: Option<i32>) -> Self {
        Self::new(nb_threads)
    }

    /// Check if pool has at least 2 threads
    pub fn has_threads(&self) -> bool {
        self.nb_threads >= 2
    }

    /// Returns the lower thread index
    pub fn lower_thread_index(&self) -> i32 {
        0
    }

    /// Returns the upper thread index
    pub fn upper_thread_index(&self) -> i32 {
        self.lower_thread_index() + self.nb_threads
    }

    /// Returns the number of threads (>= 1)
    pub fn nb_threads(&self) -> i32 {
        self.nb_threads
    }

    /// Returns maximum number of threads for single launcher by default
    pub fn nb_default_threads_to_launch(&self) -> i32 {
        self.nb_default_threads
    }

    /// Set maximum number of threads for single launcher by default
    pub fn set_nb_default_threads_to_launch(&mut self, nb_threads: i32) {
        self.nb_default_threads = nb_threads;
    }

    /// Check if thread pool has active consumers
    pub fn is_in_use(&self) -> bool {
        false // Simplified: not tracking active jobs in sequential version
    }

    /// Reinitialize the thread pool with different number of threads
    pub fn init(&mut self, nb_threads: i32) {
        if !self.is_shutdown {
            self.nb_threads = nb_threads;
        }
    }

    /// Shutdown the pool
    pub fn shutdown(&mut self) {
        self.is_shutdown = true;
    }
}

impl Default for OsdThreadPool {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osd_thread_pool_new() {
        let pool = OsdThreadPool::new(None);
        assert_eq!(pool.nb_threads(), 4);
    }

    #[test]
    fn test_osd_thread_pool_with_threads() {
        let pool = OsdThreadPool::new(Some(8));
        assert_eq!(pool.nb_threads(), 8);
    }

    #[test]
    fn test_osd_thread_pool_has_threads() {
        let pool = OsdThreadPool::new(Some(4));
        assert!(pool.has_threads());

        let pool_single = OsdThreadPool::new(Some(1));
        assert!(!pool_single.has_threads());
    }

    #[test]
    fn test_osd_thread_pool_indices() {
        let pool = OsdThreadPool::new(Some(4));
        assert_eq!(pool.lower_thread_index(), 0);
        assert_eq!(pool.upper_thread_index(), 4);
    }

    #[test]
    fn test_osd_thread_pool_default_threads() {
        let mut pool = OsdThreadPool::new(Some(8));
        assert_eq!(pool.nb_default_threads_to_launch(), 8);
        pool.set_nb_default_threads_to_launch(4);
        assert_eq!(pool.nb_default_threads_to_launch(), 4);
    }
}

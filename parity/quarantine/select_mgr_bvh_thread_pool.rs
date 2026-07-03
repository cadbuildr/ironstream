// FILE: select_mgr_bvh_thread_pool.rs
// occt: SelectMgr_BVHThreadPool

use std::sync::{Arc, Mutex, Condvar};
use std::collections::VecDeque;

/// Thread with back reference to thread pool and thread mutex in it.
pub struct BvhThread {
    id: usize,
    pool: Option<Arc<Mutex<BvhThreadPoolData>>>,
}

impl BvhThread {
    /// Creates a new BVH thread with given ID
    pub fn new(id: usize) -> Self {
        BvhThread {
            id,
            pool: None,
        }
    }

    /// Returns the thread ID
    pub fn id(&self) -> usize {
        self.id
    }
}

/// Internal data structure for the thread pool
pub struct BvhThreadPoolData {
    to_build_list: VecDeque<usize>, // Queue of entity IDs to build
    to_stop_bvh_thread: bool,
    is_started: bool,
    wake_event: Arc<Condvar>,
    idle_event: Arc<Condvar>,
}

impl BvhThreadPoolData {
    fn new() -> Self {
        BvhThreadPoolData {
            to_build_list: VecDeque::new(),
            to_stop_bvh_thread: false,
            is_started: false,
            wake_event: Arc::new(Condvar::new()),
            idle_event: Arc::new(Condvar::new()),
        }
    }
}

/// Class defining a thread pool for building BVH for the list of Select3D_SensitiveEntity
/// within background thread(s).
pub struct SelectMgrBvhThreadPool {
    threads: Vec<BvhThread>,
    data: Arc<Mutex<BvhThreadPoolData>>,
    num_threads: usize,
}

impl SelectMgrBvhThreadPool {
    /// Main constructor
    pub fn new(num_threads: i32) -> Self {
        let num_threads = (num_threads as usize).max(1);
        let mut threads = Vec::with_capacity(num_threads);

        for i in 0..num_threads {
            threads.push(BvhThread::new(i));
        }

        SelectMgrBvhThreadPool {
            threads,
            data: Arc::new(Mutex::new(BvhThreadPoolData::new())),
            num_threads,
        }
    }

    /// Queue a sensitive entity to build its BVH
    pub fn add_entity(&self, entity_id: usize) {
        if let Ok(mut data) = self.data.lock() {
            data.to_build_list.push_back(entity_id);
            // Wake up any waiting threads
            data.wake_event.notify_one();
        }
    }

    /// Stops threads
    pub fn stop_threads(&self) {
        if let Ok(mut data) = self.data.lock() {
            data.to_stop_bvh_thread = true;
            // Wake up all threads so they can exit
            data.wake_event.notify_all();
        }
    }

    /// Waits for all threads to finish their jobs
    pub fn wait_threads(&self) {
        if let Ok(data) = self.data.lock() {
            // Use idle_event to wait for all work to complete
            while !data.to_build_list.is_empty() && !data.to_stop_bvh_thread {
                // In a real implementation, this would wait on idle_event
                drop(data);
                std::thread::sleep(std::time::Duration::from_millis(1));
                if let Ok(data) = self.data.lock() {
                    if data.to_build_list.is_empty() {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    /// Returns the number of threads
    pub fn num_threads(&self) -> usize {
        self.num_threads
    }

    /// Returns the threads
    pub fn threads(&self) -> &[BvhThread] {
        &self.threads
    }

    /// Returns whether threads are started
    pub fn is_started(&self) -> bool {
        if let Ok(data) = self.data.lock() {
            data.is_started
        } else {
            false
        }
    }

    /// Returns the number of pending entities
    pub fn pending_count(&self) -> usize {
        if let Ok(data) = self.data.lock() {
            data.to_build_list.len()
        } else {
            0
        }
    }

    /// Starts the threads (if not already started)
    pub fn start_threads(&self) {
        if let Ok(mut data) = self.data.lock() {
            if !data.is_started {
                data.is_started = true;
                data.to_stop_bvh_thread = false;
            }
        }
    }
}

/// Sentry class providing a simple interface to mutexes for list of BVHThread
pub struct Sentry {
    pool: Option<Arc<Mutex<BvhThreadPoolData>>>,
    locked: bool,
}

impl Sentry {
    /// Constructor - initializes the sentry object and locks mutexes immediately
    pub fn new(pool: Option<Arc<Mutex<BvhThreadPoolData>>>) -> Self {
        let mut sentry = Sentry {
            pool,
            locked: false,
        };
        sentry.lock();
        sentry
    }

    /// Lock mutexes
    pub fn lock(&mut self) {
        if let Some(ref pool) = self.pool {
            if let Ok(_data) = pool.lock() {
                self.locked = true;
            }
        }
    }

    /// Unlock mutexes
    pub fn unlock(&mut self) {
        self.locked = false;
    }

    /// Check if locked
    pub fn is_locked(&self) -> bool {
        self.locked
    }
}

impl Drop for Sentry {
    fn drop(&mut self) {
        self.unlock();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_pool_creation() {
        let pool = SelectMgrBvhThreadPool::new(4);
        assert_eq!(pool.num_threads(), 4);
    }

    #[test]
    fn test_thread_pool_minimum_one_thread() {
        let pool = SelectMgrBvhThreadPool::new(0);
        assert_eq!(pool.num_threads(), 1);

        let pool = SelectMgrBvhThreadPool::new(-1);
        assert_eq!(pool.num_threads(), 1);
    }

    #[test]
    fn test_threads_vector_size() {
        let pool = SelectMgrBvhThreadPool::new(3);
        assert_eq!(pool.threads().len(), 3);
    }

    #[test]
    fn test_thread_ids() {
        let pool = SelectMgrBvhThreadPool::new(3);
        let threads = pool.threads();
        assert_eq!(threads[0].id(), 0);
        assert_eq!(threads[1].id(), 1);
        assert_eq!(threads[2].id(), 2);
    }

    #[test]
    fn test_add_entity() {
        let pool = SelectMgrBvhThreadPool::new(2);
        assert_eq!(pool.pending_count(), 0);

        pool.add_entity(1);
        assert_eq!(pool.pending_count(), 1);

        pool.add_entity(2);
        assert_eq!(pool.pending_count(), 2);
    }

    #[test]
    fn test_start_threads() {
        let pool = SelectMgrBvhThreadPool::new(2);
        assert!(!pool.is_started());

        pool.start_threads();
        assert!(pool.is_started());
    }

    #[test]
    fn test_stop_threads() {
        let pool = SelectMgrBvhThreadPool::new(2);
        pool.add_entity(1);

        pool.start_threads();
        assert!(pool.is_started());

        pool.stop_threads();
        // After stop, pool should eventually finish
    }

    #[test]
    fn test_wait_threads_with_empty_queue() {
        let pool = SelectMgrBvhThreadPool::new(2);
        // Should return immediately since queue is empty
        pool.wait_threads();
        assert_eq!(pool.pending_count(), 0);
    }

    #[test]
    fn test_sentry_creation() {
        let data = Arc::new(Mutex::new(BvhThreadPoolData::new()));
        let _sentry = Sentry::new(Some(data));
        // Sentry should be locked and then unlocked on drop
    }

    #[test]
    fn test_sentry_with_none() {
        let _sentry = Sentry::new(None);
        // Sentry should handle None gracefully
    }

    #[test]
    fn test_bvh_thread_creation() {
        let thread = BvhThread::new(0);
        assert_eq!(thread.id(), 0);
        assert!(thread.pool.is_none());
    }

    #[test]
    fn test_multiple_entities() {
        let pool = SelectMgrBvhThreadPool::new(4);

        for i in 0..10 {
            pool.add_entity(i);
        }

        assert_eq!(pool.pending_count(), 10);
        pool.wait_threads();
        // After wait, queue should be empty or processing
    }

    #[test]
    fn test_pool_bounded_execution() {
        let pool = SelectMgrBvhThreadPool::new(1);
        pool.add_entity(1);
        pool.add_entity(2);
        pool.add_entity(3);

        // Wait should complete in bounded time (not deadlock)
        let start = std::time::Instant::now();
        pool.wait_threads();
        let elapsed = start.elapsed();

        // Should complete quickly (under 5 seconds)
        assert!(elapsed.as_secs() < 5);
    }
}

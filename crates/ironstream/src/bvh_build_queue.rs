// FILE: bvh_build_queue.rs

//! Command-queue for parallel building of BVH nodes.
//! Uses atomics and mutexes for thread-safe queue management.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

// occt: BVH_BuildQueue
/// Command-queue for parallel building of BVH nodes.
/// Manages a work queue with atomic size tracking and busy thread counting.
pub struct BvhBuildQueue {
    queue: Mutex<VecDeque<i32>>,
    nb_threads: AtomicI32,
    size: AtomicI32,
}

impl BvhBuildQueue {
    /// Creates a new BVH build queue.
    pub fn new() -> Self {
        BvhBuildQueue {
            queue: Mutex::new(VecDeque::new()),
            nb_threads: AtomicI32::new(0),
            size: AtomicI32::new(0),
        }
    }

    /// Returns current size of BVH build queue.
    /// Uses acquire semantics to synchronize with enqueue/dequeue operations.
    pub fn size(&self) -> i32 {
        self.size.load(Ordering::Acquire)
    }

    /// Enqueues new work-item onto BVH build queue.
    pub fn enqueue(&self, work_item: i32) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(work_item);
        self.size.fetch_add(1, Ordering::Release);
    }

    /// Fetches first work-item from BVH build queue.
    /// Returns (-1, was_busy=false) if queue is empty.
    /// Updates busy thread count atomically with release/acquire semantics.
    pub fn fetch(&self, was_busy: &mut bool) -> i32 {
        let mut query = -1;

        // Fetch item from queue under lock
        {
            let mut q = self.queue.lock().unwrap();
            if !q.is_empty() {
                query = q.pop_front().unwrap();
                self.size.fetch_sub(1, Ordering::Release);
            }
        }

        // Update thread counter atomically
        if query != -1 {
            if !*was_busy {
                self.nb_threads.fetch_add(1, Ordering::Release);
            }
        } else if *was_busy {
            self.nb_threads.fetch_sub(1, Ordering::Release);
        }

        *was_busy = query != -1;
        query
    }

    /// Checks if there are active build threads.
    /// Uses acquire semantics to ensure visibility of thread counter updates.
    pub fn has_busy_threads(&self) -> bool {
        self.nb_threads.load(Ordering::Acquire) != 0
    }
}

impl Default for BvhBuildQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_default_constructor() {
        let queue = BvhBuildQueue::new();
        assert_eq!(queue.size(), 0);
        assert!(!queue.has_busy_threads());
    }

    #[test]
    fn test_enqueue_single() {
        let queue = BvhBuildQueue::new();
        queue.enqueue(42);
        assert_eq!(queue.size(), 1);
    }

    #[test]
    fn test_enqueue_multiple() {
        let queue = BvhBuildQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.size(), 3);
    }

    #[test]
    fn test_fetch_from_empty_queue() {
        let queue = BvhBuildQueue::new();
        let mut was_busy = false;
        let result = queue.fetch(&mut was_busy);

        assert_eq!(result, -1);
        assert!(!was_busy);
        assert!(!queue.has_busy_threads());
    }

    #[test]
    fn test_fetch_single_item() {
        let queue = BvhBuildQueue::new();
        queue.enqueue(42);

        let mut was_busy = false;
        let result = queue.fetch(&mut was_busy);

        assert_eq!(result, 42);
        assert!(was_busy);
        assert!(queue.has_busy_threads());
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_fetch_fifo_order() {
        let queue = BvhBuildQueue::new();

        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        let mut was_busy = false;

        assert_eq!(queue.fetch(&mut was_busy), 10);
        assert!(was_busy);

        assert_eq!(queue.fetch(&mut was_busy), 20);
        assert!(was_busy);

        assert_eq!(queue.fetch(&mut was_busy), 30);
        assert!(was_busy);

        assert_eq!(queue.fetch(&mut was_busy), -1);
        assert!(!was_busy);
    }

    #[test]
    fn test_thread_count_tracking() {
        let queue = BvhBuildQueue::new();

        queue.enqueue(1);
        queue.enqueue(2);

        assert!(!queue.has_busy_threads());

        let mut was_busy = false;

        // First fetch - thread becomes busy
        queue.fetch(&mut was_busy);
        assert!(was_busy);
        assert!(queue.has_busy_threads());

        // Second fetch while already busy
        queue.fetch(&mut was_busy);
        assert!(was_busy);
        assert!(queue.has_busy_threads());

        // Fetch from empty queue while busy
        queue.fetch(&mut was_busy);
        assert!(!was_busy);
        assert!(!queue.has_busy_threads());
    }

    #[test]
    fn test_alternating_enqueue_fetch() {
        let queue = BvhBuildQueue::new();

        queue.enqueue(1);
        assert_eq!(queue.size(), 1);

        let mut was_busy = false;
        assert_eq!(queue.fetch(&mut was_busy), 1);
        assert_eq!(queue.size(), 0);

        queue.enqueue(2);
        assert_eq!(queue.size(), 1);

        assert_eq!(queue.fetch(&mut was_busy), 2);
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_large_queue() {
        let queue = BvhBuildQueue::new();

        const COUNT: i32 = 1000;
        for i in 0..COUNT {
            queue.enqueue(i);
        }

        assert_eq!(queue.size(), COUNT);

        let mut was_busy = false;
        for i in 0..COUNT {
            assert_eq!(queue.fetch(&mut was_busy), i);
        }

        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_negative_values() {
        let queue = BvhBuildQueue::new();

        queue.enqueue(-1);
        queue.enqueue(-100);
        queue.enqueue(0);

        let mut was_busy = false;

        assert_eq!(queue.fetch(&mut was_busy), -1);
        assert_eq!(queue.fetch(&mut was_busy), -100);
        assert_eq!(queue.fetch(&mut was_busy), 0);
    }

    #[test]
    fn test_concurrent_enqueue() {
        let queue = Arc::new(BvhBuildQueue::new());

        const THREAD_COUNT: usize = 4;
        const ITEMS_PER_THREAD: i32 = 100;
        let mut handles = vec![];

        for t in 0..THREAD_COUNT {
            let q = Arc::clone(&queue);
            let handle = thread::spawn(move || {
                for i in 0..ITEMS_PER_THREAD {
                    q.enqueue((t as i32) * ITEMS_PER_THREAD + i);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(queue.size(), (THREAD_COUNT as i32) * ITEMS_PER_THREAD);
    }

    #[test]
    fn test_concurrent_fetch() {
        let queue = Arc::new(BvhBuildQueue::new());

        const ITEM_COUNT: i32 = 400;
        for i in 0..ITEM_COUNT {
            queue.enqueue(i);
        }

        const THREAD_COUNT: usize = 4;
        let fetched_counts = Arc::new(Mutex::new(vec![0; THREAD_COUNT]));
        let mut handles = vec![];

        for t in 0..THREAD_COUNT {
            let q = Arc::clone(&queue);
            let counts = Arc::clone(&fetched_counts);
            let handle = thread::spawn(move || {
                let mut was_busy = false;
                loop {
                    let item = q.fetch(&mut was_busy);
                    if item == -1 {
                        break;
                    }
                    counts.lock().unwrap()[t] += 1;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let total_fetched: usize = fetched_counts.lock().unwrap().iter().sum();
        assert_eq!(total_fetched, ITEM_COUNT as usize);
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_concurrent_enqueue_and_fetch() {
        let queue = Arc::new(BvhBuildQueue::new());

        const PRODUCER_COUNT: usize = 2;
        const CONSUMER_COUNT: usize = 2;
        const ITEMS_PER_PRODUCER: i32 = 100;

        let mut handles = vec![];
        let fetched_count = Arc::new(AtomicI32::new(0));
        let done = Arc::new(AtomicI32::new(0));

        // Producer threads
        for t in 0..PRODUCER_COUNT {
            let q = Arc::clone(&queue);
            let handle = thread::spawn(move || {
                for i in 0..ITEMS_PER_PRODUCER {
                    q.enqueue((t as i32) * ITEMS_PER_PRODUCER + i);
                }
            });
            handles.push(handle);
        }

        // Consumer threads
        for _ in 0..CONSUMER_COUNT {
            let q = Arc::clone(&queue);
            let count = Arc::clone(&fetched_count);
            let d = Arc::clone(&done);
            let handle = thread::spawn(move || {
                let mut was_busy = false;
                loop {
                    let item = q.fetch(&mut was_busy);
                    if item != -1 {
                        count.fetch_add(1, Ordering::Relaxed);
                    } else if d.load(Ordering::Relaxed) != 0 {
                        break;
                    }
                    // Prevent tight loop on empty queue
                    thread::sleep(std::time::Duration::from_micros(1));
                }
            });
            handles.push(handle);
        }

        // Wait for producers to finish
        for handle in handles.drain(0..PRODUCER_COUNT) {
            handle.join().unwrap();
        }

        // Signal consumers that production is done
        done.store(1, Ordering::Release);

        // Wait for consumers to finish
        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(
            fetched_count.load(Ordering::Relaxed),
            (PRODUCER_COUNT as i32) * ITEMS_PER_PRODUCER
        );
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_repeated_fetch_from_empty() {
        let queue = BvhBuildQueue::new();
        let mut was_busy = false;

        for _ in 0..10 {
            let result = queue.fetch(&mut was_busy);
            assert_eq!(result, -1);
            assert!(!was_busy);
        }

        assert!(!queue.has_busy_threads());
    }

    #[test]
    fn test_enqueue_zero() {
        let queue = BvhBuildQueue::new();

        queue.enqueue(0);
        assert_eq!(queue.size(), 1);

        let mut was_busy = false;
        assert_eq!(queue.fetch(&mut was_busy), 0);
        assert!(was_busy);
    }

    #[test]
    fn test_duplicate_values() {
        let queue = BvhBuildQueue::new();

        queue.enqueue(5);
        queue.enqueue(5);
        queue.enqueue(5);

        assert_eq!(queue.size(), 3);

        let mut was_busy = false;
        assert_eq!(queue.fetch(&mut was_busy), 5);
        assert_eq!(queue.fetch(&mut was_busy), 5);
        assert_eq!(queue.fetch(&mut was_busy), 5);
    }

    #[test]
    fn test_single_thread_workflow() {
        let queue = BvhBuildQueue::new();

        // Simulate single-threaded BVH building workflow
        queue.enqueue(0); // Root node

        let mut was_busy = false;
        let mut node = queue.fetch(&mut was_busy);

        assert_eq!(node, 0);
        assert!(was_busy);

        // After processing root, enqueue children
        queue.enqueue(1); // Left child
        queue.enqueue(2); // Right child

        assert_eq!(queue.size(), 2);

        node = queue.fetch(&mut was_busy);
        assert_eq!(node, 1);

        node = queue.fetch(&mut was_busy);
        assert_eq!(node, 2);

        node = queue.fetch(&mut was_busy);
        assert_eq!(node, -1);
        assert!(!was_busy);
    }
}

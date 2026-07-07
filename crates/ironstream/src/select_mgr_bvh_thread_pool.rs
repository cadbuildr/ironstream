// FILE: select_mgr_bvh_thread_pool.rs
// occt: SelectMgr_BVHThreadPool

//! Thread pool for building BVH of sensitive entities within background
//! threads, following SelectMgr_BVHThreadPool.hxx/.cxx.
//!
//! External plumbing is modelled locally: Standard_Condition becomes a
//! manual-reset [`Event`], Select3D_SensitiveEntity becomes the
//! [`SensitiveEntity`] trait, and Message::DefaultMessenger becomes a
//! per-pool message list. The pool behaviour itself — wake/idle events,
//! per-thread BVH mutexes, the worker loop, StopThreads/WaitThreads — is
//! real and mirrors the C++ source.

use std::collections::VecDeque;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;

// ---------------------------------------------------------------------------
// Manual-reset event (models Standard_Condition)
// ---------------------------------------------------------------------------

/// Manual-reset event: `wait` blocks until the event is set; the event stays
/// set until explicitly reset.
pub struct Event {
    state: Mutex<bool>,
    cond: Condvar,
}

impl Event {
    pub fn new(initial: bool) -> Self {
        Self { state: Mutex::new(initial), cond: Condvar::new() }
    }

    /// Sets the event, releasing all waiters.
    pub fn set(&self) {
        let mut st = self.state.lock().unwrap();
        *st = true;
        self.cond.notify_all();
    }

    /// Resets the event so subsequent waits block.
    pub fn reset(&self) {
        let mut st = self.state.lock().unwrap();
        *st = false;
    }

    /// Blocks until the event is set.
    pub fn wait(&self) {
        let mut st = self.state.lock().unwrap();
        while !*st {
            st = self.cond.wait(st).unwrap();
        }
    }

    /// Returns the current state without blocking.
    pub fn is_set(&self) -> bool {
        *self.state.lock().unwrap()
    }
}

// ---------------------------------------------------------------------------
// Sensitive entity (models Select3D_SensitiveEntity)
// ---------------------------------------------------------------------------

/// The slice of Select3D_SensitiveEntity the pool interacts with.
pub trait SensitiveEntity: Send + Sync {
    /// Whether the BVH still needs to be built (ToBuildBVH).
    fn to_build_bvh(&self) -> bool;

    /// Builds the BVH (BVH()). May panic; the pool catches panics the way
    /// OCCT catches exceptions.
    fn bvh(&self);
}

// ---------------------------------------------------------------------------
// Shared pool state
// ---------------------------------------------------------------------------

struct PoolShared {
    /// myBVHToBuildList (guarded by myBVHListMutex).
    to_build_list: Mutex<VecDeque<Arc<dyn SensitiveEntity>>>,
    /// myToStopBVHThread.
    to_stop: AtomicBool,
    /// myWakeEvent: raised when any sensitive is added to the list.
    wake_event: Event,
    /// myIdleEvent: raised when the list becomes empty.
    idle_event: Event,
    /// Per-thread BVH mutexes (BVHThread::myMutex), held while building.
    bvh_mutexes: Vec<Mutex<()>>,
    /// Local stand-in for Message::DefaultMessenger()->SendFail.
    fail_messages: Mutex<Vec<String>>,
}

// ---------------------------------------------------------------------------
// The pool itself (SelectMgr_BVHThreadPool)
// ---------------------------------------------------------------------------

/// Thread pool for building BVH of sensitive entities in background threads.
pub struct SelectMgrBvhThreadPool {
    shared: Arc<PoolShared>,
    handles: Mutex<Vec<JoinHandle<()>>>,
    is_started: AtomicBool,
    num_threads: usize,
}

impl SelectMgrBvhThreadPool {
    /// Main constructor: at least one thread is allocated.
    pub fn new(nb_threads: i32) -> Self {
        let num_threads = std::cmp::max(1, nb_threads) as usize;
        let bvh_mutexes = (0..num_threads).map(|_| Mutex::new(())).collect();
        Self {
            shared: Arc::new(PoolShared {
                to_build_list: Mutex::new(VecDeque::new()),
                to_stop: AtomicBool::new(false),
                wake_event: Event::new(false),
                idle_event: Event::new(true),
                bvh_mutexes,
                fail_messages: Mutex::new(Vec::new()),
            }),
            handles: Mutex::new(Vec::new()),
            is_started: AtomicBool::new(false),
            num_threads,
        }
    }

    /// Number of worker threads.
    pub fn nb_threads(&self) -> usize {
        self.num_threads
    }

    /// Whether the worker threads are running.
    pub fn is_started(&self) -> bool {
        self.is_started.load(Ordering::SeqCst)
    }

    /// Number of queued (not yet started) build requests.
    pub fn pending_count(&self) -> usize {
        self.shared.to_build_list.lock().unwrap().len()
    }

    /// Messages recorded for failed builds.
    pub fn fail_messages(&self) -> Vec<String> {
        self.shared.fail_messages.lock().unwrap().clone()
    }

    /// Queues a sensitive entity to build its BVH and starts the threads on
    /// first use. Mirrors AddEntity.
    pub fn add_entity(&self, entity: Arc<dyn SensitiveEntity>) {
        if !entity.to_build_bvh() {
            return;
        }
        {
            let mut list = self.shared.to_build_list.lock().unwrap();
            list.push_back(entity);
            self.shared.wake_event.set();
            self.shared.idle_event.reset();
        }
        if !self.is_started.swap(true, Ordering::SeqCst) {
            let mut handles = self.handles.lock().unwrap();
            for i in 0..self.num_threads {
                let shared = Arc::clone(&self.shared);
                handles.push(std::thread::spawn(move || perform_thread(&shared, i)));
            }
        }
    }

    /// Stops threads. Mirrors StopThreads.
    pub fn stop_threads(&self) {
        if !self.is_started.load(Ordering::SeqCst) {
            return;
        }
        self.shared.to_stop.store(true, Ordering::SeqCst);
        self.shared.wake_event.set();
        let handles: Vec<JoinHandle<()>> =
            std::mem::take(&mut *self.handles.lock().unwrap());
        for h in handles {
            let _ = h.join();
        }
        self.shared.to_stop.store(false, Ordering::SeqCst);
        self.is_started.store(false, Ordering::SeqCst);
    }

    /// Waits for all threads to finish their current jobs: waits for the
    /// idle event, then acquires every thread's BVH mutex (Sentry).
    /// Mirrors WaitThreads.
    pub fn wait_threads(&self) {
        self.shared.idle_event.wait();
        let _sentry = Sentry::new(Some(self));
    }
}

impl Drop for SelectMgrBvhThreadPool {
    fn drop(&mut self) {
        self.stop_threads();
    }
}

/// Worker loop. Mirrors BVHThread::performThread.
fn perform_thread(shared: &Arc<PoolShared>, thread_index: usize) {
    loop {
        shared.wake_event.wait();
        if shared.to_stop.load(Ordering::SeqCst) {
            return;
        }
        let entity = {
            let mut list = shared.to_build_list.lock().unwrap();
            match list.pop_front() {
                Some(e) => e,
                None => {
                    shared.wake_event.reset();
                    shared.idle_event.set();
                    continue;
                }
            }
        };
        // Hold this thread's BVH mutex while building (BVHThread::myMutex).
        let _thread_lock = shared.bvh_mutexes[thread_index].lock().unwrap();
        // OCC_CATCH_SIGNALS / try-catch: a failing build must not kill the
        // worker; the failure is reported to the messenger.
        if let Err(payload) = catch_unwind(AssertUnwindSafe(|| entity.bvh())) {
            let msg = payload
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "Error: Unknown exception".to_string());
            shared.fail_messages.lock().unwrap().push(msg);
        }
    }
}

// ---------------------------------------------------------------------------
// Sentry (SelectMgr_BVHThreadPool::Sentry)
// ---------------------------------------------------------------------------

/// Simple RAII interface to the mutexes of all BVH threads: locks them all
/// on construction, unlocks on drop.
pub struct Sentry<'a> {
    pool: Option<&'a SelectMgrBvhThreadPool>,
    guards: Vec<std::sync::MutexGuard<'a, ()>>,
}

impl<'a> Sentry<'a> {
    pub fn new(pool: Option<&'a SelectMgrBvhThreadPool>) -> Self {
        let mut sentry = Sentry { pool, guards: Vec::new() };
        sentry.lock();
        sentry
    }

    /// Locks the BVH mutexes of every thread in the pool.
    pub fn lock(&mut self) {
        if let Some(pool) = self.pool {
            if self.guards.is_empty() {
                for m in &pool.shared.bvh_mutexes {
                    self.guards.push(m.lock().unwrap());
                }
            }
        }
    }

    /// Unlocks the mutexes.
    pub fn unlock(&mut self) {
        self.guards.clear();
    }

    pub fn is_locked(&self) -> bool {
        !self.guards.is_empty()
    }
}

impl<'a> Drop for Sentry<'a> {
    fn drop(&mut self) {
        self.unlock();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    /// Test entity counting how many times its BVH got built.
    struct CountingEntity {
        needs_build: bool,
        built: AtomicUsize,
    }

    impl CountingEntity {
        fn new(needs_build: bool) -> Arc<Self> {
            Arc::new(Self { needs_build, built: AtomicUsize::new(0) })
        }

        fn built_count(&self) -> usize {
            self.built.load(Ordering::SeqCst)
        }
    }

    impl SensitiveEntity for CountingEntity {
        fn to_build_bvh(&self) -> bool {
            self.needs_build
        }

        fn bvh(&self) {
            self.built.fetch_add(1, Ordering::SeqCst);
        }
    }

    /// Entity whose build panics (models a Standard_Failure inside BVH()).
    struct FailingEntity;

    impl SensitiveEntity for FailingEntity {
        fn to_build_bvh(&self) -> bool {
            true
        }

        fn bvh(&self) {
            panic!("BVH build failed");
        }
    }

    #[test]
    fn test_pool_creation() {
        let pool = SelectMgrBvhThreadPool::new(4);
        assert_eq!(pool.nb_threads(), 4);
        assert!(!pool.is_started());
        assert_eq!(pool.pending_count(), 0);
    }

    #[test]
    fn test_minimum_one_thread() {
        assert_eq!(SelectMgrBvhThreadPool::new(0).nb_threads(), 1);
        assert_eq!(SelectMgrBvhThreadPool::new(-3).nb_threads(), 1);
    }

    #[test]
    fn test_add_entity_builds_bvh() {
        let pool = SelectMgrBvhThreadPool::new(2);
        let entity = CountingEntity::new(true);
        pool.add_entity(entity.clone());
        assert!(pool.is_started());
        pool.wait_threads();
        assert_eq!(entity.built_count(), 1);
        pool.stop_threads();
        assert!(!pool.is_started());
    }

    #[test]
    fn test_entity_not_needing_build_is_skipped() {
        let pool = SelectMgrBvhThreadPool::new(1);
        let entity = CountingEntity::new(false);
        pool.add_entity(entity.clone());
        // Entity was refused before starting anything.
        assert!(!pool.is_started());
        assert_eq!(pool.pending_count(), 0);
        assert_eq!(entity.built_count(), 0);
    }

    #[test]
    fn test_many_entities_all_built() {
        let pool = SelectMgrBvhThreadPool::new(3);
        let entities: Vec<_> = (0..20).map(|_| CountingEntity::new(true)).collect();
        for e in &entities {
            pool.add_entity(e.clone());
        }
        pool.wait_threads();
        for e in &entities {
            assert_eq!(e.built_count(), 1);
        }
        assert_eq!(pool.pending_count(), 0);
        pool.stop_threads();
    }

    #[test]
    fn test_failing_build_does_not_kill_worker() {
        let pool = SelectMgrBvhThreadPool::new(1);
        let good = CountingEntity::new(true);
        pool.add_entity(Arc::new(FailingEntity));
        pool.add_entity(good.clone());
        pool.wait_threads();
        assert_eq!(good.built_count(), 1);
        let messages = pool.fail_messages();
        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("BVH build failed"));
        pool.stop_threads();
    }

    #[test]
    fn test_wait_threads_on_idle_pool_returns() {
        let pool = SelectMgrBvhThreadPool::new(2);
        // Idle event starts raised: waiting on a fresh pool must not block.
        pool.wait_threads();
        assert_eq!(pool.pending_count(), 0);
    }

    #[test]
    fn test_stop_threads_idempotent() {
        let pool = SelectMgrBvhThreadPool::new(2);
        pool.stop_threads(); // not started: no-op
        assert!(!pool.is_started());

        let entity = CountingEntity::new(true);
        pool.add_entity(entity.clone());
        pool.wait_threads();
        pool.stop_threads();
        pool.stop_threads(); // second stop is a no-op
        assert!(!pool.is_started());
        assert_eq!(entity.built_count(), 1);
    }

    #[test]
    fn test_restart_after_stop() {
        let pool = SelectMgrBvhThreadPool::new(2);
        let first = CountingEntity::new(true);
        pool.add_entity(first.clone());
        pool.wait_threads();
        pool.stop_threads();

        let second = CountingEntity::new(true);
        pool.add_entity(second.clone());
        assert!(pool.is_started());
        pool.wait_threads();
        assert_eq!(second.built_count(), 1);
        pool.stop_threads();
    }

    #[test]
    fn test_sentry_lock_unlock() {
        let pool = SelectMgrBvhThreadPool::new(2);
        {
            let sentry = Sentry::new(Some(&pool));
            assert!(sentry.is_locked());
        }
        // Mutexes released on drop: a second sentry can lock again.
        let mut sentry = Sentry::new(Some(&pool));
        assert!(sentry.is_locked());
        sentry.unlock();
        assert!(!sentry.is_locked());
    }

    #[test]
    fn test_sentry_with_null_pool() {
        let sentry = Sentry::new(None);
        assert!(!sentry.is_locked());
    }

    #[test]
    fn test_event_semantics() {
        let ev = Event::new(false);
        assert!(!ev.is_set());
        ev.set();
        assert!(ev.is_set());
        ev.wait(); // set: returns immediately
        ev.reset();
        assert!(!ev.is_set());
    }
}

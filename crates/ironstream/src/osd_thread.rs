// FILE: osd_thread.rs
// occt: OSD_Thread

use std::thread;

/// Thread wrapper.
pub struct Thread {
    thread_id: thread::ThreadId,
}

impl Thread {
    pub fn current() -> Self {
        Self {
            thread_id: thread::current().id(),
        }
    }

    pub fn thread_id(&self) -> thread::ThreadId {
        self.thread_id
    }
}

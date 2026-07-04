// FILE: osd_thread_function.rs
// occt: OSD_ThreadFunction

/// Thread function wrapper.
pub type ThreadFunction = fn() -> i32;

pub struct ThreadFunctionWrapper {
    func: Option<ThreadFunction>,
}

impl ThreadFunctionWrapper {
    pub fn new(func: ThreadFunction) -> Self {
        Self { func: Some(func) }
    }

    pub fn call(&self) -> i32 {
        if let Some(f) = self.func {
            f()
        } else {
            0
        }
    }
}

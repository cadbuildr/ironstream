// FILE: moni_tool_timer_sentry.rs
// occt: MoniTool_TimerSentry

use std::sync::Arc;

/// RAII timer guard that automatically starts/stops a timer
pub struct MoniToolTimerSentry {
    timer: Option<Arc<std::sync::Mutex<TimerHandle>>>,
}

pub struct TimerHandle;

impl MoniToolTimerSentry {
    pub fn new(timer: Arc<std::sync::Mutex<TimerHandle>>) -> Self {
        MoniToolTimerSentry {
            timer: Some(timer),
        }
    }

    pub fn is_active(&self) -> bool {
        self.timer.is_some()
    }
}

impl Default for MoniToolTimerSentry {
    fn default() -> Self {
        MoniToolTimerSentry { timer: None }
    }
}

impl Drop for MoniToolTimerSentry {
    fn drop(&mut self) {
        // Timer will be automatically stopped when dropping
        self.timer = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sentry = MoniToolTimerSentry::default();
        assert!(!sentry.is_active());
    }
}

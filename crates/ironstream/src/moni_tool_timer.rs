// FILE: moni_tool_timer.rs
// occt: MoniTool_Timer

use std::time::{Duration, Instant};

/// Timer for measuring elapsed time
pub struct MoniToolTimer {
    name: String,
    start_time: Option<Instant>,
    total_time: Duration,
    is_running: bool,
}

impl MoniToolTimer {
    pub fn new(name: &str) -> Self {
        MoniToolTimer {
            name: name.to_string(),
            start_time: None,
            total_time: Duration::new(0, 0),
            is_running: false,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn start(&mut self) {
        if !self.is_running {
            self.start_time = Some(Instant::now());
            self.is_running = true;
        }
    }

    pub fn stop(&mut self) {
        if self.is_running {
            if let Some(start) = self.start_time {
                self.total_time += start.elapsed();
            }
            self.is_running = false;
            self.start_time = None;
        }
    }

    pub fn reset(&mut self) {
        self.stop();
        self.total_time = Duration::new(0, 0);
    }

    pub fn total_seconds(&self) -> f64 {
        self.total_time.as_secs_f64()
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }
}

impl Default for MoniToolTimer {
    fn default() -> Self {
        MoniToolTimer {
            name: String::new(),
            start_time: None,
            total_time: Duration::new(0, 0),
            is_running: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_create() {
        let timer = MoniToolTimer::new("test");
        assert_eq!(timer.name(), "test");
        assert!(!timer.is_running());
    }

    #[test]
    fn test_start_stop() {
        let mut timer = MoniToolTimer::new("test");
        timer.start();
        assert!(timer.is_running());
        timer.stop();
        assert!(!timer.is_running());
    }

    #[test]
    fn test_measure() {
        let mut timer = MoniToolTimer::new("test");
        timer.start();
        thread::sleep(Duration::from_millis(10));
        timer.stop();
        assert!(timer.total_seconds() > 0.0);
    }
}

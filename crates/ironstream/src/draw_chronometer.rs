// FILE: draw_chronometer.rs
// occt: Draw_Chronometer

//! A simple chronometer for timing operations.

use std::time::{Instant, Duration};

/// A chronometer for measuring elapsed time.
#[derive(Clone, Debug)]
pub struct DrawChronometer {
    start_time: Option<Instant>,
    accumulated_time: Duration,
    is_running: bool,
}

impl DrawChronometer {
    /// Create a new chronometer.
    pub fn new() -> Self {
        Self {
            start_time: None,
            accumulated_time: Duration::ZERO,
            is_running: false,
        }
    }

    /// Start the chronometer.
    pub fn start(&mut self) {
        if !self.is_running {
            self.start_time = Some(Instant::now());
            self.is_running = true;
        }
    }

    /// Stop the chronometer and accumulate the elapsed time.
    pub fn stop(&mut self) {
        if self.is_running {
            if let Some(start) = self.start_time {
                self.accumulated_time += start.elapsed();
            }
            self.start_time = None;
            self.is_running = false;
        }
    }

    /// Reset the chronometer.
    pub fn reset(&mut self) {
        self.accumulated_time = Duration::ZERO;
        self.start_time = None;
        self.is_running = false;
    }

    /// Get the elapsed time in seconds.
    pub fn elapsed_seconds(&self) -> f64 {
        let mut total = self.accumulated_time;
        if self.is_running {
            if let Some(start) = self.start_time {
                total += start.elapsed();
            }
        }
        total.as_secs_f64()
    }

    /// Get the elapsed time in milliseconds.
    pub fn elapsed_millis(&self) -> u128 {
        let mut total = self.accumulated_time;
        if self.is_running {
            if let Some(start) = self.start_time {
                total += start.elapsed();
            }
        }
        total.as_millis()
    }

    /// Check if the chronometer is running.
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Show the accumulated time (for printing).
    pub fn show(&self) -> String {
        let elapsed_sec = self.elapsed_seconds();
        format!("{:.6} seconds", elapsed_sec)
    }
}

impl Default for DrawChronometer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let chrono = DrawChronometer::new();
        assert!(!chrono.is_running());
        assert_eq!(chrono.elapsed_seconds(), 0.0);
    }

    #[test]
    fn test_start_stop() {
        let mut chrono = DrawChronometer::new();

        chrono.start();
        assert!(chrono.is_running());

        chrono.stop();
        assert!(!chrono.is_running());
    }

    #[test]
    fn test_elapsed_time() {
        let mut chrono = DrawChronometer::new();

        chrono.start();
        std::thread::sleep(Duration::from_millis(10));
        chrono.stop();

        let elapsed = chrono.elapsed_seconds();
        assert!(elapsed >= 0.01);
        assert!(elapsed < 1.0);
    }

    #[test]
    fn test_reset() {
        let mut chrono = DrawChronometer::new();

        chrono.start();
        std::thread::sleep(Duration::from_millis(5));
        chrono.stop();

        assert!(chrono.elapsed_seconds() > 0.0);

        chrono.reset();
        assert_eq!(chrono.elapsed_seconds(), 0.0);
    }

    #[test]
    fn test_accumulation() {
        let mut chrono = DrawChronometer::new();

        chrono.start();
        std::thread::sleep(Duration::from_millis(5));
        chrono.stop();

        let first_elapsed = chrono.elapsed_seconds();

        chrono.start();
        std::thread::sleep(Duration::from_millis(5));
        chrono.stop();

        let total_elapsed = chrono.elapsed_seconds();
        assert!(total_elapsed > first_elapsed);
    }

    #[test]
    fn test_show() {
        let mut chrono = DrawChronometer::new();
        chrono.start();
        std::thread::sleep(Duration::from_millis(1));
        chrono.stop();

        let output = chrono.show();
        assert!(output.contains("seconds"));
    }
}

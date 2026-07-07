// FILE: osd_perf_meter.rs
// occt: OSD_PerfMeter

use std::time::{Instant, Duration};

/// Performance meter for timing measurements.
pub struct PerfMeter {
    name: String,
    start: Option<Instant>,
    total_time: Duration,
    count: u64,
}

impl PerfMeter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            start: None,
            total_time: Duration::ZERO,
            count: 0,
        }
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        if let Some(start_time) = self.start {
            self.total_time += start_time.elapsed();
            self.count += 1;
            self.start = None;
        }
    }

    pub fn total_time(&self) -> Duration {
        self.total_time
    }

    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn average_time(&self) -> Duration {
        if self.count > 0 {
            self.total_time / self.count as u32
        } else {
            Duration::ZERO
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perf_meter() {
        let mut meter = PerfMeter::new("test");
        meter.start();
        meter.stop();
        assert_eq!(meter.count(), 1);
        assert!(meter.total_time() >= Duration::ZERO);
    }
}

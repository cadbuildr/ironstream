// FILE: osd_sys.rs
// occt: OSD_Timer, OSD_Chronometer, OSD_MemInfo

/// A stopwatch-style chronometer.
/// occt: OSD_Chronometer
#[derive(Clone, Debug)]
pub struct OsdChronometer {
    pub is_started: bool,
    elapsed_user: f64,    // seconds accumulated before last start
    elapsed_system: f64,
    start_user: f64,      // monotonic start tick (stub: counter)
    tick: f64,            // pseudo-monotonic tick (incremented manually in tests)
}

impl Default for OsdChronometer {
    fn default() -> Self {
        Self { is_started: false, elapsed_user: 0.0, elapsed_system: 0.0, start_user: 0.0, tick: 0.0 }
    }
}

impl OsdChronometer {
    pub fn new() -> Self { Self::default() }

    pub fn start(&mut self) {
        if !self.is_started {
            self.start_user = self.tick;
            self.is_started = true;
        }
    }

    pub fn stop(&mut self) {
        if self.is_started {
            self.elapsed_user += self.tick - self.start_user;
            self.is_started = false;
        }
    }

    pub fn reset(&mut self) {
        self.is_started = false;
        self.elapsed_user = 0.0;
        self.elapsed_system = 0.0;
        self.start_user = 0.0;
    }

    /// Advance the internal tick (test helper; real impl uses system clock).
    pub fn advance_tick(&mut self, seconds: f64) {
        self.tick += seconds;
        if self.is_started {
            // elapsed is updated on stop(); tick accumulates running time
        }
    }

    pub fn elapsed_time(&self) -> f64 {
        if self.is_started {
            self.elapsed_user + (self.tick - self.start_user)
        } else {
            self.elapsed_user
        }
    }

    pub fn user_time(&self) -> f64 { self.elapsed_user }
    pub fn system_time(&self) -> f64 { self.elapsed_system }
}

/// Wall-clock timer (extends Chronometer with wall time).
/// occt: OSD_Timer
#[derive(Clone, Debug)]
pub struct OsdTimer {
    pub chrono: OsdChronometer,
    wall_elapsed: f64,
    wall_start: f64,
}

impl Default for OsdTimer {
    fn default() -> Self {
        Self { chrono: OsdChronometer::default(), wall_elapsed: 0.0, wall_start: 0.0 }
    }
}

impl OsdTimer {
    pub fn new() -> Self { Self::default() }

    pub fn start(&mut self) {
        self.wall_start = self.chrono.tick;
        self.chrono.start();
    }

    pub fn stop(&mut self) {
        if self.chrono.is_started {
            self.wall_elapsed += self.chrono.tick - self.wall_start;
            self.chrono.stop();
        }
    }

    pub fn reset(&mut self) {
        self.chrono.reset();
        self.wall_elapsed = 0.0;
        self.wall_start = 0.0;
    }

    pub fn restart(&mut self) { self.reset(); self.start(); }

    pub fn advance_tick(&mut self, seconds: f64) { self.chrono.advance_tick(seconds); }

    pub fn elapsed_time(&self) -> f64 { self.chrono.elapsed_time() }

    pub fn wall_clock_time(&self) -> f64 {
        if self.chrono.is_started {
            self.wall_elapsed + (self.chrono.tick - self.wall_start)
        } else {
            self.wall_elapsed
        }
    }

    pub fn is_started(&self) -> bool { self.chrono.is_started }

    pub fn show(&self) -> String {
        format!("{:.3}s (wall: {:.3}s)", self.elapsed_time(), self.wall_clock_time())
    }
}

/// Memory information (stub: fixed values).
/// occt: OSD_MemInfo
#[derive(Clone, Debug)]
pub struct OsdMemInfo {
    pub virtual_mem_mb: f64,
    pub working_set_mb: f64,
    pub peak_working_set_mb: f64,
    pub swap_usage_mb: f64,
}

impl Default for OsdMemInfo {
    fn default() -> Self {
        Self { virtual_mem_mb: 256.0, working_set_mb: 128.0, peak_working_set_mb: 128.0, swap_usage_mb: 0.0 }
    }
}

impl OsdMemInfo {
    pub fn new() -> Self { Self::default() }

    /// Update (stub: just returns the defaults).
    pub fn update(&mut self) { /* stub: no real OS query */ }

    pub fn virtual_mem(&self) -> f64 { self.virtual_mem_mb }
    pub fn working_set(&self) -> f64 { self.working_set_mb }
    pub fn peak_working_set(&self) -> f64 { self.peak_working_set_mb }

    pub fn to_string(&self) -> String {
        format!("Virtual={:.1}MB WorkSet={:.1}MB Swap={:.1}MB",
            self.virtual_mem_mb, self.working_set_mb, self.swap_usage_mb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chrono_start_stop() {
        let mut c = OsdChronometer::new();
        c.start();
        c.advance_tick(1.5);
        c.stop();
        assert!((c.elapsed_time() - 1.5).abs() < 1e-10);
        assert!(!c.is_started);
    }

    #[test]
    fn chrono_reset() {
        let mut c = OsdChronometer::new();
        c.start();
        c.advance_tick(2.0);
        c.stop();
        c.reset();
        assert!((c.elapsed_time()).abs() < 1e-10);
    }

    #[test]
    fn timer_wall_clock() {
        let mut t = OsdTimer::new();
        t.start();
        assert!(t.is_started());
        t.advance_tick(3.0);
        t.stop();
        assert!((t.elapsed_time() - 3.0).abs() < 1e-10);
        assert!((t.wall_clock_time() - 3.0).abs() < 1e-10);
        assert!(!t.is_started());
    }

    #[test]
    fn timer_show() {
        let mut t = OsdTimer::new();
        t.start();
        t.advance_tick(0.5);
        t.stop();
        let s = t.show();
        assert!(s.contains("0.500s"));
    }

    #[test]
    fn mem_info_defaults() {
        let m = OsdMemInfo::new();
        assert!(m.virtual_mem() > 0.0);
        assert!(m.working_set() > 0.0);
        let s = m.to_string();
        assert!(s.contains("WorkSet"));
    }
}

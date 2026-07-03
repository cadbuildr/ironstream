// FILE: media_timer.rs
// occt: Media_Timer

//! Media playback timer.
#[derive(Clone, Debug, Default)]
pub struct Timer {
    elapsed_ms: u64,
    is_running: bool,
}

impl Timer {
    pub fn new() -> Self { Self { elapsed_ms: 0, is_running: false } }
    pub fn elapsed_ms(&self) -> u64 { self.elapsed_ms }
    pub fn is_running(&self) -> bool { self.is_running }
    pub fn start(&mut self) { self.is_running = true; }
    pub fn stop(&mut self) { self.is_running = false; }
    pub fn reset(&mut self) { self.elapsed_ms = 0; }
    pub fn set_elapsed(&mut self, ms: u64) { self.elapsed_ms = ms; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = Timer::new();
        assert_eq!(t.elapsed_ms(), 0);
    }
}

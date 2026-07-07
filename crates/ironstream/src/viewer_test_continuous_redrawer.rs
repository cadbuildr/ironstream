// FILE: viewer_test_continuous_redrawer.rs
// occt: ViewerTest_ContinuousRedrawer

#[derive(Clone, Debug)]
pub struct ViewerTestContinuousRedrawer {
    is_active: bool,
    frame_count: u32,
}

impl ViewerTestContinuousRedrawer {
    pub fn new() -> Self {
        ViewerTestContinuousRedrawer {
            is_active: false,
            frame_count: 0,
        }
    }

    pub fn start(&mut self) {
        self.is_active = true;
        self.frame_count = 0;
    }

    pub fn stop(&mut self) {
        self.is_active = false;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }

    pub fn next_frame(&mut self) {
        if self.is_active {
            self.frame_count += 1;
        }
    }
}

impl Default for ViewerTestContinuousRedrawer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let r = ViewerTestContinuousRedrawer::new();
        assert!(!r.is_active());
        assert_eq!(r.frame_count(), 0);
    }

    #[test]
    fn test_start_stop() {
        let mut r = ViewerTestContinuousRedrawer::new();
        assert!(!r.is_active());
        r.start();
        assert!(r.is_active());
        r.stop();
        assert!(!r.is_active());
    }

    #[test]
    fn test_frame_count() {
        let mut r = ViewerTestContinuousRedrawer::new();
        r.start();
        r.next_frame();
        r.next_frame();
        assert_eq!(r.frame_count(), 2);
    }

    #[test]
    fn test_frame_count_inactive() {
        let mut r = ViewerTestContinuousRedrawer::new();
        r.next_frame();
        assert_eq!(r.frame_count(), 0);
    }
}

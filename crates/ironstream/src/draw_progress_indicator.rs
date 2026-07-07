// FILE: draw_progress_indicator.rs
// occt: Draw_ProgressIndicator

//! Progress indicator for long operations in Draw.

/// Tracks progress of operations
pub struct DrawProgressIndicator {
    total_steps: i32,
    current_step: i32,
    is_cancelled: bool,
}

impl DrawProgressIndicator {
    /// Create a new progress indicator
    pub fn new(total_steps: i32) -> Self {
        DrawProgressIndicator {
            total_steps,
            current_step: 0,
            is_cancelled: false,
        }
    }

    /// Get total steps
    pub fn total_steps(&self) -> i32 {
        self.total_steps
    }

    /// Get current step
    pub fn current_step(&self) -> i32 {
        self.current_step
    }

    /// Increment progress
    pub fn step(&mut self) {
        if self.current_step < self.total_steps {
            self.current_step += 1;
        }
    }

    /// Check if operation is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.is_cancelled
    }

    /// Cancel the operation
    pub fn cancel(&mut self) {
        self.is_cancelled = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_creation() {
        let prog = DrawProgressIndicator::new(100);
        assert_eq!(prog.total_steps(), 100);
        assert_eq!(prog.current_step(), 0);
    }

    #[test]
    fn test_progress_step() {
        let mut prog = DrawProgressIndicator::new(5);
        prog.step();
        assert_eq!(prog.current_step(), 1);
    }
}

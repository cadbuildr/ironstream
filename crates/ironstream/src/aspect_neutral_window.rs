// FILE: aspect_neutral_window.rs
// occt: Aspect_NeutralWindow

/// Neutral window (platform-independent representation).
pub struct AspectNeutralWindow {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl AspectNeutralWindow {
    /// Create window.
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        AspectNeutralWindow { x, y, width, height }
    }

    /// Get position.
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// Get size.
    pub fn size(&self) -> (i32, i32) {
        (self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let window = AspectNeutralWindow::new(100, 200, 800, 600);
        assert_eq!(window.position(), (100, 200));
        assert_eq!(window.size(), (800, 600));
    }
}

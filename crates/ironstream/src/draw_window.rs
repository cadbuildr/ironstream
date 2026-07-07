// FILE: draw_window.rs
// occt: Draw_Window

//! Draw window for interactive display.

/// Represents a Draw window
pub struct DrawWindow {
    id: i32,
    title: String,
    width: i32,
    height: i32,
}

impl DrawWindow {
    /// Create a new window
    pub fn new(id: i32, title: impl Into<String>, width: i32, height: i32) -> Self {
        DrawWindow {
            id,
            title: title.into(),
            width,
            height,
        }
    }

    /// Get the window ID
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Get the window title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get the window width
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Get the window height
    pub fn height(&self) -> i32 {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_creation() {
        let win = DrawWindow::new(1, "MainWindow", 800, 600);
        assert_eq!(win.id(), 1);
        assert_eq!(win.title(), "MainWindow");
        assert_eq!(win.width(), 800);
        assert_eq!(win.height(), 600);
    }
}

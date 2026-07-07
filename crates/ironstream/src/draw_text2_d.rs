// FILE: draw_text2_d.rs
// occt: Draw_Text2D

//! A drawable 2D text for the Draw application.

/// Represents a 2D text drawable
pub struct DrawText2D {
    x: f64,
    y: f64,
    text: String,
    color: u32,
}

impl DrawText2D {
    /// Create a new 2D text
    pub fn new(x: f64, y: f64, text: impl Into<String>, color: u32) -> Self {
        DrawText2D {
            x,
            y,
            text: text.into(),
            color,
        }
    }

    /// Get the position
    pub fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    /// Get the text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get the color
    pub fn color(&self) -> u32 {
        self.color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text2d_creation() {
        let text = DrawText2D::new(10.0, 20.0, "Hello", 0xFF0000);
        assert_eq!(text.position(), (10.0, 20.0));
        assert_eq!(text.text(), "Hello");
    }
}

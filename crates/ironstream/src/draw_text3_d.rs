// FILE: draw_text3_d.rs
// occt: Draw_Text3D

//! A drawable 3D text for the Draw application.

/// Represents a 3D text drawable
pub struct DrawText3D {
    x: f64,
    y: f64,
    z: f64,
    text: String,
    color: u32,
}

impl DrawText3D {
    /// Create a new 3D text
    pub fn new(x: f64, y: f64, z: f64, text: impl Into<String>, color: u32) -> Self {
        DrawText3D {
            x,
            y,
            z,
            text: text.into(),
            color,
        }
    }

    /// Get the position
    pub fn position(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
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
    fn test_text3d_creation() {
        let text = DrawText3D::new(1.0, 2.0, 3.0, "Test", 0x00FF00);
        assert_eq!(text.position(), (1.0, 2.0, 3.0));
        assert_eq!(text.text(), "Test");
    }
}

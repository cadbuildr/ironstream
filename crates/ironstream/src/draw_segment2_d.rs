// FILE: draw_segment2_d.rs
// occt: Draw_Segment2D

//! A drawable 2D line segment for the Draw application.

/// Represents a 2D line segment drawable
pub struct DrawSegment2D {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    color: u32,
}

impl DrawSegment2D {
    /// Create a new 2D segment
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64, color: u32) -> Self {
        DrawSegment2D { x1, y1, x2, y2, color }
    }

    /// Get the first point
    pub fn point1(&self) -> (f64, f64) {
        (self.x1, self.y1)
    }

    /// Get the second point
    pub fn point2(&self) -> (f64, f64) {
        (self.x2, self.y2)
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
    fn test_segment2d_creation() {
        let seg = DrawSegment2D::new(0.0, 0.0, 10.0, 10.0, 0xFF0000);
        assert_eq!(seg.point1(), (0.0, 0.0));
        assert_eq!(seg.point2(), (10.0, 10.0));
        assert_eq!(seg.color(), 0xFF0000);
    }
}

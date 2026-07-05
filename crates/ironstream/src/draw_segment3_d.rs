// FILE: draw_segment3_d.rs
// occt: Draw_Segment3D

//! A drawable 3D line segment for the Draw application.

/// Represents a 3D line segment drawable
pub struct DrawSegment3D {
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
    color: u32,
}

impl DrawSegment3D {
    /// Create a new 3D segment
    pub fn new(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64, color: u32) -> Self {
        DrawSegment3D { x1, y1, z1, x2, y2, z2, color }
    }

    /// Get the first point
    pub fn point1(&self) -> (f64, f64, f64) {
        (self.x1, self.y1, self.z1)
    }

    /// Get the second point
    pub fn point2(&self) -> (f64, f64, f64) {
        (self.x2, self.y2, self.z2)
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
    fn test_segment3d_creation() {
        let seg = DrawSegment3D::new(0.0, 0.0, 0.0, 10.0, 10.0, 10.0, 0x00FF00);
        assert_eq!(seg.point1(), (0.0, 0.0, 0.0));
        assert_eq!(seg.point2(), (10.0, 10.0, 10.0));
        assert_eq!(seg.color(), 0x00FF00);
    }
}

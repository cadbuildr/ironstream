// FILE: draw_circle3_d.rs
// occt: Draw_Circle3D

//! A drawable representation of a 3D circle for the Draw application.
//! Contains a gp_Circ with angle parameters for arc drawing.

use std::f64;

/// Represents a 3D circle drawable with start and end angles
pub struct DrawCircle3D {
    /// Circle geometry in 3D space (simplified: center_x, center_y, center_z, radius)
    circ: (f64, f64, f64, f64),
    /// Start angle in radians
    a1: f64,
    /// End angle in radians
    a2: f64,
    /// Color for drawing
    color: u32,
}

impl DrawCircle3D {
    /// Create a new 3D circle drawable
    ///
    /// # Arguments
    /// * `center_x` - X coordinate of circle center
    /// * `center_y` - Y coordinate of circle center
    /// * `center_z` - Z coordinate of circle center
    /// * `radius` - Circle radius
    /// * `a1` - Start angle in radians
    /// * `a2` - End angle in radians
    /// * `color` - Drawing color
    pub fn new(
        center_x: f64,
        center_y: f64,
        center_z: f64,
        radius: f64,
        a1: f64,
        a2: f64,
        color: u32,
    ) -> Self {
        DrawCircle3D {
            circ: (center_x, center_y, center_z, radius),
            a1,
            a2,
            color,
        }
    }

    /// Get the circle center
    pub fn center(&self) -> (f64, f64, f64) {
        (self.circ.0, self.circ.1, self.circ.2)
    }

    /// Get the circle radius
    pub fn radius(&self) -> f64 {
        self.circ.3
    }

    /// Get the start angle
    pub fn angle1(&self) -> f64 {
        self.a1
    }

    /// Get the end angle
    pub fn angle2(&self) -> f64 {
        self.a2
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
    fn test_circle3d_construction() {
        let circle = DrawCircle3D::new(1.0, 2.0, 3.0, 4.0, 0.0, f64::consts::PI, 0xFF0000);
        assert_eq!(circle.center(), (1.0, 2.0, 3.0));
        assert_eq!(circle.radius(), 4.0);
        assert_eq!(circle.angle1(), 0.0);
        assert_eq!(circle.angle2(), f64::consts::PI);
        assert_eq!(circle.color(), 0xFF0000);
    }

    #[test]
    fn test_circle3d_full_arc() {
        let circle = DrawCircle3D::new(0.0, 0.0, 0.0, 5.0, 0.0, 2.0 * f64::consts::PI, 0x00FF00);
        assert_eq!(circle.center(), (0.0, 0.0, 0.0));
        assert_eq!(circle.radius(), 5.0);
        assert!((circle.angle2() - 2.0 * f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_circle3d_quarter_arc() {
        let circle = DrawCircle3D::new(10.0, 20.0, 30.0, 2.5, 0.0, f64::consts::FRAC_PI_2, 0x0000FF);
        assert_eq!(circle.radius(), 2.5);
        assert!((circle.angle2() - f64::consts::FRAC_PI_2).abs() < 1e-10);
    }
}

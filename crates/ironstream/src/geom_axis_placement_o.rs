// FILE: geom_axis_placement_o.rs
// occt: Geom_AxisPlacement

//! The abstract class AxisPlacement describes the common behavior of positioning
//! systems in 3D space, such as axis or coordinate systems.

use std::f64::consts::PI;

/// Represents an axis placement in 3D space.
/// This is an abstract base; concrete implementations are Axis1Placement and Axis2Placement.
pub struct AxisPlacement {
    location: (f64, f64, f64),
    direction: (f64, f64, f64),
}

impl AxisPlacement {
    /// Create an axis placement at origin with Z direction.
    pub fn new() -> Self {
        AxisPlacement {
            location: (0.0, 0.0, 0.0),
            direction: (0.0, 0.0, 1.0),
        }
    }

    /// Create an axis placement with specified location and direction.
    pub fn with_placement(location: (f64, f64, f64), direction: (f64, f64, f64)) -> Self {
        AxisPlacement {
            location,
            direction,
        }
    }

    /// Set the location (origin) of the axis placement.
    pub fn set_location(&mut self, location: (f64, f64, f64)) {
        self.location = location;
    }

    /// Set the direction of the axis placement.
    pub fn set_direction(&mut self, direction: (f64, f64, f64)) {
        self.direction = direction;
    }

    /// Get the location (origin) of the axis placement.
    pub fn location(&self) -> (f64, f64, f64) {
        self.location
    }

    /// Get the direction of the axis placement.
    pub fn direction(&self) -> (f64, f64, f64) {
        self.direction
    }

    /// Compute the angular value in radians between this axis and another.
    /// Result is between 0 and PI.
    pub fn angle(&self, other: &AxisPlacement) -> f64 {
        let d1 = self.direction;
        let d2 = other.direction;

        let dot = d1.0 * d2.0 + d1.1 * d2.1 + d1.2 * d2.2;

        // Clamp to [-1, 1] to avoid domain error in acos
        let clamped = dot.clamp(-1.0, 1.0);
        clamped.acos()
    }
}

impl Default for AxisPlacement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let ap = AxisPlacement::new();
        assert_eq!(ap.location(), (0.0, 0.0, 0.0));
        assert_eq!(ap.direction(), (0.0, 0.0, 1.0));
    }

    #[test]
    fn test_with_placement() {
        let ap = AxisPlacement::with_placement((1.0, 2.0, 3.0), (1.0, 0.0, 0.0));
        assert_eq!(ap.location(), (1.0, 2.0, 3.0));
        assert_eq!(ap.direction(), (1.0, 0.0, 0.0));
    }

    #[test]
    fn test_set_location() {
        let mut ap = AxisPlacement::new();
        ap.set_location((5.0, 6.0, 7.0));
        assert_eq!(ap.location(), (5.0, 6.0, 7.0));
    }

    #[test]
    fn test_set_direction() {
        let mut ap = AxisPlacement::new();
        ap.set_direction((1.0, 0.0, 0.0));
        assert_eq!(ap.direction(), (1.0, 0.0, 0.0));
    }

    #[test]
    fn test_angle_parallel() {
        let ap1 = AxisPlacement::with_placement((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let ap2 = AxisPlacement::with_placement((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        assert!((ap1.angle(&ap2)).abs() < 1e-10);
    }

    #[test]
    fn test_angle_perpendicular() {
        let ap1 = AxisPlacement::with_placement((0.0, 0.0, 0.0), (1.0, 0.0, 0.0));
        let ap2 = AxisPlacement::with_placement((0.0, 0.0, 0.0), (0.0, 1.0, 0.0));
        assert!((ap1.angle(&ap2) - PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_angle_opposite() {
        let ap1 = AxisPlacement::with_placement((0.0, 0.0, 0.0), (1.0, 0.0, 0.0));
        let ap2 = AxisPlacement::with_placement((0.0, 0.0, 0.0), (-1.0, 0.0, 0.0));
        assert!((ap1.angle(&ap2) - PI).abs() < 1e-10);
    }
}

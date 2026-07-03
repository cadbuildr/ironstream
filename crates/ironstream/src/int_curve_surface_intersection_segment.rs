// FILE: int_curve_surface_intersection_segment.rs
// occt: IntCurveSurface_IntersectionSegment

/// Represents a segment of intersection between a curve and surface.
/// Contains parametric information for the segment endpoints.
#[derive(Clone, Copy, Debug)]
pub struct IntCurveSurfaceIntersectionSegment {
    first_param: f64,
    last_param: f64,
}

impl IntCurveSurfaceIntersectionSegment {
    /// Create an intersection segment
    pub fn new(first: f64, last: f64) -> Self {
        IntCurveSurfaceIntersectionSegment {
            first_param: first,
            last_param: last,
        }
    }

    /// Get first parameter
    pub fn first_param(&self) -> f64 {
        self.first_param
    }

    /// Get last parameter
    pub fn last_param(&self) -> f64 {
        self.last_param
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let seg = IntCurveSurfaceIntersectionSegment::new(0.0, 1.0);
        assert_eq!(seg.first_param(), 0.0);
        assert_eq!(seg.last_param(), 1.0);
    }

    #[test]
    fn test_various_ranges() {
        let seg1 = IntCurveSurfaceIntersectionSegment::new(0.5, 2.5);
        assert_eq!(seg1.first_param(), 0.5);
        assert_eq!(seg1.last_param(), 2.5);

        let seg2 = IntCurveSurfaceIntersectionSegment::new(-1.0, 1.0);
        assert_eq!(seg2.first_param(), -1.0);
        assert_eq!(seg2.last_param(), 1.0);
    }

    #[test]
    fn test_clone() {
        let seg = IntCurveSurfaceIntersectionSegment::new(0.25, 0.75);
        let seg2 = seg;
        assert_eq!(seg2.first_param(), 0.25);
        assert_eq!(seg2.last_param(), 0.75);
    }
}

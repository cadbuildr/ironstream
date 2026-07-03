// FILE: int_curve_surface_intersection.rs
// occt: IntCurveSurface_Intersection

/// Base class for curve-surface intersection algorithms.
/// Stores and manages intersection results.
pub struct IntCurveSurfaceIntersection {
    nb_points: usize,
    nb_segments: usize,
}

impl IntCurveSurfaceIntersection {
    /// Create an empty intersection result
    pub fn new() -> Self {
        IntCurveSurfaceIntersection {
            nb_points: 0,
            nb_segments: 0,
        }
    }

    /// Get the number of intersection points
    pub fn nb_points(&self) -> usize {
        self.nb_points
    }

    /// Get the number of intersection segments
    pub fn nb_segments(&self) -> usize {
        self.nb_segments
    }

    /// Set number of points
    pub fn set_nb_points(&mut self, n: usize) {
        self.nb_points = n;
    }

    /// Set number of segments
    pub fn set_nb_segments(&mut self, n: usize) {
        self.nb_segments = n;
    }
}

impl Default for IntCurveSurfaceIntersection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let inter = IntCurveSurfaceIntersection::new();
        assert_eq!(inter.nb_points(), 0);
        assert_eq!(inter.nb_segments(), 0);
    }

    #[test]
    fn test_setters() {
        let mut inter = IntCurveSurfaceIntersection::new();
        inter.set_nb_points(5);
        inter.set_nb_segments(3);

        assert_eq!(inter.nb_points(), 5);
        assert_eq!(inter.nb_segments(), 3);
    }

    #[test]
    fn test_default() {
        let inter = IntCurveSurfaceIntersection::default();
        assert_eq!(inter.nb_points(), 0);
    }
}

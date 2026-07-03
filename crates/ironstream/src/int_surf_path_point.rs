// FILE: int_surf_path_point.rs
// occt: IntSurf_PathPoint

/// Represents a point along a path on surfaces
#[derive(Clone, Debug, Copy)]
pub struct PathPoint {
    u1: f64,
    v1: f64,
    u2: f64,
    v2: f64,
    is_pass_point: bool,
}

impl PathPoint {
    /// Create a new path point
    pub fn new() -> Self {
        PathPoint {
            u1: 0.0,
            v1: 0.0,
            u2: 0.0,
            v2: 0.0,
            is_pass_point: false,
        }
    }

    /// Create with parameters
    pub fn with_params(u1: f64, v1: f64, u2: f64, v2: f64, pass_point: bool) -> Self {
        PathPoint {
            u1,
            v1,
            u2,
            v2,
            is_pass_point: pass_point,
        }
    }

    pub fn u1(&self) -> f64 {
        self.u1
    }

    pub fn v1(&self) -> f64 {
        self.v1
    }

    pub fn u2(&self) -> f64 {
        self.u2
    }

    pub fn v2(&self) -> f64 {
        self.v2
    }

    pub fn is_pass_point(&self) -> bool {
        self.is_pass_point
    }

    pub fn set_pass_point(&mut self, pass: bool) {
        self.is_pass_point = pass;
    }
}

impl Default for PathPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pp = PathPoint::new();
        assert_eq!(pp.u1(), 0.0);
        assert!(!pp.is_pass_point());
    }

    #[test]
    fn test_with_params() {
        let pp = PathPoint::with_params(0.5, 0.6, 0.7, 0.8, true);
        assert_eq!(pp.u1(), 0.5);
        assert!(pp.is_pass_point());
    }
}

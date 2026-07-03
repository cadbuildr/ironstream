// FILE: int_tools_pnt_on2_faces.rs
// occt: IntTools_PntOn2Faces

use super::int_tools_pnt_on_face::IntToolsPntOnFace;

/// A pair of points, each on a different face.
#[derive(Clone, Copy, Debug)]
pub struct IntToolsPntOn2Faces {
    p1: IntToolsPntOnFace,
    p2: IntToolsPntOnFace,
    is_valid: bool,
}

impl IntToolsPntOn2Faces {
    /// Create an empty pair of points.
    pub fn new() -> Self {
        IntToolsPntOn2Faces {
            p1: IntToolsPntOnFace::new(),
            p2: IntToolsPntOnFace::new(),
            is_valid: false,
        }
    }

    /// Initialize with two points.
    pub fn init(p1: IntToolsPntOnFace, p2: IntToolsPntOnFace) -> Self {
        IntToolsPntOn2Faces {
            p1,
            p2,
            is_valid: true,
        }
    }

    /// Set the first point.
    pub fn set_p1(&mut self, p1: IntToolsPntOnFace) {
        self.p1 = p1;
    }

    /// Set the second point.
    pub fn set_p2(&mut self, p2: IntToolsPntOnFace) {
        self.p2 = p2;
    }

    /// Set the validity flag.
    pub fn set_valid(&mut self, valid: bool) {
        self.is_valid = valid;
    }

    /// Get the first point.
    pub fn p1(&self) -> IntToolsPntOnFace {
        self.p1
    }

    /// Get the second point.
    pub fn p2(&self) -> IntToolsPntOnFace {
        self.p2
    }

    /// Check if the pair is valid.
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}

impl Default for IntToolsPntOn2Faces {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pair = IntToolsPntOn2Faces::new();
        assert!(!pair.is_valid());
    }

    #[test]
    fn test_init() {
        let p1 = IntToolsPntOnFace::init(1.0, 2.0);
        let p2 = IntToolsPntOnFace::init(3.0, 4.0);
        let pair = IntToolsPntOn2Faces::init(p1, p2);
        assert!(pair.is_valid());
    }

    #[test]
    fn test_set_points() {
        let mut pair = IntToolsPntOn2Faces::new();
        let p1 = IntToolsPntOnFace::init(1.0, 2.0);
        let p2 = IntToolsPntOnFace::init(3.0, 4.0);
        pair.set_p1(p1);
        pair.set_p2(p2);
        let (u1, v1) = pair.p1().parameters();
        let (u2, v2) = pair.p2().parameters();
        assert_eq!(u1, 1.0);
        assert_eq!(u2, 3.0);
    }

    #[test]
    fn test_set_valid() {
        let mut pair = IntToolsPntOn2Faces::new();
        pair.set_valid(true);
        assert!(pair.is_valid());
    }
}

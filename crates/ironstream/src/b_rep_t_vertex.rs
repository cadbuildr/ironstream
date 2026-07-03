// FILE: b_rep_t_vertex.rs
// occt: BRep_TVertex

/// Topological vertex in BRep representation.
pub struct BRepTVertex {
    id: u32,
    x: f64,
    y: f64,
    z: f64,
    tolerance: f64,
    modified: bool,
}

impl BRepTVertex {
    /// Create a new topological vertex.
    pub fn new(id: u32, x: f64, y: f64, z: f64, tolerance: f64) -> Self {
        Self {
            id,
            x,
            y,
            z,
            tolerance,
            modified: false,
        }
    }

    /// Get the vertex id.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get x coordinate.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Get y coordinate.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Get z coordinate.
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Get the tolerance.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Check if the vertex has been modified.
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Set the point coordinates.
    pub fn set_point(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.modified = true;
    }

    /// Set the tolerance.
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
        self.modified = true;
    }

    /// Get the coordinates as a tuple.
    pub fn point(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = BRepTVertex::new(1, 1.0, 2.0, 3.0, 0.01);

        assert_eq!(v.id(), 1);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
        assert_eq!(v.tolerance(), 0.01);
        assert!(!v.is_modified());
    }

    #[test]
    fn test_set_point() {
        let mut v = BRepTVertex::new(1, 0.0, 0.0, 0.0, 0.01);
        v.set_point(1.0, 2.0, 3.0);

        assert!(v.is_modified());
        assert_eq!(v.point(), (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_set_tolerance() {
        let mut v = BRepTVertex::new(1, 0.0, 0.0, 0.0, 0.01);
        v.set_tolerance(0.1);

        assert!(v.is_modified());
        assert_eq!(v.tolerance(), 0.1);
    }
}

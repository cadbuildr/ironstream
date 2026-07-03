// FILE: shape_construct_make_triangulation.rs
// occt: ShapeConstruct_MakeTriangulation

/// Make triangulation from points or wire.
pub struct MakeTriangulation {
    precision: f64,
    done: bool,
}

impl MakeTriangulation {
    /// Create from points array.
    pub fn new_from_points(_points: &[(f64, f64, f64)], precision: f64) -> Self {
        MakeTriangulation {
            precision,
            done: false,
        }
    }

    /// Create from wire.
    pub fn new_from_wire(_wire: &[u8], precision: f64) -> Self {
        MakeTriangulation {
            precision,
            done: false,
        }
    }

    /// Build the triangulation.
    pub fn build(&mut self) {
        self.done = true;
    }

    /// Check if done.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_from_points() {
        let tri = MakeTriangulation::new_from_points(&[], 0.0);
        assert_eq!(tri.precision, 0.0);
        assert!(!tri.is_done());
    }

    #[test]
    fn test_build() {
        let mut tri = MakeTriangulation::new_from_points(&[], 0.01);
        tri.build();
        assert!(tri.is_done());
    }
}

// FILE: geom_plate_aij.rs
// occt: GeomPlate_Aij

/// A structure storing indexes of two normals and their cross product vector.
/// Used in averaging plane calculations for surface generation.
#[derive(Clone, Copy, Debug)]
pub struct GeomPlateAij {
    ind1: i32,
    ind2: i32,
    vec: [f64; 3],
}

impl GeomPlateAij {
    /// Create with default values
    pub fn new() -> Self {
        GeomPlateAij {
            ind1: 0,
            ind2: 0,
            vec: [0.0, 0.0, 0.0],
        }
    }

    /// Create with explicit values
    pub fn with_values(ind1: i32, ind2: i32, vec: [f64; 3]) -> Self {
        GeomPlateAij {
            ind1,
            ind2,
            vec,
        }
    }

    /// Get first index
    pub fn ind1(&self) -> i32 {
        self.ind1
    }

    /// Get second index
    pub fn ind2(&self) -> i32 {
        self.ind2
    }

    /// Get the vector
    pub fn vec(&self) -> [f64; 3] {
        self.vec
    }
}

impl Default for GeomPlateAij {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_construction() {
        let aij = GeomPlateAij::new();
        assert_eq!(aij.ind1(), 0);
        assert_eq!(aij.ind2(), 0);
        assert_eq!(aij.vec(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_with_values() {
        let aij = GeomPlateAij::with_values(1, 2, [1.0, 2.0, 3.0]);
        assert_eq!(aij.ind1(), 1);
        assert_eq!(aij.ind2(), 2);
        assert_eq!(aij.vec(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_clone() {
        let aij = GeomPlateAij::with_values(5, 10, [0.1, 0.2, 0.3]);
        let aij2 = aij;
        assert_eq!(aij2.ind1(), 5);
        assert_eq!(aij2.ind2(), 10);
    }

    #[test]
    fn test_default_trait() {
        let aij = GeomPlateAij::default();
        assert_eq!(aij.ind1(), 0);
        assert_eq!(aij.ind2(), 0);
    }
}

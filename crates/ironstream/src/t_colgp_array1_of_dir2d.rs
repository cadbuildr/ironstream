// FILE: t_colgp_array1_of_dir2d.rs
// occt: TColgp_Array1OfDir2d

/// A 2D direction vector (gp_Dir2d in OCCT), normalized.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dir2d {
    /// X component (normalized)
    pub x: f64,
    /// Y component (normalized)
    pub y: f64,
}

impl Dir2d {
    /// Creates a normalized 2D direction vector.
    pub fn new(x: f64, y: f64) -> Self {
        let mag = (x * x + y * y).sqrt();
        if mag == 0.0 {
            panic!("Cannot create direction from zero vector");
        }
        Dir2d {
            x: x / mag,
            y: y / mag,
        }
    }
}

/// 1-based Array1 of 2D direction vectors (gp_Dir2d), mirroring NCollection_Array1 semantics.
#[derive(Debug, Clone)]
pub struct TColgpArray1OfDir2d {
    lower: usize,
    upper: usize,
    data: Vec<Dir2d>,
}

impl TColgpArray1OfDir2d {
    /// Creates an array with bounds [lower, upper].
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper {
            panic!("Lower bound {} exceeds upper bound {}", lower, upper);
        }
        let size = upper - lower + 1;
        TColgpArray1OfDir2d {
            lower,
            upper,
            data: vec![Dir2d { x: 1.0, y: 0.0 }; size],
        }
    }

    /// Returns the lower bound.
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Returns the upper bound.
    pub fn upper(&self) -> usize {
        self.upper
    }

    /// Returns the length of the array.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Gets the element at the given index (within bounds).
    pub fn get(&self, idx: usize) -> Dir2d {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        self.data[idx - self.lower]
    }

    /// Sets the element at the given index (within bounds).
    pub fn set(&mut self, idx: usize, value: Dir2d) {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        self.data[idx - self.lower] = value;
    }

    /// Gets a reference to the element at the given index.
    pub fn at(&self, idx: usize) -> &Dir2d {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        &self.data[idx - self.lower]
    }

    /// Gets a mutable reference to the element at the given index.
    pub fn at_mut(&mut self, idx: usize) -> &mut Dir2d {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        &mut self.data[idx - self.lower]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir2d_normalization() {
        let d = Dir2d::new(3.0, 4.0);
        let mag = (d.x * d.x + d.y * d.y).sqrt();
        assert!((mag - 1.0).abs() < 1e-10);
        assert!((d.x - 0.6).abs() < 1e-10);
        assert!((d.y - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_array_creation_with_bounds() {
        let arr = TColgpArray1OfDir2d::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn test_array_set_and_get() {
        let mut arr = TColgpArray1OfDir2d::new(1, 3);
        let dir = Dir2d::new(1.0, 1.0);
        arr.set(2, dir);

        let retrieved = arr.get(2);
        assert!((retrieved.x - dir.x).abs() < 1e-10);
        assert!((retrieved.y - dir.y).abs() < 1e-10);
    }

    #[test]
    fn test_array_at_mutable() {
        let mut arr = TColgpArray1OfDir2d::new(1, 2);
        {
            let d = arr.at_mut(1);
            d.x = 0.707;
            d.y = 0.707;
        }

        let retrieved = arr.get(1);
        assert!((retrieved.x - 0.707).abs() < 1e-6);
        assert!((retrieved.y - 0.707).abs() < 1e-6);
    }

    #[test]
    fn test_array_default_initialization() {
        let arr = TColgpArray1OfDir2d::new(10, 12);
        for i in 10..=12 {
            let d = arr.get(i);
            assert_eq!(d.x, 1.0);
            assert_eq!(d.y, 0.0);
        }
    }

    #[test]
    #[should_panic]
    fn test_array_get_out_of_bounds() {
        let arr = TColgpArray1OfDir2d::new(5, 10);
        let _ = arr.get(11);
    }

    #[test]
    #[should_panic]
    fn test_dir2d_zero_vector() {
        let _ = Dir2d::new(0.0, 0.0);
    }
}

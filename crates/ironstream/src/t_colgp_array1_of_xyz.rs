// FILE: t_colgp_array1_of_xyz.rs
// occt: TColgp_Array1OfXYZ

/// A 3D coordinate triplet (gp_XYZ in OCCT).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XYZ {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
}

impl XYZ {
    /// Creates a 3D coordinate.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        XYZ { x, y, z }
    }

    /// Returns the magnitude (length) of the vector from origin.
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

/// 1-based Array1 of 3D coordinates (gp_XYZ), mirroring NCollection_Array1 semantics.
#[derive(Debug, Clone)]
pub struct TColgpArray1OfXYZ {
    lower: usize,
    upper: usize,
    data: Vec<XYZ>,
}

impl TColgpArray1OfXYZ {
    /// Creates an array with bounds [lower, upper].
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper {
            panic!("Lower bound {} exceeds upper bound {}", lower, upper);
        }
        let size = upper - lower + 1;
        TColgpArray1OfXYZ {
            lower,
            upper,
            data: vec![XYZ { x: 0.0, y: 0.0, z: 0.0 }; size],
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
    pub fn get(&self, idx: usize) -> XYZ {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        self.data[idx - self.lower]
    }

    /// Sets the element at the given index (within bounds).
    pub fn set(&mut self, idx: usize, value: XYZ) {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        self.data[idx - self.lower] = value;
    }

    /// Gets a reference to the element at the given index.
    pub fn at(&self, idx: usize) -> &XYZ {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        &self.data[idx - self.lower]
    }

    /// Gets a mutable reference to the element at the given index.
    pub fn at_mut(&mut self, idx: usize) -> &mut XYZ {
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
    fn test_xyz_creation() {
        let xyz = XYZ::new(1.0, 2.0, 3.0);
        assert_eq!(xyz.x, 1.0);
        assert_eq!(xyz.y, 2.0);
        assert_eq!(xyz.z, 3.0);
    }

    #[test]
    fn test_xyz_magnitude() {
        let xyz = XYZ::new(1.0, 2.0, 2.0);
        assert_eq!(xyz.magnitude(), 3.0);
    }

    #[test]
    fn test_array_creation_with_bounds() {
        let arr = TColgpArray1OfXYZ::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn test_array_set_and_get() {
        let mut arr = TColgpArray1OfXYZ::new(1, 3);
        let xyz = XYZ::new(1.5, 2.5, 3.5);
        arr.set(2, xyz);

        assert_eq!(arr.get(2), xyz);
    }

    #[test]
    fn test_array_at_mutable() {
        let mut arr = TColgpArray1OfXYZ::new(0, 2);
        arr.at_mut(1).x = 10.0;
        arr.at_mut(1).y = 20.0;
        arr.at_mut(1).z = 30.0;

        let retrieved = arr.get(1);
        assert_eq!(retrieved.x, 10.0);
        assert_eq!(retrieved.y, 20.0);
        assert_eq!(retrieved.z, 30.0);
    }

    #[test]
    fn test_array_default_initialization() {
        let arr = TColgpArray1OfXYZ::new(7, 9);
        for i in 7..=9 {
            let xyz = arr.get(i);
            assert_eq!(xyz.x, 0.0);
            assert_eq!(xyz.y, 0.0);
            assert_eq!(xyz.z, 0.0);
        }
    }

    #[test]
    #[should_panic]
    fn test_array_get_out_of_bounds() {
        let arr = TColgpArray1OfXYZ::new(5, 10);
        let _ = arr.get(11);
    }
}

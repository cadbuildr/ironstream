// FILE: t_colgp_h_array1_of_dir.rs
// occt: TColgp_HArray1OfDir

use std::sync::Arc;

/// A 3D direction vector (gp_Dir in OCCT), normalized.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dir {
    /// X component (normalized)
    pub x: f64,
    /// Y component (normalized)
    pub y: f64,
    /// Z component (normalized)
    pub z: f64,
}

impl Dir {
    /// Creates a normalized 3D direction vector.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let mag = (x * x + y * y + z * z).sqrt();
        if mag == 0.0 {
            panic!("Cannot create direction from zero vector");
        }
        Dir {
            x: x / mag,
            y: y / mag,
            z: z / mag,
        }
    }
}

/// Handle-based (reference-counted) 1-based Array1 of 3D direction vectors.
#[derive(Debug, Clone)]
pub struct TColgpHArray1OfDir {
    data: Arc<TColgpArray1OfDirData>,
}

#[derive(Debug)]
struct TColgpArray1OfDirData {
    lower: usize,
    upper: usize,
    items: Vec<Dir>,
}

impl TColgpHArray1OfDir {
    /// Creates a shared handle-based array with bounds [lower, upper].
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper {
            panic!("Lower bound {} exceeds upper bound {}", lower, upper);
        }
        let size = upper - lower + 1;
        TColgpHArray1OfDir {
            data: Arc::new(TColgpArray1OfDirData {
                lower,
                upper,
                items: vec![Dir { x: 1.0, y: 0.0, z: 0.0 }; size],
            }),
        }
    }

    /// Returns the lower bound.
    pub fn lower(&self) -> usize {
        self.data.lower
    }

    /// Returns the upper bound.
    pub fn upper(&self) -> usize {
        self.data.upper
    }

    /// Returns the length of the array.
    pub fn len(&self) -> usize {
        self.data.items.len()
    }

    /// Checks if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.data.items.is_empty()
    }

    /// Gets the element at the given index (within bounds).
    pub fn get(&self, idx: usize) -> Dir {
        if idx < self.data.lower || idx > self.data.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.data.lower, self.data.upper);
        }
        self.data.items[idx - self.data.lower]
    }

    /// Gets a reference to the element at the given index.
    pub fn at(&self, idx: usize) -> &Dir {
        if idx < self.data.lower || idx > self.data.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.data.lower, self.data.upper);
        }
        &self.data.items[idx - self.data.lower]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_normalization() {
        let d = Dir::new(1.0, 1.0, 1.0);
        let mag = (d.x * d.x + d.y * d.y + d.z * d.z).sqrt();
        assert!((mag - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_harray_creation_with_bounds() {
        let arr = TColgpHArray1OfDir::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn test_harray_clone_shares_data() {
        let arr1 = TColgpHArray1OfDir::new(1, 3);
        let arr2 = arr1.clone();
        assert_eq!(arr1.lower(), arr2.lower());
        assert_eq!(arr1.upper(), arr2.upper());
    }

    #[test]
    fn test_harray_get() {
        let arr = TColgpHArray1OfDir::new(1, 3);
        let d = arr.get(1);
        assert_eq!(d.x, 1.0);
        assert_eq!(d.y, 0.0);
        assert_eq!(d.z, 0.0);
    }

    #[test]
    fn test_harray_at() {
        let arr = TColgpHArray1OfDir::new(0, 2);
        let d = arr.at(1);
        assert_eq!(d.x, 1.0);
    }

    #[test]
    #[should_panic]
    fn test_harray_get_out_of_bounds() {
        let arr = TColgpHArray1OfDir::new(5, 10);
        let _ = arr.get(11);
    }
}

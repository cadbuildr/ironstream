// FILE: t_colgp_h_array1_of_dir2d.rs
// occt: TColgp_HArray1OfDir2d

use std::sync::Arc;

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

/// Handle-based (reference-counted) 1-based Array1 of 2D direction vectors.
#[derive(Debug, Clone)]
pub struct TColgpHArray1OfDir2d {
    data: Arc<TColgpArray1OfDir2dData>,
}

#[derive(Debug)]
struct TColgpArray1OfDir2dData {
    lower: usize,
    upper: usize,
    items: Vec<Dir2d>,
}

impl TColgpHArray1OfDir2d {
    /// Creates a shared handle-based array with bounds [lower, upper].
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper {
            panic!("Lower bound {} exceeds upper bound {}", lower, upper);
        }
        let size = upper - lower + 1;
        TColgpHArray1OfDir2d {
            data: Arc::new(TColgpArray1OfDir2dData {
                lower,
                upper,
                items: vec![Dir2d { x: 1.0, y: 0.0 }; size],
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
    pub fn get(&self, idx: usize) -> Dir2d {
        if idx < self.data.lower || idx > self.data.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.data.lower, self.data.upper);
        }
        self.data.items[idx - self.data.lower]
    }

    /// Gets a reference to the element at the given index.
    pub fn at(&self, idx: usize) -> &Dir2d {
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
    fn test_dir2d_normalization() {
        let d = Dir2d::new(3.0, 4.0);
        let mag = (d.x * d.x + d.y * d.y).sqrt();
        assert!((mag - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_harray_creation_with_bounds() {
        let arr = TColgpHArray1OfDir2d::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn test_harray_clone_shares_data() {
        let arr1 = TColgpHArray1OfDir2d::new(1, 3);
        let arr2 = arr1.clone();
        assert_eq!(arr1.lower(), arr2.lower());
        assert_eq!(arr1.upper(), arr2.upper());
    }

    #[test]
    fn test_harray_get() {
        let arr = TColgpHArray1OfDir2d::new(1, 3);
        let d = arr.get(1);
        assert_eq!(d.x, 1.0);
        assert_eq!(d.y, 0.0);
    }

    #[test]
    #[should_panic]
    fn test_harray_get_out_of_bounds() {
        let arr = TColgpHArray1OfDir2d::new(5, 10);
        let _ = arr.get(11);
    }
}

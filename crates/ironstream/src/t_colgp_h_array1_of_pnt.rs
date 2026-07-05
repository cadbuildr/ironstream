// FILE: t_colgp_h_array1_of_pnt.rs
// occt: TColgp_HArray1OfPnt

use std::sync::Arc;

/// A 3D point (gp_Pnt in OCCT).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pnt {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
}

impl Pnt {
    /// Creates a 3D point.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Pnt { x, y, z }
    }
}

/// Handle-based (reference-counted) 1-based Array1 of 3D points.
#[derive(Debug, Clone)]
pub struct TColgpHArray1OfPnt {
    data: Arc<TColgpArray1OfPntData>,
}

#[derive(Debug)]
struct TColgpArray1OfPntData {
    lower: usize,
    upper: usize,
    items: Vec<Pnt>,
}

impl TColgpHArray1OfPnt {
    /// Creates a shared handle-based array with bounds [lower, upper].
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper {
            panic!("Lower bound {} exceeds upper bound {}", lower, upper);
        }
        let size = upper - lower + 1;
        TColgpHArray1OfPnt {
            data: Arc::new(TColgpArray1OfPntData {
                lower,
                upper,
                items: vec![Pnt { x: 0.0, y: 0.0, z: 0.0 }; size],
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
    pub fn get(&self, idx: usize) -> Pnt {
        if idx < self.data.lower || idx > self.data.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.data.lower, self.data.upper);
        }
        self.data.items[idx - self.data.lower]
    }

    /// Gets a reference to the element at the given index.
    pub fn at(&self, idx: usize) -> &Pnt {
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
    fn test_pnt_creation() {
        let p = Pnt::new(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }

    #[test]
    fn test_harray_creation_with_bounds() {
        let arr = TColgpHArray1OfPnt::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn test_harray_clone_shares_data() {
        let arr1 = TColgpHArray1OfPnt::new(1, 3);
        let arr2 = arr1.clone();
        assert_eq!(arr1.lower(), arr2.lower());
        assert_eq!(arr1.upper(), arr2.upper());
    }

    #[test]
    fn test_harray_get() {
        let arr = TColgpHArray1OfPnt::new(1, 3);
        let p = arr.get(1);
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
        assert_eq!(p.z, 0.0);
    }

    #[test]
    #[should_panic]
    fn test_harray_get_out_of_bounds() {
        let arr = TColgpHArray1OfPnt::new(5, 10);
        let _ = arr.get(11);
    }
}

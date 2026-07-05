// FILE: t_colgp_h_array1_of_xy.rs
// occt: TColgp_HArray1OfXY

use std::sync::Arc;

/// A 2D coordinate pair (gp_XY in OCCT).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XY {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}

impl XY {
    /// Creates a 2D coordinate.
    pub fn new(x: f64, y: f64) -> Self {
        XY { x, y }
    }
}

/// Handle-based (reference-counted) 1-based Array1 of 2D coordinates.
#[derive(Debug, Clone)]
pub struct TColgpHArray1OfXY {
    data: Arc<TColgpArray1OfXYData>,
}

#[derive(Debug)]
struct TColgpArray1OfXYData {
    lower: usize,
    upper: usize,
    items: Vec<XY>,
}

impl TColgpHArray1OfXY {
    /// Creates a shared handle-based array with bounds [lower, upper].
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper {
            panic!("Lower bound {} exceeds upper bound {}", lower, upper);
        }
        let size = upper - lower + 1;
        TColgpHArray1OfXY {
            data: Arc::new(TColgpArray1OfXYData {
                lower,
                upper,
                items: vec![XY { x: 0.0, y: 0.0 }; size],
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
    pub fn get(&self, idx: usize) -> XY {
        if idx < self.data.lower || idx > self.data.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.data.lower, self.data.upper);
        }
        self.data.items[idx - self.data.lower]
    }

    /// Gets a reference to the element at the given index.
    pub fn at(&self, idx: usize) -> &XY {
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
    fn test_xy_creation() {
        let xy = XY::new(3.5, 7.2);
        assert_eq!(xy.x, 3.5);
        assert_eq!(xy.y, 7.2);
    }

    #[test]
    fn test_harray_creation_with_bounds() {
        let arr = TColgpHArray1OfXY::new(1, 4);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 4);
        assert_eq!(arr.len(), 4);
    }

    #[test]
    fn test_harray_clone_shares_data() {
        let arr1 = TColgpHArray1OfXY::new(1, 3);
        let arr2 = arr1.clone();
        assert_eq!(arr1.lower(), arr2.lower());
        assert_eq!(arr1.upper(), arr2.upper());
    }

    #[test]
    fn test_harray_get() {
        let arr = TColgpHArray1OfXY::new(1, 3);
        let xy = arr.get(1);
        assert_eq!(xy.x, 0.0);
        assert_eq!(xy.y, 0.0);
    }

    #[test]
    #[should_panic]
    fn test_harray_get_out_of_bounds() {
        let arr = TColgpHArray1OfXY::new(5, 10);
        let _ = arr.get(11);
    }
}

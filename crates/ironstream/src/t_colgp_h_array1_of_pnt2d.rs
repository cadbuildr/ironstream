// FILE: t_colgp_h_array1_of_pnt2d.rs
// occt: TColgp_HArray1OfPnt2d

use std::sync::Arc;

/// A 2D point (gp_Pnt2d in OCCT).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pnt2d {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}

impl Pnt2d {
    /// Creates a 2D point.
    pub fn new(x: f64, y: f64) -> Self {
        Pnt2d { x, y }
    }
}

/// Handle-based (reference-counted) 1-based Array1 of 2D points.
#[derive(Debug, Clone)]
pub struct TColgpHArray1OfPnt2d {
    data: Arc<TColgpArray1OfPnt2dData>,
}

#[derive(Debug)]
struct TColgpArray1OfPnt2dData {
    lower: usize,
    upper: usize,
    items: Vec<Pnt2d>,
}

impl TColgpHArray1OfPnt2d {
    /// Creates a shared handle-based array with bounds [lower, upper].
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper {
            panic!("Lower bound {} exceeds upper bound {}", lower, upper);
        }
        let size = upper - lower + 1;
        TColgpHArray1OfPnt2d {
            data: Arc::new(TColgpArray1OfPnt2dData {
                lower,
                upper,
                items: vec![Pnt2d { x: 0.0, y: 0.0 }; size],
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
    pub fn get(&self, idx: usize) -> Pnt2d {
        if idx < self.data.lower || idx > self.data.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.data.lower, self.data.upper);
        }
        self.data.items[idx - self.data.lower]
    }

    /// Gets a reference to the element at the given index.
    pub fn at(&self, idx: usize) -> &Pnt2d {
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
    fn test_pnt2d_creation() {
        let p = Pnt2d::new(3.5, 7.2);
        assert_eq!(p.x, 3.5);
        assert_eq!(p.y, 7.2);
    }

    #[test]
    fn test_harray_creation_with_bounds() {
        let arr = TColgpHArray1OfPnt2d::new(1, 4);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 4);
        assert_eq!(arr.len(), 4);
    }

    #[test]
    fn test_harray_clone_shares_data() {
        let arr1 = TColgpHArray1OfPnt2d::new(1, 3);
        let arr2 = arr1.clone();
        assert_eq!(arr1.lower(), arr2.lower());
        assert_eq!(arr1.upper(), arr2.upper());
    }

    #[test]
    fn test_harray_get() {
        let arr = TColgpHArray1OfPnt2d::new(1, 3);
        let p = arr.get(1);
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    #[should_panic]
    fn test_harray_get_out_of_bounds() {
        let arr = TColgpHArray1OfPnt2d::new(5, 10);
        let _ = arr.get(11);
    }
}

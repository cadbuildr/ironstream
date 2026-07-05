// FILE: t_colgp_h_array1_of_lin2d.rs
// occt: TColgp_HArray1OfLin2d

use std::sync::Arc;

/// A 2D line representation (gp_Lin2d in OCCT).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lin2d {
    /// Point X on line
    pub px: f64,
    /// Point Y on line
    pub py: f64,
    /// Direction X (normalized)
    pub dx: f64,
    /// Direction Y (normalized)
    pub dy: f64,
}

impl Lin2d {
    /// Creates a 2D line from a point and a direction.
    pub fn new(px: f64, py: f64, dx: f64, dy: f64) -> Self {
        let mag = (dx * dx + dy * dy).sqrt();
        if mag == 0.0 {
            panic!("Cannot create line from zero direction vector");
        }
        Lin2d {
            px,
            py,
            dx: dx / mag,
            dy: dy / mag,
        }
    }
}

/// Handle-based (reference-counted) 1-based Array1 of 2D lines.
#[derive(Debug, Clone)]
pub struct TColgpHArray1OfLin2d {
    data: Arc<TColgpArray1OfLin2dData>,
}

#[derive(Debug)]
struct TColgpArray1OfLin2dData {
    lower: usize,
    upper: usize,
    items: Vec<Lin2d>,
}

impl TColgpHArray1OfLin2d {
    /// Creates a shared handle-based array with bounds [lower, upper].
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper {
            panic!("Lower bound {} exceeds upper bound {}", lower, upper);
        }
        let size = upper - lower + 1;
        TColgpHArray1OfLin2d {
            data: Arc::new(TColgpArray1OfLin2dData {
                lower,
                upper,
                items: vec![Lin2d { px: 0.0, py: 0.0, dx: 1.0, dy: 0.0 }; size],
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
    pub fn get(&self, idx: usize) -> Lin2d {
        if idx < self.data.lower || idx > self.data.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.data.lower, self.data.upper);
        }
        self.data.items[idx - self.data.lower]
    }

    /// Gets a reference to the element at the given index.
    pub fn at(&self, idx: usize) -> &Lin2d {
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
    fn test_lin2d_creation() {
        let l = Lin2d::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(l.px, 1.0);
        assert_eq!(l.py, 2.0);
    }

    #[test]
    fn test_harray_creation_with_bounds() {
        let arr = TColgpHArray1OfLin2d::new(1, 4);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 4);
        assert_eq!(arr.len(), 4);
    }

    #[test]
    fn test_harray_clone_shares_data() {
        let arr1 = TColgpHArray1OfLin2d::new(1, 3);
        let arr2 = arr1.clone();
        assert_eq!(arr1.lower(), arr2.lower());
        assert_eq!(arr1.upper(), arr2.upper());
    }

    #[test]
    fn test_harray_get() {
        let arr = TColgpHArray1OfLin2d::new(1, 3);
        let l = arr.get(1);
        assert_eq!(l.px, 0.0);
        assert_eq!(l.py, 0.0);
    }

    #[test]
    #[should_panic]
    fn test_harray_get_out_of_bounds() {
        let arr = TColgpHArray1OfLin2d::new(5, 10);
        let _ = arr.get(11);
    }
}

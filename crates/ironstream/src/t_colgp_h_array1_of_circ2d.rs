// FILE: t_colgp_h_array1_of_circ2d.rs
// occt: TColgp_HArray1OfCirc2d

use std::sync::Arc;

/// A simple 2D circle representation (gp_Circ2d in OCCT).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circ2d {
    /// Center X
    pub cx: f64,
    /// Center Y
    pub cy: f64,
    /// Radius
    pub radius: f64,
}

impl Circ2d {
    pub fn new(cx: f64, cy: f64, radius: f64) -> Self {
        Circ2d { cx, cy, radius }
    }
}

/// Handle-based (reference-counted) 1-based Array1 of 2D circles.
#[derive(Debug, Clone)]
pub struct TColgpHArray1OfCirc2d {
    data: Arc<TColgpArray1OfCirc2dData>,
}

#[derive(Debug)]
struct TColgpArray1OfCirc2dData {
    lower: usize,
    upper: usize,
    items: Vec<Circ2d>,
}

impl TColgpHArray1OfCirc2d {
    /// Creates a shared handle-based array with bounds [lower, upper].
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper {
            panic!("Lower bound {} exceeds upper bound {}", lower, upper);
        }
        let size = upper - lower + 1;
        TColgpHArray1OfCirc2d {
            data: Arc::new(TColgpArray1OfCirc2dData {
                lower,
                upper,
                items: vec![Circ2d { cx: 0.0, cy: 0.0, radius: 0.0 }; size],
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
    pub fn get(&self, idx: usize) -> Circ2d {
        if idx < self.data.lower || idx > self.data.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.data.lower, self.data.upper);
        }
        self.data.items[idx - self.data.lower]
    }

    /// Gets a reference to the element at the given index.
    pub fn at(&self, idx: usize) -> &Circ2d {
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
    fn test_harray_creation_with_bounds() {
        let arr = TColgpHArray1OfCirc2d::new(1, 3);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 3);
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn test_harray_clone_shares_data() {
        let arr1 = TColgpHArray1OfCirc2d::new(1, 2);
        let arr2 = arr1.clone();
        // Both should point to the same Arc
        assert_eq!(arr1.lower(), arr2.lower());
        assert_eq!(arr1.upper(), arr2.upper());
    }

    #[test]
    fn test_harray_get() {
        let arr = TColgpHArray1OfCirc2d::new(1, 3);
        let c = arr.get(1);
        assert_eq!(c.cx, 0.0);
        assert_eq!(c.cy, 0.0);
        assert_eq!(c.radius, 0.0);
    }

    #[test]
    fn test_harray_at() {
        let arr = TColgpHArray1OfCirc2d::new(0, 2);
        let c = arr.at(1);
        assert_eq!(c.cx, 0.0);
    }

    #[test]
    #[should_panic]
    fn test_harray_get_out_of_bounds() {
        let arr = TColgpHArray1OfCirc2d::new(5, 10);
        let _ = arr.get(11);
    }
}

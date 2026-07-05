// FILE: t_colgp_h_array1_of_vec2d.rs
// occt: TColgp_HArray1OfVec2d

use std::sync::Arc;

/// A 2D vector (gp_Vec2d in OCCT).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2d {
    /// X component
    pub x: f64,
    /// Y component
    pub y: f64,
}

impl Vec2d {
    /// Creates a 2D vector.
    pub fn new(x: f64, y: f64) -> Self {
        Vec2d { x, y }
    }

    /// Returns the magnitude.
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

/// Handle-based (reference-counted) 1-based Array1 of 2D vectors.
#[derive(Debug, Clone)]
pub struct TColgpHArray1OfVec2d {
    data: Arc<TColgpArray1OfVec2dData>,
}

#[derive(Debug)]
struct TColgpArray1OfVec2dData {
    lower: usize,
    upper: usize,
    items: Vec<Vec2d>,
}

impl TColgpHArray1OfVec2d {
    /// Creates a shared handle-based array with bounds [lower, upper].
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper {
            panic!("Lower bound {} exceeds upper bound {}", lower, upper);
        }
        let size = upper - lower + 1;
        TColgpHArray1OfVec2d {
            data: Arc::new(TColgpArray1OfVec2dData {
                lower,
                upper,
                items: vec![Vec2d { x: 0.0, y: 0.0 }; size],
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
    pub fn get(&self, idx: usize) -> Vec2d {
        if idx < self.data.lower || idx > self.data.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.data.lower, self.data.upper);
        }
        self.data.items[idx - self.data.lower]
    }

    /// Gets a reference to the element at the given index.
    pub fn at(&self, idx: usize) -> &Vec2d {
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
    fn test_vec2d_magnitude() {
        let v = Vec2d::new(3.0, 4.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_harray_creation_with_bounds() {
        let arr = TColgpHArray1OfVec2d::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn test_harray_clone_shares_data() {
        let arr1 = TColgpHArray1OfVec2d::new(1, 3);
        let arr2 = arr1.clone();
        assert_eq!(arr1.lower(), arr2.lower());
        assert_eq!(arr1.upper(), arr2.upper());
    }

    #[test]
    fn test_harray_get() {
        let arr = TColgpHArray1OfVec2d::new(1, 3);
        let v = arr.get(1);
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
    }

    #[test]
    #[should_panic]
    fn test_harray_get_out_of_bounds() {
        let arr = TColgpHArray1OfVec2d::new(5, 10);
        let _ = arr.get(11);
    }
}

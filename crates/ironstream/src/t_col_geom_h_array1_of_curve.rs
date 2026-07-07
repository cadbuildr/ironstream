// FILE: t_col_geom_h_array1_of_curve.rs
// occt: TColGeom_HArray1OfCurve

use std::ops::Index;

/// TColGeom_HArray1OfCurve is a deprecated alias for a 1-based array of curves.
/// This is a Rust port implementing OCCT's 1-based indexing semantics.
pub struct TColGeomHArray1OfCurve {
    data: Vec<Option<String>>,
    lower: i32,
    upper: i32,
}

impl TColGeomHArray1OfCurve {
    /// Creates a new 1-based array with the given bounds.
    pub fn new(lower: i32, upper: i32) -> Self {
        if lower > upper {
            panic!("Lower bound {} must be <= upper bound {}", lower, upper);
        }
        let size = (upper - lower + 1) as usize;
        TColGeomHArray1OfCurve {
            data: vec![None; size],
            lower,
            upper,
        }
    }

    /// Returns the lower bound of the array (1-based indexing).
    pub fn lower(&self) -> i32 {
        self.lower
    }

    /// Returns the upper bound of the array (1-based indexing).
    pub fn upper(&self) -> i32 {
        self.upper
    }

    /// Returns the length of the array.
    pub fn length(&self) -> i32 {
        self.upper - self.lower + 1
    }

    /// Sets a value at the given 1-based index.
    pub fn set(&mut self, idx: i32, value: Option<String>) {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        let pos = (idx - self.lower) as usize;
        self.data[pos] = value;
    }

    /// Gets a reference to the value at the given 1-based index.
    pub fn at(&self, idx: i32) -> Option<&Option<String>> {
        if idx < self.lower || idx > self.upper {
            return None;
        }
        let pos = (idx - self.lower) as usize;
        Some(&self.data[pos])
    }

    /// Gets a mutable reference to the value at the given 1-based index.
    pub fn at_mut(&mut self, idx: i32) -> Option<&mut Option<String>> {
        if idx < self.lower || idx > self.upper {
            return None;
        }
        let pos = (idx - self.lower) as usize;
        Some(&mut self.data[pos])
    }
}

impl Index<i32> for TColGeomHArray1OfCurve {
    type Output = Option<String>;

    fn index(&self, idx: i32) -> &Self::Output {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        let pos = (idx - self.lower) as usize;
        &self.data[pos]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_and_bounds() {
        let arr = TColGeomHArray1OfCurve::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn test_set_and_get() {
        let mut arr = TColGeomHArray1OfCurve::new(0, 2);
        arr.set(0, Some("c1".to_string()));
        arr.set(1, Some("c2".to_string()));
        arr.set(2, None);

        assert_eq!(arr.at(0), Some(&Some("c1".to_string())));
        assert_eq!(arr.at(1), Some(&Some("c2".to_string())));
        assert_eq!(arr.at(2), Some(&None));
    }

    #[test]
    fn test_index_operator() {
        let mut arr = TColGeomHArray1OfCurve::new(1, 2);
        arr.set(1, Some("line".to_string()));
        assert_eq!(arr[1], Some("line".to_string()));
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds() {
        let arr = TColGeomHArray1OfCurve::new(1, 3);
        let _ = arr[10];
    }
}

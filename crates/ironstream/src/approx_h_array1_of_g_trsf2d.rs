// FILE: approx_h_array1_of_g_trsf2d.rs
// occt: Approx_HArray1OfGTrsf2d

//! Deprecated NCollection alias: HArray1<Trsf2d>

use std::sync::Arc;

/// 2D transformation (stub).
#[derive(Clone, Debug)]
pub struct Trsf2d {
    pub m11: f64,
    pub m12: f64,
    pub m21: f64,
    pub m22: f64,
}

/// Handle-based array.
pub struct ApproxHArray1OfGTrsf2d {
    data: Arc<Vec<Trsf2d>>,
}

impl ApproxHArray1OfGTrsf2d {
    /// Create array.
    pub fn new(size: usize) -> Self {
        Self {
            data: Arc::new(vec![Trsf2d { m11: 1.0, m12: 0.0, m21: 0.0, m22: 1.0 }; size]),
        }
    }

    /// Get value.
    pub fn get(&self, idx: usize) -> Option<&Trsf2d> {
        self.data.get(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array() {
        let arr = ApproxHArray1OfGTrsf2d::new(3);
        assert_eq!(arr.data.len(), 3);
    }
}

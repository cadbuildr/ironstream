// FILE: approx_array1_of_g_trsf2d.rs
// occt: Approx_Array1OfGTrsf2d

//! Deprecated NCollection alias: Array1<Trsf2d>

/// 2D transformation (stub).
#[derive(Clone, Debug)]
pub struct Trsf2d {
    pub m11: f64,
    pub m12: f64,
    pub m21: f64,
    pub m22: f64,
}

/// Array with 1-based indexing.
pub struct ApproxArray1OfGTrsf2d {
    data: Vec<Trsf2d>,
    lower: usize,
}

impl ApproxArray1OfGTrsf2d {
    /// Create array.
    pub fn new(lower: usize, upper: usize) -> Self {
        Self {
            data: vec![Trsf2d { m11: 1.0, m12: 0.0, m21: 0.0, m22: 1.0 }; upper - lower + 1],
            lower,
        }
    }

    /// Get value.
    pub fn get(&self, idx: usize) -> Option<&Trsf2d> {
        self.data.get(idx - self.lower)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array() {
        let arr = ApproxArray1OfGTrsf2d::new(1, 3);
        assert_eq!(arr.data.len(), 3);
    }
}

// FILE: approx_array1_of_ad_h_surface.rs
// occt: Approx_Array1OfAdHSurface

//! Deprecated NCollection alias: Array1<AdHSurface>

/// Adaptive H-surface (stub).
#[derive(Clone, Debug)]
pub struct AdHSurface {
    pub id: u32,
}

/// Array with 1-based indexing.
pub struct ApproxArray1OfAdHSurface {
    data: Vec<AdHSurface>,
    lower: usize,
}

impl ApproxArray1OfAdHSurface {
    /// Create array.
    pub fn new(lower: usize, upper: usize) -> Self {
        Self {
            data: vec![AdHSurface { id: 0 }; upper - lower + 1],
            lower,
        }
    }

    /// Get value.
    pub fn get(&self, idx: usize) -> Option<&AdHSurface> {
        self.data.get(idx - self.lower)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array() {
        let arr = ApproxArray1OfAdHSurface::new(1, 3);
        assert_eq!(arr.data.len(), 3);
    }
}

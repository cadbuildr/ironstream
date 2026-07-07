// FILE: approx_h_array1_of_ad_h_surface.rs
// occt: Approx_HArray1OfAdHSurface

//! Deprecated NCollection alias: HArray1<AdHSurface>

use std::sync::Arc;

/// Adaptive H-surface (stub).
#[derive(Clone, Debug)]
pub struct AdHSurface {
    pub id: u32,
}

/// Handle-based array.
pub struct ApproxHArray1OfAdHSurface {
    data: Arc<Vec<AdHSurface>>,
}

impl ApproxHArray1OfAdHSurface {
    /// Create array.
    pub fn new(size: usize) -> Self {
        Self {
            data: Arc::new(vec![AdHSurface { id: 0 }; size]),
        }
    }

    /// Get value.
    pub fn get(&self, idx: usize) -> Option<&AdHSurface> {
        self.data.get(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array() {
        let arr = ApproxHArray1OfAdHSurface::new(3);
        assert_eq!(arr.data.len(), 3);
    }
}

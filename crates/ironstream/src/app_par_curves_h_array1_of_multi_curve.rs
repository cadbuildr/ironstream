// FILE: app_par_curves_h_array1_of_multi_curve.rs
// occt: AppParCurves_HArray1OfMultiCurve

//! Deprecated NCollection alias: HArray1<MultiCurve>

use std::sync::Arc;

/// Multi-curve (stub).
#[derive(Clone, Debug)]
pub struct MultiCurve {
    pub id: u32,
}

/// Handle-based array.
pub struct AppParCurvesHArray1OfMultiCurve {
    data: Arc<Vec<MultiCurve>>,
}

impl AppParCurvesHArray1OfMultiCurve {
    /// Create array.
    pub fn new(size: usize) -> Self {
        Self {
            data: Arc::new(vec![MultiCurve { id: 0 }; size]),
        }
    }

    /// Get value.
    pub fn get(&self, idx: usize) -> Option<&MultiCurve> {
        self.data.get(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array() {
        let arr = AppParCurvesHArray1OfMultiCurve::new(5);
        assert_eq!(arr.data.len(), 5);
    }
}

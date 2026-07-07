// FILE: app_par_curves_h_array1_of_multi_point.rs
// occt: AppParCurves_HArray1OfMultiPoint

//! Deprecated NCollection alias: HArray1<MultiPoint>

use std::sync::Arc;

/// Multi-point (stub).
#[derive(Clone, Debug)]
pub struct MultiPoint {
    pub id: u32,
}

/// Handle-based array.
pub struct AppParCurvesHArray1OfMultiPoint {
    data: Arc<Vec<MultiPoint>>,
}

impl AppParCurvesHArray1OfMultiPoint {
    /// Create array.
    pub fn new(size: usize) -> Self {
        Self {
            data: Arc::new(vec![MultiPoint { id: 0 }; size]),
        }
    }

    /// Get value.
    pub fn get(&self, idx: usize) -> Option<&MultiPoint> {
        self.data.get(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array() {
        let arr = AppParCurvesHArray1OfMultiPoint::new(5);
        assert_eq!(arr.data.len(), 5);
    }
}

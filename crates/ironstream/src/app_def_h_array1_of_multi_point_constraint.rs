// FILE: app_def_h_array1_of_multi_point_constraint.rs
// occt: AppDef_HArray1OfMultiPointConstraint

//! Deprecated NCollection alias: HArray1<MultiPointConstraint> (Handle wrapper)

use std::sync::Arc;

/// Multi-point constraint (stub).
#[derive(Clone, Debug)]
pub struct MultiPointConstraint {
    pub id: u32,
}

/// Handle-based array wrapper.
pub struct AppDefHArray1OfMultiPointConstraint {
    data: Arc<Vec<MultiPointConstraint>>,
}

impl AppDefHArray1OfMultiPointConstraint {
    /// Create shared array.
    pub fn new(size: usize) -> Self {
        Self {
            data: Arc::new(vec![MultiPointConstraint { id: 0 }; size]),
        }
    }

    /// Get value.
    pub fn get(&self, idx: usize) -> Option<&MultiPointConstraint> {
        self.data.get(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_array() {
        let arr = AppDefHArray1OfMultiPointConstraint::new(3);
        assert_eq!(arr.data.len(), 3);
    }
}

// FILE: app_def_array1_of_multi_point_constraint.rs
// occt: AppDef_Array1OfMultiPointConstraint

//! Deprecated NCollection alias: Array1<MultiPointConstraint>

/// Multi-point constraint (stub).
#[derive(Clone, Debug)]
pub struct MultiPointConstraint {
    pub id: u32,
}

/// Array with 1-based indexing.
pub struct AppDefArray1OfMultiPointConstraint {
    data: Vec<MultiPointConstraint>,
    lower: usize,
}

impl AppDefArray1OfMultiPointConstraint {
    /// Create array [lower, upper].
    pub fn new(lower: usize, upper: usize) -> Self {
        Self {
            data: vec![MultiPointConstraint { id: 0 }; upper - lower + 1],
            lower,
        }
    }

    /// Get value.
    pub fn get(&self, idx: usize) -> Option<&MultiPointConstraint> {
        self.data.get(idx - self.lower)
    }

    /// Set value.
    pub fn set(&mut self, idx: usize, value: MultiPointConstraint) {
        if let Some(e) = self.data.get_mut(idx - self.lower) {
            *e = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array() {
        let mut arr = AppDefArray1OfMultiPointConstraint::new(1, 3);
        arr.set(1, MultiPointConstraint { id: 10 });
        assert_eq!(arr.get(1).map(|c| c.id), Some(10));
    }
}

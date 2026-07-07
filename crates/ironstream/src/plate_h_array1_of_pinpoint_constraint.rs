// FILE: plate_h_array1_of_pinpoint_constraint.rs
// occt: Plate_HArray1OfPinpointConstraint

//! Deprecated: Plate_HArray1OfPinpointConstraint is a handle wrapper for Plate_Array1OfPinpointConstraint.

/// Pinpoint constraint
#[derive(Debug, Clone)]
pub struct PinpointConstraint {
    id: u32,
}

impl PinpointConstraint {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// Handle array wrapper
#[derive(Debug, Clone)]
pub struct HArray1 {
    data: Vec<PinpointConstraint>,
}

impl HArray1 {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![PinpointConstraint::new(0); size],
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn value(&self, index: usize) -> Option<&PinpointConstraint> {
        self.data.get(index)
    }

    pub fn change_value(&mut self, index: usize) -> Option<&mut PinpointConstraint> {
        self.data.get_mut(index)
    }
}

pub type PlateHArray1OfPinpointConstraint = HArray1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let arr = HArray1::new(5);
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn test_value() {
        let arr = HArray1::new(3);
        assert!(arr.value(0).is_some());
        assert!(arr.value(3).is_none());
    }
}

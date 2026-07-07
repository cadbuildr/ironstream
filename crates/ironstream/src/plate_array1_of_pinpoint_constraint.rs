// FILE: plate_array1_of_pinpoint_constraint.rs
// occt: Plate_Array1OfPinpointConstraint

//! Deprecated: Plate_Array1OfPinpointConstraint is a type alias for NCollection_Array1.

use std::ops::{Index, IndexMut};

/// Pinpoint constraint placeholder
#[derive(Debug, Clone)]
pub struct PinpointConstraint {
    index: usize,
}

impl PinpointConstraint {
    pub fn new(index: usize) -> Self {
        Self { index }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

/// Array1 of pinpoint constraints
#[derive(Debug, Clone)]
pub struct Array1 {
    data: Vec<PinpointConstraint>,
    lower: usize,
}

impl Array1 {
    pub fn new(lower: usize, upper: usize) -> Self {
        let len = upper.saturating_sub(lower) + 1;
        Self {
            data: vec![PinpointConstraint::new(0); len],
            lower,
        }
    }

    pub fn lower(&self) -> usize {
        self.lower
    }

    pub fn upper(&self) -> usize {
        self.lower + self.data.len().saturating_sub(1)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn value(&self, index: usize) -> Option<&PinpointConstraint> {
        if index >= self.lower {
            self.data.get(index - self.lower)
        } else {
            None
        }
    }

    pub fn change_value(&mut self, index: usize) -> Option<&mut PinpointConstraint> {
        if index >= self.lower {
            self.data.get_mut(index - self.lower)
        } else {
            None
        }
    }
}

impl Index<usize> for Array1 {
    type Output = PinpointConstraint;

    fn index(&self, index: usize) -> &Self::Output {
        self.value(index).expect("index out of bounds")
    }
}

impl IndexMut<usize> for Array1 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.change_value(index).expect("index out of bounds")
    }
}

pub type PlateArray1OfPinpointConstraint = Array1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let arr = Array1::new(0, 9);
        assert_eq!(arr.lower(), 0);
        assert_eq!(arr.upper(), 9);
        assert_eq!(arr.len(), 10);
    }

    #[test]
    fn test_value() {
        let arr = Array1::new(1, 5);
        assert!(arr.value(1).is_some());
        assert!(arr.value(0).is_none());
    }

    #[test]
    fn test_change_value() {
        let mut arr = Array1::new(0, 2);
        if let Some(c) = arr.change_value(1) {
            assert_eq!(c.index(), 0);
        }
    }

    #[test]
    fn test_index_access() {
        let arr = Array1::new(0, 2);
        let c = &arr[0];
        assert_eq!(c.index(), 0);
    }
}

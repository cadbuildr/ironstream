// FILE: plate_sequence_of_pinpoint_constraint.rs
// occt: Plate_SequenceOfPinpointConstraint

//! Deprecated: Plate_SequenceOfPinpointConstraint is a sequence type alias.

use std::collections::VecDeque;

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

/// Sequence of constraints
#[derive(Debug, Clone)]
pub struct Sequence {
    constraints: VecDeque<PinpointConstraint>,
}

impl Sequence {
    pub fn new() -> Self {
        Self {
            constraints: VecDeque::new(),
        }
    }

    pub fn append(&mut self, c: PinpointConstraint) {
        self.constraints.push_back(c);
    }

    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    pub fn value(&self, index: usize) -> Option<&PinpointConstraint> {
        self.constraints.get(index)
    }
}

impl Default for Sequence {
    fn default() -> Self {
        Self::new()
    }
}

pub type PlateSequenceOfPinpointConstraint = Sequence;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append() {
        let mut seq = Sequence::new();
        seq.append(PinpointConstraint::new(1));
        assert_eq!(seq.len(), 1);
    }

    #[test]
    fn test_value() {
        let mut seq = Sequence::new();
        seq.append(PinpointConstraint::new(42));
        assert_eq!(seq.value(0).unwrap().id(), 42);
    }
}

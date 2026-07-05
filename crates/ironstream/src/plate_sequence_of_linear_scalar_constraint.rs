// FILE: plate_sequence_of_linear_scalar_constraint.rs
// occt: Plate_SequenceOfLinearScalarConstraint

//! Deprecated: Plate_SequenceOfLinearScalarConstraint is a sequence type alias.

use std::collections::VecDeque;

/// Linear scalar constraint
#[derive(Debug, Clone)]
pub struct LinearScalarConstraint {
    value: f32,
}

impl LinearScalarConstraint {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}

/// Sequence of constraints
#[derive(Debug, Clone)]
pub struct Sequence {
    constraints: VecDeque<LinearScalarConstraint>,
}

impl Sequence {
    pub fn new() -> Self {
        Self {
            constraints: VecDeque::new(),
        }
    }

    pub fn append(&mut self, c: LinearScalarConstraint) {
        self.constraints.push_back(c);
    }

    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    pub fn value(&self, index: usize) -> Option<&LinearScalarConstraint> {
        self.constraints.get(index)
    }
}

impl Default for Sequence {
    fn default() -> Self {
        Self::new()
    }
}

pub type PlateSequenceOfLinearScalarConstraint = Sequence;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append() {
        let mut seq = Sequence::new();
        seq.append(LinearScalarConstraint::new(1.5));
        assert_eq!(seq.len(), 1);
    }

    #[test]
    fn test_value() {
        let mut seq = Sequence::new();
        seq.append(LinearScalarConstraint::new(2.5));
        assert_eq!(seq.value(0).unwrap().value(), 2.5);
    }
}

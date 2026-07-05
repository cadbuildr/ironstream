// FILE: nl_plate_sequence_of_hgpp_constraint.rs
// occt: NLPlate_SequenceOfHGPPConstraint

use std::rc::Rc;
use std::cell::RefCell;

/// NLPlate_HGPPConstraint represents a constraint for non-linear plate surface fitting.
#[derive(Clone, Debug)]
pub struct NlplateHgppConstraint {
    id: i32,
    constraint_type: ConstraintType,
    value: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstraintType {
    Distance,
    Angle,
    Curvature,
    Other,
}

impl NlplateHgppConstraint {
    pub fn new(id: i32, constraint_type: ConstraintType, value: f64) -> Self {
        NlplateHgppConstraint {
            id,
            constraint_type,
            value,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn constraint_type(&self) -> ConstraintType {
        self.constraint_type
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

/// A handle/reference-counted wrapper for NLPlate_HGPPConstraint.
pub type NlplateHgppConstraintHandle = Rc<RefCell<NlplateHgppConstraint>>;

/// Deprecated typedef alias for backward compatibility.
/// Original OCCT: `NCollection_Sequence<opencascade::handle<NLPlate_HGPPConstraint>>`
pub type NlplateSequenceOfHgppConstraint = Vec<NlplateHgppConstraintHandle>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_creation() {
        let constraint =
            NlplateHgppConstraint::new(1, ConstraintType::Distance, 5.0);
        assert_eq!(constraint.id(), 1);
        assert_eq!(constraint.constraint_type(), ConstraintType::Distance);
        assert_eq!(constraint.value(), 5.0);
    }

    #[test]
    fn test_sequence_creation() {
        let sequence: NlplateSequenceOfHgppConstraint = Vec::new();
        assert!(sequence.is_empty());
        assert_eq!(sequence.len(), 0);
    }

    #[test]
    fn test_sequence_push() {
        let mut sequence: NlplateSequenceOfHgppConstraint = Vec::new();

        let c1 = Rc::new(RefCell::new(NlplateHgppConstraint::new(
            1,
            ConstraintType::Distance,
            2.5,
        )));
        let c2 = Rc::new(RefCell::new(NlplateHgppConstraint::new(
            2,
            ConstraintType::Angle,
            45.0,
        )));

        sequence.push(c1.clone());
        sequence.push(c2.clone());

        assert_eq!(sequence.len(), 2);
        assert_eq!(sequence[0].borrow().id(), 1);
        assert_eq!(sequence[1].borrow().id(), 2);
    }

    #[test]
    fn test_sequence_access() {
        let mut sequence: NlplateSequenceOfHgppConstraint = Vec::new();

        let constraint = Rc::new(RefCell::new(NlplateHgppConstraint::new(
            42,
            ConstraintType::Curvature,
            0.1,
        )));
        sequence.push(constraint.clone());

        let retrieved = sequence.get(0).unwrap();
        assert_eq!(retrieved.borrow().id(), 42);
        assert_eq!(retrieved.borrow().constraint_type(), ConstraintType::Curvature);
    }

    #[test]
    fn test_sequence_iteration() {
        let mut sequence: NlplateSequenceOfHgppConstraint = Vec::new();

        for i in 1..=5 {
            let c = Rc::new(RefCell::new(NlplateHgppConstraint::new(
                i,
                ConstraintType::Distance,
                i as f64,
            )));
            sequence.push(c);
        }

        assert_eq!(sequence.len(), 5);

        let mut ids = Vec::new();
        for c_handle in &sequence {
            ids.push(c_handle.borrow().id());
        }
        assert_eq!(ids, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sequence_remove() {
        let mut sequence: NlplateSequenceOfHgppConstraint = Vec::new();

        let c1 = Rc::new(RefCell::new(NlplateHgppConstraint::new(
            1,
            ConstraintType::Distance,
            1.0,
        )));
        let c2 = Rc::new(RefCell::new(NlplateHgppConstraint::new(
            2,
            ConstraintType::Angle,
            2.0,
        )));
        let c3 = Rc::new(RefCell::new(NlplateHgppConstraint::new(
            3,
            ConstraintType::Curvature,
            3.0,
        )));

        sequence.push(c1);
        sequence.push(c2);
        sequence.push(c3);

        assert_eq!(sequence.len(), 3);
        sequence.remove(1);
        assert_eq!(sequence.len(), 2);
        assert_eq!(sequence[0].borrow().id(), 1);
        assert_eq!(sequence[1].borrow().id(), 3);
    }
}

// FILE: geom_plate_h_sequence_of_curve_constraint.rs
// occt: GeomPlate_HSequenceOfCurveConstraint

//! Deprecated: Use Arc<Vec<CurveConstraint>> directly.
//! Alias for backward compatibility with OCCT.

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct CurveConstraint {
    pub curve_id: usize,
    pub order: usize,
}

impl CurveConstraint {
    pub fn new(curve_id: usize, order: usize) -> Self {
        CurveConstraint { curve_id, order }
    }
}

pub type GeomPlateHSequenceOfCurveConstraint = Arc<Vec<CurveConstraint>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_sequence_creation() {
        let vec = vec![CurveConstraint::new(1, 0), CurveConstraint::new(2, 1)];
        let h_seq: GeomPlateHSequenceOfCurveConstraint = Arc::new(vec);

        assert_eq!(h_seq.len(), 2);
        assert_eq!(h_seq[0].curve_id, 1);
        assert_eq!(h_seq[1].order, 1);
    }

    #[test]
    fn test_h_sequence_shared() {
        let vec = vec![CurveConstraint::new(10, 2)];
        let h_seq1 = Arc::new(vec);
        let h_seq2 = Arc::clone(&h_seq1);

        assert_eq!(Arc::strong_count(&h_seq1), 2);
        assert_eq!(h_seq2[0].curve_id, 10);
    }

    #[test]
    fn test_constraint_access() {
        let vec = vec![
            CurveConstraint::new(5, 0),
            CurveConstraint::new(6, 1),
            CurveConstraint::new(7, 2),
        ];
        let h_seq: GeomPlateHSequenceOfCurveConstraint = Arc::new(vec);

        assert_eq!(h_seq.len(), 3);
        assert_eq!(h_seq[2].order, 2);
    }
}

// FILE: geom_plate_h_sequence_of_point_constraint.rs
// occt: GeomPlate_HSequenceOfPointConstraint

//! Deprecated: Use Arc<Vec<PointConstraint>> directly.
//! Alias for backward compatibility with OCCT.

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct PointConstraint {
    pub point_id: usize,
    pub order: usize,
}

impl PointConstraint {
    pub fn new(point_id: usize, order: usize) -> Self {
        PointConstraint { point_id, order }
    }
}

pub type GeomPlateHSequenceOfPointConstraint = Arc<Vec<PointConstraint>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_sequence_creation() {
        let vec = vec![PointConstraint::new(1, 0), PointConstraint::new(2, 1)];
        let h_seq: GeomPlateHSequenceOfPointConstraint = Arc::new(vec);

        assert_eq!(h_seq.len(), 2);
        assert_eq!(h_seq[0].point_id, 1);
        assert_eq!(h_seq[1].order, 1);
    }

    #[test]
    fn test_h_sequence_shared() {
        let vec = vec![PointConstraint::new(10, 2)];
        let h_seq1 = Arc::new(vec);
        let h_seq2 = Arc::clone(&h_seq1);

        assert_eq!(Arc::strong_count(&h_seq1), 2);
        assert_eq!(h_seq2[0].point_id, 10);
    }

    #[test]
    fn test_constraint_access() {
        let vec = vec![
            PointConstraint::new(5, 0),
            PointConstraint::new(6, 1),
            PointConstraint::new(7, 2),
        ];
        let h_seq: GeomPlateHSequenceOfPointConstraint = Arc::new(vec);

        assert_eq!(h_seq.len(), 3);
        assert_eq!(h_seq[2].order, 2);
    }
}

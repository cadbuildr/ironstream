// FILE: geom_plate_sequence_of_point_constraint.rs
// occt: GeomPlate_SequenceOfPointConstraint

//! Deprecated: Use Vec<PointConstraint> directly.
//! Alias for backward compatibility with OCCT.

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

pub type GeomPlateSequenceOfPointConstraint = Vec<PointConstraint>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let mut seq: GeomPlateSequenceOfPointConstraint = Vec::new();
        seq.push(PointConstraint::new(1, 0));

        assert_eq!(seq.len(), 1);
        assert_eq!(seq[0].point_id, 1);
    }

    #[test]
    fn test_sequence_operations() {
        let seq = vec![
            PointConstraint::new(1, 0),
            PointConstraint::new(2, 1),
            PointConstraint::new(3, 2),
        ];

        assert_eq!(seq.len(), 3);
        assert_eq!(seq[2].order, 2);
    }

    #[test]
    fn test_sequence_iteration() {
        let seq = vec![
            PointConstraint::new(10, 0),
            PointConstraint::new(20, 1),
        ];

        let ids: Vec<usize> = seq.iter().map(|p| p.point_id).collect();
        assert_eq!(ids, vec![10, 20]);
    }
}

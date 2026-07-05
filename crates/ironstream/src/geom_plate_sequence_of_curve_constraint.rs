// FILE: geom_plate_sequence_of_curve_constraint.rs
// occt: GeomPlate_SequenceOfCurveConstraint

//! Deprecated: Use Vec<CurveConstraint> directly.
//! Alias for backward compatibility with OCCT.

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

pub type GeomPlateSequenceOfCurveConstraint = Vec<CurveConstraint>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let mut seq: GeomPlateSequenceOfCurveConstraint = Vec::new();
        seq.push(CurveConstraint::new(1, 0));

        assert_eq!(seq.len(), 1);
        assert_eq!(seq[0].curve_id, 1);
    }

    #[test]
    fn test_sequence_operations() {
        let seq = vec![
            CurveConstraint::new(1, 0),
            CurveConstraint::new(2, 1),
            CurveConstraint::new(3, 2),
        ];

        assert_eq!(seq.len(), 3);
        assert_eq!(seq[2].order, 2);
    }

    #[test]
    fn test_sequence_iteration() {
        let seq = vec![
            CurveConstraint::new(10, 0),
            CurveConstraint::new(20, 1),
        ];

        let ids: Vec<usize> = seq.iter().map(|c| c.curve_id).collect();
        assert_eq!(ids, vec![10, 20]);
    }
}

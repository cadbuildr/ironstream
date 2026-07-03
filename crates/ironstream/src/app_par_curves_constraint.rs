// FILE: app_par_curves_constraint.rs
// occt: AppParCurves_Constraint

/// Constraint types for approximation curves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppParCurvesConstraint {
    /// No constraint at this point.
    NoConstraint = 0,
    /// Curve passes through this point.
    PassPoint = 1,
    /// Tangency constraint at this point.
    TangencyPoint = 2,
    /// Curvature constraint at this point.
    CurvaturePoint = 3,
}

impl AppParCurvesConstraint {
    /// Converts an integer to constraint type.
    pub fn from_i32(val: i32) -> Option<Self> {
        match val {
            0 => Some(AppParCurvesConstraint::NoConstraint),
            1 => Some(AppParCurvesConstraint::PassPoint),
            2 => Some(AppParCurvesConstraint::TangencyPoint),
            3 => Some(AppParCurvesConstraint::CurvaturePoint),
            _ => None,
        }
    }

    /// Converts constraint to integer.
    pub fn to_i32(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_values() {
        assert_eq!(AppParCurvesConstraint::NoConstraint.to_i32(), 0);
        assert_eq!(AppParCurvesConstraint::PassPoint.to_i32(), 1);
        assert_eq!(AppParCurvesConstraint::TangencyPoint.to_i32(), 2);
        assert_eq!(AppParCurvesConstraint::CurvaturePoint.to_i32(), 3);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(
            AppParCurvesConstraint::from_i32(0),
            Some(AppParCurvesConstraint::NoConstraint)
        );
        assert_eq!(
            AppParCurvesConstraint::from_i32(1),
            Some(AppParCurvesConstraint::PassPoint)
        );
        assert_eq!(
            AppParCurvesConstraint::from_i32(2),
            Some(AppParCurvesConstraint::TangencyPoint)
        );
        assert_eq!(
            AppParCurvesConstraint::from_i32(3),
            Some(AppParCurvesConstraint::CurvaturePoint)
        );
        assert_eq!(AppParCurvesConstraint::from_i32(99), None);
    }

    #[test]
    fn test_round_trip() {
        for i in 0..4 {
            let c = AppParCurvesConstraint::from_i32(i).unwrap();
            assert_eq!(c.to_i32(), i);
        }
    }
}

// FILE: app_par_curves_constraint_couple.rs
// occt: AppParCurves_ConstraintCouple

use crate::app_par_curves_constraint::AppParCurvesConstraint;

/// Associates an index and a constraint for an object.
/// Used by AppDef_Variational when performing approximations.
#[derive(Debug, Clone, Copy)]
pub struct AppParCurvesConstraintCouple {
    index: i32,
    constraint: AppParCurvesConstraint,
}

impl AppParCurvesConstraintCouple {
    /// Creates an indefinite constraint couple.
    pub fn new() -> Self {
        AppParCurvesConstraintCouple {
            index: -1,
            constraint: AppParCurvesConstraint::NoConstraint,
        }
    }

    /// Creates a couple with given index and constraint.
    pub fn with_index_and_constraint(index: i32, constraint: AppParCurvesConstraint) -> Self {
        AppParCurvesConstraintCouple { index, constraint }
    }

    /// Returns the index of the constraint object.
    pub fn index(&self) -> i32 {
        self.index
    }

    /// Returns the constraint of the object.
    pub fn constraint(&self) -> AppParCurvesConstraint {
        self.constraint
    }

    /// Changes the index of the constraint object.
    pub fn set_index(&mut self, index: i32) {
        self.index = index;
    }

    /// Changes the constraint of the object.
    pub fn set_constraint(&mut self, constraint: AppParCurvesConstraint) {
        self.constraint = constraint;
    }
}

impl Default for AppParCurvesConstraintCouple {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cc = AppParCurvesConstraintCouple::new();
        assert_eq!(cc.index(), -1);
        assert_eq!(cc.constraint(), AppParCurvesConstraint::NoConstraint);
    }

    #[test]
    fn test_with_values() {
        let cc = AppParCurvesConstraintCouple::with_index_and_constraint(
            5,
            AppParCurvesConstraint::TangencyPoint,
        );
        assert_eq!(cc.index(), 5);
        assert_eq!(cc.constraint(), AppParCurvesConstraint::TangencyPoint);
    }

    #[test]
    fn test_set_values() {
        let mut cc = AppParCurvesConstraintCouple::new();
        cc.set_index(3);
        cc.set_constraint(AppParCurvesConstraint::PassPoint);
        assert_eq!(cc.index(), 3);
        assert_eq!(cc.constraint(), AppParCurvesConstraint::PassPoint);
    }

    #[test]
    fn test_default() {
        let cc = AppParCurvesConstraintCouple::default();
        assert_eq!(cc.index(), -1);
        assert_eq!(cc.constraint(), AppParCurvesConstraint::NoConstraint);
    }
}

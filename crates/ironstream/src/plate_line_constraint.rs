// FILE: plate_line_constraint.rs
// occt: Plate_LineConstraint

use crate::plate_d1::XY;
use crate::plate_linear_scalar_constraint::PlateLinearScalarConstraint;

/// Plate_LineConstraint: Constraint a point to belong to a straight line
pub struct PlateLineConstraint {
    lsc: PlateLinearScalarConstraint,
}

impl PlateLineConstraint {
    pub fn new(_point2d: XY, _iu: i32, _iv: i32) -> Self {
        PlateLineConstraint {
            lsc: PlateLinearScalarConstraint::new(),
        }
    }

    pub fn lsc(&self) -> &PlateLinearScalarConstraint {
        &self.lsc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pt = XY::new(1.0, 2.0);
        let lc = PlateLineConstraint::new(pt, 0, 0);
        assert_eq!(lc.lsc().get_ppc().len(), 0);
    }
}

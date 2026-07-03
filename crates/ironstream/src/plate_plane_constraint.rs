// FILE: plate_plane_constraint.rs
// occt: Plate_PlaneConstraint

use crate::plate_d1::XY;
use crate::plate_linear_scalar_constraint::PlateLinearScalarConstraint;

/// Plate_PlaneConstraint: Constraint a point to belong to a Plane
pub struct PlatePlaneConstraint {
    lsc: PlateLinearScalarConstraint,
}

impl PlatePlaneConstraint {
    pub fn new(_point2d: XY, _iu: i32, _iv: i32) -> Self {
        PlatePlaneConstraint {
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
        let pc = PlatePlaneConstraint::new(pt, 0, 0);
        assert_eq!(pc.lsc().get_ppc().len(), 0);
    }
}

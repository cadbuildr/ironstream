// FILE: plate_pinpoint_constraint.rs
// occt: Plate_PinpointConstraint

use crate::plate_d1::{XY, XYZ};

/// Plate_PinpointConstraint: Define a constraint on the Plate
#[derive(Clone, Copy, Debug)]
pub struct PlatePinpointConstraint {
    value: XYZ,
    pnt2d: XY,
    idu: i32,
    idv: i32,
}

impl PlatePinpointConstraint {
    pub fn new() -> Self {
        PlatePinpointConstraint {
            value: XYZ::new(0.0, 0.0, 0.0),
            pnt2d: XY::new(0.0, 0.0),
            idu: 0,
            idv: 0,
        }
    }

    pub fn with_values(point2d: XY, imposed_value: XYZ, iu: i32, iv: i32) -> Self {
        PlatePinpointConstraint {
            value: imposed_value,
            pnt2d: point2d,
            idu: iu,
            idv: iv,
        }
    }

    pub fn pnt2d(&self) -> &XY {
        &self.pnt2d
    }

    pub fn idu(&self) -> i32 {
        self.idu
    }

    pub fn idv(&self) -> i32 {
        self.idv
    }

    pub fn value(&self) -> &XYZ {
        &self.value
    }
}

impl Default for PlatePinpointConstraint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let ppc = PlatePinpointConstraint::new();
        assert_eq!(ppc.idu(), 0);
        assert_eq!(ppc.idv(), 0);
        assert_eq!(ppc.value().x, 0.0);
    }

    #[test]
    fn test_with_values() {
        let pt = XY::new(1.0, 2.0);
        let val = XYZ::new(3.0, 4.0, 5.0);
        let ppc = PlatePinpointConstraint::with_values(pt, val, 1, 2);
        assert_eq!(ppc.idu(), 1);
        assert_eq!(ppc.idv(), 2);
        assert_eq!(ppc.value().x, 3.0);
        assert_eq!(ppc.pnt2d().x, 1.0);
    }

    #[test]
    fn test_clone() {
        let pt = XY::new(5.0, 6.0);
        let val = XYZ::new(7.0, 8.0, 9.0);
        let ppc1 = PlatePinpointConstraint::with_values(pt, val, 3, 4);
        let ppc2 = ppc1;
        assert_eq!(ppc2.idu(), 3);
        assert_eq!(ppc2.value().z, 9.0);
    }
}

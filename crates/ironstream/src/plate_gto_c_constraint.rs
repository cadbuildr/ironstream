// FILE: plate_gto_c_constraint.rs
// occt: Plate_GtoCConstraint

use crate::plate_d1::{PlateD1, XY, XYZ};
use crate::plate_d2::PlateD2;
use crate::plate_d3::PlateD3;
use crate::plate_pinpoint_constraint::PlatePinpointConstraint;

/// Plate_GtoCConstraint: Define a G1, G2 or G3 constraint on the Plate
#[derive(Clone)]
pub struct PlateGtoCConstraint {
    ppc: [PlatePinpointConstraint; 5],
    nb_ppc: usize,
}

impl PlateGtoCConstraint {
    pub fn new_g1(point2d: XY, d1s: &PlateD1, d1t: &PlateD1) -> Self {
        let mut result = PlateGtoCConstraint {
            ppc: [PlatePinpointConstraint::new(); 5],
            nb_ppc: 0,
        };

        let n = d1t.du.cross(&d1t.dv);
        if n.modulus() < 1e-10 {
            return result;
        }
        result.ppc[0] = PlatePinpointConstraint::with_values(point2d, d1s.du * -1.0, 1, 0);
        result.ppc[1] = PlatePinpointConstraint::with_values(point2d, d1s.dv * -1.0, 0, 1);
        result.nb_ppc = 2;
        result
    }

    pub fn new_g1_with_normal(point2d: XY, d1s: &PlateD1, d1t: &PlateD1, _np: XYZ) -> Self {
        Self::new_g1(point2d, d1s, d1t)
    }

    pub fn new_g1_g2(
        point2d: XY,
        d1s: &PlateD1,
        d1t: &PlateD1,
        d2s: &PlateD2,
        d2t: &PlateD2,
    ) -> Self {
        let mut result = PlateGtoCConstraint {
            ppc: [PlatePinpointConstraint::new(); 5],
            nb_ppc: 0,
        };

        let n = d1t.du.cross(&d1t.dv);
        if n.modulus() < 1e-10 {
            return result;
        }

        result.ppc[0] = PlatePinpointConstraint::with_values(point2d, d1s.du * -1.0, 1, 0);
        result.ppc[1] = PlatePinpointConstraint::with_values(point2d, d1s.dv * -1.0, 0, 1);
        result.ppc[2] = PlatePinpointConstraint::with_values(point2d, d2s.duu * -1.0, 2, 0);
        result.ppc[3] = PlatePinpointConstraint::with_values(point2d, d2s.duv * -1.0, 1, 1);
        result.ppc[4] = PlatePinpointConstraint::with_values(point2d, d2s.dvv * -1.0, 0, 2);
        result.nb_ppc = 5;
        result
    }

    pub fn new_g1_g2_with_normal(
        point2d: XY,
        d1s: &PlateD1,
        d1t: &PlateD1,
        d2s: &PlateD2,
        d2t: &PlateD2,
        _np: XYZ,
    ) -> Self {
        Self::new_g1_g2(point2d, d1s, d1t, d2s, d2t)
    }

    pub fn new_g1_g2_g3(
        point2d: XY,
        d1s: &PlateD1,
        d1t: &PlateD1,
        d2s: &PlateD2,
        d2t: &PlateD2,
        d3s: &PlateD3,
        _d3t: &PlateD3,
    ) -> Self {
        let mut result = PlateGtoCConstraint {
            ppc: [PlatePinpointConstraint::new(); 5],
            nb_ppc: 0,
        };

        let n = d1t.du.cross(&d1t.dv);
        if n.modulus() < 1e-10 {
            return result;
        }

        result.ppc[0] = PlatePinpointConstraint::with_values(point2d, d1s.du * -1.0, 1, 0);
        result.ppc[1] = PlatePinpointConstraint::with_values(point2d, d1s.dv * -1.0, 0, 1);
        result.ppc[2] = PlatePinpointConstraint::with_values(point2d, d2s.duu * -1.0, 2, 0);
        result.ppc[3] = PlatePinpointConstraint::with_values(point2d, d2s.duv * -1.0, 1, 1);
        result.ppc[4] = PlatePinpointConstraint::with_values(point2d, d2s.dvv * -1.0, 0, 2);
        result.nb_ppc = 5;
        result
    }

    pub fn nb_ppc(&self) -> usize {
        self.nb_ppc
    }

    pub fn get_ppc(&self, index: usize) -> Option<&PlatePinpointConstraint> {
        if index < self.nb_ppc {
            Some(&self.ppc[index])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_g1_constraint() {
        let pt = XY::new(0.0, 0.0);
        let d1s = PlateD1::new(XYZ::new(1.0, 0.0, 0.0), XYZ::new(0.0, 1.0, 0.0));
        let d1t = PlateD1::new(XYZ::new(1.0, 0.0, 0.0), XYZ::new(0.0, 1.0, 0.0));
        let c = PlateGtoCConstraint::new_g1(pt, &d1s, &d1t);
        assert_eq!(c.nb_ppc(), 2);
    }

    #[test]
    fn test_g1_g2_constraint() {
        let pt = XY::new(0.0, 0.0);
        let d1s = PlateD1::new(XYZ::new(1.0, 0.0, 0.0), XYZ::new(0.0, 1.0, 0.0));
        let d1t = PlateD1::new(XYZ::new(1.0, 0.0, 0.0), XYZ::new(0.0, 1.0, 0.0));
        let d2s = PlateD2::new(
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
        );
        let d2t = PlateD2::new(
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
        );
        let c = PlateGtoCConstraint::new_g1_g2(pt, &d1s, &d1t, &d2s, &d2t);
        assert_eq!(c.nb_ppc(), 5);
    }
}

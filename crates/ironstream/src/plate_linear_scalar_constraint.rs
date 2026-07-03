// FILE: plate_linear_scalar_constraint.rs
// occt: Plate_LinearScalarConstraint

use crate::plate_d1::XYZ;
use crate::plate_pinpoint_constraint::PlatePinpointConstraint;

/// Plate_LinearScalarConstraint: Define one or several constraints as linear combination of
/// the X, Y and Z components of a set of PinPointConstraint
#[derive(Clone, Debug)]
pub struct PlateLinearScalarConstraint {
    ppc: Vec<PlatePinpointConstraint>,
    // coeff is stored as 2D: row-major, indexed [row][col]
    coeff: Vec<Vec<XYZ>>,
}

impl PlateLinearScalarConstraint {
    pub fn new() -> Self {
        PlateLinearScalarConstraint {
            ppc: Vec::new(),
            coeff: Vec::new(),
        }
    }

    pub fn from_single(ppc1: PlatePinpointConstraint, coeff: XYZ) -> Self {
        PlateLinearScalarConstraint {
            ppc: vec![ppc1],
            coeff: vec![vec![coeff]],
        }
    }

    pub fn from_arrays(ppc: Vec<PlatePinpointConstraint>, coeff: Vec<Vec<XYZ>>) -> Self {
        PlateLinearScalarConstraint { ppc, coeff }
    }

    pub fn with_dimensions(col_len: usize, row_len: usize) -> Self {
        PlateLinearScalarConstraint {
            ppc: vec![PlatePinpointConstraint::new(); col_len],
            coeff: vec![vec![XYZ::new(0.0, 0.0, 0.0); col_len]; row_len],
        }
    }

    pub fn get_ppc(&self) -> &[PlatePinpointConstraint] {
        &self.ppc
    }

    pub fn coeff(&self) -> &[Vec<XYZ>] {
        &self.coeff
    }

    pub fn set_ppc(&mut self, index: usize, value: PlatePinpointConstraint) {
        if index < self.ppc.len() {
            self.ppc[index] = value;
        }
    }

    pub fn set_coeff(&mut self, row: usize, col: usize, value: XYZ) {
        if row < self.coeff.len() && col < self.coeff[row].len() {
            self.coeff[row][col] = value;
        }
    }
}

impl Default for PlateLinearScalarConstraint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let lsc = PlateLinearScalarConstraint::new();
        assert_eq!(lsc.get_ppc().len(), 0);
        assert_eq!(lsc.coeff().len(), 0);
    }

    #[test]
    fn test_from_single() {
        let ppc = PlatePinpointConstraint::new();
        let coeff = XYZ::new(1.0, 2.0, 3.0);
        let lsc = PlateLinearScalarConstraint::from_single(ppc, coeff);
        assert_eq!(lsc.get_ppc().len(), 1);
        assert_eq!(lsc.coeff().len(), 1);
        assert_eq!(lsc.coeff()[0][0].x, 1.0);
    }

    #[test]
    fn test_with_dimensions() {
        let lsc = PlateLinearScalarConstraint::with_dimensions(2, 3);
        assert_eq!(lsc.get_ppc().len(), 2);
        assert_eq!(lsc.coeff().len(), 3);
        assert_eq!(lsc.coeff()[0].len(), 2);
    }

    #[test]
    fn test_set_ppc() {
        let mut lsc = PlateLinearScalarConstraint::with_dimensions(2, 2);
        let ppc = PlatePinpointConstraint::new();
        lsc.set_ppc(0, ppc);
        assert_eq!(lsc.get_ppc()[0].idu(), 0);
    }

    #[test]
    fn test_set_coeff() {
        let mut lsc = PlateLinearScalarConstraint::with_dimensions(2, 2);
        let coeff = XYZ::new(5.0, 6.0, 7.0);
        lsc.set_coeff(0, 0, coeff);
        assert_eq!(lsc.coeff()[0][0].x, 5.0);
    }
}

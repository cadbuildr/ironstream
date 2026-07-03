// FILE: plate_linear_xyz_constraint.rs
// occt: Plate_LinearXYZConstraint

use crate::plate_pinpoint_constraint::PlatePinpointConstraint;

/// Plate_LinearXYZConstraint: Define one or several constraints as linear combination of
/// PinPointConstraint. Unlike LinearScalarConstraint, usage of this constraint preserves X, Y and Z uncoupling.
#[derive(Clone, Debug)]
pub struct PlateLinearXYZConstraint {
    ppc: Vec<PlatePinpointConstraint>,
    // coeff is stored as 2D: row-major, indexed [row][col]
    coeff: Vec<Vec<f64>>,
}

impl PlateLinearXYZConstraint {
    pub fn new() -> Self {
        PlateLinearXYZConstraint {
            ppc: Vec::new(),
            coeff: Vec::new(),
        }
    }

    pub fn from_arrays(ppc: Vec<PlatePinpointConstraint>, coeff: Vec<Vec<f64>>) -> Self {
        PlateLinearXYZConstraint { ppc, coeff }
    }

    pub fn with_dimensions(col_len: usize, row_len: usize) -> Self {
        PlateLinearXYZConstraint {
            ppc: vec![PlatePinpointConstraint::new(); col_len],
            coeff: vec![vec![0.0; col_len]; row_len],
        }
    }

    pub fn get_ppc(&self) -> &[PlatePinpointConstraint] {
        &self.ppc
    }

    pub fn coeff(&self) -> &[Vec<f64>] {
        &self.coeff
    }

    pub fn set_ppc(&mut self, index: usize, value: PlatePinpointConstraint) {
        if index < self.ppc.len() {
            self.ppc[index] = value;
        }
    }

    pub fn set_coeff(&mut self, row: usize, col: usize, value: f64) {
        if row < self.coeff.len() && col < self.coeff[row].len() {
            self.coeff[row][col] = value;
        }
    }
}

impl Default for PlateLinearXYZConstraint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let lxyzc = PlateLinearXYZConstraint::new();
        assert_eq!(lxyzc.get_ppc().len(), 0);
        assert_eq!(lxyzc.coeff().len(), 0);
    }

    #[test]
    fn test_with_dimensions() {
        let lxyzc = PlateLinearXYZConstraint::with_dimensions(2, 3);
        assert_eq!(lxyzc.get_ppc().len(), 2);
        assert_eq!(lxyzc.coeff().len(), 3);
    }

    #[test]
    fn test_set_coeff() {
        let mut lxyzc = PlateLinearXYZConstraint::with_dimensions(2, 2);
        lxyzc.set_coeff(0, 0, 5.5);
        assert_eq!(lxyzc.coeff()[0][0], 5.5);
    }
}

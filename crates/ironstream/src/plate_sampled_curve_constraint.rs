// FILE: plate_sampled_curve_constraint.rs
// occt: Plate_SampledCurveConstraint

use crate::plate_linear_xyz_constraint::PlateLinearXYZConstraint;
use crate::plate_pinpoint_constraint::PlatePinpointConstraint;

/// Plate_SampledCurveConstraint: Define m PinPointConstraint driven by m unknown
pub struct PlateSampledCurveConstraint {
    lxyzc: PlateLinearXYZConstraint,
}

impl PlateSampledCurveConstraint {
    pub fn new(ppc_vec: &[PlatePinpointConstraint], n: usize) -> Self {
        let col_len = ppc_vec.len();
        let row_len = if n > 0 { n } else { col_len };
        let coeff = vec![vec![1.0; col_len]; row_len];

        PlateSampledCurveConstraint {
            lxyzc: PlateLinearXYZConstraint::from_arrays(ppc_vec.to_vec(), coeff),
        }
    }

    pub fn lxyzc(&self) -> &PlateLinearXYZConstraint {
        &self.lxyzc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ppc = vec![PlatePinpointConstraint::new()];
        let scc = PlateSampledCurveConstraint::new(&ppc, 1);
        assert_eq!(scc.lxyzc().get_ppc().len(), 1);
    }

    #[test]
    fn test_multiple_constraints() {
        let ppc = vec![
            PlatePinpointConstraint::new(),
            PlatePinpointConstraint::new(),
            PlatePinpointConstraint::new(),
        ];
        let scc = PlateSampledCurveConstraint::new(&ppc, 2);
        assert_eq!(scc.lxyzc().get_ppc().len(), 3);
        assert_eq!(scc.lxyzc().coeff().len(), 2);
    }
}

// FILE: plate_global_translation_constraint.rs
// occt: Plate_GlobalTranslationConstraint

use crate::plate_d1::XY;
use crate::plate_linear_xyz_constraint::PlateLinearXYZConstraint;
use crate::plate_pinpoint_constraint::PlatePinpointConstraint;

/// Plate_GlobalTranslationConstraint: Force a set of UV points to translate without deformation
pub struct PlateGlobalTranslationConstraint {
    lxyzc: PlateLinearXYZConstraint,
}

impl PlateGlobalTranslationConstraint {
    pub fn new(points: &[XY]) -> Self {
        let mut ppc_vec = Vec::new();
        for &pt in points {
            ppc_vec.push(PlatePinpointConstraint::with_values(
                pt,
                crate::plate_d1::XYZ::new(1.0, 0.0, 0.0),
                0,
                0,
            ));
        }

        let col_len = points.len();
        let mut coeff = vec![vec![0.0; col_len]; 3];
        for i in 0..col_len {
            coeff[0][i] = 1.0;
            coeff[1][i] = 1.0;
            coeff[2][i] = 1.0;
        }

        let lxyzc = PlateLinearXYZConstraint::from_arrays(ppc_vec, coeff);

        PlateGlobalTranslationConstraint { lxyzc }
    }

    pub fn lxyzc(&self) -> &PlateLinearXYZConstraint {
        &self.lxyzc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_points() {
        let points = vec![XY::new(0.0, 0.0), XY::new(1.0, 1.0)];
        let gtc = PlateGlobalTranslationConstraint::new(&points);
        assert_eq!(gtc.lxyzc().get_ppc().len(), 2);
        assert_eq!(gtc.lxyzc().coeff().len(), 3);
    }

    #[test]
    fn test_single_point() {
        let points = vec![XY::new(0.5, 0.5)];
        let gtc = PlateGlobalTranslationConstraint::new(&points);
        assert_eq!(gtc.lxyzc().get_ppc().len(), 1);
    }

    #[test]
    fn test_empty_points() {
        let points: Vec<XY> = vec![];
        let gtc = PlateGlobalTranslationConstraint::new(&points);
        assert_eq!(gtc.lxyzc().get_ppc().len(), 0);
    }
}

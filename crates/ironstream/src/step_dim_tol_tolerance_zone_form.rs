// FILE: step_dim_tol_tolerance_zone_form.rs
// occt: StepDimTol_ToleranceZoneForm

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToleranceZoneForm {
    Cylindrical,
    Spherical,
    Planar,
    Linear,
    Conical,
}

impl ToleranceZoneForm {
    pub fn to_string(&self) -> &'static str {
        match self {
            ToleranceZoneForm::Cylindrical => "CYLINDRICAL",
            ToleranceZoneForm::Spherical => "SPHERICAL",
            ToleranceZoneForm::Planar => "PLANAR",
            ToleranceZoneForm::Linear => "LINEAR",
            ToleranceZoneForm::Conical => "CONICAL",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_variants() {
        let cyl = ToleranceZoneForm::Cylindrical;
        let sph = ToleranceZoneForm::Spherical;
        assert_ne!(cyl, sph);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(ToleranceZoneForm::Cylindrical.to_string(), "CYLINDRICAL");
        assert_eq!(ToleranceZoneForm::Spherical.to_string(), "SPHERICAL");
        assert_eq!(ToleranceZoneForm::Planar.to_string(), "PLANAR");
    }
}

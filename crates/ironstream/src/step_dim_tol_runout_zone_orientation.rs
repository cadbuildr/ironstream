// FILE: step_dim_tol_runout_zone_orientation.rs
// occt: StepDimTol_RunoutZoneOrientation

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunoutZoneOrientation {
    Axial,
    Radial,
}

impl RunoutZoneOrientation {
    pub fn to_string(&self) -> &'static str {
        match self {
            RunoutZoneOrientation::Axial => "AXIAL",
            RunoutZoneOrientation::Radial => "RADIAL",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_variants() {
        let axial = RunoutZoneOrientation::Axial;
        let radial = RunoutZoneOrientation::Radial;
        assert_ne!(axial, radial);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(RunoutZoneOrientation::Axial.to_string(), "AXIAL");
        assert_eq!(RunoutZoneOrientation::Radial.to_string(), "RADIAL");
    }
}

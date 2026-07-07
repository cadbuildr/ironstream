// FILE: step_geom_trimming_preference.rs
// occt: StepGeom_TrimmingPreference

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepGeomTrimmingPreference {
    Cartesian,
    Parameter,
    Unspecified,
}

impl StepGeomTrimmingPreference {
    pub fn as_str(&self) -> &'static str {
        match self {
            StepGeomTrimmingPreference::Cartesian => "Cartesian",
            StepGeomTrimmingPreference::Parameter => "Parameter",
            StepGeomTrimmingPreference::Unspecified => "Unspecified",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trimming_preference_variants() {
        let cartesian = StepGeomTrimmingPreference::Cartesian;
        let parameter = StepGeomTrimmingPreference::Parameter;
        let unspecified = StepGeomTrimmingPreference::Unspecified;

        assert_eq!(cartesian.as_str(), "Cartesian");
        assert_eq!(parameter.as_str(), "Parameter");
        assert_eq!(unspecified.as_str(), "Unspecified");
    }

    #[test]
    fn test_trimming_preference_equality() {
        let pref1 = StepGeomTrimmingPreference::Parameter;
        let pref2 = StepGeomTrimmingPreference::Parameter;
        assert_eq!(pref1, pref2);
    }

    #[test]
    fn test_trimming_preference_copy() {
        let pref = StepGeomTrimmingPreference::Cartesian;
        let pref_copy = pref;
        assert_eq!(pref, pref_copy);
    }
}

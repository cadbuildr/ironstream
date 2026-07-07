// FILE: step_dim_tol_tolerance_zone_target.rs
// occt: StepDimTol_ToleranceZoneTarget

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToleranceZoneTarget {
    Feature,
    FeatureGroup,
}

impl ToleranceZoneTarget {
    pub fn case_num(&self) -> i32 {
        match self {
            ToleranceZoneTarget::Feature => 1,
            ToleranceZoneTarget::FeatureGroup => 2,
        }
    }

    pub fn is_feature(&self) -> bool {
        matches!(self, ToleranceZoneTarget::Feature)
    }

    pub fn is_feature_group(&self) -> bool {
        matches!(self, ToleranceZoneTarget::FeatureGroup)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_num() {
        let feature = ToleranceZoneTarget::Feature;
        let group = ToleranceZoneTarget::FeatureGroup;
        assert_eq!(feature.case_num(), 1);
        assert_eq!(group.case_num(), 2);
    }

    #[test]
    fn test_is_feature() {
        let feature = ToleranceZoneTarget::Feature;
        assert!(feature.is_feature());
        assert!(!feature.is_feature_group());
    }

    #[test]
    fn test_is_feature_group() {
        let group = ToleranceZoneTarget::FeatureGroup;
        assert!(group.is_feature_group());
        assert!(!group.is_feature());
    }
}

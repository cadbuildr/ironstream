// FILE: adv_app2_var_criterion_type_o.rs
// occt: AdvApp2Var_CriterionType

//! Criterion types for advanced surface approximation.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CriterionType {
    DistanceOverDistance,
    Distance,
    Angle,
    Curvature,
    Combined,
}

impl CriterionType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::DistanceOverDistance => "DistanceOverDistance",
            Self::Distance => "Distance",
            Self::Angle => "Angle",
            Self::Curvature => "Curvature",
            Self::Combined => "Combined",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criterion_type_name() {
        assert_eq!(CriterionType::Distance.name(), "Distance");
    }
}

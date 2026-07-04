// FILE: step_dim_tol_limit_condition.rs
// occt: StepDimTol_LimitCondition

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LimitCondition {
    MaximumLimit,
    MinimumLimit,
    NominalValue,
}

impl LimitCondition {
    pub fn to_string(&self) -> &'static str {
        match self {
            LimitCondition::MaximumLimit => "MAXIMUM_LIMIT",
            LimitCondition::MinimumLimit => "MINIMUM_LIMIT",
            LimitCondition::NominalValue => "NOMINAL_VALUE",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_variants() {
        let max = LimitCondition::MaximumLimit;
        let min = LimitCondition::MinimumLimit;
        let nom = LimitCondition::NominalValue;
        assert_ne!(max, min);
        assert_ne!(min, nom);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            LimitCondition::MaximumLimit.to_string(),
            "MAXIMUM_LIMIT"
        );
        assert_eq!(LimitCondition::MinimumLimit.to_string(), "MINIMUM_LIMIT");
    }
}

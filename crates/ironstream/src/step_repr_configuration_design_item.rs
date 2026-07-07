// FILE: step_repr_configuration_design_item.rs
// occt: StepRepr_ConfigurationDesignItem

/// StepRepr_ConfigurationDesignItem: SELECT type for ConfigurationDesignItem
/// 1 -> ProductDefinition from StepBasic
/// 2 -> ProductDefinitionFormation from StepBasic
#[derive(Clone, Debug)]
pub enum StepReprConfigurationDesignItem {
    ProductDefinition,
    ProductDefinitionFormation,
    Unknown,
}

impl StepReprConfigurationDesignItem {
    /// Empty constructor
    pub fn new() -> Self {
        StepReprConfigurationDesignItem::Unknown
    }

    /// Recognizes a kind of ConfigurationDesignItem select type
    /// Returns case number (1-2) or 0 for unknown
    pub fn case_num(&self) -> i32 {
        match self {
            StepReprConfigurationDesignItem::ProductDefinition => 1,
            StepReprConfigurationDesignItem::ProductDefinitionFormation => 2,
            StepReprConfigurationDesignItem::Unknown => 0,
        }
    }
}

impl Default for StepReprConfigurationDesignItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_numbers() {
        assert_eq!(StepReprConfigurationDesignItem::ProductDefinition.case_num(), 1);
        assert_eq!(
            StepReprConfigurationDesignItem::ProductDefinitionFormation.case_num(),
            2
        );
        assert_eq!(StepReprConfigurationDesignItem::Unknown.case_num(), 0);
    }

    #[test]
    fn test_default() {
        let item = StepReprConfigurationDesignItem::default();
        assert_eq!(item.case_num(), 0);
    }
}

// FILE: step_repr_configuration_effectivity.rs
// occt: StepRepr_ConfigurationEffectivity

/// StepRepr_ConfigurationEffectivity: Representation of STEP entity ConfigurationEffectivity
/// Inherits from StepBasic_ProductDefinitionEffectivity
#[derive(Clone, Debug)]
pub struct StepReprConfigurationEffectivity {
    effectivity_id: String,
    configuration: String,
}

impl StepReprConfigurationEffectivity {
    /// Empty constructor
    pub fn new() -> Self {
        StepReprConfigurationEffectivity {
            effectivity_id: String::new(),
            configuration: String::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, effectivity_id: String, configuration: String) {
        self.effectivity_id = effectivity_id;
        self.configuration = configuration;
    }

    /// Returns field Configuration
    pub fn configuration(&self) -> &str {
        &self.configuration
    }

    /// Set field Configuration
    pub fn set_configuration(&mut self, configuration: String) {
        self.configuration = configuration;
    }

    /// Get effectivity_id
    pub fn effectivity_id(&self) -> &str {
        &self.effectivity_id
    }

    /// Set effectivity_id
    pub fn set_effectivity_id(&mut self, effectivity_id: String) {
        self.effectivity_id = effectivity_id;
    }
}

impl Default for StepReprConfigurationEffectivity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let ce = StepReprConfigurationEffectivity::new();
        assert_eq!(ce.effectivity_id(), "");
        assert_eq!(ce.configuration(), "");
    }

    #[test]
    fn test_init() {
        let mut ce = StepReprConfigurationEffectivity::new();
        ce.init("eff1".to_string(), "config1".to_string());
        assert_eq!(ce.effectivity_id(), "eff1");
        assert_eq!(ce.configuration(), "config1");
    }

    #[test]
    fn test_set_configuration() {
        let mut ce = StepReprConfigurationEffectivity::new();
        ce.set_configuration("new_config".to_string());
        assert_eq!(ce.configuration(), "new_config");
    }

    #[test]
    fn test_set_effectivity_id() {
        let mut ce = StepReprConfigurationEffectivity::new();
        ce.set_effectivity_id("eff2".to_string());
        assert_eq!(ce.effectivity_id(), "eff2");
    }
}

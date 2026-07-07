// FILE: step_basic_effectivity_assignment.rs
// occt: StepBasic_EffectivityAssignment

/// Representation of STEP entity EffectivityAssignment
#[derive(Clone, Debug)]
pub struct EffectivityAssignment {
    assigned_effectivity: Option<String>,
}

impl EffectivityAssignment {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            assigned_effectivity: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, assigned_effectivity: String) {
        self.assigned_effectivity = Some(assigned_effectivity);
    }

    /// Get assigned effectivity
    pub fn assigned_effectivity(&self) -> Option<&str> {
        self.assigned_effectivity.as_deref()
    }

    /// Set assigned effectivity
    pub fn set_assigned_effectivity(&mut self, assigned_effectivity: String) {
        self.assigned_effectivity = Some(assigned_effectivity);
    }
}

impl Default for EffectivityAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let assign = EffectivityAssignment::new();
        assert!(assign.assigned_effectivity().is_none());
    }

    #[test]
    fn test_init() {
        let mut assign = EffectivityAssignment::new();
        assign.init("EFF001".to_string());
        assert_eq!(assign.assigned_effectivity(), Some("EFF001"));
    }

    #[test]
    fn test_set_assigned_effectivity() {
        let mut assign = EffectivityAssignment::new();
        assign.set_assigned_effectivity("EFF002".to_string());
        assert_eq!(assign.assigned_effectivity(), Some("EFF002"));
    }

    #[test]
    fn test_default() {
        let assign = EffectivityAssignment::default();
        assert!(assign.assigned_effectivity().is_none());
    }
}

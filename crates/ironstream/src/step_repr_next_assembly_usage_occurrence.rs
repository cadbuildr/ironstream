// FILE: step_repr_next_assembly_usage_occurrence.rs
// occt: StepRepr_NextAssemblyUsageOccurrence

/// StepRepr_NextAssemblyUsageOccurrence: Representation of STEP entity NextAssemblyUsageOccurrence
/// Inherits from StepRepr_AssemblyComponentUsage
#[derive(Clone, Debug)]
pub struct StepReprNextAssemblyUsageOccurrence {
    identifier: String,
}

impl StepReprNextAssemblyUsageOccurrence {
    /// Empty constructor
    pub fn new() -> Self {
        StepReprNextAssemblyUsageOccurrence {
            identifier: String::new(),
        }
    }

    /// Get identifier
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Set identifier
    pub fn set_identifier(&mut self, identifier: String) {
        self.identifier = identifier;
    }
}

impl Default for StepReprNextAssemblyUsageOccurrence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let nauo = StepReprNextAssemblyUsageOccurrence::new();
        assert_eq!(nauo.identifier(), "");
    }

    #[test]
    fn test_set_identifier() {
        let mut nauo = StepReprNextAssemblyUsageOccurrence::new();
        nauo.set_identifier("assembly1".to_string());
        assert_eq!(nauo.identifier(), "assembly1");
    }
}

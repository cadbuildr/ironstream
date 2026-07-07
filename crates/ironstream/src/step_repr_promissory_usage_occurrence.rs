// FILE: step_repr_promissory_usage_occurrence.rs
// occt: StepRepr_PromissoryUsageOccurrence

/// StepRepr_PromissoryUsageOccurrence: Promissory assembly component usage
/// Inherits from StepRepr_AssemblyComponentUsage
#[derive(Clone, Debug)]
pub struct StepReprPromissoryUsageOccurrence {
    identifier: String,
}

impl StepReprPromissoryUsageOccurrence {
    /// Create a new instance
    pub fn new() -> Self {
        StepReprPromissoryUsageOccurrence {
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

impl Default for StepReprPromissoryUsageOccurrence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let puo = StepReprPromissoryUsageOccurrence::new();
        assert_eq!(puo.identifier(), "");
    }

    #[test]
    fn test_set_identifier() {
        let mut puo = StepReprPromissoryUsageOccurrence::new();
        puo.set_identifier("promise1".to_string());
        assert_eq!(puo.identifier(), "promise1");
    }
}

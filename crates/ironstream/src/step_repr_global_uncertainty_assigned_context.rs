// FILE: step_repr_global_uncertainty_assigned_context.rs
// occt: StepRepr_GlobalUncertaintyAssignedContext

/// StepRepr_GlobalUncertaintyAssignedContext: Context with global uncertainty assigned
/// Inherits from StepRepr_RepresentationContext
#[derive(Clone, Debug)]
pub struct StepReprGlobalUncertaintyAssignedContext {
    context_identifier: String,
    context_type: String,
    uncertainties: Vec<f64>, // Simplified: storing uncertainty values
}

impl StepReprGlobalUncertaintyAssignedContext {
    /// Returns a GlobalUncertaintyAssignedContext
    pub fn new() -> Self {
        StepReprGlobalUncertaintyAssignedContext {
            context_identifier: String::new(),
            context_type: String::new(),
            uncertainties: Vec::new(),
        }
    }

    /// Initialize with all fields
    pub fn init(&mut self, context_id: String, context_type: String, uncertainties: Vec<f64>) {
        self.context_identifier = context_id;
        self.context_type = context_type;
        self.uncertainties = uncertainties;
    }

    /// Set uncertainty values
    pub fn set_uncertainty(&mut self, uncertainties: Vec<f64>) {
        self.uncertainties = uncertainties;
    }

    /// Get uncertainty values
    pub fn uncertainty(&self) -> &[f64] {
        &self.uncertainties
    }

    /// Get uncertainty value by index (1-based)
    pub fn uncertainty_value(&self, num: usize) -> Option<f64> {
        if num > 0 && num <= self.uncertainties.len() {
            Some(self.uncertainties[num - 1])
        } else {
            None
        }
    }

    /// Get number of uncertainties
    pub fn nb_uncertainty(&self) -> usize {
        self.uncertainties.len()
    }

    /// Get context identifier
    pub fn context_identifier(&self) -> &str {
        &self.context_identifier
    }

    /// Get context type
    pub fn context_type(&self) -> &str {
        &self.context_type
    }
}

impl Default for StepReprGlobalUncertaintyAssignedContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let ctx = StepReprGlobalUncertaintyAssignedContext::new();
        assert_eq!(ctx.context_identifier(), "");
        assert_eq!(ctx.context_type(), "");
        assert_eq!(ctx.nb_uncertainty(), 0);
    }

    #[test]
    fn test_init() {
        let mut ctx = StepReprGlobalUncertaintyAssignedContext::new();
        let uncertainties = vec![0.1, 0.2, 0.3];
        ctx.init("ctx1".to_string(), "type1".to_string(), uncertainties);
        assert_eq!(ctx.context_identifier(), "ctx1");
        assert_eq!(ctx.context_type(), "type1");
        assert_eq!(ctx.nb_uncertainty(), 3);
    }

    #[test]
    fn test_uncertainty_value() {
        let mut ctx = StepReprGlobalUncertaintyAssignedContext::new();
        let uncertainties = vec![0.5, 0.6];
        ctx.set_uncertainty(uncertainties);
        assert_eq!(ctx.uncertainty_value(1), Some(0.5));
        assert_eq!(ctx.uncertainty_value(2), Some(0.6));
        assert_eq!(ctx.uncertainty_value(3), None);
    }

    #[test]
    fn test_set_uncertainty() {
        let mut ctx = StepReprGlobalUncertaintyAssignedContext::new();
        assert_eq!(ctx.nb_uncertainty(), 0);
        let uncertainties = vec![0.1, 0.2];
        ctx.set_uncertainty(uncertainties);
        assert_eq!(ctx.nb_uncertainty(), 2);
    }
}

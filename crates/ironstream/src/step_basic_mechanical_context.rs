// FILE: step_basic_mechanical_context.rs
// occt: StepBasic_MechanicalContext

/// Representation of STEP entity MechanicalContext
/// Extends ProductContext with mechanical product semantics
#[derive(Clone, Debug)]
pub struct MechanicalContext {
    name: Option<String>,
}

impl MechanicalContext {
    /// Empty constructor
    pub fn new() -> Self {
        Self { name: None }
    }

    /// Initialize with name
    pub fn init(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
}

impl Default for MechanicalContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ctx = MechanicalContext::new();
        assert!(ctx.name().is_none());
    }

    #[test]
    fn test_init() {
        let mut ctx = MechanicalContext::new();
        ctx.init("context1".to_string());
        assert_eq!(ctx.name(), Some("context1"));
    }

    #[test]
    fn test_set_name() {
        let mut ctx = MechanicalContext::new();
        ctx.set_name("mech_ctx".to_string());
        assert_eq!(ctx.name(), Some("mech_ctx"));
    }

    #[test]
    fn test_default() {
        let ctx = MechanicalContext::default();
        assert!(ctx.name().is_none());
    }
}

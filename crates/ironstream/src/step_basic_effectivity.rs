// FILE: step_basic_effectivity.rs
// occt: StepBasic_Effectivity

/// Representation of STEP entity Effectivity
#[derive(Clone, Debug)]
pub struct Effectivity {
    id: Option<String>,
}

impl Effectivity {
    /// Empty constructor
    pub fn new() -> Self {
        Self { id: None }
    }

    /// Initialize with id
    pub fn init(&mut self, id: String) {
        self.id = Some(id);
    }

    /// Get id
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Set id
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }
}

impl Default for Effectivity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let eff = Effectivity::new();
        assert!(eff.id().is_none());
    }

    #[test]
    fn test_init() {
        let mut eff = Effectivity::new();
        eff.init("EFF001".to_string());
        assert_eq!(eff.id(), Some("EFF001"));
    }

    #[test]
    fn test_set_id() {
        let mut eff = Effectivity::new();
        eff.set_id("EFF002".to_string());
        assert_eq!(eff.id(), Some("EFF002"));
    }

    #[test]
    fn test_default() {
        let eff = Effectivity::default();
        assert!(eff.id().is_none());
    }
}

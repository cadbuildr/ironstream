// FILE: step_basic_external_source.rs
// occt: StepBasic_ExternalSource

/// Representation of STEP entity ExternalSource
#[derive(Clone, Debug)]
pub struct ExternalSource {
    source_id: Option<String>,
}

impl ExternalSource {
    /// Empty constructor
    pub fn new() -> Self {
        Self { source_id: None }
    }

    /// Initialize all fields
    pub fn init(&mut self, source_id: String) {
        self.source_id = Some(source_id);
    }

    /// Get source id
    pub fn source_id(&self) -> Option<&str> {
        self.source_id.as_deref()
    }

    /// Set source id
    pub fn set_source_id(&mut self, source_id: String) {
        self.source_id = Some(source_id);
    }
}

impl Default for ExternalSource {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ext_src = ExternalSource::new();
        assert!(ext_src.source_id().is_none());
    }

    #[test]
    fn test_init() {
        let mut ext_src = ExternalSource::new();
        ext_src.init("src123".to_string());
        assert_eq!(ext_src.source_id(), Some("src123"));
    }

    #[test]
    fn test_set_source_id() {
        let mut ext_src = ExternalSource::new();
        ext_src.set_source_id("src456".to_string());
        assert_eq!(ext_src.source_id(), Some("src456"));
    }

    #[test]
    fn test_default() {
        let ext_src = ExternalSource::default();
        assert!(ext_src.source_id().is_none());
    }
}

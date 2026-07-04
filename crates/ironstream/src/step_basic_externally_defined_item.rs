// FILE: step_basic_externally_defined_item.rs
// occt: StepBasic_ExternallyDefinedItem

/// Representation of STEP entity ExternallyDefinedItem
#[derive(Clone, Debug)]
pub struct ExternallyDefinedItem {
    item_id: Option<String>,
    source: Option<String>,
}

impl ExternallyDefinedItem {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            item_id: None,
            source: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, item_id: String, source: String) {
        self.item_id = Some(item_id);
        self.source = Some(source);
    }

    /// Get item id
    pub fn item_id(&self) -> Option<&str> {
        self.item_id.as_deref()
    }

    /// Set item id
    pub fn set_item_id(&mut self, item_id: String) {
        self.item_id = Some(item_id);
    }

    /// Get source
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    /// Set source
    pub fn set_source(&mut self, source: String) {
        self.source = Some(source);
    }
}

impl Default for ExternallyDefinedItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ext_def_item = ExternallyDefinedItem::new();
        assert!(ext_def_item.item_id().is_none());
        assert!(ext_def_item.source().is_none());
    }

    #[test]
    fn test_init() {
        let mut ext_def_item = ExternallyDefinedItem::new();
        ext_def_item.init("item1".to_string(), "src1".to_string());
        assert_eq!(ext_def_item.item_id(), Some("item1"));
        assert_eq!(ext_def_item.source(), Some("src1"));
    }

    #[test]
    fn test_set_fields() {
        let mut ext_def_item = ExternallyDefinedItem::new();
        ext_def_item.set_item_id("item2".to_string());
        ext_def_item.set_source("src2".to_string());
        assert_eq!(ext_def_item.item_id(), Some("item2"));
        assert_eq!(ext_def_item.source(), Some("src2"));
    }

    #[test]
    fn test_default() {
        let ext_def_item = ExternallyDefinedItem::default();
        assert!(ext_def_item.item_id().is_none());
    }
}

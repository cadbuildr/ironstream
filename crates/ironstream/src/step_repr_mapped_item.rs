// FILE: step_repr_mapped_item.rs
// occt: StepRepr_MappedItem

/// StepRepr_MappedItem: A mapped representation item
/// Inherits from StepRepr_RepresentationItem
#[derive(Clone, Debug)]
pub struct StepReprMappedItem {
    name: String,
    mapping_source: String,  // Simplified: storing identifier
    mapping_target: String,  // Simplified: storing identifier
}

impl StepReprMappedItem {
    /// Returns a MappedItem
    pub fn new() -> Self {
        StepReprMappedItem {
            name: String::new(),
            mapping_source: String::new(),
            mapping_target: String::new(),
        }
    }

    /// Initialize with name and mappings
    pub fn init(&mut self, name: String, mapping_source: String, mapping_target: String) {
        self.name = name;
        self.mapping_source = mapping_source;
        self.mapping_target = mapping_target;
    }

    /// Set mapping source
    pub fn set_mapping_source(&mut self, source: String) {
        self.mapping_source = source;
    }

    /// Get mapping source
    pub fn mapping_source(&self) -> &str {
        &self.mapping_source
    }

    /// Set mapping target
    pub fn set_mapping_target(&mut self, target: String) {
        self.mapping_target = target;
    }

    /// Get mapping target
    pub fn mapping_target(&self) -> &str {
        &self.mapping_target
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for StepReprMappedItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let item = StepReprMappedItem::new();
        assert_eq!(item.name(), "");
        assert_eq!(item.mapping_source(), "");
        assert_eq!(item.mapping_target(), "");
    }

    #[test]
    fn test_init() {
        let mut item = StepReprMappedItem::new();
        item.init(
            "mapped".to_string(),
            "source_map".to_string(),
            "target_item".to_string(),
        );
        assert_eq!(item.name(), "mapped");
        assert_eq!(item.mapping_source(), "source_map");
        assert_eq!(item.mapping_target(), "target_item");
    }

    #[test]
    fn test_set_mapping_source() {
        let mut item = StepReprMappedItem::new();
        item.set_mapping_source("new_source".to_string());
        assert_eq!(item.mapping_source(), "new_source");
    }

    #[test]
    fn test_set_mapping_target() {
        let mut item = StepReprMappedItem::new();
        item.set_mapping_target("new_target".to_string());
        assert_eq!(item.mapping_target(), "new_target");
    }

    #[test]
    fn test_set_name() {
        let mut item = StepReprMappedItem::new();
        item.set_name("new_name".to_string());
        assert_eq!(item.name(), "new_name");
    }
}

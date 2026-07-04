// FILE: step_ap242_item_identified_representation_usage.rs
// occt: StepAP242_ItemIdentifiedRepresentationUsage

/// Representation of STEP AP242 ItemIdentifiedRepresentationUsage entity.
#[derive(Clone, Debug)]
pub struct ItemIdentifiedRepresentationUsage {
    name: String,
    description: String,
}

impl ItemIdentifiedRepresentationUsage {
    pub fn new() -> Self {
        ItemIdentifiedRepresentationUsage {
            name: String::new(),
            description: String::new(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn nb_identified_item(&self) -> usize {
        0
    }
}

impl Default for ItemIdentifiedRepresentationUsage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let usage = ItemIdentifiedRepresentationUsage::new();
        assert_eq!(usage.name(), "");
        assert_eq!(usage.description(), "");
        assert_eq!(usage.nb_identified_item(), 0);
    }

    #[test]
    fn test_set_name() {
        let mut usage = ItemIdentifiedRepresentationUsage::new();
        usage.set_name("test_name".to_string());
        assert_eq!(usage.name(), "test_name");
    }
}

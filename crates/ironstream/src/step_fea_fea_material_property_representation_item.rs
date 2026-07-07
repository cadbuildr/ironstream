// FILE: step_fea_fea_material_property_representation_item.rs
// occt: StepFEA_FeaMaterialPropertyRepresentationItem

/// Representation of STEP entity FeaMaterialPropertyRepresentationItem
#[derive(Debug, Clone)]
pub struct StepFeaFeaMaterialPropertyRepresentationItem;

impl StepFeaFeaMaterialPropertyRepresentationItem {
    /// Creates a new FeaMaterialPropertyRepresentationItem
    pub fn new() -> Self {
        StepFeaFeaMaterialPropertyRepresentationItem
    }
}

impl Default for StepFeaFeaMaterialPropertyRepresentationItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_material_property_representation_item_creation() {
        let item = StepFeaFeaMaterialPropertyRepresentationItem::new();
        let _ = item;
    }

    #[test]
    fn test_fea_material_property_representation_item_default() {
        let item = StepFeaFeaMaterialPropertyRepresentationItem::default();
        let _ = item;
    }
}

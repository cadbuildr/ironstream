// FILE: step_visual_presentation_layer_assignment.rs
// occt: StepVisual_PresentationLayerAssignment

use std::sync::Arc;

#[derive(Clone)]
pub struct HasciiString {
    value: String,
}

impl HasciiString {
    pub fn new(s: String) -> Self {
        HasciiString { value: s }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Clone)]
pub struct LayeredItem {
    // Placeholder: this is a union type that can be PresentationRepresentation or RepresentationItem
    _phantom: std::marker::PhantomData<()>,
}

impl LayeredItem {
    pub fn new() -> Self {
        LayeredItem {
            _phantom: std::marker::PhantomData,
        }
    }
}

pub struct PresentationLayerAssignment {
    name: Option<Arc<HasciiString>>,
    description: Option<Arc<HasciiString>>,
    assigned_items: Option<Arc<Vec<LayeredItem>>>,
}

impl PresentationLayerAssignment {
    pub fn new() -> Self {
        PresentationLayerAssignment {
            name: None,
            description: None,
            assigned_items: None,
        }
    }

    pub fn init(
        &mut self,
        name: Option<Arc<HasciiString>>,
        description: Option<Arc<HasciiString>>,
        assigned_items: Option<Arc<Vec<LayeredItem>>>,
    ) {
        self.name = name;
        self.description = description;
        self.assigned_items = assigned_items;
    }

    pub fn set_name(&mut self, name: Option<Arc<HasciiString>>) {
        self.name = name;
    }

    pub fn name(&self) -> Option<&Arc<HasciiString>> {
        self.name.as_ref()
    }

    pub fn set_description(&mut self, description: Option<Arc<HasciiString>>) {
        self.description = description;
    }

    pub fn description(&self) -> Option<&Arc<HasciiString>> {
        self.description.as_ref()
    }

    pub fn set_assigned_items(&mut self, items: Option<Arc<Vec<LayeredItem>>>) {
        self.assigned_items = items;
    }

    pub fn assigned_items(&self) -> Option<&Arc<Vec<LayeredItem>>> {
        self.assigned_items.as_ref()
    }

    pub fn assigned_items_value(&self, num: usize) -> Option<LayeredItem> {
        self.assigned_items
            .as_ref()
            .and_then(|items| items.get(num).cloned())
    }

    pub fn nb_assigned_items(&self) -> usize {
        self.assigned_items
            .as_ref()
            .map(|items| items.len())
            .unwrap_or(0)
    }
}

impl Default for PresentationLayerAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pla = PresentationLayerAssignment::new();
        assert!(pla.name().is_none());
        assert!(pla.description().is_none());
        assert_eq!(pla.nb_assigned_items(), 0);
    }

    #[test]
    fn test_set_and_get_name() {
        let mut pla = PresentationLayerAssignment::new();
        let name = Arc::new(HasciiString::new("TestName".to_string()));
        pla.set_name(Some(name.clone()));
        assert!(pla.name().is_some());
        assert_eq!(pla.name().unwrap().as_ref().as_str(), "TestName");
    }

    #[test]
    fn test_set_and_get_description() {
        let mut pla = PresentationLayerAssignment::new();
        let desc = Arc::new(HasciiString::new("TestDesc".to_string()));
        pla.set_description(Some(desc.clone()));
        assert!(pla.description().is_some());
        assert_eq!(pla.description().unwrap().as_ref().as_str(), "TestDesc");
    }

    #[test]
    fn test_set_and_get_assigned_items() {
        let mut pla = PresentationLayerAssignment::new();
        let items = vec![LayeredItem::new(), LayeredItem::new()];
        pla.set_assigned_items(Some(Arc::new(items)));
        assert_eq!(pla.nb_assigned_items(), 2);
        assert!(pla.assigned_items_value(0).is_some());
        assert!(pla.assigned_items_value(1).is_some());
        assert!(pla.assigned_items_value(2).is_none());
    }

    #[test]
    fn test_init() {
        let mut pla = PresentationLayerAssignment::new();
        let name = Arc::new(HasciiString::new("Layer1".to_string()));
        let desc = Arc::new(HasciiString::new("Desc1".to_string()));
        let items = vec![LayeredItem::new()];
        pla.init(Some(name.clone()), Some(desc.clone()), Some(Arc::new(items)));

        assert_eq!(pla.name().unwrap().as_ref().as_str(), "Layer1");
        assert_eq!(pla.description().unwrap().as_ref().as_str(), "Desc1");
        assert_eq!(pla.nb_assigned_items(), 1);
    }
}

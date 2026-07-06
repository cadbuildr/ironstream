// FILE: step_shape_shape_dimension_representation.rs
// occt: StepShape_ShapeDimensionRepresentation

use std::sync::Arc;

/// Placeholder for StepShape_ShapeDimensionRepresentationItem
#[derive(Clone, Debug)]
pub struct ShapeDimensionRepresentationItem {
    id: usize,
}

impl ShapeDimensionRepresentationItem {
    pub fn new(id: usize) -> Self {
        ShapeDimensionRepresentationItem { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Placeholder for StepRepr_RepresentationContext
pub struct RepresentationContext {
    name: Arc<str>,
}

impl RepresentationContext {
    pub fn new(name: Arc<str>) -> Self {
        RepresentationContext { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Placeholder for StepRepr_RepresentationItem
pub struct RepresentationItem {
    id: usize,
}

impl RepresentationItem {
    pub fn new(id: usize) -> Self {
        RepresentationItem { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Represents a shape dimension representation in STEP format.
/// Inherits from StepShape_ShapeRepresentation.
pub struct ShapeDimensionRepresentation {
    name: Arc<str>,
    items_ap242: Vec<ShapeDimensionRepresentationItem>,
    context_of_items: Option<Arc<RepresentationContext>>,
}

impl ShapeDimensionRepresentation {
    /// Create a new ShapeDimensionRepresentation
    pub fn new() -> Self {
        ShapeDimensionRepresentation {
            name: Arc::from(""),
            items_ap242: Vec::new(),
            context_of_items: None,
        }
    }

    /// Initialize with AP242 items
    pub fn init_ap242(
        &mut self,
        name: Arc<str>,
        items: Vec<ShapeDimensionRepresentationItem>,
        context: Arc<RepresentationContext>,
    ) {
        self.name = name;
        self.items_ap242 = items;
        self.context_of_items = Some(context);
    }

    /// Initialize with AP214 items (RepresentationItem based)
    pub fn init_ap214(
        &mut self,
        name: Arc<str>,
        _items: Vec<RepresentationItem>,
        context: Arc<RepresentationContext>,
    ) {
        self.name = name;
        self.context_of_items = Some(context);
    }

    /// Set the AP242 items
    pub fn set_items_ap242(&mut self, items: Vec<ShapeDimensionRepresentationItem>) {
        self.items_ap242 = items;
    }

    /// Get the AP242 items
    pub fn items_ap242(&self) -> &[ShapeDimensionRepresentationItem] {
        &self.items_ap242
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name
    pub fn set_name(&mut self, name: Arc<str>) {
        self.name = name;
    }

    /// Get the context of items
    pub fn context_of_items(&self) -> Option<&Arc<RepresentationContext>> {
        self.context_of_items.as_ref()
    }

    /// Set the context of items
    pub fn set_context_of_items(&mut self, context: Arc<RepresentationContext>) {
        self.context_of_items = Some(context);
    }
}

impl Default for ShapeDimensionRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_dimension_representation_creation() {
        let sdr = ShapeDimensionRepresentation::new();
        assert_eq!(sdr.name(), "");
        assert_eq!(sdr.items_ap242().len(), 0);
    }

    #[test]
    fn test_init_ap242() {
        let mut sdr = ShapeDimensionRepresentation::new();
        let items = vec![
            ShapeDimensionRepresentationItem::new(1),
            ShapeDimensionRepresentationItem::new(2),
        ];
        let context = Arc::new(RepresentationContext::new(Arc::from("context_1")));
        let name: Arc<str> = Arc::from("sdr_ap242");

        sdr.init_ap242(name.clone(), items, context.clone());

        assert_eq!(sdr.name(), "sdr_ap242");
        assert_eq!(sdr.items_ap242().len(), 2);
        assert!(sdr.context_of_items().is_some());
    }

    #[test]
    fn test_init_ap214() {
        let mut sdr = ShapeDimensionRepresentation::new();
        let items = vec![RepresentationItem::new(1)];
        let context = Arc::new(RepresentationContext::new(Arc::from("context_2")));
        let name: Arc<str> = Arc::from("sdr_ap214");

        sdr.init_ap214(name.clone(), items, context.clone());

        assert_eq!(sdr.name(), "sdr_ap214");
        assert!(sdr.context_of_items().is_some());
    }

    #[test]
    fn test_set_items_ap242() {
        let mut sdr = ShapeDimensionRepresentation::new();
        let items = vec![
            ShapeDimensionRepresentationItem::new(10),
            ShapeDimensionRepresentationItem::new(20),
            ShapeDimensionRepresentationItem::new(30),
        ];

        sdr.set_items_ap242(items);

        assert_eq!(sdr.items_ap242().len(), 3);
        assert_eq!(sdr.items_ap242()[0].id(), 10);
    }

    #[test]
    fn test_set_context_of_items() {
        let mut sdr = ShapeDimensionRepresentation::new();
        let context = Arc::new(RepresentationContext::new(Arc::from("new_context")));

        sdr.set_context_of_items(context);

        assert!(sdr.context_of_items().is_some());
        assert_eq!(sdr.context_of_items().unwrap().name(), "new_context");
    }
}

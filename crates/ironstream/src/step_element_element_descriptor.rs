// FILE: step_element_element_descriptor.rs
// occt: StepElement_ElementDescriptor

/// Enumeration for element order/topology
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementOrder {
    Linear,
    Quadratic,
    Cubic,
}

/// Representation of STEP entity ElementDescriptor.
#[derive(Clone)]
pub struct ElementDescriptor {
    topology_order: ElementOrder,
    description: Option<String>,
}

impl ElementDescriptor {
    /// Creates a new ElementDescriptor.
    pub fn new() -> Self {
        Self {
            topology_order: ElementOrder::Linear,
            description: None,
        }
    }

    /// Initializes all fields.
    pub fn init(&mut self, topology_order: ElementOrder, description: Option<String>) {
        self.topology_order = topology_order;
        self.description = description;
    }

    /// Returns the topology order.
    pub fn topology_order(&self) -> ElementOrder {
        self.topology_order
    }

    /// Sets the topology order.
    pub fn set_topology_order(&mut self, order: ElementOrder) {
        self.topology_order = order;
    }

    /// Returns the description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Sets the description.
    pub fn set_description(&mut self, desc: Option<String>) {
        self.description = desc;
    }
}

impl Default for ElementDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_order_variants() {
        assert_ne!(ElementOrder::Linear, ElementOrder::Quadratic);
        assert_ne!(ElementOrder::Quadratic, ElementOrder::Cubic);
    }

    #[test]
    fn test_creation() {
        let desc = ElementDescriptor::new();
        assert_eq!(desc.topology_order(), ElementOrder::Linear);
        assert!(desc.description().is_none());
    }

    #[test]
    fn test_init() {
        let mut desc = ElementDescriptor::new();
        desc.init(
            ElementOrder::Quadratic,
            Some("Test Element".to_string()),
        );
        assert_eq!(desc.topology_order(), ElementOrder::Quadratic);
        assert_eq!(desc.description(), Some("Test Element"));
    }

    #[test]
    fn test_set_topology_order() {
        let mut desc = ElementDescriptor::new();
        desc.set_topology_order(ElementOrder::Cubic);
        assert_eq!(desc.topology_order(), ElementOrder::Cubic);
    }

    #[test]
    fn test_set_description() {
        let mut desc = ElementDescriptor::new();
        desc.set_description(Some("My Element".to_string()));
        assert_eq!(desc.description(), Some("My Element"));
    }

    #[test]
    fn test_default() {
        let desc = ElementDescriptor::default();
        assert_eq!(desc.topology_order(), ElementOrder::Linear);
    }
}

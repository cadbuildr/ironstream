// FILE: step_element_surface3d_element_descriptor.rs
// occt: StepElement_Surface3dElementDescriptor

use super::step_element_element2d_shape::Element2dShape;

/// Representation of STEP entity Surface3dElementDescriptor.
#[derive(Clone)]
pub struct Surface3dElementDescriptor {
    topology_order: i32,
    description: Option<String>,
    purpose: Option<Vec<String>>,
    shape: Element2dShape,
}

impl Surface3dElementDescriptor {
    /// Creates a new Surface3dElementDescriptor.
    pub fn new() -> Self {
        Self {
            topology_order: 0,
            description: None,
            purpose: None,
            shape: Element2dShape::Quadrilateral,
        }
    }

    /// Initializes all fields.
    pub fn init(
        &mut self,
        topology_order: i32,
        description: Option<String>,
        purpose: Option<Vec<String>>,
        shape: Element2dShape,
    ) {
        self.topology_order = topology_order;
        self.description = description;
        self.purpose = purpose;
        self.shape = shape;
    }

    pub fn topology_order(&self) -> i32 {
        self.topology_order
    }

    pub fn set_topology_order(&mut self, order: i32) {
        self.topology_order = order;
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_description(&mut self, desc: Option<String>) {
        self.description = desc;
    }

    pub fn purpose(&self) -> Option<&Vec<String>> {
        self.purpose.as_ref()
    }

    pub fn set_purpose(&mut self, p: Option<Vec<String>>) {
        self.purpose = p;
    }

    pub fn shape(&self) -> Element2dShape {
        self.shape
    }

    pub fn set_shape(&mut self, s: Element2dShape) {
        self.shape = s;
    }
}

impl Default for Surface3dElementDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let desc = Surface3dElementDescriptor::new();
        assert_eq!(desc.topology_order(), 0);
        assert!(desc.description().is_none());
        assert_eq!(desc.shape(), Element2dShape::Quadrilateral);
    }

    #[test]
    fn test_init() {
        let mut desc = Surface3dElementDescriptor::new();
        let purposes = vec!["Membrane".to_string(), "Bending".to_string()];

        desc.init(
            1,
            Some("Surface Descriptor".to_string()),
            Some(purposes.clone()),
            Element2dShape::Triangle,
        );

        assert_eq!(desc.topology_order(), 1);
        assert_eq!(desc.description(), Some("Surface Descriptor"));
        assert_eq!(desc.purpose().unwrap().len(), 2);
        assert_eq!(desc.shape(), Element2dShape::Triangle);
    }

    #[test]
    fn test_setters() {
        let mut desc = Surface3dElementDescriptor::new();
        desc.set_topology_order(2);
        desc.set_description(Some("Quad Surface".to_string()));
        desc.set_shape(Element2dShape::Quadrilateral);

        assert_eq!(desc.topology_order(), 2);
        assert_eq!(desc.description(), Some("Quad Surface"));
    }
}

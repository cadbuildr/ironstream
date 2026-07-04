// FILE: step_element_volume3d_element_descriptor.rs
// occt: StepElement_Volume3dElementDescriptor

use super::step_element_volume3d_element_shape::Volume3dElementShape;

/// Representation of STEP entity Volume3dElementDescriptor.
#[derive(Clone)]
pub struct Volume3dElementDescriptor {
    topology_order: i32,
    description: Option<String>,
    purpose: Option<Vec<String>>,
    shape: Volume3dElementShape,
}

impl Volume3dElementDescriptor {
    /// Creates a new Volume3dElementDescriptor.
    pub fn new() -> Self {
        Self {
            topology_order: 0,
            description: None,
            purpose: None,
            shape: Volume3dElementShape::Hexahedron,
        }
    }

    /// Initializes all fields.
    pub fn init(
        &mut self,
        topology_order: i32,
        description: Option<String>,
        purpose: Option<Vec<String>>,
        shape: Volume3dElementShape,
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

    pub fn shape(&self) -> Volume3dElementShape {
        self.shape
    }

    pub fn set_shape(&mut self, s: Volume3dElementShape) {
        self.shape = s;
    }
}

impl Default for Volume3dElementDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let desc = Volume3dElementDescriptor::new();
        assert_eq!(desc.topology_order(), 0);
        assert!(desc.description().is_none());
        assert_eq!(desc.shape(), Volume3dElementShape::Hexahedron);
    }

    #[test]
    fn test_init() {
        let mut desc = Volume3dElementDescriptor::new();
        let purposes = vec!["StressDisplacement".to_string()];

        desc.init(
            2,
            Some("Volume Descriptor".to_string()),
            Some(purposes.clone()),
            Volume3dElementShape::Tetrahedron,
        );

        assert_eq!(desc.topology_order(), 2);
        assert_eq!(desc.description(), Some("Volume Descriptor"));
        assert_eq!(desc.purpose().unwrap().len(), 1);
        assert_eq!(desc.shape(), Volume3dElementShape::Tetrahedron);
    }

    #[test]
    fn test_setters() {
        let mut desc = Volume3dElementDescriptor::new();
        desc.set_topology_order(3);
        desc.set_description(Some("Wedge Element".to_string()));
        desc.set_shape(Volume3dElementShape::Wedge);

        assert_eq!(desc.topology_order(), 3);
        assert_eq!(desc.description(), Some("Wedge Element"));
        assert_eq!(desc.shape(), Volume3dElementShape::Wedge);
    }
}

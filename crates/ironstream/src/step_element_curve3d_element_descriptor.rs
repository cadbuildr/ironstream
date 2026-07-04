// FILE: step_element_curve3d_element_descriptor.rs
// occt: StepElement_Curve3dElementDescriptor

pub struct Curve3dElementDescriptor {
    pub name: Option<String>,
    pub topology_code: i32,
    pub description: Option<String>,
}

impl Curve3dElementDescriptor {
    pub fn new() -> Self {
        Curve3dElementDescriptor {
            name: None,
            topology_code: 0,
            description: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_topology_code(&mut self, code: i32) {
        self.topology_code = code;
    }

    pub fn get_topology_code(&self) -> i32 {
        self.topology_code
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let desc = Curve3dElementDescriptor::new();
        assert!(desc.name.is_none());
        assert_eq!(desc.topology_code, 0);
    }

    #[test]
    fn test_set_topology_code() {
        let mut desc = Curve3dElementDescriptor::new();
        desc.set_topology_code(5);
        assert_eq!(desc.get_topology_code(), 5);
    }

    #[test]
    fn test_set_and_get_name() {
        let mut desc = Curve3dElementDescriptor::new();
        desc.set_name("curve3d".to_string());
        assert_eq!(desc.get_name(), Some("curve3d"));
    }
}

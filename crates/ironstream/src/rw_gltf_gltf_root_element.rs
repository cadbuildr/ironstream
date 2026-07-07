// FILE: rw_gltf_gltf_root_element.rs
// occt: RWGltf_GltfRootElement

//! Root glTF element.

#[derive(Debug, Clone)]
pub struct RootElement {
    version: (u32, u32),
    generator: String,
}

impl RootElement {
    pub fn new() -> Self {
        Self {
            version: (2, 0),
            generator: "IronStream".to_string(),
        }
    }

    pub fn version(&self) -> (u32, u32) {
        self.version
    }

    pub fn generator(&self) -> &str {
        &self.generator
    }

    pub fn set_generator(&mut self, gen: String) {
        self.generator = gen;
    }
}

impl Default for RootElement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let elem = RootElement::new();
        assert_eq!(elem.version(), (2, 0));
    }
}

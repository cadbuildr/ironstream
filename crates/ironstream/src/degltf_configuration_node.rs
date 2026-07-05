// FILE: degltf_configuration_node.rs
// occt: DEGLTF_ConfigurationNode

//! Configuration node for glTF data exchange.

/// DEGLTF_ConfigurationNode: config for glTF provider.
#[derive(Clone, Debug)]
pub struct DegltfConfigurationNode {
    name: String,
    enabled: bool,
}

impl DegltfConfigurationNode {
    /// Create a new configuration node.
    pub fn new(name: &str) -> Self {
        DegltfConfigurationNode {
            name: name.to_string(),
            enabled: true,
        }
    }

    /// Set enabled state.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = DegltfConfigurationNode::new("gltf");
        assert!(node.is_enabled());
    }

    #[test]
    fn test_enable_disable() {
        let mut node = DegltfConfigurationNode::new("gltf");
        node.set_enabled(false);
        assert!(!node.is_enabled());
    }
}

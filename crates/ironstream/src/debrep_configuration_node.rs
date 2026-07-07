// FILE: debrep_configuration_node.rs
// occt: DEBREP_ConfigurationNode

//! Configuration node for BREP data exchange.

/// DEBREP_ConfigurationNode: config for BREP provider.
#[derive(Clone, Debug)]
pub struct DebrepConfigurationNode {
    name: String,
    enabled: bool,
}

impl DebrepConfigurationNode {
    /// Create a new configuration node.
    pub fn new(name: &str) -> Self {
        DebrepConfigurationNode {
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
        let node = DebrepConfigurationNode::new("brep");
        assert!(node.is_enabled());
    }

    #[test]
    fn test_enable_disable() {
        let mut node = DebrepConfigurationNode::new("brep");
        node.set_enabled(false);
        assert!(!node.is_enabled());
    }
}

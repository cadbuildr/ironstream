// FILE: deiges_configuration_node.rs
// occt: DEIGES_ConfigurationNode

/// Configuration node for DEIGES IGES format provider.
pub struct ConfigurationNode {
    parameters: Parameters,
}

pub struct Parameters;

impl ConfigurationNode {
    pub fn new() -> Self {
        ConfigurationNode {
            parameters: Parameters,
        }
    }

    pub fn get_format(&self) -> String {
        "IGES".to_string()
    }

    pub fn get_vendor(&self) -> String {
        "OCC".to_string()
    }

    pub fn is_import_supported(&self) -> bool {
        true
    }

    pub fn is_export_supported(&self) -> bool {
        true
    }
}

impl Default for ConfigurationNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let node = ConfigurationNode::new();
        assert_eq!(node.get_format(), "IGES");
        assert_eq!(node.get_vendor(), "OCC");
        assert!(node.is_import_supported());
        assert!(node.is_export_supported());
    }
}

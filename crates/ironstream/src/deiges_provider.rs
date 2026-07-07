// FILE: deiges_provider.rs
// occt: DEIGES_Provider

/// Provider for IGES format file transfer.
pub struct Provider {
    config: ConfigurationNode,
}

pub struct ConfigurationNode;

impl Provider {
    pub fn new() -> Self {
        Provider {
            config: ConfigurationNode,
        }
    }

    pub fn get_format(&self) -> String {
        "IGES".to_string()
    }

    pub fn get_vendor(&self) -> String {
        "OCC".to_string()
    }
}

impl Default for Provider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let provider = Provider::new();
        assert_eq!(provider.get_format(), "IGES");
        assert_eq!(provider.get_vendor(), "OCC");
    }
}

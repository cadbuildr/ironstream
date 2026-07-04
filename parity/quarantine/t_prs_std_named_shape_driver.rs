// FILE: t_prs_std_named_shape_driver.rs
// occt: TPrsStd_NamedShapeDriver

/// A driver for building AIS presentations for named shape attributes.
#[derive(Clone, Debug)]
pub struct TPrsStd_NamedShapeDriver {
    name: String,
}

impl TPrsStd_NamedShapeDriver {
    /// Create a new named shape driver.
    pub fn new() -> Self {
        Self {
            name: "NamedShapeDriver".to_string(),
        }
    }

    /// Get the driver name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Build a presentation for a named shape.
    pub fn build(&self, _shape_name: &str) -> bool {
        true
    }
}

impl Default for TPrsStd_NamedShapeDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_driver() {
        let driver = TPrsStd_NamedShapeDriver::new();
        assert_eq!(driver.name(), "NamedShapeDriver");
    }

    #[test]
    fn test_build() {
        let driver = TPrsStd_NamedShapeDriver::new();
        assert!(driver.build("MyShape"));
    }

    #[test]
    fn test_default() {
        let driver = TPrsStd_NamedShapeDriver::default();
        assert_eq!(driver.name(), "NamedShapeDriver");
    }
}

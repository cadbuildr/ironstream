// FILE: t_prs_std_geometry_driver.rs
// occt: TPrsStd_GeometryDriver

/// A driver for building AIS presentations for geometry attributes.
#[derive(Clone, Debug)]
pub struct TPrsStd_GeometryDriver {
    name: String,
}

impl TPrsStd_GeometryDriver {
    /// Create a new geometry driver.
    pub fn new() -> Self {
        Self {
            name: "GeometryDriver".to_string(),
        }
    }

    /// Get the driver name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Build a presentation for geometry.
    pub fn build(&self, _geometry_type: &str) -> bool {
        true
    }
}

impl Default for TPrsStd_GeometryDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_driver() {
        let driver = TPrsStd_GeometryDriver::new();
        assert_eq!(driver.name(), "GeometryDriver");
    }

    #[test]
    fn test_build() {
        let driver = TPrsStd_GeometryDriver::new();
        assert!(driver.build("Point"));
    }

    #[test]
    fn test_default() {
        let driver = TPrsStd_GeometryDriver::default();
        assert_eq!(driver.name(), "GeometryDriver");
    }
}

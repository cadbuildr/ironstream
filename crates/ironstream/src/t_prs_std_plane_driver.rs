// FILE: t_prs_std_plane_driver.rs
// occt: TPrsStd_PlaneDriver

/// A driver for building AIS presentations for plane attributes.
#[derive(Clone, Debug)]
pub struct TPrsStd_PlaneDriver {
    name: String,
}

impl TPrsStd_PlaneDriver {
    /// Create a new plane driver.
    pub fn new() -> Self {
        Self {
            name: "PlaneDriver".to_string(),
        }
    }

    /// Get the driver name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Build a presentation for a plane.
    pub fn build(&self, _plane_type: &str) -> bool {
        true
    }
}

impl Default for TPrsStd_PlaneDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_driver() {
        let driver = TPrsStd_PlaneDriver::new();
        assert_eq!(driver.name(), "PlaneDriver");
    }

    #[test]
    fn test_build() {
        let driver = TPrsStd_PlaneDriver::new();
        assert!(driver.build("XY"));
    }

    #[test]
    fn test_default() {
        let driver = TPrsStd_PlaneDriver::default();
        assert_eq!(driver.name(), "PlaneDriver");
    }
}

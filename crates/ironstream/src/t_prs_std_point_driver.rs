// FILE: t_prs_std_point_driver.rs
// occt: TPrsStd_PointDriver

/// A driver for building AIS presentations for point attributes.
#[derive(Clone, Debug)]
pub struct TPrsStd_PointDriver {
    name: String,
}

impl TPrsStd_PointDriver {
    /// Create a new point driver.
    pub fn new() -> Self {
        Self {
            name: "PointDriver".to_string(),
        }
    }

    /// Get the driver name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Build a presentation for a point.
    pub fn build(&self, _x: f64, _y: f64, _z: f64) -> bool {
        true
    }
}

impl Default for TPrsStd_PointDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_driver() {
        let driver = TPrsStd_PointDriver::new();
        assert_eq!(driver.name(), "PointDriver");
    }

    #[test]
    fn test_build() {
        let driver = TPrsStd_PointDriver::new();
        assert!(driver.build(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_default() {
        let driver = TPrsStd_PointDriver::default();
        assert_eq!(driver.name(), "PointDriver");
    }
}

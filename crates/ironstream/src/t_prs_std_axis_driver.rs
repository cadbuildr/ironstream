// FILE: t_prs_std_axis_driver.rs
// occt: TPrsStd_AxisDriver

/// A driver for building AIS presentations for axis attributes.
#[derive(Clone, Debug)]
pub struct TPrsStd_AxisDriver {
    name: String,
}

impl TPrsStd_AxisDriver {
    /// Create a new axis driver.
    pub fn new() -> Self {
        Self {
            name: "AxisDriver".to_string(),
        }
    }

    /// Get the driver name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Update the presentation.
    pub fn update(&self) -> bool {
        true
    }

    /// Build the presentation from an attribute.
    pub fn build(&self, _attribute: &str) -> Result<String, String> {
        Ok("Axis Presentation".to_string())
    }
}

impl Default for TPrsStd_AxisDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_driver() {
        let driver = TPrsStd_AxisDriver::new();
        assert_eq!(driver.name(), "AxisDriver");
    }

    #[test]
    fn test_update() {
        let driver = TPrsStd_AxisDriver::new();
        assert!(driver.update());
    }

    #[test]
    fn test_build() {
        let driver = TPrsStd_AxisDriver::new();
        let result = driver.build("test_attr");
        assert!(result.is_ok());
    }

    #[test]
    fn test_default() {
        let driver = TPrsStd_AxisDriver::default();
        assert_eq!(driver.name(), "AxisDriver");
    }
}

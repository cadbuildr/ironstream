// FILE: d_naming_box_driver.rs
// occt: DNaming_BoxDriver

/// Driver for box creation in the naming framework
#[derive(Clone)]
pub struct DNamingBoxDriver {
    width: f64,
    height: f64,
    depth: f64,
}

impl DNamingBoxDriver {
    /// Creates a new box driver with dimensions
    pub fn new(width: f64, height: f64, depth: f64) -> Self {
        DNamingBoxDriver { width, height, depth }
    }

    /// Executes box creation
    pub fn execute(&self) -> Result<(), String> {
        Err("Box creation not yet implemented".to_string())
    }

    /// Gets the operation name
    pub fn operation_name(&self) -> &'static str {
        "Box"
    }

    /// Gets width
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Gets height
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Gets depth
    pub fn depth(&self) -> f64 {
        self.depth
    }
}

impl Default for DNamingBoxDriver {
    fn default() -> Self {
        Self::new(10.0, 10.0, 10.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let driver = DNamingBoxDriver::new(5.0, 10.0, 15.0);
        assert_eq!(driver.width(), 5.0);
        assert_eq!(driver.height(), 10.0);
        assert_eq!(driver.depth(), 15.0);
    }

    #[test]
    fn test_operation_name() {
        let driver = DNamingBoxDriver::new(1.0, 2.0, 3.0);
        assert_eq!(driver.operation_name(), "Box");
    }

    #[test]
    fn test_default() {
        let driver = DNamingBoxDriver::default();
        assert_eq!(driver.width(), 10.0);
        assert_eq!(driver.height(), 10.0);
        assert_eq!(driver.depth(), 10.0);
    }
}

// FILE: d_naming_cylinder_driver.rs
// occt: DNaming_CylinderDriver

/// Driver for cylinder creation in the naming framework
#[derive(Clone)]
pub struct DNamingCylinderDriver {
    radius: f64,
    height: f64,
}

impl DNamingCylinderDriver {
    /// Creates a new cylinder driver
    pub fn new(radius: f64, height: f64) -> Self {
        DNamingCylinderDriver { radius, height }
    }

    /// Executes cylinder creation
    pub fn execute(&self) -> Result<(), String> {
        Err("Cylinder creation not yet implemented".to_string())
    }

    /// Gets the operation name
    pub fn operation_name(&self) -> &'static str {
        "Cylinder"
    }

    pub fn radius(&self) -> f64 { self.radius }
    pub fn height(&self) -> f64 { self.height }
}

impl Default for DNamingCylinderDriver {
    fn default() -> Self { Self::new(5.0, 10.0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let driver = DNamingCylinderDriver::new(3.0, 20.0);
        assert_eq!(driver.radius(), 3.0);
        assert_eq!(driver.height(), 20.0);
    }

    #[test]
    fn test_operation_name() {
        let driver = DNamingCylinderDriver::new(1.0, 2.0);
        assert_eq!(driver.operation_name(), "Cylinder");
    }
}

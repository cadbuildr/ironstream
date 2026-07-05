// FILE: bin_m_naming.rs
// occt: BinMNaming

/// Storage/Retrieval drivers registry for TNaming attributes.
/// Manages the addition of attribute drivers to the driver table.
pub struct BinMNaming;

impl BinMNaming {
    /// Adds the TNaming attribute drivers to the driver table.
    /// Registers NamedShapeDriver and NamingDriver.
    pub fn add_drivers(
        driver_table: &mut DriverTable,
        _message_driver: Option<String>,
    ) {
        driver_table.add_driver(NamedShapeDriver::new(_message_driver.clone()));
        driver_table.add_driver(NamingDriver::new(_message_driver));
    }
}

/// Mock driver table for managing attribute drivers.
#[derive(Clone, Debug)]
pub struct DriverTable {
    drivers: Vec<String>,
}

impl DriverTable {
    pub fn new() -> Self {
        DriverTable {
            drivers: Vec::new(),
        }
    }

    pub fn add_driver(&mut self, driver: DriverType) {
        match driver {
            DriverType::NamedShape(d) => {
                self.drivers.push(format!("NamedShapeDriver: {:?}", d.name));
            }
            DriverType::Naming(d) => {
                self.drivers.push(format!("NamingDriver: {:?}", d.name));
            }
        }
    }

    pub fn driver_count(&self) -> usize {
        self.drivers.len()
    }

    pub fn has_driver(&self, name: &str) -> bool {
        self.drivers.iter().any(|d| d.contains(name))
    }
}

impl Default for DriverTable {
    fn default() -> Self {
        DriverTable::new()
    }
}

/// Named shape attribute driver.
#[derive(Clone, Debug)]
pub struct NamedShapeDriver {
    message_driver: Option<String>,
    name: String,
}

impl NamedShapeDriver {
    pub fn new(message_driver: Option<String>) -> DriverType {
        DriverType::NamedShape(NamedShapeDriver {
            message_driver,
            name: "NamedShapeDriver".to_string(),
        })
    }
}

/// Naming attribute driver.
#[derive(Clone, Debug)]
pub struct NamingDriver {
    message_driver: Option<String>,
    name: String,
}

impl NamingDriver {
    pub fn new(message_driver: Option<String>) -> DriverType {
        DriverType::Naming(NamingDriver {
            message_driver,
            name: "NamingDriver".to_string(),
        })
    }
}

/// Enumeration of available driver types.
#[derive(Clone, Debug)]
pub enum DriverType {
    NamedShape(NamedShapeDriver),
    Naming(NamingDriver),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_drivers() {
        let mut table = DriverTable::new();
        assert_eq!(table.driver_count(), 0);

        BinMNaming::add_drivers(&mut table, None);

        assert_eq!(table.driver_count(), 2);
        assert!(table.has_driver("NamedShapeDriver"));
        assert!(table.has_driver("NamingDriver"));
    }

    #[test]
    fn test_add_drivers_with_message_driver() {
        let mut table = DriverTable::new();
        BinMNaming::add_drivers(&mut table, Some("MessageDriver".to_string()));

        assert_eq!(table.driver_count(), 2);
    }

    #[test]
    fn test_driver_table_creation() {
        let table = DriverTable::new();
        assert_eq!(table.driver_count(), 0);
    }

    #[test]
    fn test_driver_table_add_driver() {
        let mut table = DriverTable::new();
        let driver = NamedShapeDriver::new(None);
        table.add_driver(driver);

        assert_eq!(table.driver_count(), 1);
        assert!(table.has_driver("NamedShapeDriver"));
    }

    #[test]
    fn test_multiple_add_drivers_calls() {
        let mut table = DriverTable::new();

        BinMNaming::add_drivers(&mut table, None);
        assert_eq!(table.driver_count(), 2);

        BinMNaming::add_drivers(&mut table, None);
        assert_eq!(table.driver_count(), 4);
    }

    #[test]
    fn test_driver_registration_order() {
        let mut table = DriverTable::new();
        BinMNaming::add_drivers(&mut table, None);

        // Both drivers should be present
        assert!(table.has_driver("NamedShapeDriver"));
        assert!(table.has_driver("NamingDriver"));
    }

    #[test]
    fn test_named_shape_driver_creation() {
        let driver = NamedShapeDriver::new(Some("TestDriver".to_string()));
        match driver {
            DriverType::NamedShape(d) => {
                assert_eq!(d.name, "NamedShapeDriver");
                assert_eq!(d.message_driver, Some("TestDriver".to_string()));
            }
            _ => panic!("Expected NamedShapeDriver"),
        }
    }

    #[test]
    fn test_naming_driver_creation() {
        let driver = NamingDriver::new(Some("TestDriver".to_string()));
        match driver {
            DriverType::Naming(d) => {
                assert_eq!(d.name, "NamingDriver");
                assert_eq!(d.message_driver, Some("TestDriver".to_string()));
            }
            _ => panic!("Expected NamingDriver"),
        }
    }
}

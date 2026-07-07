// FILE: bin_mdf.rs
// occt: BinMDF

/// Binary MDF (Model Data Frame) driver utilities.
/// Provides methods to add attribute drivers to the driver table for TDF persistence.
pub struct BinMDF;

impl BinMDF {
    /// Add attribute storage drivers to the driver table.
    pub fn add_drivers(driver_table: &mut DriverTable, _message_driver: Option<String>) {
        // Register all TDF attribute drivers
        driver_table.add_driver("TDF_Attribute");
        driver_table.add_driver("TDF_Reference");
        driver_table.add_driver("TDF_TagSource");
    }
}

/// Mock driver table for attribute driver management.
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

    pub fn add_driver(&mut self, name: &str) {
        self.drivers.push(name.to_string());
    }

    pub fn driver_count(&self) -> usize {
        self.drivers.len()
    }

    pub fn has_driver(&self, name: &str) -> bool {
        self.drivers.iter().any(|d| d.contains(name))
    }

    pub fn clear(&mut self) {
        self.drivers.clear();
    }
}

impl Default for DriverTable {
    fn default() -> Self {
        DriverTable::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_drivers() {
        let mut table = DriverTable::new();
        BinMDF::add_drivers(&mut table, None);
        assert!(table.driver_count() >= 3);
        assert!(table.has_driver("TDF_Attribute"));
    }

    #[test]
    fn test_driver_table_creation() {
        let table = DriverTable::new();
        assert_eq!(table.driver_count(), 0);
    }

    #[test]
    fn test_driver_table_add_driver() {
        let mut table = DriverTable::new();
        table.add_driver("TestDriver");
        assert_eq!(table.driver_count(), 1);
        assert!(table.has_driver("TestDriver"));
    }

    #[test]
    fn test_driver_table_clear() {
        let mut table = DriverTable::new();
        table.add_driver("Driver1");
        table.add_driver("Driver2");
        assert_eq!(table.driver_count(), 2);

        table.clear();
        assert_eq!(table.driver_count(), 0);
    }
}

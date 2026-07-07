// FILE: bin_m_data_std.rs
// occt: BinMDataStd

/// Binary serialization drivers for standard data attributes.
/// Provides methods to register all TDataStd attribute drivers.
pub struct BinMDataStd;

impl BinMDataStd {
    /// Add all standard data attribute drivers to the driver table.
    pub fn add_drivers(driver_table: &mut DriverTable, _message_driver: Option<String>) {
        // Register all TDataStd attribute drivers
        driver_table.add_driver("TDataStd_Integer");
        driver_table.add_driver("TDataStd_Real");
        driver_table.add_driver("TDataStd_String");
        driver_table.add_driver("TDataStd_AsciiString");
        driver_table.add_driver("TDataStd_IntegerArray");
        driver_table.add_driver("TDataStd_RealArray");
        driver_table.add_driver("TDataStd_BooleanArray");
        driver_table.add_driver("TDataStd_IntegerList");
        driver_table.add_driver("TDataStd_RealList");
        driver_table.add_driver("TDataStd_BooleanList");
        driver_table.add_driver("TDataStd_ByteArray");
        driver_table.add_driver("TDataStd_ExtStringArray");
    }
}

/// Driver table for attribute driver management.
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
        BinMDataStd::add_drivers(&mut table, None);
        assert!(table.driver_count() >= 12);
        assert!(table.has_driver("TDataStd_Integer"));
        assert!(table.has_driver("TDataStd_BooleanArray"));
    }

    #[test]
    fn test_driver_table_creation() {
        let table = DriverTable::new();
        assert_eq!(table.driver_count(), 0);
    }
}

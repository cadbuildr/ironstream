// FILE: xml_mdf_map_of_driver.rs
// occt: XmlMDF_MapOfDriver
//
// Faithful port of OCCT XmlMDF_MapOfDriver (Deprecated/NCollectionAliases/XmlMDF_MapOfDriver.hxx),
// a deprecated NCollection_DataMap alias: maps from type names (strings) to XmlMDF_Driver references.
// Used by OCAF XML persistence to look up attribute drivers by type name at read/write time.

use std::collections::HashMap;

/// Local model of a driver entry (minimal stub).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DriverEntry {
    pub type_name: String,
    pub driver_id: u32,
}

/// Map container: type name -> driver entry.
/// Models NCollection_DataMap<TCollection_AsciiString, Handle(XmlMDF_Driver)>.
#[derive(Debug, Clone, Default)]
pub struct MapOfDriver {
    entries: HashMap<String, DriverEntry>,
}

impl MapOfDriver {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Bind (insert or update) a driver entry.
    pub fn bind(&mut self, type_name: &str, driver_id: u32) {
        self.entries.insert(
            type_name.to_string(),
            DriverEntry {
                type_name: type_name.to_string(),
                driver_id,
            },
        );
    }

    /// Find a driver entry by type name.
    pub fn find(&self, type_name: &str) -> Option<&DriverEntry> {
        self.entries.get(type_name)
    }

    /// Check if a type name is in the map.
    pub fn contains(&self, type_name: &str) -> bool {
        self.entries.contains_key(type_name)
    }

    /// Get the number of entries.
    pub fn size(&self) -> usize {
        self.entries.len()
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Iterate over entries.
    pub fn iter(&self) -> impl Iterator<Item = &DriverEntry> {
        self.entries.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_new() {
        let map = MapOfDriver::new();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_bind_and_find() {
        let mut map = MapOfDriver::new();
        map.bind("TDataStd_Integer", 101);
        assert!(map.contains("TDataStd_Integer"));
        let entry = map.find("TDataStd_Integer").unwrap();
        assert_eq!(entry.driver_id, 101);
    }

    #[test]
    fn test_find_nonexistent() {
        let map = MapOfDriver::new();
        assert!(map.find("NonExistent").is_none());
    }

    #[test]
    fn test_contains() {
        let mut map = MapOfDriver::new();
        map.bind("TDataStd_Real", 102);
        assert!(map.contains("TDataStd_Real"));
        assert!(!map.contains("TDataStd_Comment"));
    }

    #[test]
    fn test_clear() {
        let mut map = MapOfDriver::new();
        map.bind("Type1", 1);
        map.bind("Type2", 2);
        map.clear();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_iter() {
        let mut map = MapOfDriver::new();
        map.bind("TypeA", 10);
        map.bind("TypeB", 20);
        let driver_ids: Vec<_> = map.iter().map(|e| e.driver_id).collect();
        assert_eq!(driver_ids.len(), 2);
        assert!(driver_ids.contains(&10));
        assert!(driver_ids.contains(&20));
    }

    #[test]
    fn test_rebind() {
        let mut map = MapOfDriver::new();
        map.bind("TypeX", 100);
        map.bind("TypeX", 200);
        assert_eq!(map.find("TypeX").unwrap().driver_id, 200);
    }
}

// FILE: bin_obj_mgt_r_relocation_table.rs
// occt: BinObjMgt_RRelocationTable

use std::collections::HashMap;

/// Retrieval relocation table that stores a handle to the file header section.
/// Maps integer IDs to transient objects for relocation purposes.
pub struct BinObjMgtRRelocationTable {
    map: HashMap<i32, String>,
    header_data: Option<String>,
}

impl BinObjMgtRRelocationTable {
    /// Creates a new empty relocation table.
    pub fn new() -> Self {
        BinObjMgtRRelocationTable {
            map: HashMap::new(),
            header_data: None,
        }
    }

    /// Returns a handle to the header data of the file being read.
    pub fn get_header_data(&self) -> Option<&str> {
        self.header_data.as_deref()
    }

    /// Sets the storage header data.
    pub fn set_header_data(&mut self, header_data: Option<String>) {
        self.header_data = header_data;
    }

    /// Clears the relocation table.
    pub fn clear(&mut self, _do_release_memory: bool) {
        self.map.clear();
        self.header_data = None;
    }

    /// Inserts a mapping into the table.
    pub fn insert(&mut self, key: i32, value: String) {
        self.map.insert(key, value);
    }

    /// Gets a value from the table.
    pub fn get(&self, key: i32) -> Option<&str> {
        self.map.get(&key).map(|s| s.as_str())
    }
}

impl Default for BinObjMgtRRelocationTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relocation_table_creation() {
        let table = BinObjMgtRRelocationTable::new();
        assert!(table.get_header_data().is_none());
    }

    #[test]
    fn test_set_header_data() {
        let mut table = BinObjMgtRRelocationTable::new();
        table.set_header_data(Some("header".to_string()));
        assert_eq!(table.get_header_data(), Some("header"));
    }

    #[test]
    fn test_insert_and_get() {
        let mut table = BinObjMgtRRelocationTable::new();
        table.insert(1, "object1".to_string());
        table.insert(2, "object2".to_string());
        assert_eq!(table.get(1), Some("object1"));
        assert_eq!(table.get(2), Some("object2"));
        assert_eq!(table.get(3), None);
    }

    #[test]
    fn test_clear() {
        let mut table = BinObjMgtRRelocationTable::new();
        table.set_header_data(Some("header".to_string()));
        table.insert(1, "object1".to_string());
        table.clear(true);
        assert!(table.get_header_data().is_none());
        assert_eq!(table.get(1), None);
    }
}

// FILE: xml_obj_mgt_r_relocation_table.rs
// occt: XmlObjMgt_RRelocationTable

//! Retrieval relocation table for object deserialization from XML.
//!
//! Maps integer object IDs to deserialized objects and provides access to file header data.
//! Used during read operations to track object identities and relocations.

use std::collections::HashMap;

/// Header data for storage/retrieval operations.
///
/// Contains metadata about the file being read.
#[derive(Clone, Debug)]
pub struct StorageHeaderData {
    /// File format version
    pub version: String,
    /// Creation date
    pub creation_date: String,
    /// Any additional metadata
    metadata: HashMap<String, String>,
}

impl StorageHeaderData {
    /// Create new header data.
    pub fn new() -> Self {
        Self {
            version: String::new(),
            creation_date: String::new(),
            metadata: HashMap::new(),
        }
    }

    /// Set a metadata field.
    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Get a metadata field.
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }
}

impl Default for StorageHeaderData {
    fn default() -> Self {
        Self::new()
    }
}

/// Retrieval relocation table.
///
/// Maps integer IDs to objects (represented as Box<dyn Any> in Rust).
/// Stores file header data for access during deserialization.
pub struct XmlObjMgtRRelocationTable {
    /// Map from object ID to the deserialized object
    map: HashMap<i32, Box<dyn std::any::Any>>,
    /// Header data from the file being read
    header_data: Option<Box<StorageHeaderData>>,
}

impl XmlObjMgtRRelocationTable {
    /// Create a new empty relocation table.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            header_data: None,
        }
    }

    /// Get the header data (if set).
    pub fn get_header_data(&self) -> Option<&StorageHeaderData> {
        self.header_data.as_ref().map(|b| b.as_ref())
    }

    /// Set the header data.
    pub fn set_header_data(&mut self, header: StorageHeaderData) {
        self.header_data = Some(Box::new(header));
    }

    /// Clear the header data.
    pub fn clear_header_data(&mut self) {
        self.header_data = None;
    }

    /// Insert an object into the relocation table.
    pub fn insert(&mut self, id: i32, obj: Box<dyn std::any::Any>) {
        self.map.insert(id, obj);
    }

    /// Get an object from the relocation table.
    pub fn get(&self, id: i32) -> Option<&Box<dyn std::any::Any>> {
        self.map.get(&id)
    }

    /// Get a mutable reference to an object.
    pub fn get_mut(&mut self, id: i32) -> Option<&mut Box<dyn std::any::Any>> {
        self.map.get_mut(&id)
    }

    /// Check if an object exists for the given ID.
    pub fn contains(&self, id: i32) -> bool {
        self.map.contains_key(&id)
    }

    /// Remove an object from the relocation table.
    pub fn remove(&mut self, id: i32) -> Option<Box<dyn std::any::Any>> {
        self.map.remove(&id)
    }

    /// Clear the relocation table.
    ///
    /// If `do_release_memory` is true, deallocate all memory.
    pub fn clear(&mut self, do_release_memory: bool) {
        self.header_data = None;
        if do_release_memory {
            self.map.clear();
            self.map.shrink_to_fit();
        } else {
            self.map.clear();
        }
    }

    /// Get the number of entries in the table.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Check if the table is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl Default for XmlObjMgtRRelocationTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_table() {
        let table = XmlObjMgtRRelocationTable::new();
        assert!(table.is_empty());
        assert_eq!(table.len(), 0);
    }

    #[test]
    fn test_insert_and_get() {
        let mut table = XmlObjMgtRRelocationTable::new();
        table.insert(1, Box::new(42i32));

        assert!(table.contains(1));
        assert_eq!(table.len(), 1);
    }

    #[test]
    fn test_remove() {
        let mut table = XmlObjMgtRRelocationTable::new();
        table.insert(1, Box::new(42i32));
        assert!(table.contains(1));

        let _removed = table.remove(1);
        assert!(!table.contains(1));
        assert!(table.is_empty());
    }

    #[test]
    fn test_clear_with_release() {
        let mut table = XmlObjMgtRRelocationTable::new();
        table.insert(1, Box::new(42i32));
        table.insert(2, Box::new(99i32));

        table.clear(true);
        assert!(table.is_empty());
    }

    #[test]
    fn test_clear_without_release() {
        let mut table = XmlObjMgtRRelocationTable::new();
        table.insert(1, Box::new(42i32));

        table.clear(false);
        assert!(table.is_empty());
    }

    #[test]
    fn test_header_data() {
        let mut table = XmlObjMgtRRelocationTable::new();
        let mut header = StorageHeaderData::new();
        header.version = "1.0".to_string();
        header.creation_date = "2024-01-01".to_string();

        table.set_header_data(header);
        assert!(table.get_header_data().is_some());

        let retrieved = table.get_header_data().unwrap();
        assert_eq!(retrieved.version, "1.0");
        assert_eq!(retrieved.creation_date, "2024-01-01");
    }

    #[test]
    fn test_header_metadata() {
        let mut header = StorageHeaderData::new();
        header.set_metadata("app", "test_app");
        header.set_metadata("format", "xml");

        assert_eq!(header.get_metadata("app"), Some("test_app"));
        assert_eq!(header.get_metadata("format"), Some("xml"));
        assert_eq!(header.get_metadata("missing"), None);
    }

    #[test]
    fn test_multiple_entries() {
        let mut table = XmlObjMgtRRelocationTable::new();
        for i in 0..10 {
            table.insert(i, Box::new(i * 2));
        }

        assert_eq!(table.len(), 10);
        for i in 0..10 {
            assert!(table.contains(i));
        }
    }

    #[test]
    fn test_clear_header_data() {
        let mut table = XmlObjMgtRRelocationTable::new();
        let header = StorageHeaderData::new();
        table.set_header_data(header);
        assert!(table.get_header_data().is_some());

        table.clear_header_data();
        assert!(table.get_header_data().is_none());
    }
}

// FILE: xml_obj_mgt_s_relocation_table.rs
// occt: XmlObjMgt_SRelocationTable

//! Storage relocation table for object serialization to XML.
//!
//! Maps objects to their integer indices for storage and provides access to file header data.
//! Used during write operations to track object identities and generate relocation information.

use std::collections::HashMap;

/// Simplified representation of header data for storage operations.
#[derive(Clone, Debug)]
pub struct StorageHeaderDataSrl {
    /// File format version
    pub version: String,
    /// Creation date
    pub creation_date: String,
    /// Metadata
    metadata: HashMap<String, String>,
}

impl StorageHeaderDataSrl {
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

impl Default for StorageHeaderDataSrl {
    fn default() -> Self {
        Self::new()
    }
}

/// Storage relocation table.
///
/// An indexed map that stores objects and assigns them integer indices.
/// Used to track which objects have been serialized and their relocation indices.
/// Also stores file header data for access during serialization.
pub struct XmlObjMgtSRelocationTable {
    /// Indexed sequence of objects (Any type)
    objects: Vec<Box<dyn std::any::Any>>,
    /// Map from object address to index (using a simplified approach with insertion order)
    index_map: std::collections::HashMap<String, usize>,
    /// Header data from the file being written
    header_data: Option<Box<StorageHeaderDataSrl>>,
    /// Counter for generating unique keys
    object_counter: usize,
}

impl XmlObjMgtSRelocationTable {
    /// Create a new empty storage relocation table.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            index_map: std::collections::HashMap::new(),
            header_data: None,
            object_counter: 0,
        }
    }

    /// Get the header data (if set).
    pub fn get_header_data(&self) -> Option<&StorageHeaderDataSrl> {
        self.header_data.as_ref().map(|b| b.as_ref())
    }

    /// Set the header data.
    pub fn set_header_data(&mut self, header: StorageHeaderDataSrl) {
        self.header_data = Some(Box::new(header));
    }

    /// Clear the header data.
    pub fn clear_header_data(&mut self) {
        self.header_data = None;
    }

    /// Add an object to the table and return its index.
    ///
    /// Returns the 1-based index assigned to this object.
    pub fn add(&mut self, obj: Box<dyn std::any::Any>) -> usize {
        self.objects.push(obj);
        let index = self.objects.len();
        self.index_map
            .insert(self.object_counter.to_string(), index);
        self.object_counter += 1;
        index
    }

    /// Get an object by its index (1-based).
    pub fn get(&self, index: usize) -> Option<&Box<dyn std::any::Any>> {
        if index > 0 && index <= self.objects.len() {
            Some(&self.objects[index - 1])
        } else {
            None
        }
    }

    /// Get a mutable reference to an object by index (1-based).
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Box<dyn std::any::Any>> {
        if index > 0 && index <= self.objects.len() {
            Some(&mut self.objects[index - 1])
        } else {
            None
        }
    }

    /// Get the number of objects in the table.
    pub fn len(&self) -> usize {
        self.objects.len()
    }

    /// Check if the table is empty.
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// Clear the relocation table.
    ///
    /// If `do_release_memory` is true, deallocate all memory.
    pub fn clear(&mut self, do_release_memory: bool) {
        self.header_data = None;
        if do_release_memory {
            self.objects.clear();
            self.objects.shrink_to_fit();
            self.index_map.clear();
        } else {
            self.objects.clear();
            self.index_map.clear();
        }
        self.object_counter = 0;
    }
}

impl Default for XmlObjMgtSRelocationTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_table() {
        let table = XmlObjMgtSRelocationTable::new();
        assert!(table.is_empty());
        assert_eq!(table.len(), 0);
    }

    #[test]
    fn test_add_and_get() {
        let mut table = XmlObjMgtSRelocationTable::new();

        let idx1 = table.add(Box::new(42i32));
        assert_eq!(idx1, 1);
        assert_eq!(table.len(), 1);

        let idx2 = table.add(Box::new(99i32));
        assert_eq!(idx2, 2);
        assert_eq!(table.len(), 2);
    }

    #[test]
    fn test_get_by_index() {
        let mut table = XmlObjMgtSRelocationTable::new();
        table.add(Box::new(42i32));
        table.add(Box::new(99i32));

        assert!(table.get(1).is_some());
        assert!(table.get(2).is_some());
        assert!(table.get(0).is_none());
        assert!(table.get(3).is_none());
    }

    #[test]
    fn test_clear_with_release() {
        let mut table = XmlObjMgtSRelocationTable::new();
        table.add(Box::new(42i32));
        table.add(Box::new(99i32));

        table.clear(true);
        assert!(table.is_empty());
        assert_eq!(table.len(), 0);
    }

    #[test]
    fn test_clear_without_release() {
        let mut table = XmlObjMgtSRelocationTable::new();
        table.add(Box::new(42i32));

        table.clear(false);
        assert!(table.is_empty());
    }

    #[test]
    fn test_header_data() {
        let mut table = XmlObjMgtSRelocationTable::new();
        let mut header = StorageHeaderDataSrl::new();
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
        let mut header = StorageHeaderDataSrl::new();
        header.set_metadata("app", "test_app");
        header.set_metadata("format", "xml");

        assert_eq!(header.get_metadata("app"), Some("test_app"));
        assert_eq!(header.get_metadata("format"), Some("xml"));
    }

    #[test]
    fn test_multiple_additions() {
        let mut table = XmlObjMgtSRelocationTable::new();
        for i in 0..10 {
            let idx = table.add(Box::new(i));
            assert_eq!(idx, i + 1);
        }

        assert_eq!(table.len(), 10);
        for i in 1..=10 {
            assert!(table.get(i).is_some());
        }
    }

    #[test]
    fn test_clear_header_data() {
        let mut table = XmlObjMgtSRelocationTable::new();
        let header = StorageHeaderDataSrl::new();
        table.set_header_data(header);
        assert!(table.get_header_data().is_some());

        table.clear_header_data();
        assert!(table.get_header_data().is_none());
    }
}

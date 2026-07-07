// FILE: std_storage_data.rs
// occt: StdStorage_Data

/// Storage data container
pub struct StorageData {
    header: String,
    data_items: Vec<String>,
}

impl StorageData {
    /// Create a new storage data
    pub fn new() -> Self {
        StorageData {
            header: String::new(),
            data_items: Vec::new(),
        }
    }

    /// Set the header
    pub fn set_header(&mut self, header: &str) {
        self.header = header.to_string();
    }

    /// Get the header
    pub fn header(&self) -> &str {
        &self.header
    }

    /// Add a data item
    pub fn add_data_item(&mut self, item: &str) {
        self.data_items.push(item.to_string());
    }

    /// Get data items
    pub fn data_items(&self) -> &[String] {
        &self.data_items
    }
}

impl Default for StorageData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let data = StorageData::new();
        assert_eq!(data.header(), "");
        assert!(data.data_items().is_empty());
    }

    #[test]
    fn test_set_header() {
        let mut data = StorageData::new();
        data.set_header("MyHeader");
        assert_eq!(data.header(), "MyHeader");
    }

    #[test]
    fn test_add_data_item() {
        let mut data = StorageData::new();
        data.add_data_item("item1");
        data.add_data_item("item2");

        assert_eq!(data.data_items().len(), 2);
    }
}

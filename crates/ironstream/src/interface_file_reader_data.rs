// FILE: interface_file_reader_data.rs
// occt: Interface_FileReaderData

/// Data structure for file reading operations.
#[derive(Clone, Debug)]
pub struct InterfaceFileReaderData {
    records: Vec<String>,
}

impl InterfaceFileReaderData {
    /// Creates an empty FileReaderData
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    /// Adds a record
    pub fn add_record(&mut self, record: String) {
        self.records.push(record);
    }

    /// Returns the count of records
    pub fn count(&self) -> usize {
        self.records.len()
    }

    /// Gets a record by 1-indexed position
    pub fn record(&self, num: usize) -> Option<&str> {
        if num >= 1 && num <= self.records.len() {
            Some(&self.records[num - 1])
        } else {
            None
        }
    }
}

impl Default for InterfaceFileReaderData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let data = InterfaceFileReaderData::new();
        assert_eq!(data.count(), 0);
    }

    #[test]
    fn test_add_record() {
        let mut data = InterfaceFileReaderData::new();
        data.add_record("record1".to_string());
        data.add_record("record2".to_string());
        assert_eq!(data.count(), 2);
        assert_eq!(data.record(1), Some("record1"));
        assert_eq!(data.record(2), Some("record2"));
    }
}

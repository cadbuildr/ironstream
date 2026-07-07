// FILE: pcdm_reader_filter.rs
// occt: PCDM_ReaderFilter

use std::collections::HashSet;

/// Filter modes for appending to documents
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppendMode {
    Forbid = 0,
    Protect = 1,
    Overwrite = 2,
}

/// Filter for controlling what attributes are read from a document
pub struct PCDMReaderFilter {
    skip: HashSet<String>,
    read: HashSet<String>,
    append_mode: AppendMode,
}

impl PCDMReaderFilter {
    /// Create an empty filter (read all)
    pub fn new() -> Self {
        PCDMReaderFilter {
            skip: HashSet::new(),
            read: HashSet::new(),
            append_mode: AppendMode::Forbid,
        }
    }

    /// Add a type to skip
    pub fn add_skipped(&mut self, type_name: &str) {
        self.skip.insert(type_name.to_string());
    }

    /// Add a type to read
    pub fn add_read(&mut self, type_name: &str) {
        self.read.insert(type_name.to_string());
    }

    /// Check if an attribute should be read
    pub fn is_passed(&self, attribute_type: &str) -> bool {
        if !self.read.is_empty() {
            self.read.contains(attribute_type)
        } else {
            !self.skip.contains(attribute_type)
        }
    }

    /// Get the append mode
    pub fn mode(&self) -> AppendMode {
        self.append_mode
    }

    /// Set the append mode
    pub fn set_mode(&mut self, mode: AppendMode) {
        self.append_mode = mode;
    }

    /// Check if append mode is enabled
    pub fn is_append_mode(&self) -> bool {
        self.append_mode != AppendMode::Forbid
    }

    /// Clear the filter
    pub fn clear(&mut self) {
        self.skip.clear();
        self.read.clear();
        self.append_mode = AppendMode::Forbid;
    }
}

impl Default for PCDMReaderFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_creation() {
        let filter = PCDMReaderFilter::new();
        assert_eq!(filter.mode(), AppendMode::Forbid);
    }

    #[test]
    fn test_add_skipped() {
        let mut filter = PCDMReaderFilter::new();
        filter.add_skipped("SkipType");
        assert!(!filter.is_passed("SkipType"));
    }

    #[test]
    fn test_add_read() {
        let mut filter = PCDMReaderFilter::new();
        filter.add_read("ReadType");
        assert!(filter.is_passed("ReadType"));
    }

    #[test]
    fn test_read_overrides_skip() {
        let mut filter = PCDMReaderFilter::new();
        filter.add_skipped("Type");
        filter.add_read("Type");
        assert!(filter.is_passed("Type"));
    }

    #[test]
    fn test_append_mode() {
        let mut filter = PCDMReaderFilter::new();
        filter.set_mode(AppendMode::Protect);
        assert!(filter.is_append_mode());
        assert_eq!(filter.mode(), AppendMode::Protect);
    }

    #[test]
    fn test_clear() {
        let mut filter = PCDMReaderFilter::new();
        filter.add_skipped("Type");
        filter.clear();
        assert_eq!(filter.skip.len(), 0);
    }
}

// FILE: bin_l_drivers_document_section.rs
// occt: BinLDrivers_DocumentSection

/// Document section descriptor for binary OCAF format.
/// Represents an independent part of the document that may be read before or after OCAF data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BinLDriversDocumentSection {
    name: String,
    offset: u64,
    length: u64,
    is_post_read: bool,
}

impl BinLDriversDocumentSection {
    /// Create an empty document section.
    pub fn new() -> Self {
        BinLDriversDocumentSection {
            name: String::new(),
            offset: 0,
            length: 0,
            is_post_read: false,
        }
    }

    /// Create a document section with name and post-read flag.
    pub fn with_name_and_post_read(name: &str, is_post_read: bool) -> Self {
        BinLDriversDocumentSection {
            name: name.to_string(),
            offset: 0,
            length: 0,
            is_post_read,
        }
    }

    /// Get the name of the section.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check if the section should be read after OCAF data.
    pub fn is_post_read(&self) -> bool {
        self.is_post_read
    }

    /// Get the offset of the section in the file.
    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// Set the offset of the section in the file.
    pub fn set_offset(&mut self, offset: u64) {
        self.offset = offset;
    }

    /// Get the length of the section in the file.
    pub fn length(&self) -> u64 {
        self.length
    }

    /// Set the length of the section in the file.
    pub fn set_length(&mut self, length: u64) {
        self.length = length;
    }

    /// Check if this is the TOC (table of contents) section.
    pub fn is_toc_section(&self) -> bool {
        self.name.contains("TOC") || self.name == "0:0"
    }

    /// Check if this is a shape section.
    pub fn is_shape_section(&self) -> bool {
        self.name.contains("SHAPE")
    }
}

impl Default for BinLDriversDocumentSection {
    fn default() -> Self {
        BinLDriversDocumentSection::new()
    }
}

/// Document section collection.
#[derive(Clone, Debug)]
pub struct DocumentSectionList {
    sections: Vec<BinLDriversDocumentSection>,
}

impl DocumentSectionList {
    pub fn new() -> Self {
        DocumentSectionList {
            sections: Vec::new(),
        }
    }

    pub fn add_section(&mut self, section: BinLDriversDocumentSection) {
        self.sections.push(section);
    }

    pub fn section_count(&self) -> usize {
        self.sections.len()
    }

    pub fn get_section(&self, index: usize) -> Option<&BinLDriversDocumentSection> {
        self.sections.get(index)
    }

    pub fn get_section_mut(&mut self, index: usize) -> Option<&mut BinLDriversDocumentSection> {
        self.sections.get_mut(index)
    }

    pub fn find_section_by_name(&self, name: &str) -> Option<&BinLDriversDocumentSection> {
        self.sections.iter().find(|s| s.name == name)
    }

    pub fn clear(&mut self) {
        self.sections.clear();
    }
}

impl Default for DocumentSectionList {
    fn default() -> Self {
        DocumentSectionList::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_section_creation() {
        let section = BinLDriversDocumentSection::new();
        assert_eq!(section.name(), "");
        assert!(!section.is_post_read());
        assert_eq!(section.offset(), 0);
        assert_eq!(section.length(), 0);
    }

    #[test]
    fn test_section_with_name_and_post_read() {
        let section = BinLDriversDocumentSection::with_name_and_post_read("TestSection", true);
        assert_eq!(section.name(), "TestSection");
        assert!(section.is_post_read());
    }

    #[test]
    fn test_set_offset() {
        let mut section = BinLDriversDocumentSection::new();
        section.set_offset(1000);
        assert_eq!(section.offset(), 1000);
    }

    #[test]
    fn test_set_length() {
        let mut section = BinLDriversDocumentSection::new();
        section.set_length(5000);
        assert_eq!(section.length(), 5000);
    }

    #[test]
    fn test_is_shape_section() {
        let shape_section = BinLDriversDocumentSection::with_name_and_post_read("SHAPE_SECTION", false);
        assert!(shape_section.is_shape_section());

        let other_section = BinLDriversDocumentSection::with_name_and_post_read("DATA", false);
        assert!(!other_section.is_shape_section());
    }

    #[test]
    fn test_section_list_creation() {
        let list = DocumentSectionList::new();
        assert_eq!(list.section_count(), 0);
    }

    #[test]
    fn test_section_list_add_section() {
        let mut list = DocumentSectionList::new();
        let section = BinLDriversDocumentSection::with_name_and_post_read("Section1", false);
        list.add_section(section);

        assert_eq!(list.section_count(), 1);
    }

    #[test]
    fn test_section_list_get_section() {
        let mut list = DocumentSectionList::new();
        let section = BinLDriversDocumentSection::with_name_and_post_read("TestSec", false);
        list.add_section(section.clone());

        let retrieved = list.get_section(0).unwrap();
        assert_eq!(retrieved.name(), "TestSec");
    }

    #[test]
    fn test_section_list_find_by_name() {
        let mut list = DocumentSectionList::new();
        let section = BinLDriversDocumentSection::with_name_and_post_read("NamedSection", false);
        list.add_section(section);

        let found = list.find_section_by_name("NamedSection");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name(), "NamedSection");
    }

    #[test]
    fn test_section_list_find_nonexistent() {
        let list = DocumentSectionList::new();
        let found = list.find_section_by_name("NotThere");
        assert!(found.is_none());
    }

    #[test]
    fn test_section_list_clear() {
        let mut list = DocumentSectionList::new();
        list.add_section(BinLDriversDocumentSection::new());
        list.add_section(BinLDriversDocumentSection::new());
        assert_eq!(list.section_count(), 2);

        list.clear();
        assert_eq!(list.section_count(), 0);
    }

    #[test]
    fn test_section_list_get_mut_section() {
        let mut list = DocumentSectionList::new();
        let section = BinLDriversDocumentSection::new();
        list.add_section(section);

        let mut_section = list.get_section_mut(0).unwrap();
        mut_section.set_offset(500);
        mut_section.set_length(1000);

        assert_eq!(list.get_section(0).unwrap().offset(), 500);
        assert_eq!(list.get_section(0).unwrap().length(), 1000);
    }

    #[test]
    fn test_section_equality() {
        let sec1 = BinLDriversDocumentSection::with_name_and_post_read("Section", true);
        let sec2 = BinLDriversDocumentSection::with_name_and_post_read("Section", true);
        assert_eq!(sec1, sec2);
    }

    #[test]
    fn test_section_clone() {
        let section = BinLDriversDocumentSection::with_name_and_post_read("CloneTest", false);
        let cloned = section.clone();
        assert_eq!(section, cloned);
    }

    #[test]
    fn test_multiple_sections() {
        let mut list = DocumentSectionList::new();
        for i in 0..5 {
            let section = BinLDriversDocumentSection::with_name_and_post_read(&format!("Section{}", i), i % 2 == 0);
            list.add_section(section);
        }

        assert_eq!(list.section_count(), 5);
        assert_eq!(list.get_section(2).unwrap().name(), "Section2");
    }

    #[test]
    fn test_is_toc_section() {
        let toc_section = BinLDriversDocumentSection::with_name_and_post_read("TOC", false);
        assert!(toc_section.is_toc_section());

        let null_section = BinLDriversDocumentSection::with_name_and_post_read("0:0", false);
        assert!(null_section.is_toc_section());

        let other = BinLDriversDocumentSection::with_name_and_post_read("Data", false);
        assert!(!other.is_toc_section());
    }
}

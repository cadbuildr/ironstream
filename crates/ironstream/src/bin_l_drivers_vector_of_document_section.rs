// FILE: bin_l_drivers_vector_of_document_section.rs
// occt: BinLDrivers_VectorOfDocumentSection

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct DocumentSection {
    section_id: usize,
    section_name: String,
    start_offset: u64,
    section_size: u64,
}

impl DocumentSection {
    pub fn new(section_id: usize, section_name: String, start_offset: u64, section_size: u64) -> Self {
        DocumentSection {
            section_id,
            section_name,
            start_offset,
            section_size,
        }
    }

    pub fn section_id(&self) -> usize {
        self.section_id
    }

    pub fn section_name(&self) -> &str {
        &self.section_name
    }

    pub fn start_offset(&self) -> u64 {
        self.start_offset
    }

    pub fn section_size(&self) -> u64 {
        self.section_size
    }
}

pub struct BinldriversVectorOfDocumentSection {
    data: VecDeque<DocumentSection>,
}

impl BinldriversVectorOfDocumentSection {
    pub fn new() -> Self {
        BinldriversVectorOfDocumentSection {
            data: VecDeque::new(),
        }
    }

    pub fn push(&mut self, section: DocumentSection) {
        self.data.push_back(section);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&DocumentSection> {
        self.data.get(index)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &DocumentSection> {
        self.data.iter()
    }
}

impl Default for BinldriversVectorOfDocumentSection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_creation() {
        let section = DocumentSection::new(1, "data".to_string(), 0, 100);
        assert_eq!(section.section_id(), 1);
        assert_eq!(section.section_name(), "data");
        assert_eq!(section.start_offset(), 0);
        assert_eq!(section.section_size(), 100);
    }

    #[test]
    fn test_vector_push() {
        let mut vector = BinldriversVectorOfDocumentSection::new();
        let section = DocumentSection::new(1, "data".to_string(), 0, 100);
        vector.push(section);
        assert_eq!(vector.len(), 1);
    }
}

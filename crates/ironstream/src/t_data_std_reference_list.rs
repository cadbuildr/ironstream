// FILE: t_data_std_reference_list.rs
// occt: TDataStd_ReferenceList

use std::collections::VecDeque;

/// A label reference for the data framework.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TDF_Label {
    path: String,
}

impl TDF_Label {
    /// Create a new label from a path string.
    pub fn new(path: String) -> Self {
        Self { path }
    }

    /// Get the label path.
    pub fn path(&self) -> &str {
        &self.path
    }
}

/// A list attribute containing TDF_Label references.
/// Provides efficient insertion, removal, and traversal of label references.
#[derive(Clone, Debug)]
pub struct TDataStd_ReferenceList {
    list: VecDeque<TDF_Label>,
    id: [u8; 16],
}

impl TDataStd_ReferenceList {
    /// Create a new ReferenceList attribute.
    pub fn new() -> Self {
        Self {
            list: VecDeque::new(),
            id: Self::get_id(),
        }
    }

    /// Get the standard GUID for ReferenceList attributes.
    pub fn get_id() -> [u8; 16] {
        // Standard OCCT GUID for TDataStd_ReferenceList
        [
            0x6F, 0x4A, 0x2C, 0x99, 0x5B, 0xAE, 0x4A, 0x8E, 0xC1, 0x3D, 0x6B, 0x77, 0x66, 0x22,
            0x22, 0x22,
        ]
    }

    /// Set a custom GUID for this attribute.
    pub fn set_id(&mut self, guid: [u8; 16]) {
        self.id = guid;
    }

    /// Get the ID of this attribute.
    pub fn id(&self) -> &[u8; 16] {
        &self.id
    }

    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Get the number of elements.
    pub fn extent(&self) -> usize {
        self.list.len()
    }

    /// Add a reference to the front of the list.
    pub fn prepend(&mut self, label: TDF_Label) {
        self.list.push_front(label);
    }

    /// Add a reference to the end of the list.
    pub fn append(&mut self, label: TDF_Label) {
        self.list.push_back(label);
    }

    /// Insert label before the first occurrence of before_label.
    pub fn insert_before(&mut self, label: TDF_Label, before_label: &TDF_Label) -> bool {
        for (idx, existing) in self.list.iter().enumerate() {
            if existing == before_label {
                self.list.insert(idx, label);
                return true;
            }
        }
        false
    }

    /// Insert label at the given index (1-based).
    pub fn insert_before_by_index(&mut self, index: usize, label: TDF_Label) -> bool {
        if index == 0 || index > self.list.len() {
            return false;
        }
        self.list.insert(index - 1, label);
        true
    }

    /// Insert label after the first occurrence of after_label.
    pub fn insert_after(&mut self, label: TDF_Label, after_label: &TDF_Label) -> bool {
        for (idx, existing) in self.list.iter().enumerate() {
            if existing == after_label {
                self.list.insert(idx + 1, label);
                return true;
            }
        }
        false
    }

    /// Insert label after the given index (1-based).
    pub fn insert_after_by_index(&mut self, index: usize, label: TDF_Label) -> bool {
        if index == 0 || index > self.list.len() {
            return false;
        }
        self.list.insert(index, label);
        true
    }

    /// Remove the first occurrence of a label.
    pub fn remove(&mut self, label: &TDF_Label) -> bool {
        if let Some(pos) = self.list.iter().position(|l| l == label) {
            self.list.remove(pos);
            true
        } else {
            false
        }
    }

    /// Remove label at the given index (1-based).
    pub fn remove_by_index(&mut self, index: usize) -> bool {
        if index == 0 || index > self.list.len() {
            return false;
        }
        self.list.remove(index - 1);
        true
    }

    /// Clear the list.
    pub fn clear(&mut self) {
        self.list.clear();
    }

    /// Get the first label.
    pub fn first(&self) -> Option<&TDF_Label> {
        self.list.front()
    }

    /// Get the last label.
    pub fn last(&self) -> Option<&TDF_Label> {
        self.list.back()
    }

    /// Get the list as a vector.
    pub fn list(&self) -> Vec<TDF_Label> {
        self.list.iter().cloned().collect()
    }
}

impl Default for TDataStd_ReferenceList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_label() {
        let label = TDF_Label::new("0:1:1".to_string());
        assert_eq!(label.path(), "0:1:1");
    }

    #[test]
    fn test_create_list() {
        let list = TDataStd_ReferenceList::new();
        assert!(list.is_empty());
        assert_eq!(list.extent(), 0);
    }

    #[test]
    fn test_append() {
        let mut list = TDataStd_ReferenceList::new();
        let label1 = TDF_Label::new("0:1:1".to_string());
        let label2 = TDF_Label::new("0:1:2".to_string());
        list.append(label1.clone());
        list.append(label2.clone());
        assert_eq!(list.extent(), 2);
        assert_eq!(list.first(), Some(&label1));
        assert_eq!(list.last(), Some(&label2));
    }

    #[test]
    fn test_prepend() {
        let mut list = TDataStd_ReferenceList::new();
        let label1 = TDF_Label::new("0:1:1".to_string());
        let label2 = TDF_Label::new("0:1:2".to_string());
        list.append(label2.clone());
        list.prepend(label1.clone());
        assert_eq!(list.first(), Some(&label1));
    }

    #[test]
    fn test_remove() {
        let mut list = TDataStd_ReferenceList::new();
        let label = TDF_Label::new("0:1:1".to_string());
        list.append(label.clone());
        assert!(list.remove(&label));
        assert!(list.is_empty());
    }

    #[test]
    fn test_remove_by_index() {
        let mut list = TDataStd_ReferenceList::new();
        let label1 = TDF_Label::new("0:1:1".to_string());
        let label2 = TDF_Label::new("0:1:2".to_string());
        list.append(label1);
        list.append(label2);
        assert!(list.remove_by_index(1));
        assert_eq!(list.extent(), 1);
    }

    #[test]
    fn test_clear() {
        let mut list = TDataStd_ReferenceList::new();
        list.append(TDF_Label::new("0:1:1".to_string()));
        list.clear();
        assert!(list.is_empty());
    }

    #[test]
    fn test_default() {
        let list = TDataStd_ReferenceList::default();
        assert!(list.is_empty());
    }
}

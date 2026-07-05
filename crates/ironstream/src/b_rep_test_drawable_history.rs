// FILE: b_rep_test_drawable_history.rs
// occt: BRepTest_DrawableHistory

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    operation: String,
    source_id: usize,
    result_ids: Vec<usize>,
}

impl HistoryEntry {
    pub fn new(operation: String, source_id: usize) -> Self {
        HistoryEntry {
            operation,
            source_id,
            result_ids: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result_id: usize) {
        self.result_ids.push(result_id);
    }

    pub fn operation(&self) -> &str {
        &self.operation
    }

    pub fn source_id(&self) -> usize {
        self.source_id
    }

    pub fn result_ids(&self) -> &[usize] {
        &self.result_ids
    }
}

pub struct BrepTestDrawableHistory {
    entries: VecDeque<HistoryEntry>,
}

impl BrepTestDrawableHistory {
    pub fn new() -> Self {
        BrepTestDrawableHistory {
            entries: VecDeque::new(),
        }
    }

    pub fn add_entry(&mut self, entry: HistoryEntry) {
        self.entries.push_back(entry);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&HistoryEntry> {
        self.entries.get(index)
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &HistoryEntry> {
        self.entries.iter()
    }
}

impl Default for BrepTestDrawableHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_creation() {
        let entry = HistoryEntry::new("fillet".to_string(), 1);
        assert_eq!(entry.operation(), "fillet");
        assert_eq!(entry.source_id(), 1);
        assert_eq!(entry.result_ids().len(), 0);
    }

    #[test]
    fn test_entry_add_result() {
        let mut entry = HistoryEntry::new("fillet".to_string(), 1);
        entry.add_result(2);
        entry.add_result(3);
        assert_eq!(entry.result_ids().len(), 2);
    }

    #[test]
    fn test_history_creation() {
        let history = BrepTestDrawableHistory::new();
        assert!(history.is_empty());
    }

    #[test]
    fn test_history_add_entry() {
        let mut history = BrepTestDrawableHistory::new();
        let entry = HistoryEntry::new("sweep".to_string(), 5);
        history.add_entry(entry);
        assert_eq!(history.len(), 1);
    }
}

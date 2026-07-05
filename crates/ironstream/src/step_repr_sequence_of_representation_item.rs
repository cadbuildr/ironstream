// FILE: step_repr_sequence_of_representation_item.rs
// occt: StepRepr_SequenceOfRepresentationItem

use std::vec::Vec;

pub struct StepReprSequenceOfRepresentationItem {
    data: Vec<Option<String>>,
    lower: usize,
}

impl StepReprSequenceOfRepresentationItem {
    pub fn new() -> Self {
        Self { data: Vec::new(), lower: 1 }
    }

    pub fn lower(&self) -> usize { self.lower }
    pub fn upper(&self) -> usize { self.lower + self.data.len() - 1 }
    pub fn len(&self) -> usize { self.data.len() }

    pub fn append(&mut self, value: Option<String>) {
        self.data.push(value);
    }

    pub fn value(&self, index: usize) -> Option<&Option<String>> {
        if index < self.lower || index > self.upper() { return None; }
        self.data.get(index - self.lower)
    }

    pub fn set_value(&mut self, index: usize, value: Option<String>) -> bool {
        if index < self.lower || index > self.upper() { return false; }
        if let Some(elem) = self.data.get_mut(index - self.lower) { *elem = value; true } else { false }
    }
}

impl Default for StepReprSequenceOfRepresentationItem {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut seq = StepReprSequenceOfRepresentationItem::new();
        seq.append(Some("repr".to_string()));
        assert_eq!(seq.len(), 1);
    }
}

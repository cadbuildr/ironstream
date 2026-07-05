// FILE: b_rep_blend_sequence_of_line.rs
// occt: BRepBlend_SequenceOfLine

use std::sync::Arc;

/// A blend line in surface blending operations.
/// Simplified representation of BRepBlend_Line.
#[derive(Debug, Clone)]
pub struct Line {
    line_id: usize,
    start_u: f64,
    start_v: f64,
    end_u: f64,
    end_v: f64,
}

impl Line {
    /// Creates a new blend line.
    pub fn new(line_id: usize, start_u: f64, start_v: f64, end_u: f64, end_v: f64) -> Self {
        Line {
            line_id,
            start_u,
            start_v,
            end_u,
            end_v,
        }
    }

    /// Returns the line ID.
    pub fn id(&self) -> usize {
        self.line_id
    }
}

/// Deprecated type alias: Sequence of BRepBlend_Line handles.
/// Uses Arc for reference-counted semantics.
pub struct BrepBlendSequenceOfLine {
    data: Vec<Arc<Line>>,
}

impl BrepBlendSequenceOfLine {
    /// Creates an empty sequence.
    pub fn new() -> Self {
        BrepBlendSequenceOfLine {
            data: Vec::new(),
        }
    }

    /// Appends a line to the sequence.
    pub fn append(&mut self, line: Arc<Line>) {
        self.data.push(line);
    }

    /// Prepends a line to the sequence.
    pub fn prepend(&mut self, line: Arc<Line>) {
        self.data.insert(0, line);
    }

    /// Returns the number of lines.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the first line.
    pub fn first(&self) -> Option<&Arc<Line>> {
        self.data.first()
    }

    /// Returns the last line.
    pub fn last(&self) -> Option<&Arc<Line>> {
        self.data.last()
    }

    /// Accesses a line by 1-based index.
    pub fn get(&self, index: usize) -> Option<&Arc<Line>> {
        if index < 1 || index > self.data.len() {
            return None;
        }
        self.data.get(index - 1)
    }

    /// Returns an iterator.
    pub fn iter(&self) -> impl Iterator<Item = &Arc<Line>> {
        self.data.iter()
    }

    /// Clears the sequence.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Removes the first line.
    pub fn remove_first(&mut self) -> Option<Arc<Line>> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    /// Removes the last line.
    pub fn remove_last(&mut self) -> Option<Arc<Line>> {
        self.data.pop()
    }

    /// Removes a line by 1-based index.
    pub fn remove(&mut self, index: usize) -> Option<Arc<Line>> {
        if index < 1 || index > self.data.len() {
            return None;
        }
        Some(self.data.remove(index - 1))
    }
}

impl Default for BrepBlendSequenceOfLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_creation() {
        let line = Line::new(1, 0.0, 0.0, 1.0, 1.0);
        assert_eq!(line.id(), 1);
    }

    #[test]
    fn test_sequence_creation() {
        let seq = BrepBlendSequenceOfLine::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_sequence_append() {
        let mut seq = BrepBlendSequenceOfLine::new();
        let line = Arc::new(Line::new(1, 0.0, 0.0, 1.0, 1.0));
        seq.append(line);
        assert_eq!(seq.len(), 1);
    }

    #[test]
    fn test_sequence_prepend() {
        let mut seq = BrepBlendSequenceOfLine::new();
        let line1 = Arc::new(Line::new(1, 0.0, 0.0, 1.0, 1.0));
        let line2 = Arc::new(Line::new(2, 1.0, 1.0, 2.0, 2.0));
        seq.append(line1);
        seq.prepend(line2);
        assert_eq!(seq.len(), 2);
        assert_eq!(seq.first().unwrap().id(), 2);
    }

    #[test]
    fn test_sequence_get() {
        let mut seq = BrepBlendSequenceOfLine::new();
        let line = Arc::new(Line::new(42, 0.5, 0.5, 1.5, 1.5));
        seq.append(line);
        assert_eq!(seq.get(1).unwrap().id(), 42);
    }

    #[test]
    fn test_sequence_first_last() {
        let mut seq = BrepBlendSequenceOfLine::new();
        let line1 = Arc::new(Line::new(1, 0.0, 0.0, 1.0, 1.0));
        let line2 = Arc::new(Line::new(2, 1.0, 1.0, 2.0, 2.0));
        seq.append(line1);
        seq.append(line2);
        assert_eq!(seq.first().unwrap().id(), 1);
        assert_eq!(seq.last().unwrap().id(), 2);
    }

    #[test]
    fn test_sequence_multiple() {
        let mut seq = BrepBlendSequenceOfLine::new();
        for i in 1..=5 {
            let line = Arc::new(Line::new(i, 0.0, 0.0, 1.0, 1.0));
            seq.append(line);
        }
        assert_eq!(seq.len(), 5);
        assert_eq!(seq.get(3).unwrap().id(), 3);
    }

    #[test]
    fn test_sequence_clear() {
        let mut seq = BrepBlendSequenceOfLine::new();
        seq.append(Arc::new(Line::new(1, 0.0, 0.0, 1.0, 1.0)));
        seq.append(Arc::new(Line::new(2, 1.0, 1.0, 2.0, 2.0)));
        assert_eq!(seq.len(), 2);
        seq.clear();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_sequence_remove() {
        let mut seq = BrepBlendSequenceOfLine::new();
        seq.append(Arc::new(Line::new(1, 0.0, 0.0, 1.0, 1.0)));
        seq.append(Arc::new(Line::new(2, 1.0, 1.0, 2.0, 2.0)));
        let removed = seq.remove(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().id(), 1);
        assert_eq!(seq.len(), 1);
    }

    #[test]
    fn test_sequence_remove_first() {
        let mut seq = BrepBlendSequenceOfLine::new();
        seq.append(Arc::new(Line::new(1, 0.0, 0.0, 1.0, 1.0)));
        seq.append(Arc::new(Line::new(2, 1.0, 1.0, 2.0, 2.0)));
        let removed = seq.remove_first();
        assert_eq!(removed.unwrap().id(), 1);
        assert_eq!(seq.len(), 1);
    }

    #[test]
    fn test_sequence_remove_last() {
        let mut seq = BrepBlendSequenceOfLine::new();
        seq.append(Arc::new(Line::new(1, 0.0, 0.0, 1.0, 1.0)));
        seq.append(Arc::new(Line::new(2, 1.0, 1.0, 2.0, 2.0)));
        let removed = seq.remove_last();
        assert_eq!(removed.unwrap().id(), 2);
        assert_eq!(seq.len(), 1);
    }

    #[test]
    fn test_sequence_iterator() {
        let mut seq = BrepBlendSequenceOfLine::new();
        for i in 1..=3 {
            seq.append(Arc::new(Line::new(i, 0.0, 0.0, 1.0, 1.0)));
        }
        let count = seq.iter().count();
        assert_eq!(count, 3);
    }
}

// FILE: b_rep_graph_supplement_iterator.rs
// occt: BRepGraph_SupplementIterator

use std::collections::HashMap;

/// Iterator over supplementary graph data.
pub struct BRepGraphSupplementIterator {
    items: Vec<(u32, Vec<u8>)>,
    current_index: usize,
}

impl BRepGraphSupplementIterator {
    /// Create a new iterator over supplements.
    pub fn new(supplements: &HashMap<u32, Vec<u8>>) -> Self {
        let items: Vec<_> = supplements
            .iter()
            .map(|(id, data)| (*id, data.clone()))
            .collect();

        Self {
            items,
            current_index: 0,
        }
    }

    /// Move to the next item.
    pub fn next(&mut self) -> Option<(u32, &[u8])> {
        if self.current_index < self.items.len() {
            let (id, data) = &self.items[self.current_index];
            self.current_index += 1;
            Some((*id, data.as_slice()))
        } else {
            None
        }
    }

    /// Check if there are more items.
    pub fn has_more(&self) -> bool {
        self.current_index < self.items.len()
    }

    /// Reset the iterator.
    pub fn reset(&mut self) {
        self.current_index = 0;
    }

    /// Get the total number of items.
    pub fn count(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_iterator() {
        let map = HashMap::new();
        let iter = BRepGraphSupplementIterator::new(&map);

        assert_eq!(iter.count(), 0);
        assert!(!iter.has_more());
    }

    #[test]
    fn test_iterate() {
        let mut map = HashMap::new();
        map.insert(1, vec![1, 2, 3]);
        map.insert(2, vec![4, 5, 6]);

        let mut iter = BRepGraphSupplementIterator::new(&map);

        assert_eq!(iter.count(), 2);
        assert!(iter.has_more());

        let first = iter.next();
        assert!(first.is_some());
        assert!(iter.has_more());

        let second = iter.next();
        assert!(second.is_some());
        assert!(!iter.has_more());

        let third = iter.next();
        assert!(third.is_none());
    }

    #[test]
    fn test_reset() {
        let mut map = HashMap::new();
        map.insert(1, vec![1, 2, 3]);

        let mut iter = BRepGraphSupplementIterator::new(&map);
        assert!(iter.has_more());

        iter.next();
        assert!(!iter.has_more());

        iter.reset();
        assert!(iter.has_more());
    }
}

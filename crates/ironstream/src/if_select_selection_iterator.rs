// FILE: if_select_selection_iterator.rs
// occt: IFSelect_SelectionIterator

/// An iterator over a list of Selections.
/// Each selection is present only once in the result.
#[derive(Clone, Debug)]
pub struct IFSelectSelectionIterator {
    items: Vec<String>, // simplified: store selection identifiers
    current: usize,
}

impl IFSelectSelectionIterator {
    /// Creates an empty iterator
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            current: 0,
        }
    }

    /// Adds a selection to the iterator (if not already present)
    pub fn add_item(&mut self, sel_id: String) {
        if !self.items.contains(&sel_id) {
            self.items.push(sel_id);
        }
    }

    /// Adds content from another iterator
    pub fn add_from_iter(&mut self, other: &IFSelectSelectionIterator) {
        for item in &other.items {
            self.add_item(item.clone());
        }
    }

    /// Adds a list of selections
    pub fn add_list(&mut self, list: Vec<String>) {
        for sel_id in list {
            self.add_item(sel_id);
        }
    }

    /// Returns true if there are more selections to get
    pub fn more(&self) -> bool {
        self.current < self.items.len()
    }

    /// Moves to the next selection
    pub fn next(&mut self) {
        if self.current < self.items.len() {
            self.current += 1;
        }
    }

    /// Returns the current selection (panics if no more items)
    pub fn value(&self) -> &str {
        if self.current >= self.items.len() {
            panic!("SelectionIterator: no more items");
        }
        &self.items[self.current]
    }

    /// Resets the iterator to the beginning
    pub fn reset(&mut self) {
        self.current = 0;
    }

    /// Returns the number of items
    pub fn count(&self) -> usize {
        self.items.len()
    }
}

impl Default for IFSelectSelectionIterator {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for IFSelectSelectionIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.more() {
            let item = self.items[self.current].clone();
            IFSelectSelectionIterator::next(self);
            Some(item)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let iter = IFSelectSelectionIterator::new();
        assert!(!iter.more());
        assert_eq!(iter.count(), 0);
    }

    #[test]
    fn test_add_item() {
        let mut iter = IFSelectSelectionIterator::new();
        iter.add_item("sel1".to_string());
        assert!(iter.more());
        assert_eq!(iter.value(), "sel1");
        assert_eq!(iter.count(), 1);
    }

    #[test]
    fn test_no_duplicates() {
        let mut iter = IFSelectSelectionIterator::new();
        iter.add_item("sel1".to_string());
        iter.add_item("sel1".to_string());
        assert_eq!(iter.count(), 1);
    }

    #[test]
    fn test_iteration() {
        let mut iter = IFSelectSelectionIterator::new();
        iter.add_item("sel1".to_string());
        iter.add_item("sel2".to_string());
        iter.add_item("sel3".to_string());

        assert_eq!(iter.value(), "sel1");
        iter.next();
        assert_eq!(iter.value(), "sel2");
        iter.next();
        assert_eq!(iter.value(), "sel3");
        iter.next();
        assert!(!iter.more());
    }

    #[test]
    fn test_reset() {
        let mut iter = IFSelectSelectionIterator::new();
        iter.add_item("sel1".to_string());
        iter.next();
        assert!(!iter.more());
        iter.reset();
        assert!(iter.more());
        assert_eq!(iter.value(), "sel1");
    }

    #[test]
    fn test_add_from_iter() {
        let mut iter1 = IFSelectSelectionIterator::new();
        iter1.add_item("sel1".to_string());
        iter1.add_item("sel2".to_string());

        let mut iter2 = IFSelectSelectionIterator::new();
        iter2.add_item("sel3".to_string());
        iter2.add_from_iter(&iter1);

        assert_eq!(iter2.count(), 3);
    }

    #[test]
    fn test_add_list() {
        let mut iter = IFSelectSelectionIterator::new();
        iter.add_list(vec!["sel1".to_string(), "sel2".to_string()]);
        assert_eq!(iter.count(), 2);
    }
}

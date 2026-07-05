// FILE: select_mgr_list_of_filter.rs
// occt: SelectMgr_ListOfFilter, SelectMgr_ListIteratorOfListOfFilter

/// Deprecated typedef for backward compatibility.
/// A list of filter handles using a Vec for storage.
/// Corresponds to NCollection_List<opencascade::handle<SelectMgr_Filter>>
pub struct SelectMgrListOfFilter {
    data: Vec<String>, // Use String as placeholder for SelectMgr_Filter handles
}

impl SelectMgrListOfFilter {
    /// Create a new empty list.
    pub fn new() -> Self {
        SelectMgrListOfFilter {
            data: Vec::new(),
        }
    }

    /// Append an element to the list.
    pub fn append(&mut self, value: String) {
        self.data.push(value);
    }

    /// Prepend an element to the list.
    pub fn prepend(&mut self, value: String) {
        self.data.insert(0, value);
    }

    /// Remove the first occurrence of a value.
    pub fn remove_first(&mut self) -> Option<String> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    /// Remove the last occurrence of a value.
    pub fn remove_last(&mut self) -> Option<String> {
        self.data.pop()
    }

    /// Get the number of elements in the list.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clear all elements from the list.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Create an iterator over the list elements.
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.data.iter()
    }
}

impl Default for SelectMgrListOfFilter {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for the deprecated list type.
/// Corresponds to SelectMgr_ListIteratorOfListOfFilter
pub struct SelectMgrListIteratorOfListOfFilter {
    data: Vec<String>,
    index: usize,
}

impl SelectMgrListIteratorOfListOfFilter {
    /// Create a new iterator from a list.
    pub fn new(list: &SelectMgrListOfFilter) -> Self {
        SelectMgrListIteratorOfListOfFilter {
            data: list.data.clone(),
            index: 0,
        }
    }

    /// Check if there are more elements.
    pub fn more(&self) -> bool {
        self.index < self.data.len()
    }

    /// Move to the next element.
    pub fn next(&mut self) {
        if self.more() {
            self.index += 1;
        }
    }

    /// Get the current element.
    pub fn value(&self) -> Option<&String> {
        if self.more() {
            Some(&self.data[self.index])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut list = SelectMgrListOfFilter::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        list.append("filter1".to_string());
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);

        list.append("filter2".to_string());
        list.append("filter3".to_string());
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_prepend() {
        let mut list = SelectMgrListOfFilter::new();
        list.append("second".to_string());
        list.prepend("first".to_string());

        let mut iter = SelectMgrListIteratorOfListOfFilter::new(&list);
        assert_eq!(iter.value(), Some(&"first".to_string()));
        iter.next();
        assert_eq!(iter.value(), Some(&"second".to_string()));
    }

    #[test]
    fn test_remove() {
        let mut list = SelectMgrListOfFilter::new();
        list.append("a".to_string());
        list.append("b".to_string());
        list.append("c".to_string());

        assert_eq!(list.remove_first(), Some("a".to_string()));
        assert_eq!(list.len(), 2);
        assert_eq!(list.remove_last(), Some("c".to_string()));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_iterator() {
        let mut list = SelectMgrListOfFilter::new();
        list.append("x".to_string());
        list.append("y".to_string());
        list.append("z".to_string());

        let mut iter = SelectMgrListIteratorOfListOfFilter::new(&list);
        let mut count = 0;

        while iter.more() {
            assert!(iter.value().is_some());
            count += 1;
            iter.next();
        }

        assert_eq!(count, 3);
    }
}

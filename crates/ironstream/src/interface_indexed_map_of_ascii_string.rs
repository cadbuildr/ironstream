// FILE: interface_indexed_map_of_ascii_string.rs
// occt: Interface_IndexedMapOfAsciiString

/// Deprecated alias for NCollection_IndexedMap<TCollection_AsciiString>.
/// An indexed map stores elements with both key-value (like a map) and position indexing.
pub struct InterfaceIndexedMapOfAsciiString {
    items: Vec<String>, // Maintains insertion order
}

impl InterfaceIndexedMapOfAsciiString {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, value: String) -> usize {
        // Return index if already present, otherwise add and return new index
        if let Some(pos) = self.items.iter().position(|x| x == &value) {
            pos + 1 // OCCT uses 1-based indexing
        } else {
            self.items.push(value);
            self.items.len() // Return 1-based index
        }
    }

    pub fn remove(&mut self, index: usize) -> bool {
        // OCCT uses 1-based indexing
        if index > 0 && index <= self.items.len() {
            self.items.remove(index - 1);
            true
        } else {
            false
        }
    }

    pub fn find_index(&self, value: &str) -> usize {
        // Return 0 if not found, else 1-based index
        self.items
            .iter()
            .position(|x| x == value)
            .map(|p| p + 1)
            .unwrap_or(0)
    }

    pub fn find_value(&self, index: usize) -> Option<String> {
        // OCCT uses 1-based indexing
        if index > 0 && index <= self.items.len() {
            Some(self.items[index - 1].clone())
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Default for InterfaceIndexedMapOfAsciiString {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexed_map_add() {
        let mut map = InterfaceIndexedMapOfAsciiString::new();
        assert!(map.is_empty());

        let idx1 = map.add("hello".to_string());
        assert_eq!(idx1, 1);
        assert_eq!(map.len(), 1);

        let idx2 = map.add("world".to_string());
        assert_eq!(idx2, 2);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_indexed_map_duplicate_add() {
        let mut map = InterfaceIndexedMapOfAsciiString::new();
        let idx1 = map.add("test".to_string());
        let idx2 = map.add("test".to_string());

        assert_eq!(idx1, idx2); // Same index for duplicate
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_indexed_map_find_and_retrieve() {
        let mut map = InterfaceIndexedMapOfAsciiString::new();
        map.add("alpha".to_string());
        map.add("beta".to_string());
        map.add("gamma".to_string());

        assert_eq!(map.find_index("alpha"), 1);
        assert_eq!(map.find_index("beta"), 2);
        assert_eq!(map.find_index("gamma"), 3);
        assert_eq!(map.find_index("delta"), 0); // Not found

        assert_eq!(map.find_value(1), Some("alpha".to_string()));
        assert_eq!(map.find_value(2), Some("beta".to_string()));
        assert_eq!(map.find_value(3), Some("gamma".to_string()));
        assert_eq!(map.find_value(0), None);
        assert_eq!(map.find_value(4), None);
    }

    #[test]
    fn test_indexed_map_remove() {
        let mut map = InterfaceIndexedMapOfAsciiString::new();
        map.add("a".to_string());
        map.add("b".to_string());
        map.add("c".to_string());

        assert!(map.remove(2));
        assert_eq!(map.len(), 2);
        assert_eq!(map.find_index("b"), 0);
        assert!(!map.remove(10)); // Invalid index
    }

    #[test]
    fn test_indexed_map_clear() {
        let mut map = InterfaceIndexedMapOfAsciiString::new();
        map.add("x".to_string());
        map.add("y".to_string());
        map.clear();

        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_default() {
        let map = InterfaceIndexedMapOfAsciiString::default();
        assert!(map.is_empty());
    }
}

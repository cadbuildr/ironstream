// FILE: interface_copy_map.rs
// occt: Interface_CopyMap

/// A map for tracking entity copies during duplication.
#[derive(Clone, Debug)]
pub struct InterfaceCopyMap {
    entries: Vec<(usize, usize)>, // (original_id, copy_id)
}

impl InterfaceCopyMap {
    /// Creates a CopyMap
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Adds a copy mapping
    pub fn add(&mut self, original_id: usize, copy_id: usize) {
        self.entries.push((original_id, copy_id));
    }

    /// Finds the copy for an original entity
    pub fn find(&self, original_id: usize) -> Option<usize> {
        self.entries
            .iter()
            .find(|&&(orig, _)| orig == original_id)
            .map(|&(_, copy)| copy)
    }

    /// Returns the count of entries
    pub fn count(&self) -> usize {
        self.entries.len()
    }
}

impl Default for InterfaceCopyMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let map = InterfaceCopyMap::new();
        assert_eq!(map.count(), 0);
    }

    #[test]
    fn test_add_and_find() {
        let mut map = InterfaceCopyMap::new();
        map.add(1, 10);
        assert_eq!(map.find(1), Some(10));
        assert_eq!(map.find(2), None);
    }

    #[test]
    fn test_multiple_entries() {
        let mut map = InterfaceCopyMap::new();
        map.add(1, 10);
        map.add(2, 20);
        map.add(3, 30);
        assert_eq!(map.count(), 3);
        assert_eq!(map.find(2), Some(20));
    }
}

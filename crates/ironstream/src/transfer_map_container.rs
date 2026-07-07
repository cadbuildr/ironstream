// FILE: transfer_map_container.rs
// occt: Transfer_MapContainer

/// A container that maintains a mapping of entities in a transfer process.
/// Stores associations between source and result entities.
#[derive(Clone, Debug)]
pub struct TransferMapContainer {
    /// Mapping entries (source_id, result_id)
    mappings: Vec<(u32, u32)>,
}

impl TransferMapContainer {
    /// Creates a new empty map container.
    pub fn new() -> Self {
        Self {
            mappings: Vec::new(),
        }
    }

    /// Adds a mapping from source to result.
    pub fn add_mapping(&mut self, source_id: u32, result_id: u32) {
        self.mappings.push((source_id, result_id));
    }

    /// Finds a result ID for a given source ID.
    pub fn find(&self, source_id: u32) -> Option<u32> {
        self.mappings
            .iter()
            .find(|(src, _)| *src == source_id)
            .map(|(_, res)| *res)
    }

    /// Returns the number of mappings.
    pub fn size(&self) -> usize {
        self.mappings.len()
    }

    /// Clears all mappings.
    pub fn clear(&mut self) {
        self.mappings.clear();
    }

    /// Returns whether the container is empty.
    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty()
    }

    /// Returns all mappings.
    pub fn mappings(&self) -> &[(u32, u32)] {
        &self.mappings
    }
}

impl Default for TransferMapContainer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let container = TransferMapContainer::new();
        assert!(container.is_empty());
        assert_eq!(container.size(), 0);
    }

    #[test]
    fn test_add_mapping() {
        let mut container = TransferMapContainer::new();
        container.add_mapping(1, 10);
        assert_eq!(container.size(), 1);
        assert!(!container.is_empty());
    }

    #[test]
    fn test_find() {
        let mut container = TransferMapContainer::new();
        container.add_mapping(1, 10);
        container.add_mapping(2, 20);
        container.add_mapping(3, 30);

        assert_eq!(container.find(1), Some(10));
        assert_eq!(container.find(2), Some(20));
        assert_eq!(container.find(3), Some(30));
        assert_eq!(container.find(99), None);
    }

    #[test]
    fn test_clear() {
        let mut container = TransferMapContainer::new();
        container.add_mapping(1, 10);
        container.add_mapping(2, 20);
        assert_eq!(container.size(), 2);

        container.clear();
        assert!(container.is_empty());
        assert_eq!(container.size(), 0);
    }

    #[test]
    fn test_mappings() {
        let mut container = TransferMapContainer::new();
        container.add_mapping(5, 50);
        container.add_mapping(6, 60);

        let mappings = container.mappings();
        assert_eq!(mappings.len(), 2);
        assert_eq!(mappings[0], (5, 50));
        assert_eq!(mappings[1], (6, 60));
    }
}

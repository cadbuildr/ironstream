// FILE: transfer_transient_mapper.rs
// occt: Transfer_TransientMapper

/// Maps transient entities to result entities in a transfer process.
/// Maintains bidirectional mappings between source and result transients.
#[derive(Clone, Debug)]
pub struct TransferTransientMapper {
    /// Forward mappings (source -> result)
    forward: Vec<(u32, u32)>,
    /// Whether bidirectional mapping is enabled
    bidirectional: bool,
}

impl TransferTransientMapper {
    /// Creates a new transient mapper.
    pub fn new() -> Self {
        Self {
            forward: Vec::new(),
            bidirectional: false,
        }
    }

    /// Adds a mapping from source to result.
    pub fn add_mapping(&mut self, source_id: u32, result_id: u32) {
        self.forward.push((source_id, result_id));
    }

    /// Finds a result for a given source.
    pub fn find(&self, source_id: u32) -> Option<u32> {
        self.forward
            .iter()
            .find(|(src, _)| *src == source_id)
            .map(|(_, res)| *res)
    }

    /// Returns the number of mappings.
    pub fn nb_mappings(&self) -> usize {
        self.forward.len()
    }

    /// Sets whether bidirectional mapping is enabled.
    pub fn set_bidirectional(&mut self, enabled: bool) {
        self.bidirectional = enabled;
    }

    /// Returns whether bidirectional mapping is enabled.
    pub fn is_bidirectional(&self) -> bool {
        self.bidirectional
    }

    /// Clears all mappings.
    pub fn clear(&mut self) {
        self.forward.clear();
    }
}

impl Default for TransferTransientMapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mapper = TransferTransientMapper::new();
        assert_eq!(mapper.nb_mappings(), 0);
        assert!(!mapper.is_bidirectional());
    }

    #[test]
    fn test_add_mapping() {
        let mut mapper = TransferTransientMapper::new();
        mapper.add_mapping(1, 100);
        assert_eq!(mapper.nb_mappings(), 1);

        mapper.add_mapping(2, 200);
        assert_eq!(mapper.nb_mappings(), 2);
    }

    #[test]
    fn test_find() {
        let mut mapper = TransferTransientMapper::new();
        mapper.add_mapping(10, 1000);
        mapper.add_mapping(20, 2000);

        assert_eq!(mapper.find(10), Some(1000));
        assert_eq!(mapper.find(20), Some(2000));
        assert_eq!(mapper.find(99), None);
    }

    #[test]
    fn test_bidirectional() {
        let mut mapper = TransferTransientMapper::new();
        assert!(!mapper.is_bidirectional());

        mapper.set_bidirectional(true);
        assert!(mapper.is_bidirectional());

        mapper.set_bidirectional(false);
        assert!(!mapper.is_bidirectional());
    }

    #[test]
    fn test_clear() {
        let mut mapper = TransferTransientMapper::new();
        mapper.add_mapping(1, 10);
        mapper.add_mapping(2, 20);
        assert_eq!(mapper.nb_mappings(), 2);

        mapper.clear();
        assert_eq!(mapper.nb_mappings(), 0);
    }
}

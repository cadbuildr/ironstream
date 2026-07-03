// FILE: top_ope_b_rep_ds_h_data_structure.rs
// occt: TopOpeBRepDS_HDataStructure

/// Handle to DataStructure
/// HDataStructure is typically a Handle<Transient> wrapper in OCCT
#[derive(Debug, Clone)]
pub struct HDataStructure {
    /// Data structure reference count ID
    handle_id: usize,
}

impl HDataStructure {
    /// Create a new handle
    pub fn new(handle_id: usize) -> Self {
        HDataStructure { handle_id }
    }

    /// Get handle ID
    pub fn handle_id(&self) -> usize {
        self.handle_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_data_structure_new() {
        let h = HDataStructure::new(99);
        assert_eq!(h.handle_id(), 99);
    }
}

// FILE: top_ope_b_rep_ds_p_data_structure.rs
// occt: TopOpeBRepDS_PDataStructure

// This is a pointer type alias in the original C++ code
// In Rust, we represent this as a handle/reference type
// PDataStructure is a Handle<DataStructure>

/// Handle to DataStructure (equivalent to C++ Handle<TopOpeBRepDS_DataStructure>)
/// In Rust, we use a simple wrapper for the concept
#[derive(Debug, Clone)]
pub struct PDataStructure {
    /// Data structure index/ID
    ds_id: usize,
}

impl PDataStructure {
    /// Create a new handle
    pub fn new(ds_id: usize) -> Self {
        PDataStructure { ds_id }
    }

    /// Get data structure ID
    pub fn ds_id(&self) -> usize {
        self.ds_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_data_structure_new() {
        let p = PDataStructure::new(42);
        assert_eq!(p.ds_id(), 42);
    }
}

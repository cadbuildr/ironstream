// FILE: b_rep_graph_inc_storage.rs
// occt: BRepGraphInc_Storage

/// Minimal implementation of BRepGraphInc_Storage
pub struct BRepGraphIncStorage {}

impl Default for BRepGraphIncStorage {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphIncStorage::default();
    }
}

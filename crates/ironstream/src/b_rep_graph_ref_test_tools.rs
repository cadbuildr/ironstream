// FILE: b_rep_graph_ref_test_tools.rs
// occt: BRepGraph_RefTestTools

/// Testing utilities for BRepGraph references.
pub struct BRepGraphRefTestTools;

impl BRepGraphRefTestTools {
    /// Create a test reference with dummy data.
    pub fn create_test_ref(ref_id: u32) -> (u32, Vec<u8>) {
        (ref_id, vec![0; 32])
    }

    /// Validate a reference structure.
    pub fn validate_ref(ref_id: u32, data: &[u8]) -> bool {
        ref_id > 0 && !data.is_empty()
    }

    /// Create a batch of test references.
    pub fn create_test_refs(count: u32) -> Vec<(u32, Vec<u8>)> {
        (1..=count).map(|id| Self::create_test_ref(id)).collect()
    }

    /// Compare two references for equality.
    pub fn refs_equal(ref1: &(u32, Vec<u8>), ref2: &(u32, Vec<u8>)) -> bool {
        ref1.0 == ref2.0 && ref1.1 == ref2.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_ref() {
        let (id, data) = BRepGraphRefTestTools::create_test_ref(1);
        assert_eq!(id, 1);
        assert_eq!(data.len(), 32);
    }

    #[test]
    fn test_validate_ref() {
        assert!(BRepGraphRefTestTools::validate_ref(1, &[1, 2, 3]));
        assert!(!BRepGraphRefTestTools::validate_ref(0, &[1, 2, 3]));
        assert!(!BRepGraphRefTestTools::validate_ref(1, &[]));
    }

    #[test]
    fn test_create_test_refs() {
        let refs = BRepGraphRefTestTools::create_test_refs(5);
        assert_eq!(refs.len(), 5);
        assert_eq!(refs[0].0, 1);
        assert_eq!(refs[4].0, 5);
    }

    #[test]
    fn test_refs_equal() {
        let ref1 = (1u32, vec![1, 2, 3]);
        let ref2 = (1u32, vec![1, 2, 3]);
        let ref3 = (2u32, vec![1, 2, 3]);

        assert!(BRepGraphRefTestTools::refs_equal(&ref1, &ref2));
        assert!(!BRepGraphRefTestTools::refs_equal(&ref1, &ref3));
    }
}

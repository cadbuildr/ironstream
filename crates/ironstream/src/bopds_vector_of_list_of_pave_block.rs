// FILE: bopds_vector_of_list_of_pave_block.rs
// occt: BOPDS_VectorOfListOfPaveBlock

//! NCollection alias: Vector<List<BOPDS_PaveBlock>>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_VectorOfListOfPaveBlock
pub type BOPDSVectorOfListOfPaveBlock = Vec<Vec<u32>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vec: BOPDSVectorOfListOfPaveBlock = Vec::new();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_vector_operations() {
        let mut vec: BOPDSVectorOfListOfPaveBlock = Vec::new();
        vec.push(vec![1, 2]);
        vec.push(vec![3, 4]);
        assert_eq!(vec.len(), 2);
    }
}

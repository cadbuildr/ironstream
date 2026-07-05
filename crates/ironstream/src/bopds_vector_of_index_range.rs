// FILE: bopds_vector_of_index_range.rs
// occt: BOPDS_VectorOfIndexRange

//! NCollection alias: Vector<BOPDS_IndexRange>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_VectorOfIndexRange
pub type BOPDSVectorOfIndexRange = Vec<(i32, i32)>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vec: BOPDSVectorOfIndexRange = Vec::new();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_vector_operations() {
        let mut vec: BOPDSVectorOfIndexRange = Vec::new();
        vec.push((0, 10));
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], (0, 10));
    }
}

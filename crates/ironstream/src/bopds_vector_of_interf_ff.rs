// FILE: bopds_vector_of_interf_ff.rs
// occt: BOPDS_VectorOfInterfFF

//! NCollection alias: Vector<BOPDS_InterfFF>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_VectorOfInterfFF
pub type BOPDSVectorOfInterfFF = Vec<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vec: BOPDSVectorOfInterfFF = Vec::new();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_vector_operations() {
        let mut vec: BOPDSVectorOfInterfFF = Vec::new();
        vec.push(1);
        assert_eq!(vec.len(), 1);
    }
}

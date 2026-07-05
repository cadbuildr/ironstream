// FILE: bopds_vector_of_interf_ez.rs
// occt: BOPDS_VectorOfInterfEZ

//! NCollection alias: Vector<BOPDS_InterfEZ>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_VectorOfInterfEZ
pub type BOPDSVectorOfInterfEZ = Vec<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vec: BOPDSVectorOfInterfEZ = Vec::new();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_vector_operations() {
        let mut vec: BOPDSVectorOfInterfEZ = Vec::new();
        vec.push(1);
        assert_eq!(vec.len(), 1);
    }
}

// FILE: bopds_vector_of_interf_zz.rs
// occt: BOPDS_VectorOfInterfZZ

//! NCollection alias: Vector<BOPDS_InterfZZ>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_VectorOfInterfZZ
pub type BOPDSVectorOfInterfZZ = Vec<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vec: BOPDSVectorOfInterfZZ = Vec::new();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_vector_operations() {
        let mut vec: BOPDSVectorOfInterfZZ = Vec::new();
        vec.push(1);
        assert_eq!(vec.len(), 1);
    }
}

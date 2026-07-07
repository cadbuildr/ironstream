// FILE: bopds_vector_of_interf_vz.rs
// occt: BOPDS_VectorOfInterfVZ

//! NCollection alias: Vector<BOPDS_InterfVZ>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_VectorOfInterfVZ
pub type BOPDSVectorOfInterfVZ = Vec<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vec: BOPDSVectorOfInterfVZ = Vec::new();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_vector_operations() {
        let mut vec: BOPDSVectorOfInterfVZ = Vec::new();
        vec.push(1);
        assert_eq!(vec.len(), 1);
    }
}

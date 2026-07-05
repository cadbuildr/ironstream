// FILE: bopds_vector_of_interf_fz.rs
// occt: BOPDS_VectorOfInterfFZ

//! NCollection alias: Vector<BOPDS_InterfFZ>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_VectorOfInterfFZ
pub type BOPDSVectorOfInterfFZ = Vec<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vec: BOPDSVectorOfInterfFZ = Vec::new();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_vector_operations() {
        let mut vec: BOPDSVectorOfInterfFZ = Vec::new();
        vec.push(1);
        assert_eq!(vec.len(), 1);
    }
}

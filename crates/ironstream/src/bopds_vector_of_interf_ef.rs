// FILE: bopds_vector_of_interf_ef.rs
// occt: BOPDS_VectorOfInterfEF

//! NCollection alias: Vector<BOPDS_InterfEF>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_VectorOfInterfEF
pub type BOPDSVectorOfInterfEF = Vec<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vec: BOPDSVectorOfInterfEF = Vec::new();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_vector_operations() {
        let mut vec: BOPDSVectorOfInterfEF = Vec::new();
        vec.push(1);
        assert_eq!(vec.len(), 1);
    }
}

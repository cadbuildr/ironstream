// FILE: bopds_vector_of_interf_vf.rs
// occt: BOPDS_VectorOfInterfVF

//! NCollection alias: Vector<BOPDS_InterfVF>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_VectorOfInterfVF
pub type BOPDSVectorOfInterfVF = Vec<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vec: BOPDSVectorOfInterfVF = Vec::new();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_vector_operations() {
        let mut vec: BOPDSVectorOfInterfVF = Vec::new();
        vec.push(1);
        assert_eq!(vec.len(), 1);
    }
}

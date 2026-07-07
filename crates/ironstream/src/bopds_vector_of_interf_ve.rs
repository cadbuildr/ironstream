// FILE: bopds_vector_of_interf_ve.rs
// occt: BOPDS_VectorOfInterfVE

//! NCollection alias: Vector<BOPDS_InterfVE>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_VectorOfInterfVE
pub type BOPDSVectorOfInterfVE = Vec<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vec: BOPDSVectorOfInterfVE = Vec::new();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_vector_operations() {
        let mut vec: BOPDSVectorOfInterfVE = Vec::new();
        vec.push(1);
        assert_eq!(vec.len(), 1);
    }
}

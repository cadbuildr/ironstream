// FILE: bopds_vector_of_interf_ee.rs
// occt: BOPDS_VectorOfInterfEE

//! NCollection alias: Vector<BOPDS_InterfEE>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_VectorOfInterfEE
pub type BOPDSVectorOfInterfEE = Vec<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vec: BOPDSVectorOfInterfEE = Vec::new();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_vector_operations() {
        let mut vec: BOPDSVectorOfInterfEE = Vec::new();
        vec.push(1);
        assert_eq!(vec.len(), 1);
    }
}

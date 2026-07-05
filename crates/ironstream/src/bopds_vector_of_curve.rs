// FILE: bopds_vector_of_curve.rs
// occt: BOPDS_VectorOfCurve

//! NCollection alias: Vector<BOPDS_Curve>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_VectorOfCurve
pub type BOPDSVectorOfCurve = Vec<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vec: BOPDSVectorOfCurve = Vec::new();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_vector_operations() {
        let mut vec: BOPDSVectorOfCurve = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], 1);
    }
}

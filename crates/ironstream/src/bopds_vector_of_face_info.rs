// FILE: bopds_vector_of_face_info.rs
// occt: BOPDS_VectorOfFaceInfo

//! NCollection alias: Vector<BOPDS_FaceInfo>
//! Deprecated type for backward compatibility.

/// Deprecated: BOPDS_VectorOfFaceInfo
pub type BOPDSVectorOfFaceInfo = Vec<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vec: BOPDSVectorOfFaceInfo = Vec::new();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_vector_operations() {
        let mut vec: BOPDSVectorOfFaceInfo = Vec::new();
        vec.push(1);
        assert_eq!(vec.len(), 1);
    }
}

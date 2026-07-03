// FILE: b_rep_lib_face_error.rs
// occt: BRepLib_FaceError

/// Errors that can occur at face construction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaceError {
    /// No error
    FaceDone = 0,
    /// Not initialized
    NoFace = 1,
    /// Not planar
    NotPlanar = 2,
    /// Curve projection failed
    CurveProjectionFailed = 3,
    /// Parameters out of range
    ParametersOutOfRange = 4,
}

impl FaceError {
    pub fn from_i32(val: i32) -> Option<Self> {
        match val {
            0 => Some(FaceError::FaceDone),
            1 => Some(FaceError::NoFace),
            2 => Some(FaceError::NotPlanar),
            3 => Some(FaceError::CurveProjectionFailed),
            4 => Some(FaceError::ParametersOutOfRange),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_error_values() {
        assert_eq!(FaceError::FaceDone.as_i32(), 0);
        assert_eq!(FaceError::NoFace.as_i32(), 1);
        assert_eq!(FaceError::NotPlanar.as_i32(), 2);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(FaceError::from_i32(0), Some(FaceError::FaceDone));
        assert_eq!(FaceError::from_i32(5), None);
    }
}

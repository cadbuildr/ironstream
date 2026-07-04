// FILE: topo_ds_to_step_make_face_error.rs
// occt: TopoDSToStep_MakeFaceError

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MakeFaceError {
    FaceDone,
    InfiniteFace,
    NonManifoldFace,
    NoWireMapped,
    FaceOther,
}

impl MakeFaceError {
    pub fn is_success(&self) -> bool {
        matches!(self, MakeFaceError::FaceDone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_success() {
        assert!(MakeFaceError::FaceDone.is_success());
        assert!(!MakeFaceError::InfiniteFace.is_success());
    }
}

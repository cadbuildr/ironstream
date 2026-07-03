// FILE: top_ope_b_rep_build_correct_face2d.rs
// occt: TopOpeBRepBuild_CorrectFace2d

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildCorrectFace2d {
    face_id: i32,
    corrected: bool,
}

impl TopOpeBRepBuildCorrectFace2d {
    pub fn new(face_id: i32) -> Self {
        TopOpeBRepBuildCorrectFace2d {
            face_id,
            corrected: false,
        }
    }

    pub fn correct(&mut self) {
        self.corrected = true;
    }

    pub fn is_corrected(&self) -> bool {
        self.corrected
    }

    pub fn face_id(&self) -> i32 {
        self.face_id
    }
}

impl Default for TopOpeBRepBuildCorrectFace2d {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_face2d_new() {
        let cf = TopOpeBRepBuildCorrectFace2d::new(5);
        assert_eq!(cf.face_id(), 5);
        assert!(!cf.is_corrected());
    }

    #[test]
    fn test_correct_face2d_correct() {
        let mut cf = TopOpeBRepBuildCorrectFace2d::new(1);
        assert!(!cf.is_corrected());
        cf.correct();
        assert!(cf.is_corrected());
    }

    #[test]
    fn test_correct_face2d_default() {
        let cf = TopOpeBRepBuildCorrectFace2d::default();
        assert_eq!(cf.face_id(), 0);
    }
}

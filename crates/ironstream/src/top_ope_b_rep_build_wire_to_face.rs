// FILE: top_ope_b_rep_build_wire_to_face.rs
// occt: TopOpeBRepBuild_WireToFace

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildWireToFace {
    wire_id: i32,
    face_id: i32,
}

impl TopOpeBRepBuildWireToFace {
    pub fn new() -> Self {
        TopOpeBRepBuildWireToFace {
            wire_id: 0,
            face_id: 0,
        }
    }

    pub fn with_ids(wire_id: i32, face_id: i32) -> Self {
        TopOpeBRepBuildWireToFace { wire_id, face_id }
    }

    pub fn wire_id(&self) -> i32 {
        self.wire_id
    }

    pub fn face_id(&self) -> i32 {
        self.face_id
    }

    pub fn set_face_id(&mut self, face_id: i32) {
        self.face_id = face_id;
    }
}

impl Default for TopOpeBRepBuildWireToFace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_to_face_new() {
        let wtf = TopOpeBRepBuildWireToFace::new();
        assert_eq!(wtf.wire_id(), 0);
        assert_eq!(wtf.face_id(), 0);
    }

    #[test]
    fn test_wire_to_face_with_ids() {
        let wtf = TopOpeBRepBuildWireToFace::with_ids(5, 10);
        assert_eq!(wtf.wire_id(), 5);
        assert_eq!(wtf.face_id(), 10);
    }

    #[test]
    fn test_wire_to_face_set_face_id() {
        let mut wtf = TopOpeBRepBuildWireToFace::new();
        wtf.set_face_id(20);
        assert_eq!(wtf.face_id(), 20);
    }
}

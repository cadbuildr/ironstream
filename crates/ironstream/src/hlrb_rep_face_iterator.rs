// FILE: hlrb_rep_face_iterator.rs
// occt: HLRBRep_FaceIterator

pub struct HlrbrépFaceIterator {
    faces: Vec<i32>,
    current: usize,
}

impl HlrbrépFaceIterator {
    pub fn new() -> Self {
        HlrbrépFaceIterator {
            faces: Vec::new(),
            current: 0,
        }
    }

    pub fn init(&mut self, faces: Vec<i32>) {
        self.faces = faces;
        self.current = 0;
    }

    pub fn more(&self) -> bool { self.current < self.faces.len() }
    pub fn next(&mut self) { self.current += 1; }
    pub fn current_face(&self) -> Option<i32> {
        self.faces.get(self.current).copied()
    }
}

impl Default for HlrbrépFaceIterator {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let fi = HlrbrépFaceIterator::new();
        assert!(!fi.more());
    }
}

// FILE: hlrb_rep_face_data.rs
// occt: HLRBRep_FaceData

pub struct HlrbrépFaceData {
    face_index: i32,
    nb_contours: usize,
    contours: Vec<Vec<(f64, f64)>>,
}

impl HlrbrépFaceData {
    pub fn new() -> Self {
        HlrbrépFaceData {
            face_index: 0,
            nb_contours: 0,
            contours: Vec::new(),
        }
    }

    pub fn set_face_index(&mut self, idx: i32) { self.face_index = idx; }
    pub fn face_index(&self) -> i32 { self.face_index }
    pub fn nb_contours(&self) -> usize { self.contours.len() }
    pub fn add_contour(&mut self, contour: Vec<(f64, f64)>) {
        self.contours.push(contour);
    }
}

impl Default for HlrbrépFaceData {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let fd = HlrbrépFaceData::new();
        assert_eq!(fd.face_index(), 0);
        assert_eq!(fd.nb_contours(), 0);
    }
}

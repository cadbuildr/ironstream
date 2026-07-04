// FILE: step_visual_tessellated_connecting_edge.rs
// occt: StepVisual_TessellatedConnectingEdge

use std::sync::Arc;

pub struct TessellatedFace;

pub struct TessellatedConnectingEdge {
    smooth: bool,
    face1: Option<Arc<TessellatedFace>>,
    face2: Option<Arc<TessellatedFace>>,
    line_strip_face1: Option<Arc<Vec<i32>>>,
    line_strip_face2: Option<Arc<Vec<i32>>>,
}

impl TessellatedConnectingEdge {
    pub fn new() -> Self {
        TessellatedConnectingEdge {
            smooth: false,
            face1: None,
            face2: None,
            line_strip_face1: None,
            line_strip_face2: None,
        }
    }

    pub fn smooth(&self) -> bool {
        self.smooth
    }

    pub fn set_smooth(&mut self, smooth: bool) {
        self.smooth = smooth;
    }

    pub fn face1(&self) -> Option<&Arc<TessellatedFace>> {
        self.face1.as_ref()
    }

    pub fn set_face1(&mut self, face: Option<Arc<TessellatedFace>>) {
        self.face1 = face;
    }

    pub fn face2(&self) -> Option<&Arc<TessellatedFace>> {
        self.face2.as_ref()
    }

    pub fn set_face2(&mut self, face: Option<Arc<TessellatedFace>>) {
        self.face2 = face;
    }

    pub fn line_strip_face1(&self) -> Option<&Arc<Vec<i32>>> {
        self.line_strip_face1.as_ref()
    }

    pub fn set_line_strip_face1(&mut self, strip: Option<Arc<Vec<i32>>>) {
        self.line_strip_face1 = strip;
    }

    pub fn nb_line_strip_face1(&self) -> usize {
        self.line_strip_face1.as_ref().map(|s| s.len()).unwrap_or(0)
    }

    pub fn line_strip_face1_value(&self, num: usize) -> Option<i32> {
        self.line_strip_face1
            .as_ref()
            .and_then(|s| s.get(num).copied())
    }

    pub fn line_strip_face2(&self) -> Option<&Arc<Vec<i32>>> {
        self.line_strip_face2.as_ref()
    }

    pub fn set_line_strip_face2(&mut self, strip: Option<Arc<Vec<i32>>>) {
        self.line_strip_face2 = strip;
    }

    pub fn nb_line_strip_face2(&self) -> usize {
        self.line_strip_face2.as_ref().map(|s| s.len()).unwrap_or(0)
    }

    pub fn line_strip_face2_value(&self, num: usize) -> Option<i32> {
        self.line_strip_face2
            .as_ref()
            .and_then(|s| s.get(num).copied())
    }
}

impl Default for TessellatedConnectingEdge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tce = TessellatedConnectingEdge::new();
        assert!(!tce.smooth());
        assert!(tce.face1().is_none());
        assert!(tce.face2().is_none());
        assert_eq!(tce.nb_line_strip_face1(), 0);
        assert_eq!(tce.nb_line_strip_face2(), 0);
    }

    #[test]
    fn test_set_smooth() {
        let mut tce = TessellatedConnectingEdge::new();
        tce.set_smooth(true);
        assert!(tce.smooth());
    }

    #[test]
    fn test_set_faces() {
        let mut tce = TessellatedConnectingEdge::new();
        let face1 = Arc::new(TessellatedFace);
        let face2 = Arc::new(TessellatedFace);
        tce.set_face1(Some(face1.clone()));
        tce.set_face2(Some(face2.clone()));
        assert!(tce.face1().is_some());
        assert!(tce.face2().is_some());
    }

    #[test]
    fn test_line_strips() {
        let mut tce = TessellatedConnectingEdge::new();
        let strip1 = vec![1, 2, 3];
        let strip2 = vec![4, 5, 6];
        tce.set_line_strip_face1(Some(Arc::new(strip1)));
        tce.set_line_strip_face2(Some(Arc::new(strip2)));
        assert_eq!(tce.nb_line_strip_face1(), 3);
        assert_eq!(tce.nb_line_strip_face2(), 3);
        assert_eq!(tce.line_strip_face1_value(0), Some(1));
        assert_eq!(tce.line_strip_face2_value(1), Some(5));
    }
}

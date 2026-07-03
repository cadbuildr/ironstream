// FILE: dsg_prs_angle_presentation.rs
// occt: DsgPrs_AnglePresentation

#[derive(Clone, Debug)]
pub struct DsgPrsAnglePresentation {
    pub presentation_id: u32,
    pub is_visible: bool,
}

impl DsgPrsAnglePresentation {
    pub fn new(presentation_id: u32) -> Self {
        Self { presentation_id, is_visible: true }
    }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
}

impl Default for DsgPrsAnglePresentation {
    fn default() -> Self { Self::new(0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let prs = DsgPrsAnglePresentation::new(1);
        assert_eq!(prs.presentation_id, 1);
        assert!(prs.is_visible);
    }
}

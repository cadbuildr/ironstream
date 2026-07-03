// FILE: dsg_prs_xyz_plane_presentation.rs
// occt: DsgPrs_XYZPlanePresentation

#[derive(Clone, Debug)]
pub struct DsgPrsXyzPlanePresentation {
    pub presentation_id: u32,
    pub is_visible: bool,
}

impl DsgPrsXyzPlanePresentation {
    pub fn new(presentation_id: u32) -> Self {
        Self { presentation_id, is_visible: true }
    }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
}

impl Default for DsgPrsXyzPlanePresentation {
    fn default() -> Self { Self::new(0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let prs = DsgPrsXyzPlanePresentation::new(1);
        assert_eq!(prs.presentation_id, 1);
        assert!(prs.is_visible);
    }
}

// FILE: dsg_prs_xyz_axis_presentation.rs
// occt: DsgPrs_XYZAxisPresentation

#[derive(Clone, Debug)]
pub struct DsgPrsXyzAxisPresentation {
    pub presentation_id: u32,
    pub is_visible: bool,
}

impl DsgPrsXyzAxisPresentation {
    pub fn new(presentation_id: u32) -> Self {
        Self { presentation_id, is_visible: true }
    }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
}

impl Default for DsgPrsXyzAxisPresentation {
    fn default() -> Self { Self::new(0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let prs = DsgPrsXyzAxisPresentation::new(1);
        assert_eq!(prs.presentation_id, 1);
        assert!(prs.is_visible);
    }
}

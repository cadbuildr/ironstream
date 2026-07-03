// FILE: dsg_prs_mid_point_presentation.rs
// occt: DsgPrs_MidPointPresentation

#[derive(Clone, Debug)]
pub struct DsgPrsMidPointPresentation {
    pub presentation_id: u32,
    pub is_visible: bool,
}

impl DsgPrsMidPointPresentation {
    pub fn new(presentation_id: u32) -> Self {
        Self { presentation_id, is_visible: true }
    }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
}

impl Default for DsgPrsMidPointPresentation {
    fn default() -> Self { Self::new(0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let prs = DsgPrsMidPointPresentation::new(1);
        assert_eq!(prs.presentation_id, 1);
        assert!(prs.is_visible);
    }
}

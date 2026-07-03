// FILE: dsg_prs_fix_presentation.rs
// occt: DsgPrs_FixPresentation

#[derive(Clone, Debug)]
pub struct DsgPrsFixPresentation {
    pub presentation_id: u32,
    pub is_visible: bool,
}

impl DsgPrsFixPresentation {
    pub fn new(presentation_id: u32) -> Self {
        Self { presentation_id, is_visible: true }
    }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
}

impl Default for DsgPrsFixPresentation {
    fn default() -> Self { Self::new(0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let prs = DsgPrsFixPresentation::new(1);
        assert_eq!(prs.presentation_id, 1);
        assert!(prs.is_visible);
    }
}

// FILE: dsg_prs_identic_presentation.rs
// occt: DsgPrs_IdenticPresentation

#[derive(Clone, Debug)]
pub struct DsgPrsIdenticPresentation {
    pub presentation_id: u32,
    pub is_visible: bool,
}

impl DsgPrsIdenticPresentation {
    pub fn new(presentation_id: u32) -> Self {
        Self { presentation_id, is_visible: true }
    }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
}

impl Default for DsgPrsIdenticPresentation {
    fn default() -> Self { Self::new(0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let prs = DsgPrsIdenticPresentation::new(1);
        assert_eq!(prs.presentation_id, 1);
        assert!(prs.is_visible);
    }
}

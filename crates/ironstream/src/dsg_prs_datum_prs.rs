// FILE: dsg_prs_datum_prs.rs
// occt: DsgPrs_DatumPrs

#[derive(Clone, Debug)]
pub struct DsgPrsDatumPrs {
    pub presentation_id: u32,
    pub is_visible: bool,
}

impl DsgPrsDatumPrs {
    pub fn new(presentation_id: u32) -> Self {
        Self { presentation_id, is_visible: true }
    }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
}

impl Default for DsgPrsDatumPrs {
    fn default() -> Self { Self::new(0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let prs = DsgPrsDatumPrs::new(1);
        assert_eq!(prs.presentation_id, 1);
        assert!(prs.is_visible);
    }
}

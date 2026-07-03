// FILE: ais_type_filter.rs
// occt: AIS_TypeFilter

#[derive(Clone, Debug)]
pub struct AisTypeFilter {
    pub filter_id: u32,
    pub type_id: u32,
}

impl AisTypeFilter {
    pub fn new(filter_id: u32, type_id: u32) -> Self {
        Self { filter_id, type_id }
    }
    pub fn matches(&self, type_id: u32) -> bool { self.type_id == type_id }
}

impl Default for AisTypeFilter {
    fn default() -> Self { Self::new(0, 0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matches() {
        let f = AisTypeFilter::new(1, 5);
        assert!(f.matches(5));
        assert!(!f.matches(6));
    }
}

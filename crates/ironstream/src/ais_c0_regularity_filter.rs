// FILE: ais_c0_regularity_filter.rs
// occt: AIS_C0RegularityFilter

#[derive(Clone, Debug, Default)]
pub struct C0RegularityFilter {
    enabled: bool,
}

impl C0RegularityFilter {
    pub fn new() -> Self { Self { enabled: false } }
    pub fn is_enabled(&self) -> bool { self.enabled }
    pub fn set_enabled(&mut self, e: bool) { self.enabled = e; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let f = C0RegularityFilter::new();
        assert!(!f.is_enabled());
    }
}

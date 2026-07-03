// FILE: ais_signature_filter.rs
// occt: AIS_SignatureFilter

#[derive(Clone, Debug)]
pub struct AisSignatureFilter {
    pub filter_id: u32,
    pub signature: u32,
}

impl AisSignatureFilter {
    pub fn new(filter_id: u32, signature: u32) -> Self {
        Self { filter_id, signature }
    }
    pub fn matches(&self, sig: u32) -> bool { self.signature == sig }
}

impl Default for AisSignatureFilter {
    fn default() -> Self { Self::new(0, 0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matches() {
        let f = AisSignatureFilter::new(1, 42);
        assert!(f.matches(42));
    }
}

// FILE: hlrb_rep_internal_algo.rs
// occt: HLRBRep_InternalAlgo

pub struct HlrbrépInternalAlgo {
    processing: bool,
}

impl HlrbrépInternalAlgo {
    pub fn new() -> Self {
        HlrbrépInternalAlgo {
            processing: false,
        }
    }

    pub fn start(&mut self) { self.processing = true; }
    pub fn is_processing(&self) -> bool { self.processing }
    pub fn finish(&mut self) { self.processing = false; }
}

impl Default for HlrbrépInternalAlgo {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let ia = HlrbrépInternalAlgo::new();
        assert!(!ia.is_processing());
    }
}

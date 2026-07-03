// FILE: hlr_topo_b_rep_ds_filler.rs
// occt: HLRTopoBRep_DSFiller

/// Fills HLRTopoBRep_Data structure from shapes.
#[derive(Clone, Debug)]
pub struct HLRTopoBRepDSFiller;

impl HLRTopoBRepDSFiller {
    pub fn new() -> Self {
        HLRTopoBRepDSFiller
    }

    pub fn fill(&self, _data: &str, _shape: &str) {
        // Stub implementation
    }
}

impl Default for HLRTopoBRepDSFiller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let filler = HLRTopoBRepDSFiller::new();
        filler.fill("data", "shape");
    }
}

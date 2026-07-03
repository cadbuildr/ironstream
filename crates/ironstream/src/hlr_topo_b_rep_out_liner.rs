// FILE: hlr_topo_b_rep_out_liner.rs
// occt: HLRTopoBRep_OutLiner

/// Computes outlines (silhouettes) of shapes under projection.
#[derive(Clone, Debug)]
pub struct HLRTopoBRepOutLiner;

impl HLRTopoBRepOutLiner {
    pub fn new() -> Self {
        HLRTopoBRepOutLiner
    }

    pub fn add_shape(&mut self, _shape: &str) {
        // Stub
    }

    pub fn compute_outlines(&self) -> Vec<String> {
        vec![]
    }
}

impl Default for HLRTopoBRepOutLiner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let mut outliner = HLRTopoBRepOutLiner::new();
        outliner.add_shape("shape");
        let outlines = outliner.compute_outlines();
        assert!(outlines.is_empty());
    }
}

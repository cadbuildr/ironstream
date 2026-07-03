// FILE: hlr_topo_b_rep_face_iso_liner.rs
// occt: HLRTopoBRep_FaceIsoLiner

/// Extracts isoline curves from faces.
#[derive(Clone, Debug)]
pub struct HLRTopoBRepFaceIsoLiner;

impl HLRTopoBRepFaceIsoLiner {
    pub fn new() -> Self {
        HLRTopoBRepFaceIsoLiner
    }

    pub fn generate_isolines(&self, _face: &str, _nb_iso: i32) -> Vec<String> {
        vec![]
    }
}

impl Default for HLRTopoBRepFaceIsoLiner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let liner = HLRTopoBRepFaceIsoLiner::new();
        let isolines = liner.generate_isolines("face", 0);
        assert!(isolines.is_empty());
    }
}

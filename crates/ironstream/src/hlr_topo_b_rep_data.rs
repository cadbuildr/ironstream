// FILE: hlr_topo_b_rep_data.rs
// occt: HLRTopoBRep_Data

use std::collections::HashMap;

/// Stores results of OutLine and IsoLine processes.
#[derive(Clone, Debug)]
pub struct HLRTopoBRepData {
    data_map: HashMap<String, Vec<String>>,
}

impl HLRTopoBRepData {
    pub fn new() -> Self {
        HLRTopoBRepData {
            data_map: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.data_map.clear();
    }

    pub fn clean(&mut self) {
        self.data_map.clear();
    }

    pub fn edge_has_splE(&self, _edge: &str) -> bool {
        false
    }

    pub fn face_has_intL(&self, _face: &str) -> bool {
        false
    }

    pub fn face_has_outL(&self, _face: &str) -> bool {
        false
    }

    pub fn face_has_isoL(&self, _face: &str) -> bool {
        false
    }

    pub fn is_splE_edge_edge(&self, _e1: &str, _e2: &str) -> bool {
        false
    }

    pub fn is_intL_face_edge(&self, _f: &str, _e: &str) -> bool {
        false
    }

    pub fn is_outL_face_edge(&self, _f: &str, _e: &str) -> bool {
        false
    }

    pub fn is_isoL_face_edge(&self, _f: &str, _e: &str) -> bool {
        false
    }
}

impl Default for HLRTopoBRepData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let d = HLRTopoBRepData::new();
        assert!(!d.edge_has_splE("edge"));
    }

    #[test]
    fn test_clear() {
        let mut d = HLRTopoBRepData::new();
        d.clear();
        assert!(!d.face_has_intL("face"));
    }
}

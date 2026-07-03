// FILE: hlr_topo_b_rep_face_data.rs
// occt: HLRTopoBRep_FaceData

/// Data structure for a single face in HLR topology.
#[derive(Clone, Debug)]
pub struct HLRTopoBRepFaceData {
    has_outlines: bool,
    has_isolines: bool,
    outline_count: i32,
    isoline_count: i32,
}

impl HLRTopoBRepFaceData {
    pub fn new() -> Self {
        HLRTopoBRepFaceData {
            has_outlines: false,
            has_isolines: false,
            outline_count: 0,
            isoline_count: 0,
        }
    }

    pub fn set_has_outlines(&mut self, b: bool) {
        self.has_outlines = b;
    }

    pub fn set_has_isolines(&mut self, b: bool) {
        self.has_isolines = b;
    }

    pub fn has_outlines(&self) -> bool {
        self.has_outlines
    }

    pub fn has_isolines(&self) -> bool {
        self.has_isolines
    }

    pub fn outline_count(&self) -> i32 {
        self.outline_count
    }

    pub fn isoline_count(&self) -> i32 {
        self.isoline_count
    }
}

impl Default for HLRTopoBRepFaceData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let fd = HLRTopoBRepFaceData::new();
        assert!(!fd.has_outlines());
        assert!(!fd.has_isolines());
    }

    #[test]
    fn test_setters() {
        let mut fd = HLRTopoBRepFaceData::new();
        fd.set_has_outlines(true);
        fd.set_has_isolines(true);
        assert!(fd.has_outlines());
        assert!(fd.has_isolines());
    }
}

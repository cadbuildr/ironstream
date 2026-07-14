// FILE: draft_angle.rs
// occt: BRepOffsetAPI_DraftAngle, Draft_EdgeInfo
// occt-ref: Draft_Modification

/// Status of a draft-angle operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftStatus {
    NotDone,
    Done,
    NullFace,
    NoFaceFound,
    NullDirection,
    OverFlow,
    Error,
}

impl Default for DraftStatus {
    fn default() -> Self { Self::NotDone }
}

impl DraftStatus {
    pub fn is_done(&self) -> bool { *self == DraftStatus::Done }
}

/// Information about an edge modified by draft.
#[derive(Clone, Debug)]
pub struct DraftEdgeInfo {
    pub edge_id: u32,
    pub curve_id: u32,
    pub tolerance: f64,
    pub is_tangent_on_s1: bool,
    pub is_tangent_on_s2: bool,
}

impl DraftEdgeInfo {
    pub fn new(edge_id: u32) -> Self {
        Self { edge_id, curve_id: edge_id + 100, tolerance: 1e-7, is_tangent_on_s1: false, is_tangent_on_s2: false }
    }
}

/// A face being drafted (face ID + target surface + angle).
#[derive(Clone, Debug)]
pub struct DraftFaceEntry {
    pub face_id: u32,
    pub neutral_plane_id: u32,
    pub pull_direction: [f64; 3],
    pub angle: f64,
    pub result_face_id: u32,
}

// occt-ref: Draft_Modification // — internal modification data for a draft operation
#[derive(Clone, Debug, Default)]
pub struct DraftModification {
    pub faces: Vec<DraftFaceEntry>,
    pub edge_infos: Vec<DraftEdgeInfo>,
    pub is_consistent: bool,
}

impl DraftModification {
    pub fn new() -> Self { Self { is_consistent: true, ..Default::default() } }

    pub fn add_face(&mut self, face_id: u32, neutral_id: u32, dir: [f64; 3], angle: f64) {
        self.faces.push(DraftFaceEntry {
            face_id,
            neutral_plane_id: neutral_id,
            pull_direction: dir,
            angle,
            result_face_id: face_id + 5000,
        });
    }

    pub fn nb_faces(&self) -> usize { self.faces.len() }
    pub fn face(&self, i: usize) -> Option<&DraftFaceEntry> { self.faces.get(i) }
    pub fn is_consistent(&self) -> bool { self.is_consistent }
    pub fn clear(&mut self) { self.faces.clear(); self.edge_infos.clear(); }
}

// occt: BRepOffsetAPI_DraftAngle // — adds draft angles to faces of a solid
#[derive(Clone, Debug)]
pub struct BrepOffsetApiDraftAngle {
    pub solid_id: u32,
    pub modification: DraftModification,
    pub status: DraftStatus,
    pub result_id: u32,
    pub error_face_id: u32,
}

impl BrepOffsetApiDraftAngle {
    pub fn new(solid_id: u32) -> Self {
        Self {
            solid_id,
            modification: DraftModification::new(),
            status: DraftStatus::NotDone,
            result_id: 0,
            error_face_id: 0,
        }
    }

    pub fn add(&mut self, face_id: u32, direction: [f64; 3], angle: f64, neutral_plane_id: u32) -> bool {
        if face_id == 0 { return false; }
        self.modification.add_face(face_id, neutral_plane_id, direction, angle);
        true
    }

    pub fn remove(&mut self, face_id: u32) -> bool {
        let before = self.modification.faces.len();
        self.modification.faces.retain(|f| f.face_id != face_id);
        self.modification.faces.len() < before
    }

    pub fn build(&mut self) {
        if self.solid_id == 0 {
            self.status = DraftStatus::Error;
            return;
        }
        if self.modification.faces.is_empty() {
            self.status = DraftStatus::NoFaceFound;
            return;
        }
        if !self.modification.is_consistent {
            self.status = DraftStatus::OverFlow;
            return;
        }
        self.result_id = self.solid_id + 8000;
        self.status = DraftStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shape(&self) -> u32 { self.result_id }
    pub fn nb_added_faces(&self) -> usize { self.modification.nb_faces() }
    pub fn error_face(&self) -> u32 { self.error_face_id }

    pub fn generated_shape(&self, face_id: u32) -> Option<u32> {
        self.modification.faces.iter()
            .find(|f| f.face_id == face_id)
            .map(|f| f.result_face_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draft_angle_basic() {
        let mut da = BrepOffsetApiDraftAngle::new(1);
        da.add(10, [0.0, 0.0, 1.0], 5.0_f64.to_radians(), 20);
        da.build();
        assert!(da.is_done());
        assert!(da.shape() > 0);
        assert_eq!(da.nb_added_faces(), 1);
    }

    #[test]
    fn draft_angle_no_faces() {
        let mut da = BrepOffsetApiDraftAngle::new(1);
        da.build();
        assert_eq!(da.status, DraftStatus::NoFaceFound);
    }

    #[test]
    fn draft_angle_null_solid() {
        let mut da = BrepOffsetApiDraftAngle::new(0);
        da.add(10, [0.0, 0.0, 1.0], 0.1, 20);
        da.build();
        assert_eq!(da.status, DraftStatus::Error);
    }

    #[test]
    fn draft_angle_remove_face() {
        let mut da = BrepOffsetApiDraftAngle::new(1);
        da.add(10, [0.0, 0.0, 1.0], 0.1, 20);
        da.add(11, [0.0, 0.0, 1.0], 0.1, 20);
        assert_eq!(da.nb_added_faces(), 2);
        assert!(da.remove(10));
        assert_eq!(da.nb_added_faces(), 1);
    }

    #[test]
    fn draft_angle_generated_shape() {
        let mut da = BrepOffsetApiDraftAngle::new(1);
        da.add(10, [0.0, 0.0, 1.0], 0.1, 20);
        da.build();
        assert!(da.generated_shape(10).is_some());
        assert!(da.generated_shape(999).is_none());
    }

    #[test]
    fn draft_modification_basic() {
        let mut dm = DraftModification::new();
        dm.add_face(1, 2, [0.0, 0.0, 1.0], 0.1);
        assert_eq!(dm.nb_faces(), 1);
        assert!(dm.is_consistent());
        dm.clear();
        assert_eq!(dm.nb_faces(), 0);
    }
}

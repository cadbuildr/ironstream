// FILE: draft_mod.rs

// occt: Draft_ErrorStatus
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftAngleError {
    NoError,
    FaceRecomputation,
    EdgeRecomputation,
    VertexRecomputation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftModificationStatus {
    NotDone,
    Done,
    Error,
}

pub struct DraftFaceEntry {
    pub face_id: u32,
    pub draft_angle: f64,
    pub neutral_plane: [f64; 4],
}

impl DraftFaceEntry {
    pub fn new(face_id: u32, angle: f64, normal: [f64; 3], d: f64) -> Self {
        Self {
            face_id,
            draft_angle: angle,
            neutral_plane: [normal[0], normal[1], normal[2], d],
        }
    }
}

// occt: Draft_Modification
pub struct DraftModification {
    faces: Vec<DraftFaceEntry>,
    status: DraftModificationStatus,
    error: DraftAngleError,
}

impl DraftModification {
    pub fn new() -> Self {
        Self {
            faces: Vec::new(),
            status: DraftModificationStatus::NotDone,
            error: DraftAngleError::NoError,
        }
    }

    pub fn add_face(&mut self, entry: DraftFaceEntry) -> bool {
        if entry.draft_angle.abs() > std::f64::consts::FRAC_PI_2 {
            return false;
        }
        self.faces.push(entry);
        true
    }

    pub fn perform(&mut self) {
        if self.faces.is_empty() {
            self.status = DraftModificationStatus::Error;
        } else {
            self.status = DraftModificationStatus::Done;
        }
    }

    pub fn status(&self) -> DraftModificationStatus {
        self.status
    }

    pub fn is_done(&self) -> bool {
        self.status == DraftModificationStatus::Done
    }

    pub fn error_status(&self) -> DraftAngleError {
        self.error
    }

    pub fn nb_faces(&self) -> usize {
        self.faces.len()
    }

    pub fn remove_face(&mut self, face_id: u32) {
        self.faces.retain(|f| f.face_id != face_id);
    }

    pub fn clear(&mut self) {
        self.faces.clear();
        self.status = DraftModificationStatus::NotDone;
        self.error = DraftAngleError::NoError;
    }
}

impl Default for DraftModification {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_4;

    fn entry(face_id: u32, angle: f64) -> DraftFaceEntry {
        DraftFaceEntry::new(face_id, angle, [0.0, 0.0, 1.0], 0.0)
    }

    #[test]
    fn new_starts_not_done() {
        let dm = DraftModification::new();
        assert_eq!(dm.status(), DraftModificationStatus::NotDone);
        assert!(!dm.is_done());
        assert_eq!(dm.nb_faces(), 0);
    }

    #[test]
    fn error_status_is_no_error_initially() {
        let dm = DraftModification::new();
        assert_eq!(dm.error_status(), DraftAngleError::NoError);
    }

    #[test]
    fn add_face_valid_angle() {
        let mut dm = DraftModification::new();
        let ok = dm.add_face(entry(1, FRAC_PI_4));
        assert!(ok);
        assert_eq!(dm.nb_faces(), 1);
    }

    #[test]
    fn add_face_rejects_angle_over_90_degrees() {
        let mut dm = DraftModification::new();
        let too_large = std::f64::consts::FRAC_PI_2 + 0.001;
        let ok = dm.add_face(entry(1, too_large));
        assert!(!ok);
        assert_eq!(dm.nb_faces(), 0);
    }

    #[test]
    fn add_face_accepts_negative_angle_within_range() {
        let mut dm = DraftModification::new();
        let ok = dm.add_face(entry(2, -FRAC_PI_4));
        assert!(ok);
        assert_eq!(dm.nb_faces(), 1);
    }

    #[test]
    fn perform_with_faces_sets_done() {
        let mut dm = DraftModification::new();
        dm.add_face(entry(1, FRAC_PI_4));
        dm.perform();
        assert_eq!(dm.status(), DraftModificationStatus::Done);
        assert!(dm.is_done());
    }

    #[test]
    fn perform_without_faces_sets_error() {
        let mut dm = DraftModification::new();
        dm.perform();
        assert_eq!(dm.status(), DraftModificationStatus::Error);
        assert!(!dm.is_done());
    }

    #[test]
    fn remove_face_by_id() {
        let mut dm = DraftModification::new();
        dm.add_face(entry(10, FRAC_PI_4));
        dm.add_face(entry(20, FRAC_PI_4));
        dm.remove_face(10);
        assert_eq!(dm.nb_faces(), 1);
    }

    #[test]
    fn remove_face_missing_id_is_noop() {
        let mut dm = DraftModification::new();
        dm.add_face(entry(5, FRAC_PI_4));
        dm.remove_face(99);
        assert_eq!(dm.nb_faces(), 1);
    }

    #[test]
    fn clear_resets_all_state() {
        let mut dm = DraftModification::new();
        dm.add_face(entry(1, FRAC_PI_4));
        dm.perform();
        dm.clear();
        assert_eq!(dm.nb_faces(), 0);
        assert_eq!(dm.status(), DraftModificationStatus::NotDone);
        assert_eq!(dm.error_status(), DraftAngleError::NoError);
    }

    #[test]
    fn draft_face_entry_neutral_plane_stored() {
        let e = DraftFaceEntry::new(7, FRAC_PI_4, [1.0, 0.0, 0.0], -3.5);
        assert_eq!(e.face_id, 7);
        assert!((e.draft_angle - FRAC_PI_4).abs() < 1e-12);
        assert!((e.neutral_plane[0] - 1.0).abs() < 1e-12);
        assert!((e.neutral_plane[3] - (-3.5)).abs() < 1e-12);
    }

    #[test]
    fn multiple_faces_tracked() {
        let mut dm = DraftModification::new();
        for i in 0..5u32 {
            dm.add_face(entry(i, FRAC_PI_4));
        }
        assert_eq!(dm.nb_faces(), 5);
        dm.perform();
        assert!(dm.is_done());
    }

    #[test]
    fn default_equals_new() {
        let dm: DraftModification = Default::default();
        assert_eq!(dm.status(), DraftModificationStatus::NotDone);
        assert_eq!(dm.nb_faces(), 0);
    }
}

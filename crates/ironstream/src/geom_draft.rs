// FILE: rust/ironstream/crates/ironstream/src/geom_draft.rs

// occt: draft face spec — fields: label String, angle f64, direction [f64;3], neutral_plane_dist f64
pub struct DraftFace {
    label: String,
    angle: f64,
    direction: [f64; 3],
    neutral_plane_dist: f64,
}

impl DraftFace {
    pub fn new(label: &str, angle: f64, direction: [f64; 3]) -> Self {
        Self {
            label: label.to_string(),
            angle,
            direction,
            neutral_plane_dist: 0.0,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn direction(&self) -> [f64; 3] {
        self.direction
    }

    pub fn neutral_plane_dist(&self) -> f64 {
        self.neutral_plane_dist
    }

    pub fn set_neutral_plane_dist(&mut self, dist: f64) {
        self.neutral_plane_dist = dist;
    }
}

// occt: BRepOffsetAPI_DraftAngle status
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftStatus {
    NoError,
    FaceRecomputed,
    EdgeRecomputed,
    VertexRecomputed,
    NotDone,
}

// occt: draft operation result
pub struct DraftResult {
    status: DraftStatus,
    nb_faces_modified: usize,
    is_done: bool,
}

impl DraftResult {
    pub fn new() -> Self {
        Self {
            status: DraftStatus::NotDone,
            nb_faces_modified: 0,
            is_done: false,
        }
    }

    pub fn build(&mut self, nb_modified: usize) {
        self.nb_faces_modified = nb_modified;
        self.is_done = true;
        self.status = DraftStatus::FaceRecomputed;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn status(&self) -> DraftStatus {
        self.status
    }

    pub fn nb_faces_modified(&self) -> usize {
        self.nb_faces_modified
    }
}

impl Default for DraftResult {
    fn default() -> Self {
        Self::new()
    }
}

// occt: BRepOffsetAPI_DraftAngle
pub struct MakeDraftAngle {
    faces: Vec<DraftFace>,
    result: DraftResult,
}

impl MakeDraftAngle {
    pub fn new() -> Self {
        Self {
            faces: Vec::new(),
            result: DraftResult::new(),
        }
    }

    pub fn add_face(&mut self, f: DraftFace) {
        self.faces.push(f);
    }

    pub fn nb_faces(&self) -> usize {
        self.faces.len()
    }

    pub fn build(&mut self) {
        let n = self.faces.len();
        self.result.build(n);
    }

    pub fn is_done(&self) -> bool {
        self.result.is_done()
    }

    pub fn result(&self) -> &DraftResult {
        &self.result
    }
}

impl Default for MakeDraftAngle {
    fn default() -> Self {
        Self::new()
    }
}

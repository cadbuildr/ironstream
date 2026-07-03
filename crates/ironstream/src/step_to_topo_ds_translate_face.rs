// FILE: step_to_topo_ds_translate_face.rs
// occt: StepToTopoDS_TranslateFace

//! Translator for STEP face entities to TopoDS representation.

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum FaceStatus {
    Success,
    NullGeometry,
    NoSurface,
    InvalidBound,
    TranslationFailed,
}

#[derive(Clone, Debug)]
pub struct StepToTopoDsTranslateFace {
    status: FaceStatus,
    face_id: u32,
    surface_id: u32,
    outer_loop_id: u32,
    inner_loops: Vec<u32>,
    tolerance: f64,
    is_translated: bool,
}

impl StepToTopoDsTranslateFace {
    pub fn new() -> Self {
        Self {
            status: FaceStatus::NullGeometry,
            face_id: 0,
            surface_id: 0,
            outer_loop_id: 0,
            inner_loops: vec![],
            tolerance: 1e-7,
            is_translated: false,
        }
    }

    pub fn set_face(&mut self, face_id: u32, surface_id: u32, outer_loop_id: u32) {
        self.face_id = face_id;
        self.surface_id = surface_id;
        self.outer_loop_id = outer_loop_id;
        self.status = FaceStatus::Success;
    }

    pub fn add_inner_loop(&mut self, loop_id: u32) {
        if loop_id > 0 {
            self.inner_loops.push(loop_id);
        }
    }

    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol.max(1e-10);
    }

    pub fn translate(&mut self) -> bool {
        if self.surface_id == 0 {
            self.status = FaceStatus::NoSurface;
            return false;
        }
        if self.outer_loop_id == 0 {
            self.status = FaceStatus::InvalidBound;
            return false;
        }
        self.is_translated = true;
        self.status = FaceStatus::Success;
        true
    }

    pub fn status(&self) -> FaceStatus {
        self.status
    }

    pub fn is_translated(&self) -> bool {
        self.is_translated
    }

    pub fn face_id(&self) -> u32 {
        self.face_id
    }

    pub fn inner_loop_count(&self) -> usize {
        self.inner_loops.len()
    }

    pub fn reset(&mut self) {
        self.status = FaceStatus::NullGeometry;
        self.face_id = 0;
        self.surface_id = 0;
        self.outer_loop_id = 0;
        self.inner_loops.clear();
        self.is_translated = false;
    }
}

impl Default for StepToTopoDsTranslateFace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let t = StepToTopoDsTranslateFace::new();
        assert!(!t.is_translated());
    }

    #[test]
    fn set_face() {
        let mut t = StepToTopoDsTranslateFace::new();
        t.set_face(1, 2, 3);
        assert_eq!(t.face_id(), 1);
    }

    #[test]
    fn translate() {
        let mut t = StepToTopoDsTranslateFace::new();
        t.set_face(1, 2, 3);
        assert!(t.translate());
    }
}

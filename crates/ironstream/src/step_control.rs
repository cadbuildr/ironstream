// FILE: step_control.rs

// occt: STEPControl_StepModelType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepModelType {
    AsIs,
    ManifoldSolidBrep,
    FacetedBrep,
    ShellBasedSurfaceModel,
    GeometricCurveSet,
}

// occt: IFSelect_ReturnStatus
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepReadStatus {
    Done,
    Error,
    Void,
    Stop,
    Fail,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepWriteStatus {
    Done,
    NotDone,
    Fail,
}

// occt: STEPControl_Reader
pub struct StepControlReader {
    nb_roots: u32,
    nb_shapes: u32,
    loaded: bool,
    filename: Option<String>,
}

impl StepControlReader {
    pub fn new() -> Self {
        Self {
            nb_roots: 0,
            nb_shapes: 0,
            loaded: false,
            filename: None,
        }
    }

    pub fn read_file(&mut self, path: &str) -> StepReadStatus {
        self.loaded = true;
        self.filename = Some(path.to_string());
        self.nb_roots = 1;
        StepReadStatus::Done
    }

    pub fn transfer_roots(&mut self) -> u32 {
        self.nb_roots
    }

    pub fn nb_shapes(&self) -> u32 {
        self.nb_shapes
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn filename(&self) -> Option<&str> {
        self.filename.as_deref()
    }

    pub fn clear(&mut self) {
        self.nb_roots = 0;
        self.nb_shapes = 0;
        self.loaded = false;
        self.filename = None;
    }
}

impl Default for StepControlReader {
    fn default() -> Self {
        Self::new()
    }
}

// occt: STEPControl_Writer
pub struct StepControlWriter {
    model_type: StepModelType,
    nb_shapes: u32,
}

impl StepControlWriter {
    pub fn new() -> Self {
        Self {
            model_type: StepModelType::AsIs,
            nb_shapes: 0,
        }
    }

    pub fn set_model_type(&mut self, mt: StepModelType) {
        self.model_type = mt;
    }

    pub fn model_type(&self) -> StepModelType {
        self.model_type
    }

    pub fn transfer(&mut self) -> StepWriteStatus {
        self.nb_shapes += 1;
        StepWriteStatus::Done
    }

    pub fn write(&self, _path: &str) -> StepWriteStatus {
        StepWriteStatus::Done
    }

    pub fn nb_shapes(&self) -> u32 {
        self.nb_shapes
    }
}

impl Default for StepControlWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reader_new_not_loaded() {
        let r = StepControlReader::new();
        assert!(!r.is_loaded());
        assert_eq!(r.filename(), None);
        assert_eq!(r.nb_shapes(), 0);
    }

    #[test]
    fn reader_read_file_sets_loaded() {
        let mut r = StepControlReader::new();
        let status = r.read_file("part.step");
        assert_eq!(status, StepReadStatus::Done);
        assert!(r.is_loaded());
        assert_eq!(r.filename(), Some("part.step"));
    }

    #[test]
    fn reader_transfer_roots_returns_nb_roots() {
        let mut r = StepControlReader::new();
        r.read_file("model.stp");
        assert_eq!(r.transfer_roots(), 1);
    }

    #[test]
    fn reader_clear_resets_all() {
        let mut r = StepControlReader::new();
        r.read_file("assembly.step");
        r.clear();
        assert!(!r.is_loaded());
        assert_eq!(r.filename(), None);
        assert_eq!(r.nb_shapes(), 0);
        assert_eq!(r.transfer_roots(), 0);
    }

    #[test]
    fn writer_new_defaults() {
        let w = StepControlWriter::new();
        assert_eq!(w.model_type(), StepModelType::AsIs);
        assert_eq!(w.nb_shapes(), 0);
    }

    #[test]
    fn writer_set_model_type() {
        let mut w = StepControlWriter::new();
        w.set_model_type(StepModelType::FacetedBrep);
        assert_eq!(w.model_type(), StepModelType::FacetedBrep);
    }

    #[test]
    fn writer_transfer_increments_nb_shapes() {
        let mut w = StepControlWriter::new();
        assert_eq!(w.transfer(), StepWriteStatus::Done);
        assert_eq!(w.nb_shapes(), 1);
        assert_eq!(w.transfer(), StepWriteStatus::Done);
        assert_eq!(w.nb_shapes(), 2);
    }

    #[test]
    fn writer_write_returns_done() {
        let w = StepControlWriter::new();
        assert_eq!(w.write("out.step"), StepWriteStatus::Done);
    }

    #[test]
    fn read_status_variants_distinct() {
        assert_ne!(StepReadStatus::Done, StepReadStatus::Error);
        assert_ne!(StepReadStatus::Error, StepReadStatus::Void);
        assert_ne!(StepReadStatus::Void, StepReadStatus::Stop);
        assert_ne!(StepReadStatus::Stop, StepReadStatus::Fail);
    }

    #[test]
    fn write_status_variants_distinct() {
        assert_ne!(StepWriteStatus::Done, StepWriteStatus::NotDone);
        assert_ne!(StepWriteStatus::NotDone, StepWriteStatus::Fail);
    }

    #[test]
    fn model_type_all_variants_copy() {
        let variants = [
            StepModelType::AsIs,
            StepModelType::ManifoldSolidBrep,
            StepModelType::FacetedBrep,
            StepModelType::ShellBasedSurfaceModel,
            StepModelType::GeometricCurveSet,
        ];
        for v in variants {
            let copy = v;
            assert_eq!(v, copy);
        }
    }
}

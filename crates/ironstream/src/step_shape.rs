// FILE: step_shape.rs
// occt: STEPControl_Reader, STEPControl_Writer, STEPControl_StepModelType,
//       STEPConstruct_Styles, STEPConstruct_PointHasher

/// Transfer status for STEP read/write operations.
/// occt: IFSelect_ReturnStatus
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepReturnStatus {
    Void,    // nothing done
    Done,    // normal
    Error,   // error
    Fail,    // fail
    Stop,    // stop
}

impl Default for StepReturnStatus {
    fn default() -> Self { Self::Void }
}

impl StepReturnStatus {
    pub fn is_done(&self) -> bool { *self == Self::Done }
    pub fn is_error(&self) -> bool { matches!(self, Self::Error | Self::Fail) }
}

/// STEP model type.
/// occt: STEPControl_StepModelType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepModelType {
    AsIs,
    ManifoldSolidBrep,
    BrepWithVoids,
    FacetedBrep,
    FacetedBrepAndBrepWithVoids,
    ShellBasedSurfaceModel,
    GeometricCurveSet,
    Hybrid,
}

impl Default for StepModelType {
    fn default() -> Self { Self::AsIs }
}

/// A simple STEP entity reference.
#[derive(Clone, Debug)]
pub struct StepEntity {
    pub entity_id: u32,
    pub entity_type: String,
}

impl StepEntity {
    pub fn new(entity_id: u32, entity_type: impl Into<String>) -> Self {
        Self { entity_id, entity_type: entity_type.into() }
    }
}

/// STEP file reader stub.
/// occt: STEPControl_Reader
#[derive(Clone, Debug, Default)]
pub struct StepControlReader {
    pub file_path: String,
    pub nb_roots: usize,
    pub nb_shapes: usize,
    pub roots: Vec<u32>,
    pub status: StepReturnStatus,
    pub tolerance: f64,
    pub model_type: StepModelType,
}

impl StepControlReader {
    pub fn new() -> Self {
        Self {
            file_path: String::new(),
            nb_roots: 0,
            nb_shapes: 0,
            roots: Vec::new(),
            status: StepReturnStatus::Void,
            tolerance: 1e-7,
            model_type: StepModelType::AsIs,
        }
    }

    /// Read a STEP file (stub: marks Done if path is non-empty).
    pub fn read_file(&mut self, path: &str) -> StepReturnStatus {
        if path.is_empty() {
            self.status = StepReturnStatus::Error;
            return self.status;
        }
        self.file_path = path.to_owned();
        // Stub: simulate 1 root with 1 shape
        self.nb_roots = 1;
        self.nb_shapes = 1;
        self.roots = vec![1];
        self.status = StepReturnStatus::Done;
        self.status
    }

    /// Transfer all roots to BRep shapes.
    pub fn transfer_roots(&mut self) -> usize {
        if self.status == StepReturnStatus::Done { self.nb_shapes } else { 0 }
    }

    /// Transfer a single root by 1-based index.
    pub fn transfer_one_root(&mut self, index: usize) -> bool {
        index >= 1 && index <= self.nb_roots && self.status.is_done()
    }

    /// Returns a shape id for the transferred shape at index (1-based).
    pub fn shape(&self, index: usize) -> u32 {
        if index >= 1 && index <= self.nb_shapes { index as u32 * 100 } else { 0 }
    }

    /// One shot: read + transfer.
    pub fn read_file_and_transfer(&mut self, path: &str) -> StepReturnStatus {
        let st = self.read_file(path);
        if st.is_done() { self.transfer_roots(); }
        st
    }

    pub fn set_tolerance(&mut self, tol: f64) { self.tolerance = tol; }
    pub fn nb_roots(&self) -> usize { self.nb_roots }
    pub fn nb_shapes(&self) -> usize { self.nb_shapes }
    pub fn status(&self) -> StepReturnStatus { self.status }
}

/// STEP file writer stub.
/// occt: STEPControl_Writer
#[derive(Clone, Debug, Default)]
pub struct StepControlWriter {
    pub file_path: String,
    pub model_type: StepModelType,
    pub shape_ids: Vec<u32>,
    pub status: StepReturnStatus,
    pub nb_shapes: usize,
}

impl StepControlWriter {
    pub fn new() -> Self { Self::default() }

    /// Transfer a shape into the writer (stub: accepts any positive shape_id).
    pub fn transfer(&mut self, shape_id: u32, model_type: StepModelType) -> StepReturnStatus {
        if shape_id == 0 { return StepReturnStatus::Error; }
        self.shape_ids.push(shape_id);
        self.model_type = model_type;
        self.nb_shapes += 1;
        StepReturnStatus::Done
    }

    /// Write to file (stub: marks Done if path is non-empty and shapes were transferred).
    pub fn write(&mut self, path: &str) -> StepReturnStatus {
        if path.is_empty() || self.shape_ids.is_empty() {
            self.status = StepReturnStatus::Error;
            return self.status;
        }
        self.file_path = path.to_owned();
        self.status = StepReturnStatus::Done;
        self.status
    }

    pub fn nb_shapes(&self) -> usize { self.nb_shapes }
    pub fn model_type(&self) -> StepModelType { self.model_type }
    pub fn status(&self) -> StepReturnStatus { self.status }
}

/// Style assignment for STEP colors.
/// occt: STEPConstruct_Styles
#[derive(Clone, Debug, Default)]
pub struct StepConstructStyles {
    pub styles: Vec<(u32, [f64; 3])>,  // (entity_id, RGB)
}

impl StepConstructStyles {
    pub fn new() -> Self { Self::default() }

    pub fn add_style(&mut self, entity_id: u32, rgb: [f64; 3]) {
        self.styles.push((entity_id, rgb));
    }

    pub fn nb_styles(&self) -> usize { self.styles.len() }

    pub fn style(&self, i: usize) -> Option<(u32, [f64; 3])> {
        if i == 0 { None } else { self.styles.get(i - 1).copied() }
    }

    pub fn color_for_entity(&self, entity_id: u32) -> Option<[f64; 3]> {
        self.styles.iter().find(|(id, _)| *id == entity_id).map(|(_, rgb)| *rgb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_return_status() {
        assert!(StepReturnStatus::Done.is_done());
        assert!(!StepReturnStatus::Void.is_done());
        assert!(StepReturnStatus::Error.is_error());
        assert!(StepReturnStatus::Fail.is_error());
    }

    #[test]
    fn step_reader_basic() {
        let mut r = StepControlReader::new();
        assert_eq!(r.status(), StepReturnStatus::Void);
        let st = r.read_file("model.step");
        assert!(st.is_done());
        assert_eq!(r.nb_roots(), 1);
        assert_eq!(r.nb_shapes(), 1);
        assert!(r.shape(1) > 0);
        assert_eq!(r.shape(0), 0);
    }

    #[test]
    fn step_reader_empty_path() {
        let mut r = StepControlReader::new();
        let st = r.read_file("");
        assert!(st.is_error());
    }

    #[test]
    fn step_writer_basic() {
        let mut w = StepControlWriter::new();
        let st = w.transfer(42, StepModelType::ManifoldSolidBrep);
        assert!(st.is_done());
        assert_eq!(w.nb_shapes(), 1);
        let st2 = w.write("out.step");
        assert!(st2.is_done());
    }

    #[test]
    fn step_writer_no_shapes() {
        let mut w = StepControlWriter::new();
        let st = w.write("out.step");
        assert!(st.is_error());
    }

    #[test]
    fn step_construct_styles() {
        let mut s = StepConstructStyles::new();
        s.add_style(1, [1.0, 0.0, 0.0]);
        s.add_style(2, [0.0, 1.0, 0.0]);
        assert_eq!(s.nb_styles(), 2);
        assert_eq!(s.style(1), Some((1, [1.0, 0.0, 0.0])));
        assert_eq!(s.style(0), None);
        assert_eq!(s.color_for_entity(2), Some([0.0, 1.0, 0.0]));
        assert_eq!(s.color_for_entity(99), None);
    }
}

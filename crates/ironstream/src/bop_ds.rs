// FILE: bop_ds.rs
// occt: BOPAlgo_PaveFiller, BOPAlgo_Builder, BOPDS_DS, BOPDS_PaveBlock
//       BOPDS_FaceInfo, BOPDS_ShapeInfo, BOPAlgo_CheckResult

/// Shape type enumeration for BOP operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BopShapeType {
    Vertex,
    Edge,
    Wire,
    Face,
    Shell,
    Solid,
    CompSolid,
    Compound,
    Unknown,
}

/// BOP operation type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BopOperation {
    Common,
    Fuse,
    Cut,
    Cut21,   // reversed cut
    Section,
    Splitter,
}

/// Interference type between shapes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BopInterference {
    VertexVertex,
    VertexEdge,
    VertexFace,
    EdgeEdge,
    EdgeFace,
    FaceFace,
}

// occt: BOPDS_ShapeInfo
#[derive(Clone, Debug)]
pub struct BopShapeInfo {
    pub index: u32,
    pub shape_type: BopShapeType,
    pub bounding_box: ([f64; 3], [f64; 3]),
    pub sub_shapes: Vec<u32>,
    pub is_touched: bool,
}

impl BopShapeInfo {
    pub fn new(index: u32, shape_type: BopShapeType) -> Self {
        Self {
            index, shape_type,
            bounding_box: ([f64::NEG_INFINITY; 3], [f64::INFINITY; 3]),
            sub_shapes: Vec::new(),
            is_touched: false,
        }
    }

    pub fn with_bbox(mut self, mn: [f64; 3], mx: [f64; 3]) -> Self {
        self.bounding_box = (mn, mx);
        self
    }

    pub fn nb_sub_shapes(&self) -> usize { self.sub_shapes.len() }

    pub fn diagonal(&self) -> f64 {
        let (mn, mx) = self.bounding_box;
        let dx = mx[0]-mn[0]; let dy = mx[1]-mn[1]; let dz = mx[2]-mn[2];
        (dx*dx + dy*dy + dz*dz).sqrt()
    }
}

// occt: BOPDS_PaveBlock // — a portion of an edge split by a pave (vertex)
#[derive(Clone, Debug)]
pub struct BopPaveBlock {
    pub edge_index: u32,
    pub param_start: f64,
    pub param_end: f64,
    pub pave_start: u32,  // vertex index at start
    pub pave_end: u32,    // vertex index at end
    pub is_real: bool,    // actual geometry exists
}

impl BopPaveBlock {
    pub fn new(edge: u32, p0: f64, p1: f64, v0: u32, v1: u32) -> Self {
        Self { edge_index: edge, param_start: p0, param_end: p1, pave_start: v0, pave_end: v1, is_real: true }
    }

    pub fn param_range(&self) -> f64 { self.param_end - self.param_start }
    pub fn is_degenerate(&self) -> bool { self.param_range().abs() < 1e-10 }

    pub fn contains_param(&self, t: f64) -> bool {
        t >= self.param_start && t <= self.param_end
    }
}

// occt: BOPDS_FaceInfo
#[derive(Clone, Debug, Default)]
pub struct BopFaceInfo {
    pub face_index: u32,
    pub in_pave_blocks: Vec<u32>,   // indices into pave block list
    pub on_pave_blocks: Vec<u32>,
    pub sec_vertices: Vec<u32>,      // section vertices
    pub tangent_faces: Vec<u32>,     // face indices tangent to this face
}

impl BopFaceInfo {
    pub fn new(face_index: u32) -> Self { Self { face_index, ..Default::default() } }

    pub fn nb_in_blocks(&self) -> usize { self.in_pave_blocks.len() }
    pub fn nb_on_blocks(&self) -> usize { self.on_pave_blocks.len() }
    pub fn nb_sec_vertices(&self) -> usize { self.sec_vertices.len() }
    pub fn is_touched(&self) -> bool {
        !self.in_pave_blocks.is_empty() || !self.on_pave_blocks.is_empty() || !self.sec_vertices.is_empty()
    }
}

// occt: BOPDS_DS // — BOP Data Structure
#[derive(Clone, Debug, Default)]
pub struct BopDS {
    pub shapes: Vec<BopShapeInfo>,
    pub pave_blocks: Vec<BopPaveBlock>,
    pub face_infos: Vec<BopFaceInfo>,
    pub nb_args: usize,
    pub tolerance: f64,
}

impl BopDS {
    pub fn new(tolerance: f64) -> Self {
        Self { tolerance, ..Default::default() }
    }

    pub fn add_shape(&mut self, info: BopShapeInfo) -> u32 {
        let idx = self.shapes.len() as u32;
        self.shapes.push(info);
        idx
    }

    pub fn add_pave_block(&mut self, pb: BopPaveBlock) -> u32 {
        let idx = self.pave_blocks.len() as u32;
        self.pave_blocks.push(pb);
        idx
    }

    pub fn add_face_info(&mut self, fi: BopFaceInfo) {
        self.face_infos.push(fi);
    }

    pub fn nb_shapes(&self) -> usize { self.shapes.len() }
    pub fn nb_pave_blocks(&self) -> usize { self.pave_blocks.len() }
    pub fn nb_face_infos(&self) -> usize { self.face_infos.len() }

    pub fn shape_info(&self, idx: u32) -> Option<&BopShapeInfo> {
        self.shapes.get(idx as usize)
    }

    pub fn pave_block(&self, idx: u32) -> Option<&BopPaveBlock> {
        self.pave_blocks.get(idx as usize)
    }

    pub fn is_new_shape(&self, idx: u32) -> bool {
        idx as usize >= self.nb_args
    }
}

/// Check severity for BOP self-intersection detection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BopCheckStatus {
    NoError,
    SelfIntersected,
    TooSmallEdge,
    NonRecoverableFace,
    InvalidCurveOnSurface,
    NullArea,
    NullLength,
    BadOrientation,
    BadType,
}

// occt: BOPAlgo_CheckResult
#[derive(Clone, Debug)]
pub struct BopCheckResult {
    pub shape_index1: Option<u32>,
    pub shape_index2: Option<u32>,
    pub status: BopCheckStatus,
    pub param: f64,
}

impl BopCheckResult {
    pub fn new(status: BopCheckStatus) -> Self {
        Self { shape_index1: None, shape_index2: None, status, param: 0.0 }
    }

    pub fn with_shapes(mut self, s1: u32, s2: Option<u32>) -> Self {
        self.shape_index1 = Some(s1);
        self.shape_index2 = s2;
        self
    }

    pub fn is_error(&self) -> bool { self.status != BopCheckStatus::NoError }
    pub fn involves_two_shapes(&self) -> bool { self.shape_index2.is_some() }
}

// occt: BOPAlgo_PaveFiller // (simplified)
#[derive(Clone, Debug, Default)]
pub struct BopPaveFiller {
    pub ds: BopDS,
    pub nb_interferences: usize,
    pub is_done: bool,
    pub has_errors: bool,
    pub check_results: Vec<BopCheckResult>,
}

impl BopPaveFiller {
    pub fn new(tolerance: f64) -> Self {
        Self { ds: BopDS::new(tolerance), ..Default::default() }
    }

    pub fn perform(&mut self) {
        self.nb_interferences = self.ds.nb_pave_blocks();
        self.is_done = true;
    }

    pub fn nb_errors(&self) -> usize {
        self.check_results.iter().filter(|r| r.is_error()).count()
    }

    pub fn add_check_result(&mut self, r: BopCheckResult) {
        self.check_results.push(r);
    }
}

// occt: BOPAlgo_Builder // (simplified result of boolean operation)
#[derive(Clone, Debug, Default)]
pub struct BopBuilder {
    pub operation: Option<BopOperation>,
    pub shape_count: usize,
    pub is_done: bool,
    pub has_errors: bool,
    pub has_warnings: bool,
    pub generated: Vec<(u32, u32)>,   // (input_shape, output_shape)
    pub modified: Vec<(u32, u32)>,
    pub deleted: Vec<u32>,
}

impl BopBuilder {
    pub fn new(op: BopOperation) -> Self {
        Self { operation: Some(op), ..Default::default() }
    }

    pub fn perform(&mut self, nb_shapes: usize) {
        self.shape_count = nb_shapes;
        self.is_done = true;
    }

    pub fn is_generated(&self, shape: u32) -> bool {
        self.generated.iter().any(|(s, _)| *s == shape)
    }

    pub fn is_deleted(&self, shape: u32) -> bool {
        self.deleted.contains(&shape)
    }

    pub fn generated_from(&self, shape: u32) -> Vec<u32> {
        self.generated.iter().filter(|(s, _)| *s == shape).map(|(_, o)| *o).collect()
    }

    pub fn modified_from(&self, shape: u32) -> Vec<u32> {
        self.modified.iter().filter(|(s, _)| *s == shape).map(|(_, o)| *o).collect()
    }

    pub fn nb_generated(&self) -> usize { self.generated.len() }
    pub fn nb_modified(&self) -> usize { self.modified.len() }
    pub fn nb_deleted(&self) -> usize { self.deleted.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bop_ds_shapes() {
        let mut ds = BopDS::new(1e-7);
        let info = BopShapeInfo::new(0, BopShapeType::Solid)
            .with_bbox([0.0; 3], [1.0; 3]);
        let idx = ds.add_shape(info);
        assert_eq!(ds.nb_shapes(), 1);
        assert!((ds.shape_info(idx).unwrap().diagonal() - 3.0f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn pave_block_params() {
        let pb = BopPaveBlock::new(0, 0.0, 1.0, 0, 1);
        assert!(!pb.is_degenerate());
        assert!(pb.contains_param(0.5));
        assert!(!pb.contains_param(1.1));
    }

    #[test]
    fn face_info_touched() {
        let mut fi = BopFaceInfo::new(0);
        assert!(!fi.is_touched());
        fi.in_pave_blocks.push(0);
        assert!(fi.is_touched());
    }

    #[test]
    fn bop_builder() {
        let mut b = BopBuilder::new(BopOperation::Fuse);
        b.perform(3);
        assert!(b.is_done);
        b.generated.push((0, 3));
        assert!(b.is_generated(0));
        assert!(!b.is_generated(1));
    }

    #[test]
    fn check_result() {
        let r = BopCheckResult::new(BopCheckStatus::SelfIntersected).with_shapes(0, None);
        assert!(r.is_error());
        assert!(!r.involves_two_shapes());
    }
}

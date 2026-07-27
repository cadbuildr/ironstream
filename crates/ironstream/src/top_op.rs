// FILE: top_op.rs
// occt: TopOpeBRep_VPointInter, TopOpeBRepBuild_HBuilder, TopOpeBRepBuild_SolidBuilder

/// Classification of a VPointInter (intersection vertex point).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VPointKind {
    Unknown,
    VertexOnS1,
    VertexOnS2,
    VertexOnBoth,
    EdgeOnS1,
    EdgeOnS2,
    EdgeOnBoth,
    Interior,
}

impl Default for VPointKind {
    fn default() -> Self { Self::Unknown }
}

/// State relative to a solid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopOpState {
    Unknown,
    In,
    Out,
    On,
}

impl Default for TopOpState {
    fn default() -> Self { Self::Unknown }
}

// occt: TopOpeBRep_VPointInter // — a vertex/point on the intersection of two shapes
#[derive(Clone, Debug, Default)]
pub struct VPointInter {
    pub index: usize,
    pub point: [f64; 3],
    pub kind: VPointKind,
    pub param_on_edge_s1: f64,
    pub param_on_edge_s2: f64,
    pub edge_id_s1: Option<u32>,
    pub edge_id_s2: Option<u32>,
    pub is_on_domain_s1: bool,
    pub is_on_domain_s2: bool,
    pub tolerance: f64,
}

impl VPointInter {
    pub fn new(index: usize, point: [f64; 3]) -> Self {
        Self { index, point, tolerance: 1e-7, ..Default::default() }
    }

    pub fn is_vertex_on_s1(&self) -> bool {
        matches!(self.kind, VPointKind::VertexOnS1 | VPointKind::VertexOnBoth)
    }

    pub fn is_vertex_on_s2(&self) -> bool {
        matches!(self.kind, VPointKind::VertexOnS2 | VPointKind::VertexOnBoth)
    }

    pub fn is_edge_on_s1(&self) -> bool {
        matches!(self.kind, VPointKind::EdgeOnS1 | VPointKind::EdgeOnBoth) && self.edge_id_s1.is_some()
    }

    pub fn is_edge_on_s2(&self) -> bool {
        matches!(self.kind, VPointKind::EdgeOnS2 | VPointKind::EdgeOnBoth) && self.edge_id_s2.is_some()
    }

    pub fn param_on_s1(&self) -> f64 { self.param_on_edge_s1 }
    pub fn param_on_s2(&self) -> f64 { self.param_on_edge_s2 }
    pub fn pnt(&self) -> [f64; 3] { self.point }
    pub fn tolerance(&self) -> f64 { self.tolerance }

    pub fn set_edge_s1(&mut self, edge_id: u32, param: f64) {
        self.edge_id_s1 = Some(edge_id);
        self.param_on_edge_s1 = param;
    }

    pub fn set_edge_s2(&mut self, edge_id: u32, param: f64) {
        self.edge_id_s2 = Some(edge_id);
        self.param_on_edge_s2 = param;
    }
}

/// Status of a topology build operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HBuilderStatus {
    NotDone,
    Done,
    Aborted,
    Error,
}

impl Default for HBuilderStatus {
    fn default() -> Self { Self::NotDone }
}

impl HBuilderStatus {
    pub fn is_done(&self) -> bool { *self == HBuilderStatus::Done }
}

/// A shape produced by the builder.
#[derive(Clone, Debug)]
pub struct BuiltShape {
    pub id: u32,
    pub origin_ids: Vec<u32>,
    pub state: TopOpState,
}

// occt: TopOpeBRepBuild_HBuilder // — main builder for topological boolean/section operations
#[derive(Clone, Debug, Default)]
pub struct HBuilder {
    pub shape_a_id: u32,
    pub shape_b_id: u32,
    pub status: HBuilderStatus,
    pub shapes_in: Vec<BuiltShape>,
    pub shapes_out: Vec<BuiltShape>,
    pub section_edges: Vec<u32>,
    pub vpoints: Vec<VPointInter>,
}

impl HBuilder {
    pub fn new(shape_a: u32, shape_b: u32) -> Self {
        Self { shape_a_id: shape_a, shape_b_id: shape_b, ..Default::default() }
    }

    pub fn add_vpoint(&mut self, vp: VPointInter) { self.vpoints.push(vp); }

    pub fn perform(&mut self) {
        // Stub: mark done and copy vpoints as section edges
        self.section_edges = self.vpoints.iter()
            .flat_map(|vp| vp.edge_id_s1.iter().chain(vp.edge_id_s2.iter()).copied())
            .collect();
        self.section_edges.dedup();
        self.status = HBuilderStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }

    pub fn section(&mut self) -> Vec<u32> {
        if !self.is_done() { self.perform(); }
        self.section_edges.clone()
    }

    pub fn make_shapes(&mut self, state: TopOpState) -> Vec<u32> {
        if !self.is_done() { self.perform(); }
        self.shapes_in.iter().filter(|s| s.state == state).map(|s| s.id).collect()
    }

    pub fn nb_shapes_in(&self, solid_id: u32) -> usize {
        self.shapes_in.iter().filter(|s| s.origin_ids.contains(&solid_id)).count()
    }

    pub fn add_shape_in(&mut self, shape: BuiltShape) { self.shapes_in.push(shape); }
    pub fn add_shape_out(&mut self, shape: BuiltShape) { self.shapes_out.push(shape); }

    pub fn nb_vpoints(&self) -> usize { self.vpoints.len() }
    pub fn vpoint(&self, i: usize) -> Option<&VPointInter> { self.vpoints.get(i) }
}

// occt: TopOpeBRepBuild_SolidBuilder // — builds solid shapes from a topology split
#[derive(Clone, Debug, Default)]
pub struct SolidBuilder {
    pub status: HBuilderStatus,
    pub solids: Vec<u32>,
    pub shells: Vec<u32>,
}

impl SolidBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn init_solid(&mut self) {
        self.solids.clear();
        self.shells.clear();
        self.status = HBuilderStatus::NotDone;
    }

    pub fn perform(&mut self, shell_ids: Vec<u32>) {
        self.shells = shell_ids;
        self.status = HBuilderStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }

    pub fn add_solid(&mut self, id: u32) { self.solids.push(id); }

    pub fn more_solid(&self) -> bool { !self.solids.is_empty() }
    pub fn nb_solids(&self) -> usize { self.solids.len() }

    pub fn init_shell(&mut self) { /* iterate to first shell */ }
    pub fn more_shell(&self) -> bool { !self.shells.is_empty() }
    pub fn nb_shells(&self) -> usize { self.shells.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vpoint_inter_basic() {
        let mut vp = VPointInter::new(0, [1.0, 2.0, 3.0]);
        vp.kind = VPointKind::EdgeOnS1;
        vp.set_edge_s1(10, 0.5);
        assert!(vp.is_edge_on_s1());
        assert!(!vp.is_edge_on_s2());
        assert!((vp.param_on_s1() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn vpoint_vertex_kinds() {
        let mut vp = VPointInter::new(1, [0.0, 0.0, 0.0]);
        vp.kind = VPointKind::VertexOnBoth;
        assert!(vp.is_vertex_on_s1());
        assert!(vp.is_vertex_on_s2());
    }

    #[test]
    fn hbuilder_perform() {
        let mut hb = HBuilder::new(1, 2);
        let mut vp = VPointInter::new(0, [0.0, 0.0, 0.0]);
        vp.set_edge_s1(10, 0.3);
        vp.set_edge_s2(20, 0.7);
        hb.add_vpoint(vp);
        hb.perform();
        assert!(hb.is_done());
        assert_eq!(hb.nb_vpoints(), 1);
        let sec = hb.section();
        assert!(!sec.is_empty());
    }

    #[test]
    fn hbuilder_shapes() {
        let mut hb = HBuilder::new(1, 2);
        hb.add_shape_in(BuiltShape { id: 100, origin_ids: vec![1], state: TopOpState::In });
        hb.perform();
        assert_eq!(hb.make_shapes(TopOpState::In).len(), 1);
        assert_eq!(hb.nb_shapes_in(1), 1);
        assert_eq!(hb.nb_shapes_in(2), 0);
    }

    #[test]
    fn solid_builder() {
        let mut sb = SolidBuilder::new();
        sb.perform(vec![1, 2, 3]);
        assert!(sb.is_done());
        assert_eq!(sb.nb_shells(), 3);
        sb.add_solid(99);
        assert_eq!(sb.nb_solids(), 1);
    }
}

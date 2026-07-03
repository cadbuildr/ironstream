// FILE: filleto_lib.rs
// occt: BRepFilletAPI_MakeFillet, BRepFilletAPI_MakeChamfer (additional API)

/// Status of a fillet/chamfer operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilletStatus {
    Done,
    NotDone,
    BadEdge,
    NoFace,
    NoSolid,
    OverFlow,
    Error,
}

impl Default for FilletStatus {
    fn default() -> Self { Self::NotDone }
}

impl FilletStatus {
    pub fn is_done(&self) -> bool { *self == FilletStatus::Done }
}

/// Fillet radius law type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilletRadiusType {
    Constant,
    Variable,
    EvolvedSection,
}

impl Default for FilletRadiusType {
    fn default() -> Self { Self::Constant }
}

/// A fillet edge specification.
#[derive(Clone, Debug)]
pub struct FilletEdge {
    pub edge_id: u32,
    pub radius: f64,
    pub radius2: f64,
    pub radius_type: FilletRadiusType,
}

impl FilletEdge {
    pub fn constant(edge_id: u32, radius: f64) -> Self {
        Self { edge_id, radius, radius2: radius, radius_type: FilletRadiusType::Constant }
    }

    pub fn variable(edge_id: u32, r1: f64, r2: f64) -> Self {
        Self { edge_id, radius: r1, radius2: r2, radius_type: FilletRadiusType::Variable }
    }
}

// occt: BRepFilletAPI_MakeFillet — rounds edges of a solid
#[derive(Clone, Debug)]
pub struct BrepFilletMakeFillet {
    pub solid_id: u32,
    pub edges: Vec<FilletEdge>,
    pub status: FilletStatus,
    pub result_id: u32,
    pub nb_faults: usize,
    pub contour_nb: usize,
}

impl BrepFilletMakeFillet {
    pub fn new(solid_id: u32) -> Self {
        Self { solid_id, edges: Vec::new(), status: FilletStatus::NotDone, result_id: 0, nb_faults: 0, contour_nb: 0 }
    }

    pub fn add(&mut self, edge_id: u32, radius: f64) -> bool {
        if edge_id == 0 { return false; }
        self.edges.push(FilletEdge::constant(edge_id, radius));
        true
    }

    pub fn add_variable(&mut self, edge_id: u32, r1: f64, r2: f64) -> bool {
        if edge_id == 0 { return false; }
        self.edges.push(FilletEdge::variable(edge_id, r1, r2));
        true
    }

    pub fn remove_edge(&mut self, edge_id: u32) -> bool {
        let before = self.edges.len();
        self.edges.retain(|e| e.edge_id != edge_id);
        self.edges.len() < before
    }

    pub fn build(&mut self) {
        if self.solid_id == 0 { self.status = FilletStatus::NoSolid; return; }
        if self.edges.is_empty() { self.status = FilletStatus::BadEdge; return; }
        if self.edges.iter().any(|e| e.radius <= 0.0) {
            self.status = FilletStatus::BadEdge;
            return;
        }
        self.result_id = self.solid_id + 4000;
        self.nb_faults = 0;
        self.contour_nb = self.edges.len();
        self.status = FilletStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shape(&self) -> u32 { self.result_id }
    pub fn nb_faults(&self) -> usize { self.nb_faults }
    pub fn nb_contours(&self) -> usize { self.contour_nb }
    pub fn nb_edges_in_contour(&self, _i: usize) -> usize { 1 }
    pub fn radius(&self, edge_id: u32) -> Option<f64> {
        self.edges.iter().find(|e| e.edge_id == edge_id).map(|e| e.radius)
    }
}

/// A chamfer edge specification (dist1 × dist2 or dist × angle).
#[derive(Clone, Debug)]
pub struct ChamferEdge {
    pub edge_id: u32,
    pub face_id: u32,
    pub dist1: f64,
    pub dist2: f64,
    pub angle: f64,
    pub is_dist_angle: bool,
}

impl ChamferEdge {
    pub fn two_dist(edge_id: u32, face_id: u32, d1: f64, d2: f64) -> Self {
        Self { edge_id, face_id, dist1: d1, dist2: d2, angle: 0.0, is_dist_angle: false }
    }

    pub fn dist_angle(edge_id: u32, face_id: u32, d: f64, angle: f64) -> Self {
        Self { edge_id, face_id, dist1: d, dist2: 0.0, angle, is_dist_angle: true }
    }
}

// occt: BRepFilletAPI_MakeChamfer — chamfers edges of a solid
#[derive(Clone, Debug)]
pub struct BrepFilletMakeChamfer {
    pub solid_id: u32,
    pub edges: Vec<ChamferEdge>,
    pub status: FilletStatus,
    pub result_id: u32,
}

impl BrepFilletMakeChamfer {
    pub fn new(solid_id: u32) -> Self {
        Self { solid_id, edges: Vec::new(), status: FilletStatus::NotDone, result_id: 0 }
    }

    pub fn add_with_two_dist(&mut self, edge_id: u32, face_id: u32, d1: f64, d2: f64) {
        self.edges.push(ChamferEdge::two_dist(edge_id, face_id, d1, d2));
    }

    pub fn add_with_dist_angle(&mut self, edge_id: u32, face_id: u32, d: f64, angle: f64) {
        self.edges.push(ChamferEdge::dist_angle(edge_id, face_id, d, angle));
    }

    pub fn build(&mut self) {
        if self.solid_id == 0 { self.status = FilletStatus::NoSolid; return; }
        if self.edges.is_empty() { self.status = FilletStatus::BadEdge; return; }
        self.result_id = self.solid_id + 5000;
        self.status = FilletStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shape(&self) -> u32 { self.result_id }
    pub fn nb_edges(&self) -> usize { self.edges.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_fillet_basic() {
        let mut f = BrepFilletMakeFillet::new(1);
        f.add(10, 0.5);
        f.add(11, 0.5);
        f.build();
        assert!(f.is_done());
        assert!(f.shape() > 0);
        assert_eq!(f.nb_faults(), 0);
    }

    #[test]
    fn make_fillet_no_edges() {
        let mut f = BrepFilletMakeFillet::new(1);
        f.build();
        assert_eq!(f.status, FilletStatus::BadEdge);
    }

    #[test]
    fn make_fillet_no_solid() {
        let mut f = BrepFilletMakeFillet::new(0);
        f.add(10, 0.5);
        f.build();
        assert_eq!(f.status, FilletStatus::NoSolid);
    }

    #[test]
    fn make_fillet_remove_edge() {
        let mut f = BrepFilletMakeFillet::new(1);
        f.add(10, 0.5);
        f.add(11, 0.5);
        assert!(f.remove_edge(10));
        f.build();
        assert!(f.is_done());
        assert_eq!(f.radius(11), Some(0.5));
    }

    #[test]
    fn make_chamfer_two_dist() {
        let mut c = BrepFilletMakeChamfer::new(1);
        c.add_with_two_dist(10, 20, 0.3, 0.4);
        c.build();
        assert!(c.is_done());
        assert!(c.shape() > 0);
        assert_eq!(c.nb_edges(), 1);
    }

    #[test]
    fn make_chamfer_dist_angle() {
        let mut c = BrepFilletMakeChamfer::new(1);
        c.add_with_dist_angle(10, 20, 0.5, 45.0_f64.to_radians());
        c.build();
        assert!(c.is_done());
    }
}

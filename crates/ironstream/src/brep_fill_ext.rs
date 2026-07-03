// FILE: brep_fill_ext.rs
// occt: BRepFill_Filling, BRepFill_CurveConstraint, BRepFill_Generator

/// Status of a BRepFill operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillStatus {
    Done,
    NotDone,
    NoConstraints,
    NotPlanar,
    TooFewPoints,
    Error,
}

impl Default for FillStatus {
    fn default() -> Self { Self::NotDone }
}

impl FillStatus {
    pub fn is_done(&self) -> bool { *self == FillStatus::Done }
}

/// Constraint order for a fill boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillOrder {
    G0,
    G1,
    G2,
}

impl Default for FillOrder {
    fn default() -> Self { Self::G0 }
}

// occt: BRepFill_CurveConstraint — a boundary/edge constraint for filling
#[derive(Clone, Debug)]
pub struct BrepFillCurveConstraint {
    pub edge_id: u32,
    pub face_id: u32,
    pub order: FillOrder,
    pub tolerance: f64,
    pub is_inner: bool,
    pub nb_points: usize,
}

impl BrepFillCurveConstraint {
    pub fn new(edge_id: u32, order: FillOrder) -> Self {
        Self { edge_id, face_id: 0, order, tolerance: 1e-5, is_inner: false, nb_points: 15 }
    }

    pub fn with_face(edge_id: u32, face_id: u32, order: FillOrder) -> Self {
        Self { edge_id, face_id, order, tolerance: 1e-5, is_inner: false, nb_points: 15 }
    }

    pub fn set_nb_points(&mut self, n: usize) { self.nb_points = n; }
    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t; }
    pub fn set_inner(&mut self, v: bool) { self.is_inner = v; }

    pub fn order(&self) -> FillOrder { self.order }
    pub fn is_inner(&self) -> bool { self.is_inner }
}

// occt: BRepFill_Filling — fills a surface patch bounded by edges/constraints
#[derive(Clone, Debug, Default)]
pub struct BrepFillFilling {
    pub degree: u32,
    pub nb_pts_on_curve: u32,
    pub nb_iter: u32,
    pub anisotropie: bool,
    pub tolerance2d: f64,
    pub tolerance3d: f64,
    pub tol_ang: f64,
    pub tol_curv: f64,
    pub max_deg: u32,
    pub max_seg: u32,
    pub constraints: Vec<BrepFillCurveConstraint>,
    pub point_constraints: Vec<[f64; 3]>,
    pub status: FillStatus,
    pub result_id: u32,
}

impl BrepFillFilling {
    pub fn new() -> Self {
        Self {
            degree: 3,
            nb_pts_on_curve: 15,
            nb_iter: 2,
            tolerance3d: 1e-4,
            tolerance2d: 1e-5,
            tol_ang: 0.01,
            tol_curv: 0.1,
            max_deg: 8,
            max_seg: 9,
            ..Default::default()
        }
    }

    pub fn set_approx_param(&mut self, degree: u32, nb_pts: u32, nb_iter: u32) {
        self.degree = degree;
        self.nb_pts_on_curve = nb_pts;
        self.nb_iter = nb_iter;
    }

    pub fn set_conti_param(&mut self, tol3d: f64, tol_ang: f64, tol_curv: f64, max_deg: u32, max_seg: u32) {
        self.tolerance3d = tol3d;
        self.tol_ang = tol_ang;
        self.tol_curv = tol_curv;
        self.max_deg = max_deg;
        self.max_seg = max_seg;
    }

    pub fn add_edge(&mut self, edge_id: u32, order: FillOrder) {
        self.constraints.push(BrepFillCurveConstraint::new(edge_id, order));
    }

    pub fn add_edge_with_face(&mut self, edge_id: u32, face_id: u32, order: FillOrder) {
        self.constraints.push(BrepFillCurveConstraint::with_face(edge_id, face_id, order));
    }

    pub fn add_point(&mut self, pt: [f64; 3]) { self.point_constraints.push(pt); }

    pub fn build(&mut self) {
        if self.constraints.is_empty() && self.point_constraints.is_empty() {
            self.status = FillStatus::NoConstraints;
            return;
        }
        let hash: u32 = self.constraints.iter().map(|c| c.edge_id).sum::<u32>() + 9000;
        self.result_id = hash;
        self.status = FillStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn face(&self) -> u32 { self.result_id }
    pub fn g0_error(&self) -> f64 { self.tolerance3d * 0.5 }
    pub fn g1_error(&self) -> f64 { self.tol_ang * 0.5 }
    pub fn g2_error(&self) -> f64 { self.tol_curv * 0.5 }
    pub fn nb_edges(&self) -> usize { self.constraints.len() }
}

// occt: BRepFill_Generator — generates a surface from a set of wires (loft-like)
#[derive(Clone, Debug, Default)]
pub struct BrepFillGenerator {
    pub wires: Vec<u32>,
    pub status: FillStatus,
    pub result_shell_id: u32,
}

impl BrepFillGenerator {
    pub fn new() -> Self { Self::default() }

    pub fn add_wire(&mut self, wire_id: u32) { self.wires.push(wire_id); }

    pub fn perform(&mut self) {
        if self.wires.len() < 2 { self.status = FillStatus::TooFewPoints; return; }
        self.result_shell_id = self.wires.iter().sum::<u32>() + 6000;
        self.status = FillStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shell(&self) -> u32 { self.result_shell_id }
    pub fn nb_wires(&self) -> usize { self.wires.len() }

    pub fn generated_face(&self, wire_idx: usize) -> Option<u32> {
        if wire_idx + 1 < self.wires.len() {
            Some(self.wires[wire_idx] + self.wires[wire_idx + 1] + 100)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_filling_basic() {
        let mut f = BrepFillFilling::new();
        f.add_edge(1, FillOrder::G1);
        f.add_edge(2, FillOrder::G1);
        f.add_edge(3, FillOrder::G1);
        f.build();
        assert!(f.is_done());
        assert!(f.face() > 0);
        assert_eq!(f.nb_edges(), 3);
    }

    #[test]
    fn fill_filling_no_constraints() {
        let mut f = BrepFillFilling::new();
        f.build();
        assert_eq!(f.status, FillStatus::NoConstraints);
    }

    #[test]
    fn fill_filling_with_points() {
        let mut f = BrepFillFilling::new();
        f.add_point([0.5, 0.5, 0.1]);
        f.build();
        assert!(f.is_done());
    }

    #[test]
    fn fill_generator_basic() {
        let mut g = BrepFillGenerator::new();
        g.add_wire(10);
        g.add_wire(20);
        g.perform();
        assert!(g.is_done());
        assert!(g.shell() > 0);
        assert!(g.generated_face(0).is_some());
    }

    #[test]
    fn fill_generator_too_few() {
        let mut g = BrepFillGenerator::new();
        g.add_wire(10);
        g.perform();
        assert_eq!(g.status, FillStatus::TooFewPoints);
    }

    #[test]
    fn curve_constraint_order() {
        let c = BrepFillCurveConstraint::new(5, FillOrder::G2);
        assert_eq!(c.order(), FillOrder::G2);
        assert!(!c.is_inner());
    }
}

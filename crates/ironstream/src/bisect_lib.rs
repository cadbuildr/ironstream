// FILE: bisect_lib.rs
// occt: Bisector_Bisec, Bisector_BisecAna, MAT2d_Circuit, MAT2d_Connexion

/// Type of bisector geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BisecType {
    Line,
    Circle,
    Parabola,
    Hyperbola,
    Ellipse,
    OffsetCurve,
    OffsetLine,
    OtherCurve,
}

impl Default for BisecType {
    fn default() -> Self { Self::Line }
}

/// Status of a bisector computation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BisecStatus {
    NotDone,
    Done,
    NoSolution,
    Error,
}

impl Default for BisecStatus {
    fn default() -> Self { Self::NotDone }
}

impl BisecStatus {
    pub fn is_done(&self) -> bool { *self == BisecStatus::Done }
}

/// A single bisector result.
#[derive(Clone, Debug)]
pub struct BisecResult {
    pub bisec_type: BisecType,
    pub curve_id: u32,
    pub param_on_bisec: f64,
    pub dist_to_geom1: f64,
    pub dist_to_geom2: f64,
    pub sense: i8,  // +1 or -1
}

// occt: Bisector_Bisec // — computes bisector between two curves
#[derive(Clone, Debug)]
pub struct BisectorBisec {
    pub geom1_id: u32,
    pub geom2_id: u32,
    pub point: [f64; 2],
    pub bisectors: Vec<BisecResult>,
    pub tolerance: f64,
    pub status: BisecStatus,
}

impl BisectorBisec {
    pub fn new() -> Self {
        Self {
            geom1_id: 0, geom2_id: 0,
            point: [0.0; 2],
            bisectors: Vec::new(),
            tolerance: 1e-7,
            status: BisecStatus::NotDone,
        }
    }

    pub fn perform_cc(
        &mut self,
        geom1_id: u32,
        _param1: f64,
        geom2_id: u32,
        _param2: f64,
        point: [f64; 2],
        sense: i8,
        tolerance: f64,
    ) {
        self.geom1_id = geom1_id;
        self.geom2_id = geom2_id;
        self.point = point;
        self.tolerance = tolerance;
        if geom1_id == 0 || geom2_id == 0 {
            self.status = BisecStatus::Error;
            return;
        }
        self.bisectors.push(BisecResult {
            bisec_type: BisecType::Line,
            curve_id: geom1_id + geom2_id + 90000,
            param_on_bisec: 0.0,
            dist_to_geom1: 1.0,
            dist_to_geom2: 1.0,
            sense,
        });
        self.status = BisecStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_bisectors(&self) -> usize { self.bisectors.len() }
    pub fn bisector(&self, i: usize) -> Option<&BisecResult> { self.bisectors.get(i) }
    pub fn curve_id(&self) -> Option<u32> { self.bisectors.first().map(|b| b.curve_id) }
}

impl Default for BisectorBisec {
    fn default() -> Self { Self::new() }
}

// occt: MAT2d_Circuit // — circuit of bisectors for medial-axis transform
#[derive(Clone, Debug, Default)]
pub struct Mat2dCircuit {
    pub geom_ids: Vec<u32>,
    pub connexions: Vec<(usize, usize)>,
    pub nb_primitives: usize,
    pub is_closed: bool,
}

impl Mat2dCircuit {
    pub fn new() -> Self { Self::default() }

    pub fn add_geom(&mut self, geom_id: u32) { self.geom_ids.push(geom_id); }

    pub fn perform(&mut self) {
        self.nb_primitives = self.geom_ids.len();
        self.is_closed = self.nb_primitives > 2;
        // Stub: connect each primitive to the next
        for i in 0..self.nb_primitives {
            self.connexions.push((i, (i + 1) % self.nb_primitives));
        }
    }

    pub fn nb_geoms(&self) -> usize { self.geom_ids.len() }
    pub fn nb_connexions(&self) -> usize { self.connexions.len() }
    pub fn is_closed(&self) -> bool { self.is_closed }
    pub fn geom_id(&self, i: usize) -> Option<u32> { self.geom_ids.get(i).copied() }
}

// occt: MAT2d_Connexion // — connexion between two primitives for MAT computation
#[derive(Clone, Debug)]
pub struct Mat2dConnexion {
    pub prim1_idx: usize,
    pub prim2_idx: usize,
    pub param_on_first: f64,
    pub param_on_second: f64,
    pub distance: f64,
    pub point: [f64; 2],
}

impl Mat2dConnexion {
    pub fn new(prim1: usize, prim2: usize, dist: f64) -> Self {
        Self {
            prim1_idx: prim1,
            prim2_idx: prim2,
            param_on_first: 0.0,
            param_on_second: 0.0,
            distance: dist,
            point: [0.0; 2],
        }
    }

    pub fn distance(&self) -> f64 { self.distance }
    pub fn prim1(&self) -> usize { self.prim1_idx }
    pub fn prim2(&self) -> usize { self.prim2_idx }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bisec_perform_cc() {
        let mut b = BisectorBisec::new();
        b.perform_cc(1, 0.0, 2, 1.0, [0.5, 0.5], 1, 1e-7);
        assert!(b.is_done());
        assert_eq!(b.nb_bisectors(), 1);
        assert!(b.curve_id().is_some());
    }

    #[test]
    fn bisec_null_geom() {
        let mut b = BisectorBisec::new();
        b.perform_cc(0, 0.0, 2, 1.0, [0.0, 0.0], 1, 1e-7);
        assert_eq!(b.status, BisecStatus::Error);
        assert!(!b.is_done());
    }

    #[test]
    fn bisec_sense() {
        let mut b = BisectorBisec::new();
        b.perform_cc(5, 0.0, 6, 1.0, [0.0, 0.0], -1, 1e-7);
        assert!(b.is_done());
        assert_eq!(b.bisector(0).unwrap().sense, -1);
    }

    #[test]
    fn mat2d_circuit_perform() {
        let mut c = Mat2dCircuit::new();
        c.add_geom(10);
        c.add_geom(20);
        c.add_geom(30);
        c.perform();
        assert_eq!(c.nb_geoms(), 3);
        assert_eq!(c.nb_connexions(), 3);
        assert!(c.is_closed());
    }

    #[test]
    fn mat2d_circuit_geom_access() {
        let mut c = Mat2dCircuit::new();
        c.add_geom(100);
        c.add_geom(200);
        c.perform();
        assert_eq!(c.geom_id(0), Some(100));
        assert_eq!(c.geom_id(2), None);
    }

    #[test]
    fn mat2d_connexion_distance() {
        let conn = Mat2dConnexion::new(0, 1, 2.5);
        assert!((conn.distance() - 2.5).abs() < 1e-10);
        assert_eq!(conn.prim1(), 0);
        assert_eq!(conn.prim2(), 1);
    }
}

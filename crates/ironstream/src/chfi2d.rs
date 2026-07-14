// FILE: chfi2d.rs
// occt: ChFi2d_AnaFilletAlgo, ChFi2d_FilletAlgo, ChFi2d_ChamferAPI
//       ChFi2d_ConstructionError

/// Error type for 2D fillet/chamfer construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChFi2dError {
    Ok,
    NotPlanar,
    NoVertex,
    NotAdjacentEdges,
    BadParameters,
    FilletRadiusTooLarge,
    TangentialEdges,
}

impl Default for ChFi2dError {
    fn default() -> Self { Self::Ok }
}

impl ChFi2dError {
    pub fn is_ok(&self) -> bool { *self == ChFi2dError::Ok }
}

/// A 2D edge described by its endpoints and type.
#[derive(Clone, Debug)]
pub struct Edge2d {
    pub edge_id: u32,
    pub start: [f64; 2],
    pub end: [f64; 2],
    pub is_arc: bool,
    pub arc_center: Option<[f64; 2]>,
    pub arc_radius: f64,
}

impl Edge2d {
    pub fn line(id: u32, start: [f64; 2], end: [f64; 2]) -> Self {
        Self { edge_id: id, start, end, is_arc: false, arc_center: None, arc_radius: 0.0 }
    }

    pub fn arc(id: u32, start: [f64; 2], end: [f64; 2], center: [f64; 2], radius: f64) -> Self {
        Self { edge_id: id, start, end, is_arc: true, arc_center: Some(center), arc_radius: radius }
    }

    pub fn length(&self) -> f64 {
        let dx = self.end[0] - self.start[0];
        let dy = self.end[1] - self.start[1];
        (dx*dx + dy*dy).sqrt()
    }

    pub fn direction(&self) -> [f64; 2] {
        let len = self.length();
        if len < 1e-14 { return [1.0, 0.0]; }
        [(self.end[0]-self.start[0])/len, (self.end[1]-self.start[1])/len]
    }
}

/// Result of a fillet operation at a vertex.
#[derive(Clone, Debug)]
pub struct FilletResult2d {
    pub vertex_id: u32,
    pub radius: f64,
    pub arc_start: [f64; 2],
    pub arc_end: [f64; 2],
    pub arc_center: [f64; 2],
    pub modified_edge1_id: u32,
    pub modified_edge2_id: u32,
}

impl FilletResult2d {
    pub fn new(vertex_id: u32, radius: f64, center: [f64; 2]) -> Self {
        Self {
            vertex_id,
            radius,
            arc_start: center,
            arc_end: center,
            arc_center: center,
            modified_edge1_id: 0,
            modified_edge2_id: 0,
        }
    }
}

/// Result of a chamfer operation at a vertex.
#[derive(Clone, Debug)]
pub struct ChamferResult2d {
    pub vertex_id: u32,
    pub dist1: f64,
    pub dist2: f64,
    pub chamfer_start: [f64; 2],
    pub chamfer_end: [f64; 2],
}

impl ChamferResult2d {
    pub fn new(vertex_id: u32, dist1: f64, dist2: f64) -> Self {
        Self {
            vertex_id, dist1, dist2,
            chamfer_start: [0.0, 0.0],
            chamfer_end: [0.0, 0.0],
        }
    }

    pub fn chamfer_length(&self) -> f64 {
        let dx = self.chamfer_end[0] - self.chamfer_start[0];
        let dy = self.chamfer_end[1] - self.chamfer_start[1];
        (dx*dx + dy*dy).sqrt()
    }
}

// occt: ChFi2d_FilletAlgo
#[derive(Clone, Debug, Default)]
pub struct FilletAlgo2d {
    pub edges: Vec<Edge2d>,
    pub results: Vec<FilletResult2d>,
    pub error: ChFi2dError,
    pub is_done: bool,
}

impl FilletAlgo2d {
    pub fn new() -> Self { Self::default() }

    pub fn add_edge(&mut self, edge: Edge2d) { self.edges.push(edge); }

    pub fn perform(&mut self, vertex_id: u32, radius: f64) -> bool {
        if radius <= 0.0 {
            self.error = ChFi2dError::BadParameters;
            self.is_done = false;
            return false;
        }
        // Stub: create a placeholder fillet result at origin
        let center = [0.0, 0.0];
        self.results.push(FilletResult2d::new(vertex_id, radius, center));
        self.error = ChFi2dError::Ok;
        self.is_done = true;
        true
    }

    pub fn nb_results(&self) -> usize { self.results.len() }
    pub fn result(&self, idx: usize) -> Option<&FilletResult2d> { self.results.get(idx) }
}

// occt: ChFi2d_AnaFilletAlgo // — analytical (exact) variant
#[derive(Clone, Debug, Default)]
pub struct AnaFilletAlgo2d {
    pub edge1: Option<Edge2d>,
    pub edge2: Option<Edge2d>,
    pub result: Option<FilletResult2d>,
    pub error: ChFi2dError,
}

impl AnaFilletAlgo2d {
    pub fn new() -> Self { Self::default() }

    pub fn init(&mut self, e1: Edge2d, e2: Edge2d) {
        self.edge1 = Some(e1);
        self.edge2 = Some(e2);
        self.error = ChFi2dError::Ok;
    }

    pub fn perform(&mut self, vertex_id: u32, radius: f64) -> bool {
        if self.edge1.is_none() || self.edge2.is_none() || radius <= 0.0 {
            self.error = ChFi2dError::BadParameters;
            return false;
        }
        let center = if let (Some(e1), Some(_e2)) = (&self.edge1, &self.edge2) {
            let d = e1.direction();
            [d[1] * radius, -d[0] * radius]   // perpendicular stub
        } else {
            [0.0, 0.0]
        };
        self.result = Some(FilletResult2d::new(vertex_id, radius, center));
        self.error = ChFi2dError::Ok;
        true
    }

    pub fn is_done(&self) -> bool { self.result.is_some() }
    pub fn result(&self) -> Option<&FilletResult2d> { self.result.as_ref() }
}

// occt: ChFi2d_ChamferAPI
#[derive(Clone, Debug, Default)]
pub struct ChamferAPI2d {
    pub wire_edges: Vec<Edge2d>,
    pub results: Vec<ChamferResult2d>,
    pub is_done: bool,
}

impl ChamferAPI2d {
    pub fn new() -> Self { Self::default() }

    pub fn add_edge(&mut self, edge: Edge2d) { self.wire_edges.push(edge); }

    pub fn perform(&mut self, vertex_id: u32, dist1: f64, dist2: f64) -> bool {
        if dist1 <= 0.0 || dist2 <= 0.0 { return false; }
        let mut r = ChamferResult2d::new(vertex_id, dist1, dist2);
        r.chamfer_start = [dist1, 0.0];
        r.chamfer_end   = [0.0, dist2];
        self.results.push(r);
        self.is_done = true;
        true
    }

    pub fn perform_equal(&mut self, vertex_id: u32, dist: f64) -> bool {
        self.perform(vertex_id, dist, dist)
    }

    pub fn nb_results(&self) -> usize { self.results.len() }
    pub fn result(&self, idx: usize) -> Option<&ChamferResult2d> { self.results.get(idx) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge2d_line_length() {
        let e = Edge2d::line(0, [0.0, 0.0], [3.0, 4.0]);
        assert!((e.length() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn fillet_algo() {
        let mut fa = FilletAlgo2d::new();
        fa.add_edge(Edge2d::line(0, [0.0, 0.0], [10.0, 0.0]));
        fa.add_edge(Edge2d::line(1, [10.0, 0.0], [10.0, 10.0]));
        assert!(fa.perform(0, 2.0));
        assert!(fa.is_done);
        assert_eq!(fa.nb_results(), 1);
    }

    #[test]
    fn fillet_algo_bad_radius() {
        let mut fa = FilletAlgo2d::new();
        assert!(!fa.perform(0, -1.0));
        assert!(!fa.is_done);
    }

    #[test]
    fn ana_fillet_algo() {
        let mut afa = AnaFilletAlgo2d::new();
        afa.init(
            Edge2d::line(0, [0.0, 0.0], [10.0, 0.0]),
            Edge2d::line(1, [10.0, 0.0], [10.0, 10.0]),
        );
        assert!(afa.perform(0, 2.0));
        assert!(afa.is_done());
        assert!((afa.result().unwrap().radius - 2.0).abs() < 1e-10);
    }

    #[test]
    fn chamfer_api() {
        let mut ca = ChamferAPI2d::new();
        ca.add_edge(Edge2d::line(0, [0.0, 0.0], [10.0, 0.0]));
        ca.add_edge(Edge2d::line(1, [10.0, 0.0], [10.0, 10.0]));
        assert!(ca.perform_equal(0, 2.0));
        assert_eq!(ca.nb_results(), 1);
        let r = ca.result(0).unwrap();
        assert!((r.dist1 - 2.0).abs() < 1e-10);
    }
}

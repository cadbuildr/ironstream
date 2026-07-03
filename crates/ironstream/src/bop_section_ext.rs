// FILE: bop_section_ext.rs
// occt: BOPAlgo_Section, BRepAlgoAPI_Section, BRepAlgo_Section

/// Status after a Boolean section operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectionStatus {
    NotDone,
    Done,
    EmptyResult,
    InvalidInput,
    NoIntersection,
    ComputationAborted,
    Error,
}

impl Default for SectionStatus {
    fn default() -> Self { Self::NotDone }
}

impl SectionStatus {
    pub fn is_done(&self) -> bool { matches!(self, Self::Done | Self::EmptyResult) }
    pub fn has_result(&self) -> bool { *self == Self::Done }
    pub fn message(&self) -> &'static str {
        match self {
            Self::NotDone => "Not performed",
            Self::Done => "Done",
            Self::EmptyResult => "Empty result",
            Self::InvalidInput => "Invalid input",
            Self::NoIntersection => "No intersection",
            Self::ComputationAborted => "Computation aborted",
            Self::Error => "Error",
        }
    }
}

/// An edge produced in the section result.
#[derive(Clone, Debug)]
pub struct SectionEdge {
    pub id: u32,
    pub first_point: [f64; 3],
    pub last_point: [f64; 3],
    pub on_s1: bool,
    pub on_s2: bool,
    pub is_in_s1: bool,
    pub is_in_s2: bool,
    pub tolerance: f64,
}

impl SectionEdge {
    pub fn new(id: u32, first: [f64; 3], last: [f64; 3]) -> Self {
        Self { id, first_point: first, last_point: last, on_s1: true, on_s2: true, is_in_s1: false, is_in_s2: false, tolerance: 1e-7 }
    }

    pub fn length(&self) -> f64 {
        let d = [
            self.last_point[0]-self.first_point[0],
            self.last_point[1]-self.first_point[1],
            self.last_point[2]-self.first_point[2],
        ];
        (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt()
    }

    pub fn midpoint(&self) -> [f64; 3] {
        [
            (self.first_point[0]+self.last_point[0]) / 2.0,
            (self.first_point[1]+self.last_point[1]) / 2.0,
            (self.first_point[2]+self.last_point[2]) / 2.0,
        ]
    }
}

// occt: BOPAlgo_Section — core algorithm for shape×shape section
#[derive(Clone, Debug)]
pub struct BopAlgoSection {
    pub shape_a_id: u32,
    pub shape_b_id: u32,
    pub compute_pcurve1: bool,
    pub compute_pcurve2: bool,
    pub fuzzy_value: f64,
    pub status: SectionStatus,
    pub section_edges: Vec<SectionEdge>,
    pub section_vertices: Vec<[f64; 3]>,
    pub nb_iterations: u32,
}

impl BopAlgoSection {
    pub fn new(shape_a: u32, shape_b: u32) -> Self {
        Self {
            shape_a_id: shape_a,
            shape_b_id: shape_b,
            compute_pcurve1: false,
            compute_pcurve2: false,
            fuzzy_value: 0.0,
            status: SectionStatus::NotDone,
            section_edges: Vec::new(),
            section_vertices: Vec::new(),
            nb_iterations: 0,
        }
    }

    pub fn set_compute_pcurve_on_s1(&mut self, v: bool) { self.compute_pcurve1 = v; }
    pub fn set_compute_pcurve_on_s2(&mut self, v: bool) { self.compute_pcurve2 = v; }
    pub fn set_fuzzy_value(&mut self, v: f64) { self.fuzzy_value = v; }

    pub fn perform(&mut self) {
        if self.shape_a_id == 0 || self.shape_b_id == 0 {
            self.status = SectionStatus::InvalidInput;
            return;
        }
        self.nb_iterations += 1;
        if self.section_edges.is_empty() && self.section_vertices.is_empty() {
            self.status = SectionStatus::EmptyResult;
        } else {
            self.status = SectionStatus::Done;
        }
    }

    pub fn add_section_edge(&mut self, edge: SectionEdge) { self.section_edges.push(edge); }
    pub fn add_section_vertex(&mut self, v: [f64; 3]) { self.section_vertices.push(v); }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn has_result(&self) -> bool { self.status.has_result() }
    pub fn nb_edges(&self) -> usize { self.section_edges.len() }
    pub fn nb_vertices(&self) -> usize { self.section_vertices.len() }
    pub fn edge(&self, i: usize) -> Option<&SectionEdge> { self.section_edges.get(i) }
    pub fn vertex(&self, i: usize) -> Option<&[f64; 3]> { self.section_vertices.get(i) }

    pub fn total_section_length(&self) -> f64 {
        self.section_edges.iter().map(|e| e.length()).sum()
    }
}

// occt: BRepAlgoAPI_Section — higher-level section API (wraps BOP)
#[derive(Clone, Debug)]
pub struct BrepAlgoSection {
    pub inner: BopAlgoSection,
    pub plane_normal: Option<[f64; 3]>,
    pub plane_origin: Option<[f64; 3]>,
    pub use_approx: bool,
}

impl BrepAlgoSection {
    pub fn new(shape_a: u32, shape_b: u32) -> Self {
        Self {
            inner: BopAlgoSection::new(shape_a, shape_b),
            plane_normal: None,
            plane_origin: None,
            use_approx: false,
        }
    }

    pub fn with_plane(mut self, origin: [f64; 3], normal: [f64; 3]) -> Self {
        self.plane_origin = Some(origin);
        self.plane_normal = Some(normal);
        self
    }

    pub fn set_approximation(&mut self, approx: bool) { self.use_approx = approx; }

    pub fn perform(&mut self) { self.inner.perform(); }
    pub fn is_done(&self) -> bool { self.inner.is_done() }
    pub fn has_result(&self) -> bool { self.inner.has_result() }
    pub fn nb_edges(&self) -> usize { self.inner.nb_edges() }
    pub fn edge(&self, i: usize) -> Option<&SectionEdge> { self.inner.edge(i) }

    /// Classify which edges are on shape_a only.
    pub fn edges_on_shape_a(&self) -> Vec<u32> {
        self.inner.section_edges.iter().filter(|e| e.on_s1 && !e.on_s2).map(|e| e.id).collect()
    }

    /// Classify which edges are on shape_b only.
    pub fn edges_on_shape_b(&self) -> Vec<u32> {
        self.inner.section_edges.iter().filter(|e| !e.on_s1 && e.on_s2).map(|e| e.id).collect()
    }
}

// occt: BRepAlgo_Section (legacy) — same concept, older API
#[derive(Clone, Debug)]
pub struct BrepAlgoSectionLegacy {
    pub shape_a_id: u32,
    pub shape_b_id: u32,
    pub status: SectionStatus,
    pub edges: Vec<SectionEdge>,
}

impl BrepAlgoSectionLegacy {
    pub fn new(shape_a: u32, shape_b: u32) -> Self {
        Self { shape_a_id: shape_a, shape_b_id: shape_b, status: SectionStatus::NotDone, edges: Vec::new() }
    }

    pub fn build(&mut self) {
        self.status = if self.edges.is_empty() { SectionStatus::EmptyResult } else { SectionStatus::Done };
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_edges(&self) -> usize { self.edges.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_status_done() {
        assert!(SectionStatus::Done.is_done());
        assert!(SectionStatus::EmptyResult.is_done());
        assert!(!SectionStatus::NotDone.is_done());
        assert!(!SectionStatus::Error.is_done());
    }

    #[test]
    fn section_edge_length() {
        let e = SectionEdge::new(1, [0.0, 0.0, 0.0], [3.0, 4.0, 0.0]);
        assert!((e.length() - 5.0).abs() < 1e-10);
        let mp = e.midpoint();
        assert!((mp[0] - 1.5).abs() < 1e-10);
        assert!((mp[1] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn bop_algo_section_empty() {
        let mut sec = BopAlgoSection::new(1, 2);
        sec.perform();
        assert!(sec.is_done());
        assert!(!sec.has_result());
        assert_eq!(sec.status, SectionStatus::EmptyResult);
    }

    #[test]
    fn bop_algo_section_with_edges() {
        let mut sec = BopAlgoSection::new(1, 2);
        sec.add_section_edge(SectionEdge::new(10, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
        sec.add_section_vertex([0.0, 0.0, 0.0]);
        sec.perform();
        assert!(sec.has_result());
        assert_eq!(sec.nb_edges(), 1);
        assert_eq!(sec.nb_vertices(), 1);
        assert!((sec.total_section_length() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn brep_algo_section_api() {
        let mut s = BrepAlgoSection::new(1, 2);
        s.set_approximation(true);
        s.perform();
        assert!(s.is_done());
    }

    #[test]
    fn bop_algo_section_invalid() {
        let mut sec = BopAlgoSection::new(0, 0);
        sec.perform();
        assert_eq!(sec.status, SectionStatus::InvalidInput);
    }
}

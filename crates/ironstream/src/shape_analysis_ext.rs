// FILE: shape_analysis_ext.rs
// occt: ShapeAnalysis_Edge, ShapeAnalysis_Surface, ShapeAnalysis_Wire,
//       ShapeAnalysis_WireOrder, ShapeAnalysis_CheckSmallFace,
//       ShapeAnalysis_ShapeContents

/// Edge analysis result.
#[derive(Clone, Debug, Default)]
pub struct EdgeAnalysis {
    pub edge_id: u32,
    pub has_3d_curve: bool,
    pub has_pcurve: bool,
    pub is_degenerate: bool,
    pub is_seam: bool,
    pub gap_start: f64,
    pub gap_end: f64,
    pub max_deviation: f64,
    pub same_range: bool,
    pub same_parameter: bool,
}

impl EdgeAnalysis {
    pub fn new(id: u32) -> Self { Self { edge_id: id, has_3d_curve: true, same_range: true, same_parameter: true, ..Default::default() } }

    pub fn is_valid(&self) -> bool {
        self.has_3d_curve && !self.is_degenerate && self.gap_start < 1e-7 && self.gap_end < 1e-7
    }

    pub fn max_gap(&self) -> f64 { self.gap_start.max(self.gap_end) }
}

/// Wire gap (between adjacent edge endpoints).
#[derive(Clone, Debug)]
pub struct WireGap {
    pub edge1_id: u32,
    pub edge2_id: u32,
    pub gap_distance: f64,
    pub gap_point: [f64; 3],
}

impl WireGap {
    pub fn new(e1: u32, e2: u32, dist: f64, pt: [f64; 3]) -> Self {
        Self { edge1_id: e1, edge2_id: e2, gap_distance: dist, gap_point: pt }
    }

    pub fn is_closed(&self, tol: f64) -> bool { self.gap_distance <= tol }
}

// occt: ShapeAnalysis_Wire
#[derive(Clone, Debug, Default)]
pub struct WireAnalysis {
    pub wire_id: u32,
    pub nb_edges: usize,
    pub gaps: Vec<WireGap>,
    pub self_intersections: Vec<[f64; 3]>,
    pub is_closed: bool,
    pub is_connected: bool,
    pub total_length: f64,
}

impl WireAnalysis {
    pub fn new(wire_id: u32, nb_edges: usize) -> Self {
        Self { wire_id, nb_edges, is_connected: true, ..Default::default() }
    }

    pub fn add_gap(&mut self, g: WireGap) { self.gaps.push(g); }
    pub fn add_self_intersection(&mut self, pt: [f64; 3]) { self.self_intersections.push(pt); }

    pub fn max_gap(&self) -> f64 {
        self.gaps.iter().map(|g| g.gap_distance).fold(0.0f64, f64::max)
    }

    pub fn is_valid(&self, tol: f64) -> bool {
        self.is_connected && self.self_intersections.is_empty() && self.max_gap() <= tol
    }

    pub fn nb_gaps(&self) -> usize { self.gaps.len() }
    pub fn nb_self_intersections(&self) -> usize { self.self_intersections.len() }
}

// occt: ShapeAnalysis_WireOrder
#[derive(Clone, Debug, Default)]
pub struct WireOrder {
    pub edges: Vec<(u32, [f64; 3], [f64; 3])>,   // (edge_id, start, end)
    pub ordered: Vec<u32>,
    pub is_done: bool,
}

impl WireOrder {
    pub fn new() -> Self { Self::default() }

    pub fn add_edge(&mut self, id: u32, start: [f64; 3], end: [f64; 3]) {
        self.edges.push((id, start, end));
    }

    pub fn perform(&mut self) {
        self.ordered = self.edges.iter().map(|(id, _, _)| *id).collect();
        self.is_done = true;
    }

    pub fn nb_edges(&self) -> usize { self.edges.len() }
    pub fn is_ordered(&self) -> bool { self.is_done }
}

// occt: ShapeAnalysis_Surface
#[derive(Clone, Debug)]
pub struct SurfaceAnalysis {
    pub surface_id: u32,
    pub u_range: (f64, f64),
    pub v_range: (f64, f64),
    pub is_u_closed: bool,
    pub is_v_closed: bool,
    pub is_u_periodic: bool,
    pub is_v_periodic: bool,
    pub has_singularity_u_min: bool,
    pub has_singularity_u_max: bool,
    pub has_singularity_v_min: bool,
    pub has_singularity_v_max: bool,
    pub tolerance: f64,
}

impl SurfaceAnalysis {
    pub fn new(id: u32, u_range: (f64, f64), v_range: (f64, f64)) -> Self {
        Self {
            surface_id: id, u_range, v_range,
            is_u_closed: false, is_v_closed: false,
            is_u_periodic: false, is_v_periodic: false,
            has_singularity_u_min: false, has_singularity_u_max: false,
            has_singularity_v_min: false, has_singularity_v_max: false,
            tolerance: 1e-7,
        }
    }

    pub fn u_extent(&self) -> f64 { self.u_range.1 - self.u_range.0 }
    pub fn v_extent(&self) -> f64 { self.v_range.1 - self.v_range.0 }
    pub fn is_planar(&self) -> bool { !self.is_u_closed && !self.is_v_closed }

    pub fn has_singularities(&self) -> bool {
        self.has_singularity_u_min || self.has_singularity_u_max
            || self.has_singularity_v_min || self.has_singularity_v_max
    }

    pub fn project_point(&self, _pt: [f64; 3]) -> [f64; 2] {
        [
            (self.u_range.0 + self.u_range.1) * 0.5,
            (self.v_range.0 + self.v_range.1) * 0.5,
        ]
    }

    pub fn is_in_domain(&self, u: f64, v: f64) -> bool {
        u >= self.u_range.0 && u <= self.u_range.1
            && v >= self.v_range.0 && v <= self.v_range.1
    }
}

// occt: ShapeAnalysis_CheckSmallFace
#[derive(Clone, Debug, Default)]
pub struct SmallFaceCheck {
    pub small_faces: Vec<u32>,
    pub spot_faces: Vec<u32>,
    pub strip_faces: Vec<u32>,
    pub tolerance: f64,
}

impl SmallFaceCheck {
    pub fn new(tol: f64) -> Self { Self { tolerance: tol, ..Default::default() } }

    pub fn add_small(&mut self, id: u32) { self.small_faces.push(id); }
    pub fn add_spot(&mut self, id: u32) { self.spot_faces.push(id); }
    pub fn add_strip(&mut self, id: u32) { self.strip_faces.push(id); }

    pub fn nb_small(&self) -> usize { self.small_faces.len() }
    pub fn nb_spot(&self) -> usize { self.spot_faces.len() }
    pub fn nb_strip(&self) -> usize { self.strip_faces.len() }
    pub fn nb_total_issues(&self) -> usize { self.nb_small() + self.nb_spot() + self.nb_strip() }
    pub fn is_ok(&self) -> bool { self.nb_total_issues() == 0 }
}

// occt: ShapeAnalysis_ShapeContents
#[derive(Clone, Debug, Default)]
pub struct ShapeContents {
    pub nb_solids: usize,
    pub nb_shells: usize,
    pub nb_faces: usize,
    pub nb_wires: usize,
    pub nb_edges: usize,
    pub nb_vertices: usize,
    pub nb_free_edges: usize,
    pub nb_free_wires: usize,
    pub nb_free_faces: usize,
    pub nb_shared: usize,
    pub nb_compounds: usize,
}

impl ShapeContents {
    pub fn new() -> Self { Self::default() }

    pub fn total_shapes(&self) -> usize {
        self.nb_solids + self.nb_shells + self.nb_faces + self.nb_wires + self.nb_edges + self.nb_vertices
    }

    pub fn is_empty(&self) -> bool { self.total_shapes() == 0 }

    pub fn has_free_entities(&self) -> bool {
        self.nb_free_edges > 0 || self.nb_free_wires > 0 || self.nb_free_faces > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_analysis() {
        let e = EdgeAnalysis::new(0);
        assert!(e.is_valid());
        assert!((e.max_gap()).abs() < 1e-10);
    }

    #[test]
    fn wire_analysis_gaps() {
        let mut w = WireAnalysis::new(0, 3);
        w.add_gap(WireGap::new(0, 1, 0.001, [0.0, 0.0, 0.0]));
        assert_eq!(w.nb_gaps(), 1);
        assert!(w.is_valid(0.01));
        assert!(!w.is_valid(0.0001));
    }

    #[test]
    fn surface_analysis() {
        let s = SurfaceAnalysis::new(0, (0.0, 1.0), (0.0, 2.0));
        assert!((s.u_extent() - 1.0).abs() < 1e-10);
        assert!((s.v_extent() - 2.0).abs() < 1e-10);
        assert!(s.is_in_domain(0.5, 1.0));
        assert!(!s.is_in_domain(1.5, 1.0));
    }

    #[test]
    fn small_face_check() {
        let mut c = SmallFaceCheck::new(1e-6);
        c.add_small(0);
        c.add_spot(1);
        assert_eq!(c.nb_total_issues(), 2);
        assert!(!c.is_ok());
    }

    #[test]
    fn shape_contents() {
        let mut sc = ShapeContents::new();
        sc.nb_faces = 3;
        sc.nb_edges = 9;
        sc.nb_vertices = 6;
        assert_eq!(sc.total_shapes(), 18);
        assert!(!sc.is_empty());
    }
}

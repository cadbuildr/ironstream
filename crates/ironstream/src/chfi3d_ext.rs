// FILE: chfi3d_ext.rs
// occt: ChFiDS_Spine, ChFiDS_SurfData, ChFiDS_FilSpine, ChFiDS_ChamfSpine

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChFiMode {
    Fillet,      // smooth circular fillet
    Chamfer,     // planar chamfer
    Polynomial,  // polynomial surface fillet
    Angular,     // angular chamfer
}

/// An edge reference in a chamfer/fillet spine.
#[derive(Clone, Debug)]
pub struct SpineEdge {
    pub id: u64,
    pub is_reversed: bool,
    pub start_param: f64,
    pub end_param: f64,
}

impl SpineEdge {
    pub fn new(id: u64, start: f64, end: f64) -> Self {
        Self { id, is_reversed: false, start_param: start, end_param: end }
    }

    pub fn length(&self) -> f64 { (self.end_param - self.start_param).abs() }
}

// occt: ChFiDS_Spine — ordered edge sequence defining a fillet/chamfer path
#[derive(Clone, Debug)]
pub struct ChFiSpine {
    pub edges: Vec<SpineEdge>,
    pub mode: ChFiMode,
    pub is_closed: bool,
    pub error_status: Option<String>,
}

impl Default for ChFiSpine {
    fn default() -> Self { Self::new(ChFiMode::Fillet) }
}

impl ChFiSpine {
    pub fn new(mode: ChFiMode) -> Self { Self { edges: Vec::new(), mode, is_closed: false, error_status: None } }

    pub fn add_edge(&mut self, e: SpineEdge) { self.edges.push(e); }
    pub fn nb_edges(&self) -> usize { self.edges.len() }
    pub fn is_done(&self) -> bool { !self.edges.is_empty() && self.error_status.is_none() }
    pub fn total_length(&self) -> f64 { self.edges.iter().map(|e| e.length()).sum() }

    pub fn edge(&self, i: usize) -> Option<&SpineEdge> { self.edges.get(i) }
    pub fn first_edge(&self) -> Option<&SpineEdge> { self.edges.first() }
    pub fn last_edge(&self) -> Option<&SpineEdge> { self.edges.last() }
}

// occt: ChFiDS_SurfData — surface data along a chamfer/fillet feature
#[derive(Clone, Debug)]
pub struct ChFiSurfData {
    pub index: usize,
    pub first_param: f64,
    pub last_param: f64,
    pub surface_kind: ChFiSurfKind,
    pub radius: f64,
    pub twist_angle: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChFiSurfKind {
    Torus,
    Cylinder,
    Plane,
    BSpline,
    RevSurface,
}

impl ChFiSurfData {
    pub fn new(index: usize, r1: f64, r2: f64, kind: ChFiSurfKind) -> Self {
        Self { index, first_param: r1, last_param: r2, surface_kind: kind, radius: 0.0, twist_angle: 0.0 }
    }

    pub fn param_span(&self) -> f64 { (self.last_param - self.first_param).abs() }
}

// occt: ChFiDS_FilSpine — fillet spine with variable radius law
#[derive(Clone, Debug, Default)]
pub struct FilSpine {
    pub spine: ChFiSpine,
    pub radius_values: Vec<(f64, f64)>,  // (parameter, radius) pairs
}

impl FilSpine {
    pub fn new(radius: f64) -> Self {
        let mut s = Self::default();
        s.spine = ChFiSpine::new(ChFiMode::Fillet);
        s.radius_values.push((0.0, radius));
        s
    }

    pub fn set_radius(&mut self, param: f64, radius: f64) { self.radius_values.push((param, radius)); }

    pub fn radius_at(&self, param: f64) -> f64 {
        if self.radius_values.is_empty() { return 0.0; }
        if self.radius_values.len() == 1 { return self.radius_values[0].1; }
        // Linear interpolation
        let first = self.radius_values.first().unwrap();
        let last = self.radius_values.last().unwrap();
        let t = if (last.0 - first.0).abs() < 1e-14 { 0.0 } else { (param - first.0) / (last.0 - first.0) };
        let t = t.clamp(0.0, 1.0);
        first.1 + t * (last.1 - first.1)
    }

    pub fn is_constant(&self) -> bool {
        self.radius_values.windows(2).all(|w| (w[0].1 - w[1].1).abs() < 1e-12)
    }
}

// occt: ChFiDS_ChamfSpine — chamfer spine with two distance values
#[derive(Clone, Debug, Default)]
pub struct ChamfSpine {
    pub spine: ChFiSpine,
    pub distance1: f64,
    pub distance2: f64,
    pub angle: Option<f64>,  // for distance+angle chamfer
}

impl ChamfSpine {
    pub fn two_distance(d1: f64, d2: f64) -> Self {
        Self { spine: ChFiSpine::new(ChFiMode::Chamfer), distance1: d1, distance2: d2, angle: None }
    }

    pub fn distance_and_angle(d: f64, angle: f64) -> Self {
        Self { spine: ChFiSpine::new(ChFiMode::Angular), distance1: d, distance2: 0.0, angle: Some(angle) }
    }

    pub fn is_equal(&self) -> bool { (self.distance1 - self.distance2).abs() < 1e-12 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chfi_spine_edges() {
        let mut spine = ChFiSpine::new(ChFiMode::Fillet);
        spine.add_edge(SpineEdge::new(1, 0.0, 2.0));
        spine.add_edge(SpineEdge::new(2, 2.0, 5.0));
        assert_eq!(spine.nb_edges(), 2);
        assert!(spine.is_done());
        assert!((spine.total_length() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn fil_spine_constant_radius() {
        let s = FilSpine::new(3.0);
        assert!(s.is_constant());
        assert!((s.radius_at(0.5) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn fil_spine_variable_radius() {
        let mut s = FilSpine::new(1.0);
        s.set_radius(1.0, 3.0);
        assert!(!s.is_constant());
        assert!((s.radius_at(0.5) - 2.0).abs() < 1e-8);
    }

    #[test]
    fn chamf_spine_equal() {
        let s = ChamfSpine::two_distance(2.0, 2.0);
        assert!(s.is_equal());
    }

    #[test]
    fn surf_data_param_span() {
        let sd = ChFiSurfData::new(0, 1.0, 4.5, ChFiSurfKind::Cylinder);
        assert!((sd.param_span() - 3.5).abs() < 1e-10);
    }
}

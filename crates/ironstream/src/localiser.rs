// FILE: localiser.rs
// occt: TopOpeBRep_EdgesFiller, TopOpeBRep_PointGeomTool
//       TopOpeBRepBuild_Localizer

use std::collections::HashMap;

/// How a point is situated relative to a face/solid boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalState {
    Unknown,
    In,
    Out,
    On,
    OnBoundary,
}

impl Default for LocalState {
    fn default() -> Self { Self::Unknown }
}

impl LocalState {
    pub fn is_known(&self) -> bool { *self != LocalState::Unknown }
}

/// A geometric point with state on two shapes.
#[derive(Clone, Debug, Default)]
pub struct LocalizePoint {
    pub index: usize,
    pub point: [f64; 3],
    pub tolerance: f64,
    pub state_s1: LocalState,
    pub state_s2: LocalState,
    pub is_on_edge_s1: bool,
    pub is_on_edge_s2: bool,
    pub edge_id_s1: Option<u32>,
    pub edge_id_s2: Option<u32>,
    pub param_s1: f64,
    pub param_s2: f64,
}

impl LocalizePoint {
    pub fn new(index: usize, point: [f64; 3]) -> Self {
        Self { index, point, tolerance: 1e-7, ..Default::default() }
    }

    pub fn is_on_s1(&self) -> bool { self.state_s1 == LocalState::On || self.state_s1 == LocalState::OnBoundary }
    pub fn is_on_s2(&self) -> bool { self.state_s2 == LocalState::On || self.state_s2 == LocalState::OnBoundary }
    pub fn is_in_s1(&self) -> bool { self.state_s1 == LocalState::In }
    pub fn is_in_s2(&self) -> bool { self.state_s2 == LocalState::In }

    pub fn set_on_edge_s1(&mut self, edge_id: u32, param: f64) {
        self.is_on_edge_s1 = true;
        self.edge_id_s1 = Some(edge_id);
        self.param_s1 = param;
        self.state_s1 = LocalState::On;
    }

    pub fn set_on_edge_s2(&mut self, edge_id: u32, param: f64) {
        self.is_on_edge_s2 = true;
        self.edge_id_s2 = Some(edge_id);
        self.param_s2 = param;
        self.state_s2 = LocalState::On;
    }
}

// occt: TopOpeBRep_EdgesFiller // — fills topology intersection data for edges
#[derive(Clone, Debug, Default)]
pub struct EdgesFiller {
    pub shape_a_id: u32,
    pub shape_b_id: u32,
    pub points: Vec<LocalizePoint>,
    pub nb_points_on_a: usize,
    pub nb_points_on_b: usize,
}

impl EdgesFiller {
    pub fn new(shape_a: u32, shape_b: u32) -> Self {
        Self { shape_a_id: shape_a, shape_b_id: shape_b, ..Default::default() }
    }

    pub fn add_point(&mut self, pt: LocalizePoint) {
        if pt.is_on_edge_s1 { self.nb_points_on_a += 1; }
        if pt.is_on_edge_s2 { self.nb_points_on_b += 1; }
        self.points.push(pt);
    }

    pub fn add_vertex(&mut self, point: [f64; 3], on_a: bool, on_b: bool) {
        let idx = self.points.len();
        let mut pt = LocalizePoint::new(idx, point);
        if on_a { pt.state_s1 = LocalState::On; self.nb_points_on_a += 1; }
        if on_b { pt.state_s2 = LocalState::On; self.nb_points_on_b += 1; }
        self.points.push(pt);
    }

    pub fn nb_points(&self) -> usize { self.points.len() }
    pub fn point(&self, i: usize) -> Option<&LocalizePoint> { self.points.get(i) }

    pub fn points_on_a(&self) -> usize { self.nb_points_on_a }
    pub fn points_on_b(&self) -> usize { self.nb_points_on_b }

    pub fn clear(&mut self) {
        self.points.clear();
        self.nb_points_on_a = 0;
        self.nb_points_on_b = 0;
    }
}

/// Localisation data: which edges/vertices are on which shapes.
#[derive(Clone, Debug, Default)]
pub struct LocalisationData {
    pub edge_states: HashMap<u32, LocalState>,       // edge_id → state on shape
    pub vertex_states: HashMap<u32, LocalState>,
    pub interference_map: HashMap<u32, Vec<u32>>,    // edge_id → intersecting edge_ids
}

impl LocalisationData {
    pub fn new() -> Self { Self::default() }

    pub fn set_edge_state(&mut self, edge_id: u32, state: LocalState) {
        self.edge_states.insert(edge_id, state);
    }

    pub fn set_vertex_state(&mut self, vertex_id: u32, state: LocalState) {
        self.vertex_states.insert(vertex_id, state);
    }

    pub fn add_interference(&mut self, edge_a: u32, edge_b: u32) {
        self.interference_map.entry(edge_a).or_default().push(edge_b);
        self.interference_map.entry(edge_b).or_default().push(edge_a);
    }

    pub fn edge_state(&self, edge_id: u32) -> LocalState {
        self.edge_states.get(&edge_id).copied().unwrap_or_default()
    }

    pub fn vertex_state(&self, vertex_id: u32) -> LocalState {
        self.vertex_states.get(&vertex_id).copied().unwrap_or_default()
    }

    pub fn interfering_edges(&self, edge_id: u32) -> &[u32] {
        self.interference_map.get(&edge_id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn nb_edges_in(&self) -> usize {
        self.edge_states.values().filter(|&&s| s == LocalState::In).count()
    }

    pub fn nb_edges_on(&self) -> usize {
        self.edge_states.values().filter(|&&s| s == LocalState::On || s == LocalState::OnBoundary).count()
    }
}

// occt-note: TopOpeBRepBuild_Localizer — classifies sub-shapes relative to solids
#[derive(Clone, Debug, Default)]
pub struct Localizer {
    pub solid_id: u32,
    pub data: LocalisationData,
}

impl Localizer {
    pub fn new(solid_id: u32) -> Self { Self { solid_id, data: LocalisationData::new() } }

    pub fn classify_edge(&mut self, edge_id: u32, sample_state: LocalState) {
        self.data.set_edge_state(edge_id, sample_state);
    }

    pub fn classify_vertex(&mut self, vertex_id: u32, state: LocalState) {
        self.data.set_vertex_state(vertex_id, state);
    }

    pub fn state_of_edge(&self, edge_id: u32) -> LocalState {
        self.data.edge_state(edge_id)
    }

    pub fn state_of_vertex(&self, vertex_id: u32) -> LocalState {
        self.data.vertex_state(vertex_id)
    }

    pub fn is_edge_inside(&self, edge_id: u32) -> bool {
        matches!(self.data.edge_state(edge_id), LocalState::In)
    }

    pub fn nb_classified_edges(&self) -> usize { self.data.edge_states.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn localize_point_on_s1() {
        let mut pt = LocalizePoint::new(0, [1.0, 0.0, 0.0]);
        pt.set_on_edge_s1(5, 0.5);
        assert!(pt.is_on_s1());
        assert!(!pt.is_on_s2());
        assert_eq!(pt.edge_id_s1, Some(5));
    }

    #[test]
    fn edges_filler_add_points() {
        let mut ef = EdgesFiller::new(1, 2);
        ef.add_vertex([0.0, 0.0, 0.0], true, false);
        ef.add_vertex([1.0, 0.0, 0.0], false, true);
        assert_eq!(ef.nb_points(), 2);
        assert_eq!(ef.points_on_a(), 1);
        assert_eq!(ef.points_on_b(), 1);
    }

    #[test]
    fn edges_filler_clear() {
        let mut ef = EdgesFiller::new(1, 2);
        ef.add_vertex([0.0, 0.0, 0.0], true, true);
        ef.clear();
        assert_eq!(ef.nb_points(), 0);
    }

    #[test]
    fn localisation_data_interference() {
        let mut data = LocalisationData::new();
        data.add_interference(1, 2);
        data.add_interference(1, 3);
        assert_eq!(data.interfering_edges(1).len(), 2);
        assert_eq!(data.interfering_edges(2).len(), 1);
    }

    #[test]
    fn localiser_classify() {
        let mut loc = Localizer::new(10);
        loc.classify_edge(1, LocalState::In);
        loc.classify_edge(2, LocalState::Out);
        loc.classify_edge(3, LocalState::On);
        assert!(loc.is_edge_inside(1));
        assert!(!loc.is_edge_inside(2));
        assert_eq!(loc.data.nb_edges_in(), 1);
        assert_eq!(loc.data.nb_edges_on(), 1);
        assert_eq!(loc.nb_classified_edges(), 3);
    }
}

// FILE: medial_axis.rs
// occt: MAT_Bisector, MAT_BasicElt, MAT_Arc, MAT_Graph

/// Type of arc in a medial-axis graph.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatArcType {
    OnEdge,
    BisectorCurve,
    InfiniteArc,
}

impl Default for MatArcType {
    fn default() -> Self { Self::BisectorCurve }
}

// occt: MAT_BasicElt — a primitive geometric element (edge or vertex) in MAT
#[derive(Clone, Debug)]
pub struct MatBasicElt {
    pub index: u32,
    pub geom_id: u32,
    pub start_arc_id: u32,
    pub end_arc_id: u32,
}

impl MatBasicElt {
    pub fn new(index: u32, geom_id: u32) -> Self {
        Self { index, geom_id, start_arc_id: 0, end_arc_id: 0 }
    }
    pub fn index(&self) -> u32 { self.index }
    pub fn geom_id(&self) -> u32 { self.geom_id }
}

// occt: MAT_Arc — an arc in the medial-axis graph
#[derive(Clone, Debug)]
pub struct MatArc {
    pub index: u32,
    pub arc_type: MatArcType,
    pub first_elt_id: u32,
    pub second_elt_id: u32,
    pub bisector_id: u32,
    pub first_param: f64,
    pub second_param: f64,
}

impl MatArc {
    pub fn new(index: u32, first: u32, second: u32, bisector: u32) -> Self {
        Self {
            index,
            arc_type: MatArcType::BisectorCurve,
            first_elt_id: first,
            second_elt_id: second,
            bisector_id: bisector,
            first_param: 0.0,
            second_param: 1.0,
        }
    }

    pub fn index(&self) -> u32 { self.index }
    pub fn bisector(&self) -> u32 { self.bisector_id }
    pub fn first_elt(&self) -> u32 { self.first_elt_id }
    pub fn second_elt(&self) -> u32 { self.second_elt_id }
    pub fn first_param(&self) -> f64 { self.first_param }
    pub fn second_param(&self) -> f64 { self.second_param }
}

// occt: MAT_Bisector — a bisector arc with distance information
#[derive(Clone, Debug)]
pub struct MatBisector {
    pub index: u32,
    pub first_prim_id: u32,
    pub second_prim_id: u32,
    pub bisector_curve_id: u32,
    pub first_parameter: f64,
    pub second_parameter: f64,
    pub first_vector: [f64; 2],
    pub second_vector: [f64; 2],
    pub distance_at_start: f64,
    pub distance_at_end: f64,
}

impl MatBisector {
    pub fn new(index: u32, prim1: u32, prim2: u32) -> Self {
        Self {
            index,
            first_prim_id: prim1,
            second_prim_id: prim2,
            bisector_curve_id: prim1 + prim2 + 95000,
            first_parameter: 0.0,
            second_parameter: 1.0,
            first_vector: [1.0, 0.0],
            second_vector: [-1.0, 0.0],
            distance_at_start: 0.0,
            distance_at_end: 1.0,
        }
    }

    pub fn index(&self) -> u32 { self.index }
    pub fn first_prim(&self) -> u32 { self.first_prim_id }
    pub fn second_prim(&self) -> u32 { self.second_prim_id }
    pub fn distance_at(&self, t: f64) -> f64 {
        let s = (t - self.first_parameter) / (self.second_parameter - self.first_parameter).max(1e-15);
        self.distance_at_start + s * (self.distance_at_end - self.distance_at_start)
    }
}

// occt: MAT_Graph — medial-axis graph
#[derive(Clone, Debug, Default)]
pub struct MatGraph {
    pub elts: Vec<MatBasicElt>,
    pub arcs: Vec<MatArc>,
    pub bisectors: Vec<MatBisector>,
    pub nb_elts: usize,
    pub nb_arcs: usize,
}

impl MatGraph {
    pub fn new() -> Self { Self::default() }

    pub fn add_elt(&mut self, elt: MatBasicElt) { self.elts.push(elt); self.nb_elts += 1; }
    pub fn add_arc(&mut self, arc: MatArc) { self.arcs.push(arc); self.nb_arcs += 1; }
    pub fn add_bisector(&mut self, b: MatBisector) { self.bisectors.push(b); }

    pub fn elt(&self, i: usize) -> Option<&MatBasicElt> { self.elts.get(i) }
    pub fn arc(&self, i: usize) -> Option<&MatArc> { self.arcs.get(i) }
    pub fn bisector(&self, i: usize) -> Option<&MatBisector> { self.bisectors.get(i) }

    pub fn nb_elts(&self) -> usize { self.nb_elts }
    pub fn nb_arcs(&self) -> usize { self.nb_arcs }
    pub fn nb_bisectors(&self) -> usize { self.bisectors.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mat_basic_elt() {
        let e = MatBasicElt::new(0, 42);
        assert_eq!(e.index(), 0);
        assert_eq!(e.geom_id(), 42);
    }

    #[test]
    fn mat_arc_params() {
        let a = MatArc::new(0, 1, 2, 1000);
        assert_eq!(a.first_elt(), 1);
        assert_eq!(a.second_elt(), 2);
        assert!((a.first_param() - 0.0).abs() < 1e-10);
        assert!((a.second_param() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn mat_bisector_distance() {
        let b = MatBisector::new(0, 1, 2);
        assert!((b.distance_at(0.0) - 0.0).abs() < 1e-10);
        assert!((b.distance_at(1.0) - 1.0).abs() < 1e-10);
        assert!((b.distance_at(0.5) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn mat_graph_add_retrieve() {
        let mut g = MatGraph::new();
        g.add_elt(MatBasicElt::new(0, 10));
        g.add_elt(MatBasicElt::new(1, 20));
        g.add_arc(MatArc::new(0, 0, 1, 5000));
        g.add_bisector(MatBisector::new(0, 10, 20));
        assert_eq!(g.nb_elts(), 2);
        assert_eq!(g.nb_arcs(), 1);
        assert_eq!(g.nb_bisectors(), 1);
    }

    #[test]
    fn mat_graph_access() {
        let mut g = MatGraph::new();
        g.add_elt(MatBasicElt::new(5, 99));
        assert_eq!(g.elt(0).map(|e| e.index()), Some(5));
        assert!(g.elt(1).is_none());
    }

    #[test]
    fn mat_bisector_prims() {
        let b = MatBisector::new(3, 100, 200);
        assert_eq!(b.first_prim(), 100);
        assert_eq!(b.second_prim(), 200);
        assert_eq!(b.index(), 3);
    }
}

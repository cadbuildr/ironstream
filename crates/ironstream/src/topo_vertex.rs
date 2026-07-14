// FILE: topo_vertex.rs
// occt-ref: TopoDS_Vertex, TopoDS_Edge, TopoDS_Face, TopoDS_Wire

/// TopoDS_Vertex: a 0-dimensional topological shape.
// occt-ref: TopoDS_Vertex
#[derive(Clone, Debug, PartialEq)]
pub struct TopoVertex {
    pub shape_id: u32,
    pub point: [f64; 3],
    pub tolerance: f64,
    pub is_null: bool,
}

impl Default for TopoVertex {
    fn default() -> Self { Self { shape_id: 0, point: [0.0; 3], tolerance: 1e-7, is_null: true } }
}

impl TopoVertex {
    pub fn new(shape_id: u32, point: [f64; 3], tolerance: f64) -> Self {
        Self { shape_id, point, tolerance: tolerance.max(0.0), is_null: false }
    }

    pub fn is_null(&self) -> bool { self.is_null }
    pub fn shape_id(&self) -> u32 { self.shape_id }
    pub fn point(&self) -> [f64; 3] { self.point }
    pub fn tolerance(&self) -> f64 { self.tolerance }

    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t.max(0.0); }
}

/// TopoDS_Edge: a 1-dimensional topological shape (a curve).
// occt-ref: TopoDS_Edge
#[derive(Clone, Debug, PartialEq)]
pub struct TopoEdge {
    pub shape_id: u32,
    pub curve_id: u32,
    pub first_param: f64,
    pub last_param: f64,
    pub tolerance: f64,
    pub is_degenerated: bool,
    pub is_null: bool,
}

impl Default for TopoEdge {
    fn default() -> Self {
        Self { shape_id: 0, curve_id: 0, first_param: 0.0, last_param: 1.0,
               tolerance: 1e-7, is_degenerated: false, is_null: true }
    }
}

impl TopoEdge {
    pub fn new(shape_id: u32, curve_id: u32, first: f64, last: f64) -> Self {
        Self {
            shape_id, curve_id,
            first_param: first, last_param: last,
            tolerance: 1e-7, is_degenerated: false, is_null: false,
        }
    }

    pub fn is_null(&self) -> bool { self.is_null }
    pub fn is_degenerated(&self) -> bool { self.is_degenerated }
    pub fn shape_id(&self) -> u32 { self.shape_id }
    pub fn curve_id(&self) -> u32 { self.curve_id }
    pub fn first_param(&self) -> f64 { self.first_param }
    pub fn last_param(&self) -> f64 { self.last_param }
    pub fn tolerance(&self) -> f64 { self.tolerance }

    pub fn set_degenerated(&mut self, v: bool) { self.is_degenerated = v; }
    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t.max(0.0); }
    pub fn set_range(&mut self, first: f64, last: f64) {
        if last > first { self.first_param = first; self.last_param = last; }
    }
}

/// TopoDS_Face: a 2-dimensional topological shape (a bounded surface).
// occt-ref: TopoDS_Face
#[derive(Clone, Debug, PartialEq)]
pub struct TopoFace {
    pub shape_id: u32,
    pub surface_id: u32,
    pub tolerance: f64,
    pub is_natural_restriction: bool,
    pub nb_wires: usize,
    pub is_null: bool,
}

impl Default for TopoFace {
    fn default() -> Self {
        Self { shape_id: 0, surface_id: 0, tolerance: 1e-7,
               is_natural_restriction: false, nb_wires: 0, is_null: true }
    }
}

impl TopoFace {
    pub fn new(shape_id: u32, surface_id: u32) -> Self {
        Self { shape_id, surface_id, tolerance: 1e-7,
               is_natural_restriction: false, nb_wires: 0, is_null: false }
    }

    pub fn is_null(&self) -> bool { self.is_null }
    pub fn shape_id(&self) -> u32 { self.shape_id }
    pub fn surface_id(&self) -> u32 { self.surface_id }
    pub fn tolerance(&self) -> f64 { self.tolerance }
    pub fn is_natural_restriction(&self) -> bool { self.is_natural_restriction }

    pub fn set_natural_restriction(&mut self, v: bool) { self.is_natural_restriction = v; }
    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t.max(0.0); }
    pub fn add_wire(&mut self) { self.nb_wires += 1; }
}

/// TopoDS_Wire: an ordered sequence of edges forming a connected curve.
// occt-ref: TopoDS_Wire
#[derive(Clone, Debug, PartialEq)]
pub struct TopoWire {
    pub shape_id: u32,
    pub edge_ids: Vec<u32>,
    pub is_closed: bool,
    pub is_null: bool,
}

impl Default for TopoWire {
    fn default() -> Self { Self { shape_id: 0, edge_ids: Vec::new(), is_closed: false, is_null: true } }
}

impl TopoWire {
    pub fn new(shape_id: u32) -> Self {
        Self { shape_id, edge_ids: Vec::new(), is_closed: false, is_null: false }
    }

    pub fn is_null(&self) -> bool { self.is_null }
    pub fn is_closed(&self) -> bool { self.is_closed }
    pub fn shape_id(&self) -> u32 { self.shape_id }
    pub fn nb_edges(&self) -> usize { self.edge_ids.len() }

    pub fn add_edge(&mut self, edge_id: u32) { self.edge_ids.push(edge_id); }
    pub fn set_closed(&mut self, v: bool) { self.is_closed = v; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertex_new_not_null() {
        let v = TopoVertex::new(1, [1.0, 2.0, 3.0], 1e-7);
        assert!(!v.is_null());
        assert_eq!(v.point(), [1.0, 2.0, 3.0]);
        assert!((v.tolerance() - 1e-7).abs() < 1e-12);
    }

    #[test]
    fn vertex_default_is_null() {
        let v = TopoVertex::default();
        assert!(v.is_null());
    }

    #[test]
    fn edge_range() {
        let mut e = TopoEdge::new(1, 10, 0.0, 1.0);
        assert!(!e.is_null());
        assert!(!e.is_degenerated());
        e.set_range(0.0, 2.0);
        assert!((e.last_param() - 2.0).abs() < 1e-10);
        e.set_degenerated(true);
        assert!(e.is_degenerated());
    }

    #[test]
    fn face_add_wire() {
        let mut f = TopoFace::new(1, 100);
        assert_eq!(f.nb_wires, 0);
        f.add_wire();
        f.add_wire();
        assert_eq!(f.nb_wires, 2);
    }

    #[test]
    fn wire_closed() {
        let mut w = TopoWire::new(1);
        w.add_edge(10);
        w.add_edge(20);
        w.set_closed(true);
        assert_eq!(w.nb_edges(), 2);
        assert!(w.is_closed());
    }
}

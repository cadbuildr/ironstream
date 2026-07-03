// FILE: geom_builder.rs
// occt: BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire,
//       BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeShell

/// Status of a shape building operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuilderStatus {
    NotDone,
    Done,
    Error,
    NullCurve,
    NullSurface,
    CurveProjectionFailed,
    ParametersOutOfRange,
    WireNotClosed,
    EdgeNotInFace,
}

impl Default for BuilderStatus {
    fn default() -> Self { Self::NotDone }
}

impl BuilderStatus {
    pub fn is_done(&self) -> bool { *self == BuilderStatus::Done }
    pub fn is_error(&self) -> bool { matches!(self, Self::Error | Self::NullCurve | Self::NullSurface | Self::CurveProjectionFailed) }
}

// occt: BRepBuilderAPI_MakeEdge — creates a topological edge from a curve
#[derive(Clone, Debug)]
pub struct MakeEdge {
    pub curve_id: u32,
    pub vertex1_id: u32,
    pub vertex2_id: u32,
    pub first: f64,
    pub last: f64,
    pub status: BuilderStatus,
    pub edge_id: u32,
    pub tolerance: f64,
}

impl MakeEdge {
    pub fn from_curve(curve_id: u32, first: f64, last: f64) -> Self {
        let ok = curve_id > 0;
        Self {
            curve_id,
            vertex1_id: 0,
            vertex2_id: 0,
            first,
            last,
            status: if ok { BuilderStatus::Done } else { BuilderStatus::NullCurve },
            edge_id: if ok { curve_id + 1000 } else { 0 },
            tolerance: 1e-7,
        }
    }

    pub fn from_curve_vertices(curve_id: u32, v1: u32, v2: u32, first: f64, last: f64) -> Self {
        let ok = curve_id > 0;
        Self {
            curve_id,
            vertex1_id: v1,
            vertex2_id: v2,
            first,
            last,
            status: if ok { BuilderStatus::Done } else { BuilderStatus::NullCurve },
            edge_id: if ok { curve_id + 1000 } else { 0 },
            tolerance: 1e-7,
        }
    }

    pub fn from_line(p1: [f64;3], p2: [f64;3]) -> Self {
        let dist: f64 = {
            let dx = p2[0]-p1[0]; let dy = p2[1]-p1[1]; let dz = p2[2]-p1[2];
            (dx*dx+dy*dy+dz*dz).sqrt()
        };
        let ok = dist > 1e-15;
        Self {
            curve_id: 1,
            vertex1_id: 0,
            vertex2_id: 0,
            first: 0.0,
            last: dist,
            status: if ok { BuilderStatus::Done } else { BuilderStatus::Error },
            edge_id: if ok { 1001 } else { 0 },
            tolerance: 1e-7,
        }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn edge(&self) -> u32 { self.edge_id }
    pub fn status(&self) -> BuilderStatus { self.status }
    pub fn first_parameter(&self) -> f64 { self.first }
    pub fn last_parameter(&self) -> f64 { self.last }
}

// occt: BRepBuilderAPI_MakeWire — builds a topological wire from edges
#[derive(Clone, Debug, Default)]
pub struct MakeWire {
    pub edges: Vec<u32>,
    pub status: BuilderStatus,
    pub wire_id: u32,
    pub is_closed: bool,
    nb_wire: u32,
}

impl MakeWire {
    pub fn new() -> Self { Self { nb_wire: 0, ..Default::default() } }

    pub fn add_edge(&mut self, edge_id: u32) {
        if edge_id == 0 { self.status = BuilderStatus::Error; return; }
        self.edges.push(edge_id);
        self.wire_id = edge_id + 2000;
        self.status = BuilderStatus::Done;
    }

    pub fn add_wire(&mut self, wire_id: u32) {
        if wire_id == 0 { self.status = BuilderStatus::Error; return; }
        self.nb_wire += 1;
        self.status = BuilderStatus::Done;
    }

    pub fn perform(&mut self) {
        if self.edges.is_empty() && self.nb_wire == 0 {
            self.status = BuilderStatus::Error;
            return;
        }
        // Stub: wire_id from last edge
        if let Some(&last_edge) = self.edges.last() {
            self.wire_id = last_edge + 2000;
        }
        // check closure: stub always closed if >1 edge
        self.is_closed = self.edges.len() > 1;
        self.status = BuilderStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn wire(&self) -> u32 { self.wire_id }
    pub fn nb_edges(&self) -> usize { self.edges.len() }
    pub fn is_closed(&self) -> bool { self.is_closed }
    pub fn error(&self) -> BuilderStatus { self.status }
}

// occt: BRepBuilderAPI_MakeFace — builds a topological face from a surface + optional wires
#[derive(Clone, Debug)]
pub struct MakeFace {
    pub surface_id: u32,
    pub outer_wire: u32,
    pub inner_wires: Vec<u32>,
    pub u_first: f64,
    pub u_last: f64,
    pub v_first: f64,
    pub v_last: f64,
    pub tolerance: f64,
    pub status: BuilderStatus,
    pub face_id: u32,
}

impl MakeFace {
    pub fn from_surface(surface_id: u32, u1: f64, u2: f64, v1: f64, v2: f64, tol: f64) -> Self {
        let ok = surface_id > 0;
        Self {
            surface_id,
            outer_wire: 0,
            inner_wires: Vec::new(),
            u_first: u1, u_last: u2,
            v_first: v1, v_last: v2,
            tolerance: tol,
            status: if ok { BuilderStatus::Done } else { BuilderStatus::NullSurface },
            face_id: if ok { surface_id + 3000 } else { 0 },
        }
    }

    pub fn from_wire(wire_id: u32, only_plane: bool) -> Self {
        let ok = wire_id > 0;
        Self {
            surface_id: 0,
            outer_wire: wire_id,
            inner_wires: Vec::new(),
            u_first: 0.0, u_last: 1.0,
            v_first: 0.0, v_last: 1.0,
            tolerance: 1e-7,
            status: if ok { BuilderStatus::Done } else { BuilderStatus::WireNotClosed },
            face_id: if ok { wire_id + 3000 } else { 0 },
        }
    }

    pub fn add_inner_wire(&mut self, wire_id: u32) {
        if wire_id > 0 { self.inner_wires.push(wire_id); }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn face(&self) -> u32 { self.face_id }
    pub fn nb_inner_wires(&self) -> usize { self.inner_wires.len() }
    pub fn status(&self) -> BuilderStatus { self.status }
}

// occt: BRepBuilderAPI_MakeShell — builds a shell from a surface
#[derive(Clone, Debug)]
pub struct MakeShell {
    pub surface_id: u32,
    pub u_first: f64,
    pub u_last: f64,
    pub v_first: f64,
    pub v_last: f64,
    pub is_segment: bool,
    pub status: BuilderStatus,
    pub shell_id: u32,
}

impl MakeShell {
    pub fn new(surface_id: u32, u1: f64, u2: f64, v1: f64, v2: f64, segment: bool) -> Self {
        let ok = surface_id > 0;
        Self {
            surface_id,
            u_first: u1, u_last: u2,
            v_first: v1, v_last: v2,
            is_segment: segment,
            status: if ok { BuilderStatus::Done } else { BuilderStatus::NullSurface },
            shell_id: if ok { surface_id + 4000 } else { 0 },
        }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shell(&self) -> u32 { self.shell_id }
    pub fn status(&self) -> BuilderStatus { self.status }
    pub fn nb_faces(&self) -> usize { if self.is_done() { 1 } else { 0 } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_edge_from_curve() {
        let e = MakeEdge::from_curve(1, 0.0, 1.0);
        assert!(e.is_done());
        assert!(e.edge() > 0);
    }

    #[test]
    fn make_edge_null_curve() {
        let e = MakeEdge::from_curve(0, 0.0, 1.0);
        assert!(!e.is_done());
        assert_eq!(e.status(), BuilderStatus::NullCurve);
    }

    #[test]
    fn make_edge_from_line() {
        let e = MakeEdge::from_line([0.0,0.0,0.0], [1.0,0.0,0.0]);
        assert!(e.is_done());
        assert!((e.last_parameter() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn make_wire_add_edges() {
        let mut w = MakeWire::new();
        w.add_edge(100);
        w.add_edge(101);
        assert!(w.is_done());
        assert_eq!(w.nb_edges(), 2);
        assert!(w.wire() > 0);
    }

    #[test]
    fn make_face_from_surface() {
        let f = MakeFace::from_surface(5, 0.0, 1.0, 0.0, 1.0, 1e-6);
        assert!(f.is_done());
        assert_eq!(f.nb_inner_wires(), 0);
    }

    #[test]
    fn make_shell_basic() {
        let s = MakeShell::new(10, 0.0, 1.0, 0.0, 1.0, false);
        assert!(s.is_done());
        assert_eq!(s.nb_faces(), 1);
        let s2 = MakeShell::new(0, 0.0, 1.0, 0.0, 1.0, false);
        assert!(!s2.is_done());
    }
}

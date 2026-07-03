// FILE: rust/ironstream/crates/ironstream/src/brep_edge_maker.rs
//! `BRepBuilderAPI_MakeEdge` / `BRepBuilderAPI_MakeVertex` — edge and vertex
//! construction stubs, mirroring the corresponding OCCT builder classes.

// ---------------------------------------------------------------------------
// EdgeMaker
// ---------------------------------------------------------------------------

// occt: BRepBuilderAPI_MakeEdge
/// Stub builder mirroring `BRepBuilderAPI_MakeEdge`.
///
/// Constructs a topological edge from a geometric curve description (line,
/// circular arc, or an arbitrary named curve) together with its parameter
/// bounds.  The `edge` field carries a human-readable label describing the
/// geometry; `done` is set to `true` once construction succeeds.
#[derive(Debug, Clone)]
pub struct EdgeMaker {
    // occt: BRepBuilderAPI_MakeEdge — resulting TopoDS_Edge handle (stub: label string)
    pub edge: String,
    // occt: BRepBuilderAPI_MakeEdge::IsDone
    pub done: bool,
}

impl EdgeMaker {
    /// Build an edge from two 3-D end-points, representing a straight line
    /// segment.
    ///
    /// Mirrors `BRepBuilderAPI_MakeEdge(gp_Pnt, gp_Pnt)`.
    // occt: BRepBuilderAPI_MakeEdge(const gp_Pnt& P1, const gp_Pnt& P2)
    pub fn from_line(p1: [f64; 3], p2: [f64; 3]) -> Self {
        let edge = format!(
            "line({},{},{})-({},{},{})",
            p1[0], p1[1], p1[2], p2[0], p2[1], p2[2]
        );
        Self { edge, done: true }
    }

    /// Build an edge from a circular arc defined by its centre, radius, and
    /// start/end angles (in radians).
    ///
    /// Mirrors `BRepBuilderAPI_MakeEdge(Handle(Geom_Circle), Standard_Real, Standard_Real)`.
    // occt: BRepBuilderAPI_MakeEdge(const Handle(Geom_Circle)&, Standard_Real U1, Standard_Real U2)
    pub fn from_circle(center: [f64; 3], r: f64, u1: f64, u2: f64) -> Self {
        let edge = format!(
            "circle(center=({},{},{}),r={},u1={},u2={})",
            center[0], center[1], center[2], r, u1, u2
        );
        Self { edge, done: true }
    }

    /// Build an edge from an arbitrary named curve with parameter bounds.
    ///
    /// `curve` is a label identifying the underlying `Geom_Curve`; `u1` and
    /// `u2` are the first and last parameters.
    ///
    /// Mirrors `BRepBuilderAPI_MakeEdge(Handle(Geom_Curve), Standard_Real, Standard_Real)`.
    // occt: BRepBuilderAPI_MakeEdge(const Handle(Geom_Curve)&, Standard_Real U1, Standard_Real U2)
    pub fn from_curve(curve: &str, u1: f64, u2: f64) -> Self {
        let edge = format!("curve({},u1={},u2={})", curve, u1, u2);
        Self { edge, done: true }
    }

    /// Return a reference to the label of the constructed edge.
    ///
    /// Mirrors `BRepBuilderAPI_MakeEdge::Edge()`.
    // occt: BRepBuilderAPI_MakeEdge::Edge() -> const TopoDS_Edge&
    pub fn edge(&self) -> &str {
        &self.edge
    }

    /// Return `true` when the edge was successfully constructed.
    ///
    /// Mirrors `BRepBuilderAPI_MakeEdge::IsDone()`.
    // occt: BRepBuilderAPI_MakeEdge::IsDone() -> Standard_Boolean
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ---------------------------------------------------------------------------
// VertexMaker
// ---------------------------------------------------------------------------

// occt: BRepBuilderAPI_MakeVertex
/// Stub builder mirroring `BRepBuilderAPI_MakeVertex`.
///
/// Wraps a single 3-D point as a topological vertex.  `vertex` carries the
/// point coordinates; `done` is set to `true` on construction.
#[derive(Debug, Clone)]
pub struct VertexMaker {
    // occt: BRepBuilderAPI_MakeVertex — resulting TopoDS_Vertex (stub: point coords)
    pub vertex: [f64; 3],
    // occt: BRepBuilderAPI_MakeVertex::IsDone
    pub done: bool,
}

impl VertexMaker {
    /// Construct a vertex from a 3-D point.
    ///
    /// Mirrors `BRepBuilderAPI_MakeVertex(const gp_Pnt& P)`.
    // occt: BRepBuilderAPI_MakeVertex(const gp_Pnt& P)
    pub fn new(pt: [f64; 3]) -> Self {
        Self { vertex: pt, done: true }
    }

    /// Return the coordinates of the constructed vertex.
    ///
    /// Mirrors `BRepBuilderAPI_MakeVertex::Vertex()`.
    // occt: BRepBuilderAPI_MakeVertex::Vertex() -> const TopoDS_Vertex&
    pub fn vertex(&self) -> [f64; 3] {
        self.vertex
    }

    /// Return `true` when the vertex was successfully constructed.
    ///
    /// Mirrors `BRepBuilderAPI_MakeVertex::IsDone()`.
    // occt: BRepBuilderAPI_MakeVertex::IsDone() -> Standard_Boolean
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ---------------------------------------------------------------------------
// Free-function helpers
// ---------------------------------------------------------------------------

/// Construct a straight-line edge label from two 3-D points.
///
/// Convenience wrapper around [`EdgeMaker::from_line`] that returns the edge
/// label directly, mirroring typical single-call usage of
/// `BRepBuilderAPI_MakeEdge(P1, P2).Edge()`.
// occt: BRepBuilderAPI_MakeEdge(const gp_Pnt& P1, const gp_Pnt& P2) — convenience
pub fn make_edge_line(p1: [f64; 3], p2: [f64; 3]) -> String {
    EdgeMaker::from_line(p1, p2).edge
}

/// Construct a circular-arc edge label from a centre, radius, and angle range.
///
/// Convenience wrapper around [`EdgeMaker::from_circle`] that returns the edge
/// label directly, mirroring typical single-call usage of
/// `BRepBuilderAPI_MakeEdge(circle, u1, u2).Edge()`.
// occt: BRepBuilderAPI_MakeEdge(const Handle(Geom_Circle)&, U1, U2) — convenience
pub fn make_edge_arc(center: [f64; 3], r: f64, u1: f64, u2: f64) -> String {
    EdgeMaker::from_circle(center, r, u1, u2).edge
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_edge_is_done() {
        let m = EdgeMaker::from_line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert!(m.is_done());
        assert!(m.edge().contains("line"));
    }

    #[test]
    fn circle_edge_is_done() {
        let m = EdgeMaker::from_circle([0.0, 0.0, 0.0], 5.0, 0.0, std::f64::consts::PI);
        assert!(m.is_done());
        assert!(m.edge().contains("circle"));
    }

    #[test]
    fn curve_edge_is_done() {
        let m = EdgeMaker::from_curve("bspline_1", 0.0, 1.0);
        assert!(m.is_done());
        assert!(m.edge().contains("bspline_1"));
    }

    #[test]
    fn vertex_maker_roundtrip() {
        let pt = [3.0, 1.4, 2.7];
        let v = VertexMaker::new(pt);
        assert!(v.is_done());
        assert_eq!(v.vertex(), pt);
    }

    #[test]
    fn make_edge_line_helper() {
        let label = make_edge_line([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert!(label.starts_with("line("));
    }

    #[test]
    fn make_edge_arc_helper() {
        let label = make_edge_arc([0.0, 0.0, 0.0], 2.0, 0.0, 1.5707963);
        assert!(label.starts_with("circle("));
    }
}

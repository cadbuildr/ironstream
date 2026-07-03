// FILE: rust/ironstream/crates/ironstream/src/geom_wire_builder.rs
//! Zero-dependency port of `BRepBuilderAPI_MakeWire` / `BRepLib_MakeWire`.
//!
//! Builds an ordered, connected sequence of [`WireEdge`] segments and tracks
//! connectivity status via [`WireBuilderStatus`].

// ─────────────────────────────────────────────────────────────────────────────
// WireEdge
// ─────────────────────────────────────────────────────────────────────────────

/// A directed line-segment edge that can be part of a wire.
// occt: edge in wire — BRepBuilderAPI_MakeWire stores TopoDS_Edge handles;
//       here each edge is identified by an application-supplied `id` and
//       carries its own geometry (start / end 3-D points).
#[derive(Clone, Debug, PartialEq)]
pub struct WireEdge {
    id: usize,
    start: [f64; 3],
    end: [f64; 3],
    is_reversed: bool,
}

impl WireEdge {
    /// Create a new edge with the given application `id`, `start`, and `end`
    /// points.  `is_reversed` is `false` by default.
    // occt: BRepBuilderAPI_MakeWire::Add(const TopoDS_Edge&)
    pub fn new(id: usize, start: [f64; 3], end: [f64; 3]) -> Self {
        Self { id, start, end, is_reversed: false }
    }

    /// Application-level identifier supplied at construction.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Start point of the edge in its current orientation.
    pub fn start(&self) -> [f64; 3] {
        self.start
    }

    /// End point of the edge in its current orientation.
    pub fn end(&self) -> [f64; 3] {
        self.end
    }

    /// Euclidean length of the edge.
    // occt: BRep_Tool::EdgeLength / GCPnts_AbscissaPoint
    pub fn length(&self) -> f64 {
        let dx = self.end[0] - self.start[0];
        let dy = self.end[1] - self.start[1];
        let dz = self.end[2] - self.start[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Return a copy of the edge with `start` and `end` swapped and
    /// `is_reversed` toggled.
    // occt: TopoDS_Edge::Reversed() — reverses the shape orientation
    pub fn reversed(&self) -> WireEdge {
        WireEdge {
            id: self.id,
            start: self.end,
            end: self.start,
            is_reversed: !self.is_reversed,
        }
    }

    /// Whether this edge has been reversed relative to its original
    /// construction orientation.
    // occt: TopoDS_Shape::Orientation() == TopAbs_REVERSED
    pub fn is_reversed(&self) -> bool {
        self.is_reversed
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// WireBuilderStatus
// ─────────────────────────────────────────────────────────────────────────────

/// Outcome of a [`WireBuilder`] session.
// occt: BRepBuilderAPI_WireError enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WireBuilderStatus {
    /// The wire is valid and all edges are connected.
    // occt: BRepBuilderAPI_WireDone
    WireDone,
    /// No edges have been added yet.
    // occt: BRepBuilderAPI_EmptyWire
    EmptyWire,
    /// An added edge could not be connected to the existing wire.
    // occt: BRepBuilderAPI_DisconnectedWire
    DisconnectedWire,
    /// More than two edges share a vertex, making the wire non-manifold.
    // occt: BRepBuilderAPI_NonManifoldWire
    NonManifoldWire,
}

// ─────────────────────────────────────────────────────────────────────────────
// WireBuilder
// ─────────────────────────────────────────────────────────────────────────────

/// Incremental builder for a connected sequence of [`WireEdge`] segments.
///
/// Mirrors the public interface of `BRepBuilderAPI_MakeWire`:
/// edges are added one at a time; the builder checks that each new edge
/// connects (within [`CONNECTIVITY_TOL`]) to the tail of the growing wire,
/// reversing it automatically when the reversed orientation connects instead.
// occt: BRepBuilderAPI_MakeWire
pub struct WireBuilder {
    edges: Vec<WireEdge>,
    status: WireBuilderStatus,
}

/// Tolerance used for vertex-coincidence checks (≈ modelling precision).
// occt: Precision::Confusion() defaults to 1e-7
pub const CONNECTIVITY_TOL: f64 = 1e-7;

/// Return the squared Euclidean distance between two 3-D points.
fn dist_sq(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    dx * dx + dy * dy + dz * dz
}

/// `true` when `a` and `b` coincide within [`CONNECTIVITY_TOL`].
fn pts_coincide(a: [f64; 3], b: [f64; 3]) -> bool {
    dist_sq(a, b) <= CONNECTIVITY_TOL * CONNECTIVITY_TOL
}

impl WireBuilder {
    /// Create an empty builder.  Initial status is [`WireBuilderStatus::EmptyWire`].
    // occt: BRepBuilderAPI_MakeWire()
    pub fn new() -> Self {
        Self { edges: Vec::new(), status: WireBuilderStatus::EmptyWire }
    }

    /// Add `e` to the wire.
    ///
    /// * If the wire is empty the edge is accepted unconditionally.
    /// * Otherwise the builder checks whether `e.start()` coincides with the
    ///   last edge's `end()`; if so the edge is appended as-is.  If instead
    ///   `e.end()` coincides with the last edge's `end()`, the edge is
    ///   appended in reversed orientation.
    /// * If neither end connects, the status becomes
    ///   [`WireBuilderStatus::DisconnectedWire`] and the edge is **not** added.
    // occt: BRepBuilderAPI_MakeWire::Add(const TopoDS_Edge&)
    pub fn add(&mut self, e: WireEdge) {
        if self.status == WireBuilderStatus::DisconnectedWire
            || self.status == WireBuilderStatus::NonManifoldWire
        {
            // Once disconnected the builder is poisoned; no more edges accepted.
            return;
        }

        if self.edges.is_empty() {
            self.edges.push(e);
            self.status = WireBuilderStatus::WireDone;
            return;
        }

        let tail = self.edges.last().unwrap().end();

        if pts_coincide(tail, e.start()) {
            // Natural orientation connects.
            self.edges.push(e);
            self.status = WireBuilderStatus::WireDone;
        } else if pts_coincide(tail, e.end()) {
            // Reversed orientation connects.
            self.edges.push(e.reversed());
            self.status = WireBuilderStatus::WireDone;
        } else {
            self.status = WireBuilderStatus::DisconnectedWire;
        }
    }

    /// Number of edges currently stored in the wire.
    pub fn nb_edges(&self) -> usize {
        self.edges.len()
    }

    /// Current build status.
    // occt: BRepBuilderAPI_MakeWire::Error()
    pub fn status(&self) -> WireBuilderStatus {
        self.status
    }

    /// `true` when the status is [`WireBuilderStatus::WireDone`].
    // occt: BRepBuilderAPI_Command::IsDone()
    pub fn is_done(&self) -> bool {
        self.status == WireBuilderStatus::WireDone
    }

    /// Check whether the last edge's end point coincides with the first edge's
    /// start point; if so mark the wire as closed (`WireDone` is already set),
    /// otherwise set [`WireBuilderStatus::DisconnectedWire`].
    ///
    /// Has no effect on an empty wire.
    // occt: BRepLib::BuildCurve3d / wire closure check in BRepBuilderAPI
    pub fn close(&mut self) {
        if self.edges.is_empty() {
            return;
        }
        let first_start = self.edges.first().unwrap().start();
        let last_end = self.edges.last().unwrap().end();
        if pts_coincide(last_end, first_start) {
            self.status = WireBuilderStatus::WireDone;
        } else {
            self.status = WireBuilderStatus::DisconnectedWire;
        }
    }

    /// Sum of the lengths of all edges in the wire.
    // occt: wire-walking with GCPnts_AbscissaPoint to sum curve lengths
    pub fn total_length(&self) -> f64 {
        self.edges.iter().map(|e| e.length()).sum()
    }

    /// `true` when the wire is non-empty, currently done, and the last edge's
    /// end coincides with the first edge's start.
    // occt: BRep_Tool::IsClosed(const TopoDS_Wire&)
    pub fn is_closed(&self) -> bool {
        if self.edges.len() < 2 {
            return false;
        }
        let first_start = self.edges.first().unwrap().start();
        let last_end = self.edges.last().unwrap().end();
        pts_coincide(last_end, first_start)
    }
}

impl Default for WireBuilder {
    fn default() -> Self {
        Self::new()
    }
}

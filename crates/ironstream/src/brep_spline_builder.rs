// FILE: rust/ironstream/crates/ironstream/src/brep_spline_builder.rs
//! `BRepBuilderAPI` wire/edge construction helpers — edge and wire builders
//! mirroring OCCT's `BRepBuilderAPI_MakeEdge` and `BRepBuilderAPI_MakeWire`,
//! plus the `BRep_Tool` distance utility, all in zero-dependency Rust.

/// Compute the Euclidean distance between two 3-D points.
///
// occt-ref: BRep_Tool // utility
pub fn edge_length(start: [f64; 3], end: [f64; 3]) -> f64 {
    let dx = end[0] - start[0];
    let dy = end[1] - start[1];
    let dz = end[2] - start[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

// ---------------------------------------------------------------------------
// BrepEdgeParams
// ---------------------------------------------------------------------------

/// Parameters used to construct a single B-Rep edge from two end-points.
///
// occt-ref: BRepBuilderAPI_MakeEdge // parameters
#[derive(Debug, Clone, PartialEq)]
pub struct BrepEdgeParams {
    /// Start point of the edge.
    start: [f64; 3],
    /// End point of the edge.
    end: [f64; 3],
    /// Vertex coincidence tolerance (default: `1e-7`).
    tolerance: f64,
    /// Whether an explicit parameter range `[u_start, u_end]` has been set.
    pub has_parameter_range: bool,
    /// Parameter value at the start of the edge curve.
    pub u_start: f64,
    /// Parameter value at the end of the edge curve.
    pub u_end: f64,
}

impl BrepEdgeParams {
    /// Construct edge parameters from two 3-D points using the default
    /// tolerance (`1e-7`) and no explicit parameter range.
    ///
    /// // occt-note: BRepBuilderAPI_MakeEdge(P1, P2)
    pub fn new(start: [f64; 3], end: [f64; 3]) -> Self {
        Self {
            start,
            end,
            tolerance: 1e-7,
            has_parameter_range: false,
            u_start: 0.0,
            u_end: 0.0,
        }
    }

    /// Attach an explicit curve-parameter range and return `self` (builder
    /// pattern).
    ///
    /// // occt-note: BRepBuilderAPI_MakeEdge(curve, p1, p2)
    pub fn with_range(mut self, u_start: f64, u_end: f64) -> Self {
        self.u_start = u_start;
        self.u_end = u_end;
        self.has_parameter_range = true;
        self
    }

    /// Return the start point.
    pub fn start(&self) -> [f64; 3] {
        self.start
    }

    /// Return the end point.
    pub fn end(&self) -> [f64; 3] {
        self.end
    }

    /// Return the vertex coincidence tolerance.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Override the vertex coincidence tolerance.
    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol;
    }
}

// ---------------------------------------------------------------------------
// BrepMakeEdge
// ---------------------------------------------------------------------------

/// Builder that creates a single topological edge from `BrepEdgeParams`.
///
// occt-ref: BRepBuilderAPI_MakeEdge // stub
#[derive(Debug, Clone)]
pub struct BrepMakeEdge {
    /// The parameters used to define this edge.
    pub params: BrepEdgeParams,
    /// `true` after `build()` has been called successfully.
    pub is_done: bool,
    /// Euclidean length of the edge (set by `build()`).
    pub length: f64,
}

impl BrepMakeEdge {
    /// Create a new edge builder from the given parameters.
    ///
    // occt-ref: BRepBuilderAPI_MakeEdge // ::BRepBuilderAPI_MakeEdge
    pub fn new(params: BrepEdgeParams) -> Self {
        Self {
            params,
            is_done: false,
            length: 0.0,
        }
    }

    /// Build the edge: compute its length and mark it as done.
    ///
    // occt-ref: BRepBuilderAPI_MakeEdge // ::Build
    pub fn build(&mut self) {
        self.length = edge_length(self.params.start(), self.params.end());
        self.is_done = true;
    }

    /// Returns `true` if `build()` has been called.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Returns the Euclidean length of the edge (0.0 before `build()`).
    pub fn length(&self) -> f64 {
        self.length
    }
}

// ---------------------------------------------------------------------------
// BrepWireBuilder
// ---------------------------------------------------------------------------

/// Builder that assembles a sequence of edges into a wire.
///
// occt-ref: BRepBuilderAPI_MakeWire
#[derive(Debug, Default)]
pub struct BrepWireBuilder {
    /// Ordered edges that form the wire.
    pub edges: Vec<BrepMakeEdge>,
    /// `true` after `build()` succeeds (all constituent edges are done).
    pub is_done: bool,
    /// Number of edges in the wire (set by `build()`).
    pub nb_edges: usize,
    /// `true` if the wire is detected to be closed (set by `build()` via
    /// `detect_closed()`).
    pub is_closed: bool,
}

impl BrepWireBuilder {
    /// Create an empty wire builder.
    ///
    // occt-ref: BRepBuilderAPI_MakeWire // ::BRepBuilderAPI_MakeWire
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a built (or unbuilt) edge to the wire.
    ///
    // occt-ref: BRepBuilderAPI_MakeWire // ::Add
    pub fn add_edge(&mut self, e: BrepMakeEdge) {
        self.edges.push(e);
    }

    /// Build the wire:
    /// - `is_done` is set to `true` only when every edge's `is_done` flag is
    ///   `true`.
    /// - `nb_edges` reflects the total edge count.
    /// - `detect_closed()` is called to update `is_closed`.
    ///
    // occt-ref: BRepBuilderAPI_MakeWire // ::Build
    pub fn build(&mut self) {
        self.nb_edges = self.edges.len();
        self.is_done = self.edges.iter().all(|e| e.is_done());
        self.detect_closed();
    }

    /// Detect whether the wire is closed by comparing the start of the first
    /// edge to the end of the last edge within the tolerance of the first
    /// edge's parameters.
    ///
    /// A wire with fewer than two edges is never considered closed.
    ///
    // occt-ref: BRepBuilderAPI_MakeWire // closed-wire detection
    pub fn detect_closed(&mut self) {
        if self.edges.len() < 2 {
            self.is_closed = false;
            return;
        }
        let first = &self.edges[0];
        let last = &self.edges[self.edges.len() - 1];
        let tol = first.params.tolerance();
        let dist = edge_length(first.params.start(), last.params.end());
        self.is_closed = dist <= tol;
    }

    /// Returns `true` if all edges were successfully built.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Returns the number of edges in the wire.
    pub fn nb_edges(&self) -> usize {
        self.nb_edges
    }

    /// Returns `true` if the wire is closed (start of first edge coincides
    /// with end of last edge within tolerance).
    pub fn is_closed(&self) -> bool {
        self.is_closed
    }
}

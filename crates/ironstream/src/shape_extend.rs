// FILE: rust/ironstream/crates/ironstream/src/shape_extend.rs
//! `ShapeExtend` — wire/edge data structures and status codes mirroring
//! OpenCascade's `ShapeExtend` package.
//!
//! # Scope
//!
//! * [`ShapeExtendStatus`]  — operation status flags (Ok / Done* / Fail*).
//! * [`ShapeExtendMsg`]     — diagnostic message carrying a status and text.
//! * [`ShapeExtendWireData`]— edge-sequence container representing a wire.
//! * [`ShapeExtendEdge`]    — per-edge attribute record.

// ---------------------------------------------------------------------------
// ShapeExtendStatus
// ---------------------------------------------------------------------------

/// Operation status codes for ShapeExtend algorithms.
///
// occt-ref: ShapeExtend_Status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShapeExtendStatus {
    /// No particular condition; algorithm neither succeeded nor failed.
    Ok,
    /// Algorithm performed at least one action (aggregate of Done1..Done8).
    Done,
    /// Specific done flag 1.
    Done1,
    /// Specific done flag 2.
    Done2,
    /// Specific done flag 3.
    Done3,
    /// Specific done flag 4.
    Done4,
    /// Specific done flag 5.
    Done5,
    /// Specific done flag 6.
    Done6,
    /// Specific done flag 7.
    Done7,
    /// Specific done flag 8.
    Done8,
    /// Algorithm failed (aggregate of Fail1..Fail4).
    Fail,
    /// Specific failure flag 1.
    Fail1,
    /// Specific failure flag 2.
    Fail2,
    /// Specific failure flag 3.
    Fail3,
    /// Specific failure flag 4.
    Fail4,
}

// ---------------------------------------------------------------------------
// ShapeExtendMsg
// ---------------------------------------------------------------------------

/// Diagnostic message produced by a ShapeExtend algorithm.
///
// occt-note: shape extension message
#[derive(Debug, Clone)]
pub struct ShapeExtendMsg {
    status: ShapeExtendStatus,
    text: String,
    nb_shapes: usize,
}

impl ShapeExtendMsg {
    /// Construct a new message with the given status and text.
    /// `nb_shapes` is initialised to `0`.
    pub fn new(status: ShapeExtendStatus, text: &str) -> Self {
        Self {
            status,
            text: text.to_owned(),
            nb_shapes: 0,
        }
    }

    /// Return the status code associated with this message.
    pub fn status(&self) -> ShapeExtendStatus {
        self.status
    }

    /// Return the human-readable description of this message.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Return the number of shapes this message relates to.
    pub fn nb_shapes(&self) -> usize {
        self.nb_shapes
    }

    /// Set the number of shapes this message relates to.
    pub fn set_nb_shapes(&mut self, nb: usize) {
        self.nb_shapes = nb;
    }
}

// ---------------------------------------------------------------------------
// ShapeExtendWireData
// ---------------------------------------------------------------------------

/// Ordered sequence of edge labels representing a wire.
///
/// Edges are stored as string labels so that this crate remains free of
/// topology dependencies.  In a full kernel each label would reference a
/// `TopoDS_Edge`.
///
// occt-ref: ShapeExtend_WireData
#[derive(Debug, Clone)]
pub struct ShapeExtendWireData {
    edges: Vec<String>,
    nb_edges: usize,
    is_manifold: bool,
}

impl ShapeExtendWireData {
    /// Construct an empty wire with no edges.
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
            nb_edges: 0,
            is_manifold: true,
        }
    }

    /// Add an edge label to the wire.
    ///
    /// If `at_end` is `true` the edge is appended; otherwise it is prepended.
    pub fn add_edge(&mut self, label: &str, at_end: bool) {
        if at_end {
            self.edges.push(label.to_owned());
        } else {
            self.edges.insert(0, label.to_owned());
        }
        self.nb_edges = self.edges.len();
    }

    /// Return the number of edges in the wire.
    pub fn nb_edges(&self) -> usize {
        self.nb_edges
    }

    /// Return the edge label at position `i` (1-based, matching OCCT convention).
    ///
    /// # Panics
    ///
    /// Panics if `i` is `0` or greater than `nb_edges`.
    pub fn edge(&self, i: usize) -> &str {
        assert!(i >= 1 && i <= self.nb_edges, "edge index out of range");
        &self.edges[i - 1]
    }

    /// Return whether this wire is manifold.
    pub fn is_manifold(&self) -> bool {
        self.is_manifold
    }

    /// Set the manifold flag.
    pub fn set_manifold(&mut self, manifold: bool) {
        self.is_manifold = manifold;
    }

    /// Reverse the order of the edges in the wire.
    pub fn reverse(&mut self) {
        self.edges.reverse();
    }

    /// Remove all edges from the wire.
    pub fn clear(&mut self) {
        self.edges.clear();
        self.nb_edges = 0;
    }
}

impl Default for ShapeExtendWireData {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// ShapeExtendEdge
// ---------------------------------------------------------------------------

/// Attribute record for a single topological edge.
///
// occt-note: ShapeExtend_Edge info
#[derive(Debug, Clone)]
pub struct ShapeExtendEdge {
    edge_label: String,
    pcurve_on_s1: bool,
    pcurve_on_s2: bool,
    is_free: bool,
}

impl ShapeExtendEdge {
    /// Construct a new edge record with the given label.
    ///
    /// Both pcurve flags default to `false`; `is_free` defaults to `false`.
    pub fn new(label: &str) -> Self {
        Self {
            edge_label: label.to_owned(),
            pcurve_on_s1: false,
            pcurve_on_s2: false,
            is_free: false,
        }
    }

    /// Return the edge label.
    pub fn edge_label(&self) -> &str {
        &self.edge_label
    }

    /// Return whether a pcurve on surface 1 is present.
    pub fn has_pcurve_on_s1(&self) -> bool {
        self.pcurve_on_s1
    }

    /// Set whether a pcurve on surface 1 is present.
    pub fn set_pcurve_on_s1(&mut self, value: bool) {
        self.pcurve_on_s1 = value;
    }

    /// Return whether a pcurve on surface 2 is present.
    pub fn has_pcurve_on_s2(&self) -> bool {
        self.pcurve_on_s2
    }

    /// Return whether this edge is free (not shared between two faces).
    pub fn is_free(&self) -> bool {
        self.is_free
    }

    /// Set whether this edge is free.
    pub fn set_free(&mut self, value: bool) {
        self.is_free = value;
    }
}

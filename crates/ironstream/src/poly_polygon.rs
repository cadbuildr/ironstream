// FILE: rust/ironstream/crates/ironstream/src/poly_polygon.rs
//! `poly_polygon` — standalone polygon types mirroring OpenCascade's `Poly`
//! package polygon classes.
//!
//! Provides three types:
//!
//! * [`PolyPolygon3D`] — a 3-D polygon whose nodes are stored as `[f64; 3]`
//!   tuples, with an optional array of curve parameters at each node.
//! * [`PolyPolygon2D`] — the 2-D counterpart with `[f64; 2]` nodes.
//! * [`PolyPolygonOnTriangulation`] — a polygon that references existing nodes
//!   in a `Poly_Triangulation` by index rather than storing coordinates.
//!
//! All types are **zero-dependency** (std only).  Indexing is **0-based**
//! throughout (unlike OCCT's 1-based convention) for idiomatic Rust; the
//! `node(i)` and `parameter(i)` methods panic on out-of-range `i` with a
//! descriptive message.

// ─── PolyPolygon3D ───────────────────────────────────────────────────────────

/// A 3-D polygon: an ordered sequence of 3-D nodes with an optional array of
/// curve-parameter values at each node and a linear deflection hint.
///
/// Mirrors `Poly_Polygon3D` from OpenCascade's `Poly` package.
// occt-ref: Poly_Polygon3D
#[derive(Clone, Debug)]
pub struct PolyPolygon3D {
    /// Ordered list of 3-D node coordinates `[x, y, z]`.
    nodes: Vec<[f64; 3]>,
    /// Whether curve parameters are stored alongside each node.
    has_parameters: bool,
    /// Curve-parameter value at each node (same length as `nodes` when present).
    parameters: Vec<f64>,
    /// Linear deflection of the polygon with respect to the underlying curve.
    deflection: f64,
}

impl PolyPolygon3D {
    /// Construct a 3-D polygon from a node list.
    /// `deflection` is initialised to `0.0`; no parameters are stored.
    ///
    /// Mirrors `Poly_Polygon3D(Nodes)`.
    pub fn new(nodes: Vec<[f64; 3]>) -> Self {
        Self {
            nodes,
            has_parameters: false,
            parameters: Vec::new(),
            deflection: 0.0,
        }
    }

    /// Construct a 3-D polygon with nodes *and* per-node curve parameters.
    ///
    /// # Panics
    /// Panics when `parameters.len() != nodes.len()`.
    ///
    /// Mirrors `Poly_Polygon3D(Nodes, Parameters)`.
    pub fn with_parameters(nodes: Vec<[f64; 3]>, parameters: Vec<f64>) -> Self {
        assert_eq!(
            nodes.len(),
            parameters.len(),
            "PolyPolygon3D::with_parameters: nodes.len() ({}) != parameters.len() ({})",
            nodes.len(),
            parameters.len()
        );
        Self {
            nodes,
            has_parameters: true,
            parameters,
            deflection: 0.0,
        }
    }

    /// Number of nodes in the polygon.
    ///
    /// Mirrors `NbNodes()`.
    #[inline]
    pub fn nb_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Return the node at 0-based index `i`.
    ///
    /// # Panics
    /// Panics when `i >= nb_nodes()`.
    ///
    /// Mirrors `Nodes()(i)`.
    #[inline]
    pub fn node(&self, i: usize) -> [f64; 3] {
        assert!(
            i < self.nodes.len(),
            "PolyPolygon3D::node: index {i} out of range [0, {})",
            self.nodes.len()
        );
        self.nodes[i]
    }

    /// Return the current linear deflection.
    ///
    /// Mirrors `Deflection()`.
    #[inline]
    pub fn deflection(&self) -> f64 {
        self.deflection
    }

    /// Set the linear deflection.
    ///
    /// Mirrors `Deflection(d)`.
    #[inline]
    pub fn set_deflection(&mut self, d: f64) {
        self.deflection = d;
    }

    /// Return `true` when per-node curve parameters are available.
    ///
    /// Mirrors `HasParameters()`.
    #[inline]
    pub fn has_parameters(&self) -> bool {
        self.has_parameters
    }

    /// Return the curve-parameter value at 0-based index `i`.
    ///
    /// # Panics
    /// Panics when no parameters are stored or `i >= nb_nodes()`.
    ///
    /// Mirrors `Parameter(i)`.
    #[inline]
    pub fn parameter(&self, i: usize) -> f64 {
        assert!(
            self.has_parameters,
            "PolyPolygon3D::parameter: no parameters stored; construct with with_parameters()"
        );
        assert!(
            i < self.parameters.len(),
            "PolyPolygon3D::parameter: index {i} out of range [0, {})",
            self.parameters.len()
        );
        self.parameters[i]
    }
}

// ─── PolyPolygon2D ───────────────────────────────────────────────────────────

/// A 2-D polygon: an ordered sequence of 2-D nodes with an optional array of
/// curve-parameter values at each node and a linear deflection hint.
///
/// Mirrors `Poly_Polygon2D` from OpenCascade's `Poly` package.
// occt-ref: Poly_Polygon2D
#[derive(Clone, Debug)]
pub struct PolyPolygon2D {
    /// Ordered list of 2-D node coordinates `[u, v]`.
    nodes: Vec<[f64; 2]>,
    /// Whether curve parameters are stored alongside each node.
    has_parameters: bool,
    /// Curve-parameter value at each node (same length as `nodes` when present).
    parameters: Vec<f64>,
    /// Linear deflection of the polygon with respect to the underlying curve.
    deflection: f64,
}

impl PolyPolygon2D {
    /// Construct a 2-D polygon from a node list.
    /// `deflection` is initialised to `0.0`; no parameters are stored.
    ///
    /// Mirrors `Poly_Polygon2D(Nodes)`.
    pub fn new(nodes: Vec<[f64; 2]>) -> Self {
        Self {
            nodes,
            has_parameters: false,
            parameters: Vec::new(),
            deflection: 0.0,
        }
    }

    /// Construct a 2-D polygon with nodes *and* per-node curve parameters.
    ///
    /// # Panics
    /// Panics when `parameters.len() != nodes.len()`.
    ///
    /// Mirrors `Poly_Polygon2D(Nodes, Parameters)`.
    pub fn with_parameters(nodes: Vec<[f64; 2]>, parameters: Vec<f64>) -> Self {
        assert_eq!(
            nodes.len(),
            parameters.len(),
            "PolyPolygon2D::with_parameters: nodes.len() ({}) != parameters.len() ({})",
            nodes.len(),
            parameters.len()
        );
        Self {
            nodes,
            has_parameters: true,
            parameters,
            deflection: 0.0,
        }
    }

    /// Number of nodes in the polygon.
    ///
    /// Mirrors `NbNodes()`.
    #[inline]
    pub fn nb_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Return the node at 0-based index `i`.
    ///
    /// # Panics
    /// Panics when `i >= nb_nodes()`.
    ///
    /// Mirrors `Nodes()(i)`.
    #[inline]
    pub fn node(&self, i: usize) -> [f64; 2] {
        assert!(
            i < self.nodes.len(),
            "PolyPolygon2D::node: index {i} out of range [0, {})",
            self.nodes.len()
        );
        self.nodes[i]
    }

    /// Return the current linear deflection.
    ///
    /// Mirrors `Deflection()`.
    #[inline]
    pub fn deflection(&self) -> f64 {
        self.deflection
    }

    /// Set the linear deflection.
    ///
    /// Mirrors `Deflection(d)`.
    #[inline]
    pub fn set_deflection(&mut self, d: f64) {
        self.deflection = d;
    }

    /// Return `true` when per-node curve parameters are available.
    ///
    /// Mirrors `HasParameters()`.
    #[inline]
    pub fn has_parameters(&self) -> bool {
        self.has_parameters
    }

    /// Return the curve-parameter value at 0-based index `i`.
    ///
    /// # Panics
    /// Panics when no parameters are stored or `i >= nb_nodes()`.
    ///
    /// Mirrors `Parameter(i)`.
    #[inline]
    pub fn parameter(&self, i: usize) -> f64 {
        assert!(
            self.has_parameters,
            "PolyPolygon2D::parameter: no parameters stored; construct with with_parameters()"
        );
        assert!(
            i < self.parameters.len(),
            "PolyPolygon2D::parameter: index {i} out of range [0, {})",
            self.parameters.len()
        );
        self.parameters[i]
    }
}

// ─── PolyPolygonOnTriangulation ──────────────────────────────────────────────

/// A polygon that lives on the surface of a triangulation: rather than storing
/// 3-D or 2-D coordinates it holds a list of indices into an external
/// `Poly_Triangulation` node array, with optional per-node curve parameters
/// and a linear deflection hint.
///
/// Mirrors `Poly_PolygonOnTriangulation` from OpenCascade's `Poly` package.
// occt-ref: Poly_PolygonOnTriangulation
#[derive(Clone, Debug)]
pub struct PolyPolygonOnTriangulation {
    /// 0-based indices into a companion triangulation's node array.
    node_indices: Vec<usize>,
    /// Whether curve parameters are stored alongside each node index.
    has_parameters: bool,
    /// Curve-parameter value at each node (same length as `node_indices` when present).
    parameters: Vec<f64>,
    /// Linear deflection of the polygon with respect to the underlying curve.
    deflection: f64,
}

impl PolyPolygonOnTriangulation {
    /// Construct a polygon-on-triangulation from a list of triangulation node
    /// indices.  `deflection` is initialised to `0.0`; no parameters are stored.
    ///
    /// Mirrors `Poly_PolygonOnTriangulation(Nodes)`.
    pub fn new(node_indices: Vec<usize>) -> Self {
        Self {
            node_indices,
            has_parameters: false,
            parameters: Vec::new(),
            deflection: 0.0,
        }
    }

    /// Number of node indices stored.
    ///
    /// Mirrors `NbNodes()`.
    #[inline]
    pub fn nb_nodes(&self) -> usize {
        self.node_indices.len()
    }

    /// Return the triangulation node index at 0-based position `i`.
    ///
    /// # Panics
    /// Panics when `i >= nb_nodes()`.
    ///
    /// Mirrors `Node(i)`.
    #[inline]
    pub fn node(&self, i: usize) -> usize {
        assert!(
            i < self.node_indices.len(),
            "PolyPolygonOnTriangulation::node: index {i} out of range [0, {})",
            self.node_indices.len()
        );
        self.node_indices[i]
    }

    /// Return the current linear deflection.
    ///
    /// Mirrors `Deflection()`.
    #[inline]
    pub fn deflection(&self) -> f64 {
        self.deflection
    }

    /// Set the linear deflection.
    ///
    /// Mirrors `Deflection(d)`.
    #[inline]
    pub fn set_deflection(&mut self, d: f64) {
        self.deflection = d;
    }

    /// Return `true` when per-node curve parameters are available.
    ///
    /// Mirrors `HasParameters()`.
    #[inline]
    pub fn has_parameters(&self) -> bool {
        self.has_parameters
    }

    /// Return the curve-parameter value at 0-based index `i`.
    ///
    /// # Panics
    /// Panics when no parameters are stored or `i >= nb_nodes()`.
    ///
    /// Mirrors `Parameter(i)`.
    #[inline]
    pub fn parameter(&self, i: usize) -> f64 {
        assert!(
            self.has_parameters,
            "PolyPolygonOnTriangulation::parameter: no parameters stored"
        );
        assert!(
            i < self.parameters.len(),
            "PolyPolygonOnTriangulation::parameter: index {i} out of range [0, {})",
            self.parameters.len()
        );
        self.parameters[i]
    }
}

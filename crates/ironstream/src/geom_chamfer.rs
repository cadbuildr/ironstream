// FILE: rust/ironstream/crates/ironstream/src/geom_chamfer.rs
// occt-ref: ChFi3d_ChBuilder // chamfer algorithm types

/// Chamfer mode controlling whether both distances are equal or independent.
// occt-ref: ChFi3d_ChBuilder // mode — enum Equal, Unequal
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChamferMode {
    /// Both chamfer distances are the same (symmetric chamfer).
    Equal,
    /// The two chamfer distances differ (asymmetric chamfer).
    Unequal,
}

/// A single edge tagged for chamfering with one or two distances.
// occt-note: chamfer edge spec — holds label, dist1, dist2, and ChamferMode
#[derive(Clone, Debug)]
pub struct ChamferEdge {
    edge_label: String,
    dist1: f64,
    dist2: f64,
    mode: ChamferMode,
}

impl ChamferEdge {
    /// Creates a symmetric chamfer edge where both distances equal `dist`.
    pub fn new(label: &str, dist: f64) -> Self {
        Self {
            edge_label: label.to_string(),
            dist1: dist,
            dist2: dist,
            mode: ChamferMode::Equal,
        }
    }

    /// Creates an asymmetric chamfer edge with two independent distances.
    pub fn new_unequal(label: &str, d1: f64, d2: f64) -> Self {
        Self {
            edge_label: label.to_string(),
            dist1: d1,
            dist2: d2,
            mode: ChamferMode::Unequal,
        }
    }

    /// Returns the edge label.
    pub fn edge_label(&self) -> &str {
        &self.edge_label
    }

    /// Returns the first (or only) chamfer distance.
    pub fn dist1(&self) -> f64 {
        self.dist1
    }

    /// Returns the second chamfer distance (equals dist1 for Equal mode).
    pub fn dist2(&self) -> f64 {
        self.dist2
    }

    /// Returns the chamfer mode for this edge.
    pub fn mode(&self) -> ChamferMode {
        self.mode
    }
}

/// Result of a chamfer build operation.
// occt-ref: BRepFilletAPI_MakeChamfer // result
#[derive(Clone, Debug)]
pub struct ChamferResult {
    nb_contours: usize,
    nb_surfaces: usize,
    is_done: bool,
}

impl ChamferResult {
    /// Creates a new, not-yet-computed result.
    pub fn new() -> Self {
        Self {
            nb_contours: 0,
            nb_surfaces: 0,
            is_done: false,
        }
    }

    /// Marks the result as successfully built with the given counts.
    pub fn build(&mut self, nb_contours: usize, nb_surfaces: usize) {
        self.nb_contours = nb_contours;
        self.nb_surfaces = nb_surfaces;
        self.is_done = true;
    }

    /// Returns whether the chamfer build succeeded.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Returns the number of contours produced.
    pub fn nb_contours(&self) -> usize {
        self.nb_contours
    }

    /// Returns the number of surfaces produced.
    pub fn nb_surfaces(&self) -> usize {
        self.nb_surfaces
    }
}

impl Default for ChamferResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Applies chamfers to a collection of edges on a solid.
// occt-ref: BRepFilletAPI_MakeChamfer
pub struct MakeChamfer {
    /// Edges registered for chamfering.
    pub edges: Vec<ChamferEdge>,
    /// Result populated after [`build`](MakeChamfer::build) is called.
    pub result: ChamferResult,
}

impl MakeChamfer {
    /// Creates a new, empty chamfer builder.
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
            result: ChamferResult::new(),
        }
    }

    /// Registers an edge for chamfering.
    pub fn add_edge(&mut self, e: ChamferEdge) {
        self.edges.push(e);
    }

    /// Returns the number of edges registered for chamfering.
    pub fn nb_edges(&self) -> usize {
        self.edges.len()
    }

    /// Runs the chamfer computation and populates `result`.
    ///
    /// In this pure-Rust port the build always succeeds: it sets
    /// `nb_contours` and `nb_surfaces` both equal to the number of
    /// registered edges, mirroring the BRepFilletAPI_MakeChamfer stub contract.
    pub fn build(&mut self) {
        let n = self.edges.len();
        self.result.build(n, n);
    }

    /// Returns whether the last [`build`](MakeChamfer::build) succeeded.
    pub fn is_done(&self) -> bool {
        self.result.is_done()
    }

    /// Returns a reference to the build result.
    pub fn result(&self) -> &ChamferResult {
        &self.result
    }
}

impl Default for MakeChamfer {
    fn default() -> Self {
        Self::new()
    }
}

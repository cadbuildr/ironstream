// FILE: rust/ironstream/crates/ironstream/src/geom_fillet.rs
// occt-note: ChFi3d/ChFi2d fillet algorithm types

/// Fillet surface approximation strategy.
// occt-note: fillet type — enum ChFi3d_FilletShape (Rational, QuasiAngular, Polynomial)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilletType {
    /// Rational Bezier approximation of the fillet surface.
    Rational,
    /// Quasi-angular parameterisation of the fillet surface.
    QuasiAngular,
    /// Polynomial approximation of the fillet surface.
    Polynomial,
}

/// A single edge tagged for filleting with an associated radius.
// occt: ChFi3d_FilBuilder // edge to fillet
#[derive(Clone, Debug)]
pub struct FilletEdge {
    edge_label: String,
    radius: f64,
}

impl FilletEdge {
    /// Creates a new fillet edge with the given label and radius.
    pub fn new(label: &str, radius: f64) -> Self {
        Self {
            edge_label: label.to_string(),
            radius,
        }
    }

    /// Returns the edge label.
    pub fn label(&self) -> &str {
        &self.edge_label
    }

    /// Returns the fillet radius.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Sets the fillet radius.
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
}

/// Result of a 3D fillet build operation.
// occt-ref: ChFi3d_Builder // result
#[derive(Clone, Debug)]
pub struct FilletResult {
    nb_contours: usize,
    nb_surfaces: usize,
    is_done: bool,
    error_label: String,
}

impl FilletResult {
    /// Creates a new, not-yet-computed result.
    pub fn new() -> Self {
        Self {
            nb_contours: 0,
            nb_surfaces: 0,
            is_done: false,
            error_label: String::new(),
        }
    }

    /// Marks the result as successfully computed.
    pub fn set_done(&mut self, nb_contours: usize, nb_surfaces: usize) {
        self.nb_contours = nb_contours;
        self.nb_surfaces = nb_surfaces;
        self.is_done = true;
        self.error_label.clear();
    }

    /// Returns whether the fillet build succeeded.
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

    /// Returns the error label, empty when the build succeeded.
    pub fn error_label(&self) -> &str {
        &self.error_label
    }

    /// Records an error message and leaves is_done as false.
    pub fn set_error(&mut self, msg: &str) {
        self.error_label = msg.to_string();
        self.is_done = false;
    }
}

impl Default for FilletResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Applies constant-radius fillets to a collection of edges.
// occt: ChFi3d_FilBuilder // — applies fillets to edges
pub struct FilletBuilder {
    /// Approximation strategy for the fillet surfaces.
    pub fillet_type: FilletType,
    /// Edges to be filleted.
    pub edges: Vec<FilletEdge>,
    /// Result populated after [`build`](FilletBuilder::build) is called.
    pub result: FilletResult,
    /// Geometric tolerance used during surface construction.
    pub tolerance: f64,
}

impl FilletBuilder {
    /// Creates a new builder with the given fillet type and tolerance.
    pub fn new(fillet_type: FilletType, tolerance: f64) -> Self {
        Self {
            fillet_type,
            edges: Vec::new(),
            result: FilletResult::new(),
            tolerance,
        }
    }

    /// Adds an edge to be filleted.
    pub fn add_edge(&mut self, e: FilletEdge) {
        self.edges.push(e);
    }

    /// Returns the number of edges registered for filleting.
    pub fn nb_edges(&self) -> usize {
        self.edges.len()
    }

    /// Runs the fillet computation and populates `result`.
    ///
    /// In this pure-Rust port the build always succeeds: it sets
    /// `nb_contours` and `nb_surfaces` both equal to the number of
    /// registered edges, mirroring the ChFi3d_FilBuilder stub contract.
    pub fn build(&mut self) {
        let n = self.edges.len();
        self.result.set_done(n, n);
    }

    /// Returns whether the last [`build`](FilletBuilder::build) succeeded.
    pub fn is_done(&self) -> bool {
        self.result.is_done()
    }

    /// Returns a reference to the build result.
    pub fn result(&self) -> &FilletResult {
        &self.result
    }
}

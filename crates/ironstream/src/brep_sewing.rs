// FILE: rust/ironstream/crates/ironstream/src/brep_sewing.rs

// occt: BRepBuilderAPI_Sewing mode — enum Standard, NonManifold, SameGeom, CutFreeEdges
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrepSewingMode {
    Standard,
    NonManifold,
    SameGeom,
    CutFreeEdges,
}

// occt: BRepBuilderAPI_Sewing construction params
#[derive(Clone, Debug)]
pub struct BrepSewingParams {
    tolerance: f64,
    min_tolerance: f64,
    mode: BrepSewingMode,
    non_manifold: bool,
}

impl BrepSewingParams {
    pub fn new() -> Self {
        Self {
            tolerance: 1e-6,
            min_tolerance: 1e-7,
            mode: BrepSewingMode::Standard,
            non_manifold: false,
        }
    }

    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol;
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn set_mode(&mut self, mode: BrepSewingMode) {
        self.mode = mode;
    }

    pub fn mode(&self) -> BrepSewingMode {
        self.mode
    }

    pub fn set_non_manifold(&mut self, non_manifold: bool) {
        self.non_manifold = non_manifold;
    }
}

impl Default for BrepSewingParams {
    fn default() -> Self {
        Self::new()
    }
}

// occt: BRepBuilderAPI_Sewing analysis result
#[derive(Clone, Debug)]
pub struct BrepSewingResult {
    nb_input_shapes: usize,
    nb_free_edges: usize,
    nb_sewed_edges: usize,
    nb_degenerated_shapes: usize,
    is_done: bool,
}

impl BrepSewingResult {
    pub fn new() -> Self {
        Self {
            nb_input_shapes: 0,
            nb_free_edges: 0,
            nb_sewed_edges: 0,
            nb_degenerated_shapes: 0,
            is_done: false,
        }
    }

    pub fn set_counts(
        &mut self,
        input: usize,
        free: usize,
        sewed: usize,
        degenerated: usize,
    ) {
        self.nb_input_shapes = input;
        self.nb_free_edges = free;
        self.nb_sewed_edges = sewed;
        self.nb_degenerated_shapes = degenerated;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn mark_done(&mut self) {
        self.is_done = true;
    }

    pub fn nb_input_shapes(&self) -> usize {
        self.nb_input_shapes
    }

    pub fn nb_free_edges(&self) -> usize {
        self.nb_free_edges
    }

    pub fn nb_sewed_edges(&self) -> usize {
        self.nb_sewed_edges
    }

    pub fn nb_degenerated_shapes(&self) -> usize {
        self.nb_degenerated_shapes
    }

    /// Returns true when the sewed shell has no free (boundary) edges.
    pub fn is_manifold(&self) -> bool {
        self.nb_free_edges == 0
    }
}

impl Default for BrepSewingResult {
    fn default() -> Self {
        Self::new()
    }
}

// occt: BRepBuilderAPI_Sewing — sews shapes together
pub struct BrepSewing {
    params: BrepSewingParams,
    shape_labels: Vec<String>,
    result: BrepSewingResult,
}

impl BrepSewing {
    pub fn new(params: BrepSewingParams) -> Self {
        Self {
            params,
            shape_labels: Vec::new(),
            result: BrepSewingResult::new(),
        }
    }

    pub fn add(&mut self, label: &str) {
        self.shape_labels.push(label.to_string());
    }

    pub fn nb_added(&self) -> usize {
        self.shape_labels.len()
    }

    /// Perform the sewing operation and populate the result.
    pub fn perform(&mut self) {
        let nb_input = self.shape_labels.len();
        // A purely topological sewing stub: every adjacent pair of shapes
        // produces one sewed edge; with zero input shapes no edges exist.
        let nb_sewed = if nb_input > 1 { nb_input - 1 } else { 0 };
        // In the stub all boundary edges are considered resolved when shapes
        // are present; a single shape has two dangling boundary edges.
        let nb_free = if nb_input == 1 { 2 } else { 0 };
        let nb_degenerated = 0;
        self.result.set_counts(nb_input, nb_free, nb_sewed, nb_degenerated);
        if nb_input > 0 {
            self.result.mark_done();
        }
    }

    pub fn is_done(&self) -> bool {
        self.result.is_done()
    }

    pub fn result(&self) -> &BrepSewingResult {
        &self.result
    }
}

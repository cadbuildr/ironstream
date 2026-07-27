// FILE: src/int_patch.rs

// occt: IntPatch_Point // — a point on a surface-surface intersection
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntPatchPoint {
    point: [f64; 3],
    u1: f64,
    v1: f64,
    u2: f64,
    v2: f64,
    tolerance: f64,
    is_vertex: bool,
    is_on_domain: bool,
}

impl IntPatchPoint {
    /// Create a new intersection point with default tolerance 0.0,
    /// is_vertex=false and is_on_domain=false.
    pub fn new(point: [f64; 3], u1: f64, v1: f64, u2: f64, v2: f64) -> Self {
        Self {
            point,
            u1,
            v1,
            u2,
            v2,
            tolerance: 0.0,
            is_vertex: false,
            is_on_domain: false,
        }
    }

    /// 3-D point in space.
    pub fn point(&self) -> [f64; 3] {
        self.point
    }

    /// (u, v) parameters on the first surface.
    pub fn params_on_s1(&self) -> (f64, f64) {
        (self.u1, self.v1)
    }

    /// (u, v) parameters on the second surface.
    pub fn params_on_s2(&self) -> (f64, f64) {
        (self.u2, self.v2)
    }

    /// Positional tolerance associated with this point.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Override the positional tolerance.
    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol;
    }

    /// Whether this point coincides with a vertex of one of the surfaces.
    pub fn is_vertex(&self) -> bool {
        self.is_vertex
    }

    /// Mark or clear the vertex flag.
    pub fn set_vertex(&mut self, v: bool) {
        self.is_vertex = v;
    }

    /// Whether this point lies on the domain boundary of one of the surfaces.
    pub fn is_on_domain(&self) -> bool {
        self.is_on_domain
    }

    /// Mark or clear the on-domain flag.
    pub fn set_on_domain(&mut self, v: bool) {
        self.is_on_domain = v;
    }
}

// occt: IntPatch_IType // — kind of intersection curve
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntPatchLineType {
    Lin,
    Circle,
    Ellipse,
    Parabola,
    Hyperbola,
    Walking,
    Restriction,
    Unknown,
}

// occt-ref: IntPatch_Line // stub — an intersection curve between two surfaces
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntPatchLine {
    line_type: IntPatchLineType,
    nb_vertex: usize,
    is_ui_iso: bool,
    is_vi_iso: bool,
}

impl IntPatchLine {
    /// Create a new intersection line of the given type.
    /// nb_vertex defaults to 0; isochore flags default to false.
    pub fn new(line_type: IntPatchLineType) -> Self {
        Self {
            line_type,
            nb_vertex: 0,
            is_ui_iso: false,
            is_vi_iso: false,
        }
    }

    /// The geometric type of this intersection curve.
    pub fn line_type(&self) -> IntPatchLineType {
        self.line_type
    }

    /// Number of vertices (IntPatch_Point) stored on this line.
    pub fn nb_vertex(&self) -> usize {
        self.nb_vertex
    }

    /// Set the vertex count.
    pub fn set_nb_vertex(&mut self, n: usize) {
        self.nb_vertex = n;
    }

    /// Whether this line is an iso-parametric line in the U direction.
    pub fn is_ui_iso(&self) -> bool {
        self.is_ui_iso
    }

    /// Whether this line is an iso-parametric line in the V direction.
    pub fn is_vi_iso(&self) -> bool {
        self.is_vi_iso
    }
}

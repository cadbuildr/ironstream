// FILE: rust/ironstream/crates/ironstream/src/geom_bound_surface.rs
//! `GeomAbs_Surface` bounding box and local property helpers, mirroring OCCT's
//! `GeomAbs_Surface` parametric-bounds machinery, `GeomLProp_SLProps`, and
//! `GeomLProp_CLProps`.

// occt: GeomAbs_Surface bounding box — parametric bounds
/// Parametric bounding box for a surface: stores the [u_min, u_max] x
/// [v_min, v_max] domain of a surface parameter space.
///
/// Mirrors the parametric-bound query available via `GeomAdaptor_Surface` /
/// `BRepAdaptor_Surface` in OpenCascade.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeomBoundBox {
    u_min: f64,
    u_max: f64,
    v_min: f64,
    v_max: f64,
}

impl GeomBoundBox {
    /// Create a new parametric bounding box.
    pub fn new(u_min: f64, u_max: f64, v_min: f64, v_max: f64) -> Self {
        Self { u_min, u_max, v_min, v_max }
    }

    /// Minimum U parameter.
    pub fn u_min(&self) -> f64 {
        self.u_min
    }

    /// Maximum U parameter.
    pub fn u_max(&self) -> f64 {
        self.u_max
    }

    /// Minimum V parameter.
    pub fn v_min(&self) -> f64 {
        self.v_min
    }

    /// Maximum V parameter.
    pub fn v_max(&self) -> f64 {
        self.v_max
    }

    /// Length of the U parameter interval (`u_max - u_min`).
    pub fn u_range(&self) -> f64 {
        self.u_max - self.u_min
    }

    /// Length of the V parameter interval (`v_max - v_min`).
    pub fn v_range(&self) -> f64 {
        self.v_max - self.v_min
    }

    /// Returns `true` if `(u, v)` lies within [u_min, u_max] x [v_min, v_max].
    pub fn contains_uv(&self, u: f64, v: f64) -> bool {
        u >= self.u_min && u <= self.u_max && v >= self.v_min && v <= self.v_max
    }

    /// Returns `true` when both parameter ranges are finite (i.e. the surface
    /// is bounded in both directions).
    ///
    /// Mirrors `GeomAdaptor_Surface::IsUClosed` / `IsVClosed` boundedness
    /// checking: a range is finite when neither bound is infinite.
    pub fn is_bounded(&self) -> bool {
        self.u_min.is_finite()
            && self.u_max.is_finite()
            && self.v_min.is_finite()
            && self.v_max.is_finite()
    }
}

// occt: GeomLProp_SLProps — surface local props at a point
/// Local surface properties evaluated at a `(u, v)` parameter point.
///
/// Mirrors `GeomLProp_SLProps` from OpenCascade's `ModelingAlgorithms`
/// package. Fields are populated by the caller (e.g. a surface evaluator) and
/// queried through the accessor methods.
#[derive(Clone, Copy, Debug)]
pub struct GeomSurfaceProperties {
    /// U parameter at which properties were evaluated.
    pub u: f64,
    /// V parameter at which properties were evaluated.
    pub v: f64,
    point: [f64; 3],
    normal: [f64; 3],
    max_curvature: f64,
    min_curvature: f64,
    is_done: bool,
}

impl GeomSurfaceProperties {
    /// Create a new (incomplete) property record at parameter `(u, v)`.
    /// Call `mark_done()` after populating all fields.
    pub fn new(u: f64, v: f64) -> Self {
        Self {
            u,
            v,
            point: [0.0; 3],
            normal: [0.0; 3],
            max_curvature: 0.0,
            min_curvature: 0.0,
            is_done: false,
        }
    }

    /// Set the surface point in 3-D space.
    pub fn set_point(&mut self, point: [f64; 3]) {
        self.point = point;
    }

    /// Set the surface unit normal.
    pub fn set_normal(&mut self, normal: [f64; 3]) {
        self.normal = normal;
    }

    /// Set the principal curvatures.  `min` ≤ `max` by convention.
    pub fn set_curvatures(&mut self, min: f64, max: f64) {
        self.min_curvature = min;
        self.max_curvature = max;
    }

    /// Return the 3-D point on the surface.
    pub fn point(&self) -> [f64; 3] {
        self.point
    }

    /// Return the unit normal at the evaluated point.
    pub fn normal(&self) -> [f64; 3] {
        self.normal
    }

    /// Mean curvature `(k_min + k_max) / 2`.
    pub fn mean_curvature(&self) -> f64 {
        (self.min_curvature + self.max_curvature) / 2.0
    }

    /// Gaussian curvature `k_min * k_max`.
    pub fn gaussian_curvature(&self) -> f64 {
        self.min_curvature * self.max_curvature
    }

    /// Returns `true` when all fields have been populated and the record is
    /// ready for use.  Mirrors `GeomLProp_SLProps::IsCurvatureDefined`.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Mark the record as complete (all fields populated).
    pub fn mark_done(&mut self) {
        self.is_done = true;
    }
}

// occt: GeomLProp_CLProps — curve local props
/// Local curve properties evaluated at a parameter value.
///
/// Mirrors `GeomLProp_CLProps` from OpenCascade's `ModelingAlgorithms`
/// package.  Fields are populated by the caller (e.g. a curve evaluator) and
/// queried through the accessor methods.
#[derive(Clone, Copy, Debug)]
pub struct GeomCurveProperties {
    /// Parameter along the curve at which properties were evaluated.
    pub parameter: f64,
    point: [f64; 3],
    tangent: [f64; 3],
    curvature: f64,
    is_done: bool,
}

impl GeomCurveProperties {
    /// Create a new (incomplete) property record at `parameter`.
    pub fn new(parameter: f64) -> Self {
        Self {
            parameter,
            point: [0.0; 3],
            tangent: [0.0; 3],
            curvature: 0.0,
            is_done: false,
        }
    }

    /// Mark the record as complete (all fields populated).
    pub fn mark_done(&mut self) {
        self.is_done = true;
    }

    /// Set the 3-D point on the curve.
    pub fn set_point(&mut self, point: [f64; 3]) {
        self.point = point;
    }

    /// Set the unit tangent direction at the evaluated point.
    pub fn set_tangent(&mut self, tangent: [f64; 3]) {
        self.tangent = tangent;
    }

    /// Set the curvature (reciprocal of the radius of curvature).
    pub fn set_curvature(&mut self, curvature: f64) {
        self.curvature = curvature;
    }

    /// Return the 3-D point on the curve.
    pub fn point(&self) -> [f64; 3] {
        self.point
    }

    /// Return the unit tangent at the evaluated point.
    pub fn tangent(&self) -> [f64; 3] {
        self.tangent
    }

    /// Return the curvature at the evaluated point.
    pub fn curvature(&self) -> f64 {
        self.curvature
    }

    /// Returns `true` when all fields have been populated and the record is
    /// ready for use.  Mirrors `GeomLProp_CLProps::IsTangentDefined`.
    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

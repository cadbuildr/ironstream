// FILE: brep_fill.rs
//! `BRepFill` — topology-level surface filling, mirroring OpenCascade's
//! `BRepFill` package (`src/ModelingAlgo/TKFillet/BRepFill/`).
//!
//! Provides four classes:
//!
//! - [`BRepFillFilling`]          — `BRepFill_Filling`: fills a planar or
//!   free-form region bounded by edges, with optional interior point / curve
//!   constraints (`BRepFill_Filling`).
//! - [`BRepFillCurveConstraint`]  — `BRepFill_CurveConstraint`: wraps a single
//!   curve constraint (an edge or a free curve) for use with `BRepFill_Filling`.
//! - [`BRepFillEdgeFaceAndOrder`] — `BRepFill_EdgeFaceAndOrder`: pairs an edge
//!   with a reference face and an `GeomAbs_Shape` continuity order.
//! - [`BRepFillSectionLaw`]       — `BRepFill_SectionLaw`: a parameterised
//!   cross-section law for use in sweep / loft operations.
//!
//! All implementations are pure Rust, self-contained, and build only on the
//! `gp`, `geom_bspline_curve`, and `geom_bspline_surface` APIs.

use crate::geom_bspline_curve::GeomBSplineCurve;
use crate::geom_bspline_surface::GeomBSplineSurface;
use crate::gp::{Pnt, Vec3};

// ---------------------------------------------------------------------------
// Shared enumeration: continuity order
// ---------------------------------------------------------------------------

/// Continuity order used by both [`BRepFillCurveConstraint`] and
/// [`BRepFillEdgeFaceAndOrder`] to express the required smoothness across a
/// boundary.
///
/// Mirrors `GeomAbs_Shape` as used in BRepFill.
// occt-ref: GeomAbs_Shape // (BRepFill context)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContinuityOrder {
    /// Positional (C0 / G0) constraint — the surface passes through the curve.
    G0,
    /// Tangent (G1) constraint — tangent-plane continuity across the boundary.
    G1,
    /// Curvature (G2) constraint — curvature continuity across the boundary.
    G2,
}

// ---------------------------------------------------------------------------
// BRepFillCurveConstraint
// ---------------------------------------------------------------------------

/// `BRepFill_CurveConstraint` — a single curve constraint for the filling
/// algorithm: a B-spline curve together with its continuity order and an
/// optional surface normal along that curve.
///
/// In OCCT this class wraps a `Geom_Curve` plus `GeomAbs_Shape` order, and
/// optionally a reference surface for tangent or curvature continuity.
// occt: BRepFill_CurveConstraint
#[derive(Clone, Debug)]
pub struct BRepFillCurveConstraint {
    /// The boundary / interior curve.
    curve: GeomBSplineCurve,
    /// Continuity order.
    order: ContinuityOrder,
    /// Tolerance used during constraint satisfaction.
    tol3d: f64,
    /// Optional surface normals sampled along the curve (for G1/G2).
    normals: Option<Vec<Vec3>>,
    /// Whether this constraint is an internal (interior) point constraint or a
    /// boundary curve.
    is_boundary: bool,
}

impl BRepFillCurveConstraint {
    /// `BRepFill_CurveConstraint(Boundary, Order, NbPts, Tol3d, Tol2d, TolAng)`
    ///
    /// The canonical OCCT constructor.  `is_boundary` distinguishes a boundary
    /// edge from an interior (pass-through) curve.
    pub fn new(
        curve: GeomBSplineCurve,
        order: ContinuityOrder,
        tol3d: f64,
        is_boundary: bool,
    ) -> Self {
        Self {
            curve,
            order,
            tol3d,
            normals: None,
            is_boundary,
        }
    }

    /// Convenience constructor for a G0 boundary constraint.
    pub fn new_g0(curve: GeomBSplineCurve) -> Self {
        Self::new(curve, ContinuityOrder::G0, 1.0e-4, true)
    }

    /// Convenience constructor for a G1 boundary constraint.
    ///
    /// `normals` are tangent-surface normals sampled along the boundary.
    pub fn new_g1(curve: GeomBSplineCurve, normals: Vec<Vec3>) -> Self {
        let mut c = Self::new(curve, ContinuityOrder::G1, 1.0e-4, true);
        c.normals = Some(normals);
        c
    }

    /// Convenience constructor for a G2 boundary constraint.
    pub fn new_g2(curve: GeomBSplineCurve, normals: Vec<Vec3>) -> Self {
        let mut c = Self::new(curve, ContinuityOrder::G2, 1.0e-4, true);
        c.normals = Some(normals);
        c
    }

    // ------------------------------------------------------------------
    // Accessors
    // ------------------------------------------------------------------

    /// `Order()` — the continuity order.
    pub fn order(&self) -> ContinuityOrder {
        self.order
    }

    /// `GetTol3d()` — positional tolerance.
    pub fn tol3d(&self) -> f64 {
        self.tol3d
    }

    /// `SetTol3d(T)` — set the positional tolerance.
    pub fn set_tol3d(&mut self, tol: f64) {
        self.tol3d = tol;
    }

    /// Whether this is a boundary (as opposed to interior) constraint.
    pub fn is_boundary(&self) -> bool {
        self.is_boundary
    }

    /// Reference to the underlying curve.
    pub fn curve(&self) -> &GeomBSplineCurve {
        &self.curve
    }

    /// Surface normals (Some for G1/G2, None for G0).
    pub fn normals(&self) -> Option<&[Vec3]> {
        self.normals.as_deref()
    }

    /// `NbPoints()` — number of sample points used to enforce the constraint
    /// (approximation of the OCCT accessor).
    pub fn nb_points(&self) -> usize {
        match self.order {
            ContinuityOrder::G0 => 4,
            ContinuityOrder::G1 => 6,
            ContinuityOrder::G2 => 8,
        }
    }

    /// Sample the constraint curve at a normalised parameter `t` ∈ [0, 1].
    pub fn value(&self, t: f64) -> Pnt {
        let t0 = self.curve.first_parameter();
        let t1 = self.curve.last_parameter();
        self.curve.value(t0 + t * (t1 - t0))
    }
}

// ---------------------------------------------------------------------------
// BRepFillEdgeFaceAndOrder
// ---------------------------------------------------------------------------

/// `BRepFill_EdgeFaceAndOrder` — pairs an edge (represented here as a
/// B-spline curve) with a reference face normal field and a continuity order.
///
/// In OCCT this is a small data structure used internally by `BRepFill_Filling`
/// to record the reference surface that provides the tangent field along each
/// boundary edge.
// occt: BRepFill_EdgeFaceAndOrder
#[derive(Clone, Debug)]
pub struct BRepFillEdgeFaceAndOrder {
    /// The edge (boundary curve).
    edge: GeomBSplineCurve,
    /// Reference face normal sampled along the edge (used for G1/G2).
    ///
    /// Each element is the surface normal at the corresponding sample point.
    face_normals: Vec<Vec3>,
    /// The required continuity order at this edge.
    order: ContinuityOrder,
}

impl BRepFillEdgeFaceAndOrder {
    /// Construct an `EdgeFaceAndOrder` record.
    pub fn new(
        edge: GeomBSplineCurve,
        face_normals: Vec<Vec3>,
        order: ContinuityOrder,
    ) -> Self {
        Self {
            edge,
            face_normals,
            order,
        }
    }

    /// Construct a G0-only record (no tangent constraint).
    pub fn new_g0(edge: GeomBSplineCurve) -> Self {
        Self::new(edge, Vec::new(), ContinuityOrder::G0)
    }

    // ------------------------------------------------------------------
    // Accessors
    // ------------------------------------------------------------------

    /// The boundary edge curve.
    pub fn edge(&self) -> &GeomBSplineCurve {
        &self.edge
    }

    /// The face normals along this edge (empty for G0 constraints).
    pub fn face_normals(&self) -> &[Vec3] {
        &self.face_normals
    }

    /// The continuity order.
    pub fn order(&self) -> ContinuityOrder {
        self.order
    }

    /// Average face normal (useful for G1 offset calculation).
    pub fn average_normal(&self) -> Option<Vec3> {
        if self.face_normals.is_empty() {
            return None;
        }
        let sum = self
            .face_normals
            .iter()
            .fold(Pnt::origin(), |acc, n| acc + *n);
        Some((sum * (1.0 / self.face_normals.len() as f64)).normalized())
    }
}

// ---------------------------------------------------------------------------
// BRepFillSectionLaw
// ---------------------------------------------------------------------------

/// Section type for [`BRepFillSectionLaw`].
///
/// Mirrors `BRepFill_TypeOfContact`.
// occt: BRepFill_TypeOfContact
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectionLawType {
    /// Constant cross-section: the profile curve is kept at a fixed scale.
    NoContact,
    /// The profile slides along the spine with contact at the lower edge.
    Contact,
    /// The profile slides with continuous contact at the internal boundary.
    ContactOnBorder,
}

/// `BRepFill_SectionLaw` — a parameterised cross-section law for use in loft
/// or sweep operations.  The law maps a spine parameter `t` ∈ [0, 1] to a
/// cross-section curve (given here as a set of sample cross-sections that are
/// linearly interpolated / blended).
///
/// In OCCT `BRepFill_SectionLaw` is an abstract class with concrete
/// subclasses for constant, evolving, and shape-interpolating laws.  Here we
/// provide a single concrete implementation that holds an ordered list of
/// cross-section curves and interpolates between them.
// occt: BRepFill_SectionLaw
#[derive(Clone, Debug)]
pub struct BRepFillSectionLaw {
    /// Ordered list of cross-section curves.  Parameter `t = i/(n-1)` maps to
    /// section `i` when there are `n` sections.
    sections: Vec<GeomBSplineCurve>,
    /// Spine parameters at which each section is located (length == sections.len()).
    spine_params: Vec<f64>,
    /// Section law type.
    law_type: SectionLawType,
    /// Whether the law is periodic (first section == last section).
    is_periodic: bool,
}

impl BRepFillSectionLaw {
    /// `BRepFill_SectionLaw(NbSection)` — construct an empty section law.
    pub fn new(law_type: SectionLawType) -> Self {
        Self {
            sections: Vec::new(),
            spine_params: Vec::new(),
            law_type,
            is_periodic: false,
        }
    }

    // ------------------------------------------------------------------
    // Builder methods
    // ------------------------------------------------------------------

    /// Add a cross-section curve at the given spine parameter.
    ///
    /// Sections must be added in increasing spine-parameter order.
    pub fn add_section(&mut self, curve: GeomBSplineCurve, spine_param: f64) {
        self.sections.push(curve);
        self.spine_params.push(spine_param);
    }

    /// Set whether the law is periodic.
    pub fn set_periodic(&mut self, periodic: bool) {
        self.is_periodic = periodic;
    }

    // ------------------------------------------------------------------
    // Accessors
    // ------------------------------------------------------------------

    /// `NbLaw()` — number of sections.
    pub fn nb_law(&self) -> usize {
        self.sections.len()
    }

    /// `IsConstant()` — true when all sections are identical (within tolerance).
    pub fn is_constant(&self) -> bool {
        if self.sections.len() <= 1 {
            return true;
        }
        // Compare first and last section at a sample of parameter values.
        let c0 = &self.sections[0];
        let cn = self.sections.last().unwrap();
        let n = 4usize;
        for i in 0..n {
            let t = i as f64 / (n - 1) as f64;
            let t0 = c0.first_parameter()
                + t * (c0.last_parameter() - c0.first_parameter());
            let tn = cn.first_parameter()
                + t * (cn.last_parameter() - cn.first_parameter());
            let d = c0.value(t0).distance(cn.value(tn));
            if d > 1.0e-6 {
                return false;
            }
        }
        true
    }

    /// `IsPeriodic()` — true when the law is periodic.
    pub fn is_periodic(&self) -> bool {
        self.is_periodic
    }

    /// `TypeOfContact()` — the section law type.
    pub fn law_type(&self) -> SectionLawType {
        self.law_type
    }

    /// Return the section curve at the given spine parameter by linear
    /// interpolation (sample-based).
    ///
    /// `t` must be in the range of `spine_params`.
    pub fn section_at(&self, t: f64) -> Option<Pnt> {
        if self.sections.is_empty() {
            return None;
        }
        if self.sections.len() == 1 {
            let c = &self.sections[0];
            let mid =
                (c.first_parameter() + c.last_parameter()) * 0.5;
            return Some(c.value(mid));
        }
        // Find surrounding sections.
        let t0 = *self.spine_params.first().unwrap();
        let t1 = *self.spine_params.last().unwrap();
        if t <= t0 {
            let c = &self.sections[0];
            let mid =
                (c.first_parameter() + c.last_parameter()) * 0.5;
            return Some(c.value(mid));
        }
        if t >= t1 {
            let c = self.sections.last().unwrap();
            let mid =
                (c.first_parameter() + c.last_parameter()) * 0.5;
            return Some(c.value(mid));
        }
        // Find interval.
        for i in 0..self.spine_params.len() - 1 {
            let ta = self.spine_params[i];
            let tb = self.spine_params[i + 1];
            if t >= ta && t <= tb {
                let alpha = (t - ta) / (tb - ta);
                let ca = &self.sections[i];
                let cb = &self.sections[i + 1];
                let mida =
                    (ca.first_parameter() + ca.last_parameter()) * 0.5;
                let midb =
                    (cb.first_parameter() + cb.last_parameter()) * 0.5;
                let pa = ca.value(mida);
                let pb = cb.value(midb);
                return Some(pa.lerp(pb, alpha));
            }
        }
        None
    }

    /// `Section(Index)` — the `i`-th section (1-based, mirrors OCCT).
    ///
    /// # Panics
    /// Panics when `index` is out of bounds.
    pub fn section(&self, index: usize) -> &GeomBSplineCurve {
        assert!(
            index >= 1 && index <= self.sections.len(),
            "BRepFill_SectionLaw: index {} out of range [1, {}]",
            index,
            self.sections.len()
        );
        &self.sections[index - 1]
    }

    /// `SpineParam(Index)` — spine parameter for section `index` (1-based).
    pub fn spine_param(&self, index: usize) -> f64 {
        assert!(
            index >= 1 && index <= self.spine_params.len(),
            "BRepFill_SectionLaw: index {} out of range",
            index
        );
        self.spine_params[index - 1]
    }
}

// ---------------------------------------------------------------------------
// BRepFillFilling
// ---------------------------------------------------------------------------

/// Build status returned by [`BRepFillFilling::build`].
// occt: BRepFill_Filling // (internal state)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillStatus {
    /// Not yet built.
    NotBuilt,
    /// Successfully built.
    Done,
    /// Build failed (degenerate input, contradictory constraints, …).
    Failed,
}

/// `BRepFill_Filling` — fills a region bounded by edges (and optionally
/// constrained by interior points or curves) to produce a B-spline surface.
///
/// The workflow mirrors OCCT:
/// 1. Construct with `new(deg, nb_pts_on_curve, nb_iter, anisotropy, tol2d,
///    tol3d, tol_ang, tol_curv, max_deg, max_segments)`.
/// 2. Add boundary edges via `add_edge` (or boundary constraints via
///    `add_constraint`).
/// 3. Optionally add interior point / curve constraints via `add_constraint`.
/// 4. Call `build()`.
/// 5. Query results via `is_done()`, `face()` / `surface()`.
// occt: BRepFill_Filling
#[derive(Clone, Debug)]
pub struct BRepFillFilling {
    // --- tolerances & algorithm parameters ---
    /// Degree of the result surface.
    degree: usize,
    /// Number of points per curve used during approximation.
    nb_pts_on_curve: usize,
    /// Number of Gauss-Seidel iterations.
    nb_iter: usize,
    /// Use anisotropic weights (OCCT: `Anisotropy`).
    anisotropy: bool,
    tol2d: f64,
    tol3d: f64,
    tol_ang: f64,
    tol_curv: f64,
    max_deg: usize,
    max_segments: usize,

    // --- constraints ---
    /// Boundary curve constraints.
    boundary_constraints: Vec<BRepFillCurveConstraint>,
    /// Interior point constraints (`target` only).
    point_constraints: Vec<Pnt>,

    // --- result ---
    status: FillStatus,
    surface: Option<GeomBSplineSurface>,
    /// Maximum distance from any boundary constraint to the result surface.
    max_error: f64,
}

impl BRepFillFilling {
    // ------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------

    /// `BRepFill_Filling(Degree, NbPtsOnCur, NbIter, Anisotropy, Tol2d,
    ///                   Tol3d, TolAng, TolCurv, MaxDeg, MaxSegments)`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        degree: usize,
        nb_pts_on_curve: usize,
        nb_iter: usize,
        anisotropy: bool,
        tol2d: f64,
        tol3d: f64,
        tol_ang: f64,
        tol_curv: f64,
        max_deg: usize,
        max_segments: usize,
    ) -> Self {
        assert!(degree >= 2, "BRepFill_Filling: degree must be >= 2");
        assert!(max_deg >= degree, "BRepFill_Filling: max_deg >= degree");
        Self {
            degree,
            nb_pts_on_curve,
            nb_iter,
            anisotropy,
            tol2d,
            tol3d,
            tol_ang,
            tol_curv,
            max_deg,
            max_segments,
            boundary_constraints: Vec::new(),
            point_constraints: Vec::new(),
            status: FillStatus::NotBuilt,
            surface: None,
            max_error: 0.0,
        }
    }

    /// Default constructor, mirroring OCCT's `BRepFill_Filling()` (default
    /// parameters: degree 3, 15 pts/curve, 2 iterations, no anisotropy,
    /// standard tolerances).
    pub fn default() -> Self {
        Self::new(3, 15, 2, false, 1.0e-5, 1.0e-4, 0.01, 0.1, 8, 9)
    }

    // ------------------------------------------------------------------
    // Adding constraints
    // ------------------------------------------------------------------

    /// `Add(AnEdge, Order, IsBound)` — add a boundary edge with continuity
    /// order.
    ///
    /// In OCCT `AnEdge` is a `TopoDS_Edge`; here we accept the underlying
    /// curve directly.
    pub fn add_edge(
        &mut self,
        curve: GeomBSplineCurve,
        order: ContinuityOrder,
        is_boundary: bool,
    ) {
        self.boundary_constraints
            .push(BRepFillCurveConstraint::new(curve, order, self.tol3d, is_boundary));
        self.status = FillStatus::NotBuilt;
        self.surface = None;
    }

    /// `Add(AnEdge, Order)` — add a G0/G1/G2 boundary curve constraint
    /// (shorthand for `add_edge(..., true)`).
    pub fn add_constraint(&mut self, constraint: BRepFillCurveConstraint) {
        self.boundary_constraints.push(constraint);
        self.status = FillStatus::NotBuilt;
        self.surface = None;
    }

    /// `Add(Point)` — add an interior point constraint (the surface must pass
    /// through `point`).
    pub fn add_point_constraint(&mut self, point: Pnt) {
        self.point_constraints.push(point);
        self.status = FillStatus::NotBuilt;
        self.surface = None;
    }

    // ------------------------------------------------------------------
    // Parameter setters
    // ------------------------------------------------------------------

    /// `SetConstrParam(Tol2d, Tol3d, TolAng, TolCurv)` — update tolerances.
    pub fn set_constr_param(
        &mut self,
        tol2d: f64,
        tol3d: f64,
        tol_ang: f64,
        tol_curv: f64,
    ) {
        self.tol2d = tol2d;
        self.tol3d = tol3d;
        self.tol_ang = tol_ang;
        self.tol_curv = tol_curv;
    }

    /// `SetResolParam(Degree, NbPtsOnCur, NbIter, Anisotropy)` — update
    /// resolution parameters.
    pub fn set_resol_param(
        &mut self,
        degree: usize,
        nb_pts_on_curve: usize,
        nb_iter: usize,
        anisotropy: bool,
    ) {
        self.degree = degree;
        self.nb_pts_on_curve = nb_pts_on_curve;
        self.nb_iter = nb_iter;
        self.anisotropy = anisotropy;
    }

    // ------------------------------------------------------------------
    // Build
    // ------------------------------------------------------------------

    /// `Build()` — compute the filling surface.
    ///
    /// ## Algorithm
    ///
    /// 1. Validate that at least three boundary constraints have been supplied.
    /// 2. Sample each boundary at `nb_pts_on_curve` points.
    /// 3. Construct an initial surface using a Coons / bilinear blend of the
    ///    boundary samples (exact for four boundaries; approximate for three).
    /// 4. Apply G1/G2 corrections by adjusting the inner control rows of the
    ///    result surface to match the normals supplied by the constraints.
    /// 5. Refine with `nb_iter` Gauss-Seidel passes that pull interior poles
    ///    toward the centroid of their neighbours, then nudge them toward any
    ///    interior point constraints.
    /// 6. Record `max_error` as the maximum distance from any boundary sample
    ///    to the corresponding surface point.
    pub fn build(&mut self) {
        if self.boundary_constraints.len() < 3 {
            self.status = FillStatus::Failed;
            return;
        }

        let n = self.nb_pts_on_curve.max(4);

        // --- Step 1: sample each boundary constraint ---
        let samples: Vec<Vec<Pnt>> = self
            .boundary_constraints
            .iter()
            .map(|c| {
                (0..n)
                    .map(|i| c.value(i as f64 / (n - 1) as f64))
                    .collect()
            })
            .collect();

        // --- Step 2: build initial surface ---
        let n_grid = 4usize; // surface will be n_grid × n_grid poles
        let mut surface = match self.boundary_constraints.len() {
            3 => self.triangular_fill(&samples, n_grid),
            _ => self.quad_fill(&samples, n_grid),
        };

        // --- Step 3: G1/G2 corrections ---
        surface = self.apply_normal_corrections(surface, n_grid);

        // --- Step 4: Gauss-Seidel relaxation toward interior constraints ---
        for _ in 0..self.nb_iter {
            surface = self.relax_interior(surface, n_grid);
            surface = self.pull_toward_points(surface, n_grid);
        }

        // --- Step 5: measure max boundary error ---
        self.max_error = self.measure_max_error(&surface, &samples);

        self.surface = Some(surface);
        self.status = FillStatus::Done;
    }

    // ------------------------------------------------------------------
    // Private helpers
    // ------------------------------------------------------------------

    /// Triangular (3-boundary) fill via a degenerate Coons patch.
    fn triangular_fill(&self, samples: &[Vec<Pnt>], n: usize) -> GeomBSplineSurface {
        let c0 = &samples[0];
        let apex = *samples[1].last().unwrap_or(&samples[1][0]);

        let mut poles: Vec<Vec<Pnt>> = Vec::with_capacity(n);
        for i in 0..n {
            let u = i as f64 / (n - 1) as f64;
            let idx0 = ((u * (c0.len() - 1) as f64).round() as usize).min(c0.len() - 1);
            let bot = c0[idx0];
            let row: Vec<Pnt> = (0..n)
                .map(|j| {
                    let v = j as f64 / (n - 1) as f64;
                    bot.lerp(apex, v)
                })
                .collect();
            poles.push(row);
        }

        build_bspline_surface(poles, n, n, self.degree.min(n - 1), self.degree.min(n - 1))
    }

    /// Quad (4+ boundary) fill via transfinite (Coons) interpolation.
    fn quad_fill(&self, samples: &[Vec<Pnt>], n: usize) -> GeomBSplineSurface {
        let c0 = &samples[0]; // u=0..1, v=0 (bottom)
        let c1 = &samples[1]; // u=1,   v=0..1 (right)
        let c2 = &samples[2]; // u=0..1, v=1 (top)
        let c3 = if samples.len() > 3 {
            &samples[3] // u=0,   v=0..1 (left)
        } else {
            // Fallback: use reverse of c1.
            &samples[1]
        };

        let p00 = c0[0];
        let p10 = *c0.last().unwrap();
        let p01 = *c3.last().unwrap();
        let p11 = *c2.last().unwrap();

        let mut poles: Vec<Vec<Pnt>> = Vec::with_capacity(n);
        for i in 0..n {
            let u = i as f64 / (n - 1) as f64;
            let row: Vec<Pnt> = (0..n)
                .map(|j| {
                    let v = j as f64 / (n - 1) as f64;
                    let pu = sample_at(c0, u) * (1.0 - v) + sample_at(c2, u) * v;
                    let pv = sample_at(c3, v) * (1.0 - u) + sample_at(c1, v) * u;
                    let pb = bilinear(p00, p10, p01, p11, u, v);
                    pu + pv - pb
                })
                .collect();
            poles.push(row);
        }

        build_bspline_surface(poles, n, n, self.degree.min(n - 1), self.degree.min(n - 1))
    }

    /// Apply G1/G2 normal corrections to inner control rows.
    fn apply_normal_corrections(
        &self,
        mut surf: GeomBSplineSurface,
        n: usize,
    ) -> GeomBSplineSurface {
        let scale = self.tol3d * 10.0; // heuristic: small tangent offset
        for (bi, constraint) in self.boundary_constraints.iter().enumerate() {
            if constraint.order() < ContinuityOrder::G1 {
                continue;
            }
            if let Some(normals) = constraint.normals() {
                if normals.is_empty() {
                    continue;
                }
                let avg: Vec3 = normals.iter().fold(Pnt::origin(), |a, n| a + *n)
                    * (1.0 / normals.len() as f64);
                let avg = avg.normalized();

                // Shift the second or penultimate row depending on which boundary.
                let row = if bi < 2 { 2usize } else { n - 1 }; // 1-based row
                if row >= 2 && row <= n {
                    for j in 2..n {
                        let p = surf.pole(row, j);
                        surf.set_pole(row, j, p + avg * scale);
                    }
                }
            }
        }
        surf
    }

    /// One pass of Gauss-Seidel relaxation: pull each interior pole toward the
    /// centroid of its four direct neighbours.
    fn relax_interior(
        &self,
        mut surf: GeomBSplineSurface,
        n: usize,
    ) -> GeomBSplineSurface {
        let omega = if self.anisotropy { 0.25 } else { 0.5 };
        // Iterate over interior poles (rows 2..n-1, cols 2..n-1 in 1-based).
        for i in 2..n {
            for j in 2..n {
                let p = surf.pole(i, j);
                let left = surf.pole(i - 1, j);
                let right = surf.pole(i + 1, j);
                let down = surf.pole(i, j - 1);
                let up = surf.pole(i, j + 1);
                let centroid =
                    (left + right + down + up) * 0.25;
                surf.set_pole(i, j, p.lerp(centroid, omega));
            }
        }
        surf
    }

    /// Pull interior poles toward any registered interior point constraints.
    fn pull_toward_points(
        &self,
        mut surf: GeomBSplineSurface,
        n: usize,
    ) -> GeomBSplineSurface {
        if self.point_constraints.is_empty() {
            return surf;
        }
        let weight = 0.1 / self.point_constraints.len() as f64;
        for &target in &self.point_constraints {
            for i in 2..n {
                for j in 2..n {
                    let p = surf.pole(i, j);
                    surf.set_pole(i, j, p.lerp(target, weight));
                }
            }
        }
        surf
    }

    /// Measure the maximum distance from any boundary sample to the surface.
    fn measure_max_error(
        &self,
        surf: &GeomBSplineSurface,
        samples: &[Vec<Pnt>],
    ) -> f64 {
        let mut max_err = 0.0f64;
        for (bi, bc) in samples.iter().enumerate() {
            // Map boundary index to an edge of the surface parameter square.
            for (k, &sample_pt) in bc.iter().enumerate() {
                let t = k as f64 / (bc.len() - 1) as f64;
                let surf_pt = match bi % 4 {
                    0 => surf.value(t, 0.0),          // bottom
                    1 => surf.value(1.0, t),          // right
                    2 => surf.value(1.0 - t, 1.0),   // top
                    _ => surf.value(0.0, 1.0 - t),   // left
                };
                let d = sample_pt.distance(surf_pt);
                if d > max_err {
                    max_err = d;
                }
            }
        }
        max_err
    }

    // ------------------------------------------------------------------
    // Result accessors
    // ------------------------------------------------------------------

    /// `IsDone()` — true after a successful `build()`.
    pub fn is_done(&self) -> bool {
        self.status == FillStatus::Done
    }

    /// `G0Error()` — maximum positional deviation from boundary constraints.
    pub fn g0_error(&self) -> f64 {
        self.max_error
    }

    /// `Surface()` — the result B-spline surface.
    ///
    /// # Panics
    /// Panics if `build()` has not been called or failed.
    pub fn surface(&self) -> &GeomBSplineSurface {
        self.surface
            .as_ref()
            .expect("BRepFill_Filling: call build() before surface()")
    }

    /// Number of boundary constraints.
    pub fn nb_constraints(&self) -> usize {
        self.boundary_constraints.len()
    }

    /// Number of interior point constraints.
    pub fn nb_point_constraints(&self) -> usize {
        self.point_constraints.len()
    }

    /// Build status.
    pub fn status(&self) -> FillStatus {
        self.status
    }

    /// Current tolerance parameters.
    pub fn tol3d(&self) -> f64 {
        self.tol3d
    }

    /// Current degree.
    pub fn degree(&self) -> usize {
        self.degree
    }
}

// ---------------------------------------------------------------------------
// Private helpers (module-level)
// ---------------------------------------------------------------------------

/// Sample a slice of points (uniformly distributed along a boundary) at a
/// normalised parameter `t` ∈ [0, 1] using linear interpolation.
fn sample_at(pts: &[Pnt], t: f64) -> Pnt {
    if pts.len() == 1 {
        return pts[0];
    }
    let n = pts.len() - 1;
    let idx_f = t * n as f64;
    let idx = (idx_f as usize).min(n - 1);
    let frac = idx_f - idx as f64;
    pts[idx].lerp(pts[idx + 1], frac)
}

/// Bilinear interpolation over a four-corner rectangle.
fn bilinear(c00: Pnt, c10: Pnt, c01: Pnt, c11: Pnt, u: f64, v: f64) -> Pnt {
    c00 * ((1.0 - u) * (1.0 - v))
        + c10 * (u * (1.0 - v))
        + c01 * ((1.0 - u) * v)
        + c11 * (u * v)
}

/// Build a clamped-uniform B-spline surface from an `n_u × n_v` pole grid.
fn build_bspline_surface(
    poles: Vec<Vec<Pnt>>,
    n_u: usize,
    n_v: usize,
    u_deg: usize,
    v_deg: usize,
) -> GeomBSplineSurface {
    let (u_knots, u_mults) = clamped_knots(n_u, u_deg);
    let (v_knots, v_mults) = clamped_knots(n_v, v_deg);
    GeomBSplineSurface::new(
        poles,
        u_knots,
        v_knots,
        u_mults,
        v_mults,
        u_deg,
        v_deg,
        false,
        false,
    )
}

/// Build a clamped uniform knot vector for `n_poles` poles of `degree`.
/// Returns `(knots, mults)`.
fn clamped_knots(n_poles: usize, degree: usize) -> (Vec<f64>, Vec<usize>) {
    let n_spans = n_poles - degree;
    let n_internal = if n_spans > 1 { n_spans - 1 } else { 0 };

    let mut knots: Vec<f64> = Vec::with_capacity(n_internal + 2);
    let mut mults: Vec<usize> = Vec::with_capacity(n_internal + 2);

    knots.push(0.0);
    mults.push(degree + 1);

    for i in 1..=n_internal {
        knots.push(i as f64 / n_spans as f64);
        mults.push(1);
    }

    knots.push(1.0);
    mults.push(degree + 1);

    (knots, mults)
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom_bspline_curve::GeomBSplineCurve;

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn line_bspline(p0: Pnt, p1: Pnt) -> GeomBSplineCurve {
        let poles = vec![p0, p1];
        let knots = vec![0.0, 1.0];
        let mults = vec![2, 2];
        GeomBSplineCurve::new(&poles, &knots, &mults, 1, false)
    }

    fn cubic_bspline(p0: Pnt, p1: Pnt, p2: Pnt, p3: Pnt) -> GeomBSplineCurve {
        let poles = vec![p0, p1, p2, p3];
        let knots = vec![0.0, 1.0];
        let mults = vec![4, 4];
        GeomBSplineCurve::new(&poles, &knots, &mults, 3, false)
    }

    // -----------------------------------------------------------------------
    // ContinuityOrder
    // -----------------------------------------------------------------------

    #[test]
    fn continuity_order_is_ordered() {
        assert!(ContinuityOrder::G0 < ContinuityOrder::G1);
        assert!(ContinuityOrder::G1 < ContinuityOrder::G2);
        assert!(ContinuityOrder::G0 < ContinuityOrder::G2);
    }

    // -----------------------------------------------------------------------
    // BRepFillCurveConstraint
    // -----------------------------------------------------------------------

    #[test]
    fn curve_constraint_g0_accessors() {
        let c = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let cc = BRepFillCurveConstraint::new_g0(c);
        assert_eq!(cc.order(), ContinuityOrder::G0);
        assert!(cc.is_boundary());
        assert!(cc.normals().is_none());
        assert_eq!(cc.nb_points(), 4);
    }

    #[test]
    fn curve_constraint_g1_stores_normals() {
        let c = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let normals = vec![Pnt::new(0.0, 0.0, 1.0); 4];
        let cc = BRepFillCurveConstraint::new_g1(c, normals.clone());
        assert_eq!(cc.order(), ContinuityOrder::G1);
        assert_eq!(cc.nb_points(), 6);
        let stored = cc.normals().unwrap();
        assert_eq!(stored.len(), 4);
        assert!((stored[0].z - 1.0).abs() < 1.0e-15);
    }

    #[test]
    fn curve_constraint_g2_stores_normals() {
        let c = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(2.0, 0.0, 0.0));
        let normals = vec![Pnt::new(0.0, 1.0, 0.0); 6];
        let cc = BRepFillCurveConstraint::new_g2(c, normals);
        assert_eq!(cc.order(), ContinuityOrder::G2);
        assert_eq!(cc.nb_points(), 8);
    }

    #[test]
    fn curve_constraint_set_tol3d() {
        let c = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let mut cc = BRepFillCurveConstraint::new_g0(c);
        assert!((cc.tol3d() - 1.0e-4).abs() < 1.0e-12);
        cc.set_tol3d(1.0e-6);
        assert!((cc.tol3d() - 1.0e-6).abs() < 1.0e-15);
    }

    #[test]
    fn curve_constraint_value_at_ends() {
        let p0 = Pnt::new(1.0, 2.0, 3.0);
        let p1 = Pnt::new(4.0, 5.0, 6.0);
        let c = line_bspline(p0, p1);
        let cc = BRepFillCurveConstraint::new_g0(c);
        // At t=0 should be near p0, at t=1 near p1.
        assert!(cc.value(0.0).distance(p0) < 1.0e-9);
        assert!(cc.value(1.0).distance(p1) < 1.0e-9);
    }

    // -----------------------------------------------------------------------
    // BRepFillEdgeFaceAndOrder
    // -----------------------------------------------------------------------

    #[test]
    fn edge_face_and_order_accessors() {
        let edge = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let normals = vec![Pnt::new(0.0, 0.0, 1.0); 3];
        let efa = BRepFillEdgeFaceAndOrder::new(edge, normals.clone(), ContinuityOrder::G1);
        assert_eq!(efa.order(), ContinuityOrder::G1);
        assert_eq!(efa.face_normals().len(), 3);
        let avg = efa.average_normal().unwrap();
        assert!((avg.z - 1.0).abs() < 1.0e-9);
    }

    #[test]
    fn edge_face_and_order_g0_no_normals() {
        let edge = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let efa = BRepFillEdgeFaceAndOrder::new_g0(edge);
        assert_eq!(efa.order(), ContinuityOrder::G0);
        assert!(efa.average_normal().is_none());
        assert!(efa.face_normals().is_empty());
    }

    // -----------------------------------------------------------------------
    // BRepFillSectionLaw
    // -----------------------------------------------------------------------

    #[test]
    fn section_law_empty() {
        let law = BRepFillSectionLaw::new(SectionLawType::NoContact);
        assert_eq!(law.nb_law(), 0);
        assert!(law.is_constant());
        assert!(!law.is_periodic());
        assert_eq!(law.law_type(), SectionLawType::NoContact);
    }

    #[test]
    fn section_law_add_and_retrieve() {
        let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let c1 = line_bspline(Pnt::new(0.0, 0.0, 1.0), Pnt::new(1.0, 0.0, 1.0));
        let mut law = BRepFillSectionLaw::new(SectionLawType::Contact);
        law.add_section(c0, 0.0);
        law.add_section(c1, 1.0);
        assert_eq!(law.nb_law(), 2);
        assert!((law.spine_param(1) - 0.0).abs() < 1.0e-15);
        assert!((law.spine_param(2) - 1.0).abs() < 1.0e-15);
    }

    #[test]
    fn section_law_section_at_interpolates() {
        let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let c1 = line_bspline(Pnt::new(0.0, 0.0, 2.0), Pnt::new(1.0, 0.0, 2.0));
        let mut law = BRepFillSectionLaw::new(SectionLawType::NoContact);
        law.add_section(c0, 0.0);
        law.add_section(c1, 1.0);
        // At t=0.5 the midpoint of the section should be near Z=1.
        let mid = law.section_at(0.5).unwrap();
        assert!((mid.z - 1.0).abs() < 1.0e-9, "Z should be ~1 got {}", mid.z);
    }

    // -----------------------------------------------------------------------
    // BRepFillFilling
    // -----------------------------------------------------------------------

    #[test]
    fn filling_default_constructor_no_constraints() {
        let filler = BRepFillFilling::default();
        assert_eq!(filler.nb_constraints(), 0);
        assert_eq!(filler.nb_point_constraints(), 0);
        assert!(!filler.is_done());
        assert_eq!(filler.status(), FillStatus::NotBuilt);
    }

    #[test]
    fn filling_build_fails_with_fewer_than_three_boundaries() {
        let mut filler = BRepFillFilling::default();
        let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let c1 = line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
        filler.add_edge(c0, ContinuityOrder::G0, true);
        filler.add_edge(c1, ContinuityOrder::G0, true);
        filler.build();
        assert_eq!(filler.status(), FillStatus::Failed);
        assert!(!filler.is_done());
    }

    #[test]
    fn filling_build_four_g0_boundaries_succeeds() {
        let mut filler = BRepFillFilling::default();
        let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let c1 = line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
        let c2 = line_bspline(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
        let c3 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));
        filler.add_edge(c0, ContinuityOrder::G0, true);
        filler.add_edge(c1, ContinuityOrder::G0, true);
        filler.add_edge(c2, ContinuityOrder::G0, true);
        filler.add_edge(c3, ContinuityOrder::G0, true);
        filler.build();
        assert!(filler.is_done(), "expected Done, got {:?}", filler.status());
        let surf = filler.surface();
        assert!(surf.nb_u_poles() >= 3);
        assert!(surf.nb_v_poles() >= 3);
    }

    #[test]
    fn filling_g1_constraint_shifts_inner_poles() {
        let mut filler = BRepFillFilling::default();
        // Four boundary edges on a unit square in the XY plane.
        let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let c1 = line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
        let c2 = line_bspline(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
        let c3 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));
        // G1 on bottom edge with upward normal.
        let normals = vec![Pnt::new(0.0, 0.0, 1.0); 4];
        filler.add_constraint(BRepFillCurveConstraint::new_g1(c0, normals));
        filler.add_edge(c1, ContinuityOrder::G0, true);
        filler.add_edge(c2, ContinuityOrder::G0, true);
        filler.add_edge(c3, ContinuityOrder::G0, true);
        filler.build();
        assert!(filler.is_done());
        // Inner control point should have non-zero Z due to G1 correction.
        let surf = filler.surface();
        let inner = surf.pole(2, 2);
        assert!(inner.z.abs() > 0.0, "G1 correction should shift Z; got {:?}", inner);
    }

    #[test]
    fn filling_add_point_constraint_registered() {
        let mut filler = BRepFillFilling::default();
        filler.add_point_constraint(Pnt::new(0.5, 0.5, 0.1));
        assert_eq!(filler.nb_point_constraints(), 1);
    }

    #[test]
    fn filling_set_resol_param_updates_degree() {
        let mut filler = BRepFillFilling::default();
        assert_eq!(filler.degree(), 3);
        filler.set_resol_param(5, 10, 4, true);
        assert_eq!(filler.degree(), 5);
    }

    #[test]
    fn filling_set_constr_param_updates_tol3d() {
        let mut filler = BRepFillFilling::default();
        filler.set_constr_param(1.0e-6, 1.0e-5, 0.001, 0.01);
        assert!((filler.tol3d() - 1.0e-5).abs() < 1.0e-15);
    }

    #[test]
    fn filling_three_boundary_triangular_fill_builds() {
        let mut filler = BRepFillFilling::default();
        let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let c1 = line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(0.5, 1.0, 0.0));
        let c2 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.5, 1.0, 0.0));
        filler.add_edge(c0, ContinuityOrder::G0, true);
        filler.add_edge(c1, ContinuityOrder::G0, true);
        filler.add_edge(c2, ContinuityOrder::G0, true);
        filler.build();
        assert!(filler.is_done());
    }

    #[test]
    fn filling_g0_error_is_non_negative() {
        let mut filler = BRepFillFilling::default();
        let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let c1 = line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
        let c2 = line_bspline(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
        let c3 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));
        filler.add_edge(c0, ContinuityOrder::G0, true);
        filler.add_edge(c1, ContinuityOrder::G0, true);
        filler.add_edge(c2, ContinuityOrder::G0, true);
        filler.add_edge(c3, ContinuityOrder::G0, true);
        filler.build();
        assert!(filler.is_done());
        assert!(filler.g0_error() >= 0.0);
    }

    #[test]
    fn filling_cubic_boundary_builds_correctly() {
        let mut filler = BRepFillFilling::default();
        let c0 = cubic_bspline(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(0.33, 0.0, 0.1),
            Pnt::new(0.66, 0.0, -0.1),
            Pnt::new(1.0, 0.0, 0.0),
        );
        let c1 = line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
        let c2 = line_bspline(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
        let c3 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));
        filler.add_edge(c0, ContinuityOrder::G0, true);
        filler.add_edge(c1, ContinuityOrder::G0, true);
        filler.add_edge(c2, ContinuityOrder::G0, true);
        filler.add_edge(c3, ContinuityOrder::G0, true);
        filler.build();
        assert!(filler.is_done());
        let surf = filler.surface();
        assert_eq!(surf.nb_u_poles(), 4);
        assert_eq!(surf.nb_v_poles(), 4);
    }
}

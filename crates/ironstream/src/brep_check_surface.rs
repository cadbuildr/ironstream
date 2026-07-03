// FILE: rust/ironstream/crates/ironstream/src/brep_check_surface.rs
//! `BRepCheck` surface validation — pure-Rust, zero external dependencies.
//!
//! Mirrors the surface-specific subset of OCCT's `BRepCheck` package,
//! providing per-surface status codes, per-surface result records, and a
//! lightweight surface checker that aggregates results across many surfaces.

// ---------------------------------------------------------------------------
// SurfaceCheckStatus
// ---------------------------------------------------------------------------

/// Status codes for a single surface validation run.
///
/// Mirrors a subset of OCCT's `BRepCheck_Status` enumeration, limited to
/// the codes that are meaningful for surface-level checks.
// occt: BRepCheck_Status subset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurfaceCheckStatus {
    /// Surface passed all checks.
    // occt: BRepCheck_NoError
    NoError,
    /// The same-domain flag stored on the surface is inconsistent.
    // occt: BRepCheck_InvalidSameDomainFlag
    InvalidSameDomainFlag,
    /// The parametric (u, v) range is degenerate (zero or inverted extent).
    // occt: BRepCheck_InvalidRange
    InvalidRange,
    /// The curvature radius at some point on the surface is not representable
    /// (zero, negative, or non-finite).
    // occt: BRepCheck_InvalidCurvatureRadius
    InvalidCurvatureRadius,
    /// The orientation of the surface normal is inconsistent with its context.
    // occt: BRepCheck_BadOrientation
    BadOrientation,
    /// A generic check failure that does not fall into one of the above
    /// categories.
    // occt: BRepCheck_CheckFail
    CheckFail,
}

impl SurfaceCheckStatus {
    /// Returns `true` when this status represents a passing check.
    pub fn is_ok(self) -> bool {
        self == SurfaceCheckStatus::NoError
    }

    /// Human-readable description of this status code.
    pub fn description(self) -> &'static str {
        match self {
            SurfaceCheckStatus::NoError => "No error",
            SurfaceCheckStatus::InvalidSameDomainFlag => "Invalid same-domain flag",
            SurfaceCheckStatus::InvalidRange => "Invalid parametric range",
            SurfaceCheckStatus::InvalidCurvatureRadius => "Invalid curvature radius",
            SurfaceCheckStatus::BadOrientation => "Bad orientation",
            SurfaceCheckStatus::CheckFail => "Generic check failure",
        }
    }
}

// ---------------------------------------------------------------------------
// SurfaceCheckResult
// ---------------------------------------------------------------------------

/// Validation result for a single surface.
///
/// Mirrors the per-surface result record produced by OCCT's `BRepCheck`
/// surface checking routines.  Each result stores the surface label, its
/// parametric (u, v) ranges, the maximum numerical error observed during
/// checking, and the overall status.
// occt: BRepCheck surface result
#[derive(Debug, Clone)]
pub struct SurfaceCheckResult {
    /// Human-readable identifier for the surface (e.g. a label or index
    /// string).
    surface_label: String,
    /// Overall validation status for this surface.
    status: SurfaceCheckStatus,
    /// Parametric u-range `(u_min, u_max)`.
    u_range: (f64, f64),
    /// Parametric v-range `(v_min, v_max)`.
    v_range: (f64, f64),
    /// Maximum numerical error accumulated during the check (0.0 means no
    /// error was measured).
    max_error: f64,
}

impl SurfaceCheckResult {
    /// Create a new result for the surface named `surface_label`.
    ///
    /// Initial state: `NoError`, zero ranges `(0.0, 0.0)`, `max_error = 0.0`.
    pub fn new(surface_label: &str) -> Self {
        Self {
            surface_label: surface_label.to_owned(),
            status: SurfaceCheckStatus::NoError,
            u_range: (0.0, 0.0),
            v_range: (0.0, 0.0),
            max_error: 0.0,
        }
    }

    // -- status --------------------------------------------------------------

    /// Returns the current validation status.
    pub fn status(&self) -> SurfaceCheckStatus {
        self.status
    }

    /// Overwrite the validation status.
    pub fn set_status(&mut self, status: SurfaceCheckStatus) {
        self.status = status;
    }

    // -- label ---------------------------------------------------------------

    /// Returns the surface label string.
    pub fn surface_label(&self) -> &str {
        &self.surface_label
    }

    // -- u_range -------------------------------------------------------------

    /// Returns the parametric u-range `(u_min, u_max)`.
    pub fn u_range(&self) -> (f64, f64) {
        self.u_range
    }

    /// Set the parametric u-range.
    pub fn set_u_range(&mut self, u_min: f64, u_max: f64) {
        self.u_range = (u_min, u_max);
    }

    // -- v_range -------------------------------------------------------------

    /// Returns the parametric v-range `(v_min, v_max)`.
    pub fn v_range(&self) -> (f64, f64) {
        self.v_range
    }

    /// Set the parametric v-range.
    pub fn set_v_range(&mut self, v_min: f64, v_max: f64) {
        self.v_range = (v_min, v_max);
    }

    // -- max_error -----------------------------------------------------------

    /// Returns the maximum numerical error measured during this check.
    pub fn max_error(&self) -> f64 {
        self.max_error
    }

    /// Set the maximum numerical error.
    pub fn set_max_error(&mut self, err: f64) {
        self.max_error = err;
    }

    // -- validity ------------------------------------------------------------

    /// Returns `true` when the surface passed all checks (`NoError` status).
    pub fn is_valid(&self) -> bool {
        self.status == SurfaceCheckStatus::NoError
    }
}

// ---------------------------------------------------------------------------
// SurfaceChecker
// ---------------------------------------------------------------------------

/// Lightweight surface checker that accumulates [`SurfaceCheckResult`] records.
///
/// Mirrors the role of OCCT's `BRepCheck` surface checker, which iterates
/// over surfaces in a shape and produces one result per surface.
// occt: BRepCheck surface checker
#[derive(Debug, Default)]
pub struct SurfaceChecker {
    /// Accumulated results, one per surface that has been checked.
    results: Vec<SurfaceCheckResult>,
}

impl SurfaceChecker {
    /// Create a new, empty checker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Run a check for a surface identified by `label`.
    ///
    /// - When `valid` is `true` the returned result carries `NoError`.
    /// - When `valid` is `false` the returned result carries `CheckFail`.
    ///
    /// The result is **also** appended to the internal results list so that
    /// [`all_valid`] and [`nb_results`] reflect it immediately.
    pub fn check(&mut self, label: &str, valid: bool) -> SurfaceCheckResult {
        let mut result = SurfaceCheckResult::new(label);
        if !valid {
            result.set_status(SurfaceCheckStatus::CheckFail);
        }
        self.results.push(result.clone());
        result
    }

    /// Append an externally constructed result to the checker's list.
    pub fn add_result(&mut self, r: SurfaceCheckResult) {
        self.results.push(r);
    }

    /// Returns the total number of results collected so far.
    pub fn nb_results(&self) -> usize {
        self.results.len()
    }

    /// Returns `true` when every collected result has `NoError` status (or
    /// when no results have been collected yet).
    pub fn all_valid(&self) -> bool {
        self.results.iter().all(|r| r.is_valid())
    }
}

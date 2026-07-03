// FILE: src/topo_test.rs
//! `topo_test` — lightweight BRepCheck topology validation types, mirroring
//! OCCT's `BRepCheck` package with no external dependencies.
//!
//! Provides [`BrepCheckStatus`], [`BrepCheckError`], [`BrepCheckResult`], and
//! [`BrepCheckAnalyzer`] for driving topology-validation test scenarios without
//! requiring the full `brep_check` infrastructure.

// ---------------------------------------------------------------------------
// BrepCheckStatus
// ---------------------------------------------------------------------------

/// All validation status codes that can appear on a topological sub-shape.
///
/// Mirrors OCCT's `BRep_Check` status enumeration.  `Ok` / `NoError` both
/// mean the shape passed all checks; the remaining variants name specific
/// failure modes.
// occt: BRep_Check status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrepCheckStatus {
    /// Shape passed all checks — no errors detected.
    Ok,
    /// No error (alias for `Ok`, matches OCCT `BRepCheck_NoError`).
    NoError,
    /// A 3-D point on a curve does not lie on the expected curve.
    InvalidPointOnCurve,
    /// A point does not lie on the expected surface.
    InvalidPointOnSurface,
    /// A parametric range for an edge or curve is empty or inverted.
    InvalidRange,
    /// A wire contains no edges.
    EmptyWire,
    /// A shell contains no faces.
    EmptyShell,
    /// An edge appears more than the expected number of times in a wire.
    RedundantEdge,
    /// A wire crosses itself.
    SelfIntersectingWire,
    /// The geometric and topological descriptions of the shape are inconsistent.
    GeometryInconsistency,
    /// A surface of revolution has an invalid curvature radius.
    InvalidCurvatureRadius,
    /// An unclassified or unrecognised error occurred during validation.
    Unknown,
}

impl BrepCheckStatus {
    /// Returns `true` when this status indicates a passing check.
    pub fn is_ok(self) -> bool {
        matches!(self, BrepCheckStatus::Ok | BrepCheckStatus::NoError)
    }
}

// ---------------------------------------------------------------------------
// BrepCheckError
// ---------------------------------------------------------------------------

/// A single error entry produced during BRepCheck validation.
///
/// Each entry records the kind of problem (`status`) and a human-readable
/// identifier for the shape on which the problem was detected (`shape_label`).
// occt: BRepCheck error entry
#[derive(Debug, Clone)]
pub struct BrepCheckError {
    status: BrepCheckStatus,
    shape_label: String,
}

impl BrepCheckError {
    /// Create a new error entry.
    ///
    /// `status` is the validation failure code; `shape_label` is a
    /// caller-supplied identifier such as `"Edge_1"` or `"Wire_outer"`.
    pub fn new(status: BrepCheckStatus, shape_label: &str) -> Self {
        Self {
            status,
            shape_label: shape_label.to_owned(),
        }
    }

    /// Return the validation status for this error.
    pub fn status(&self) -> BrepCheckStatus {
        self.status
    }

    /// Return the label of the shape on which the error was detected.
    pub fn shape_label(&self) -> &str {
        &self.shape_label
    }
}

// ---------------------------------------------------------------------------
// BrepCheckResult
// ---------------------------------------------------------------------------

/// Validation result for a single topological shape.
///
/// After analysis, a result holds zero or more [`BrepCheckError`] entries.
/// An empty error list means the shape is valid.
// occt: BRepCheck_Result
#[derive(Debug, Clone)]
pub struct BrepCheckResult {
    shape_label: String,
    errors: Vec<BrepCheckError>,
}

impl BrepCheckResult {
    /// Create a new, initially valid result for the shape identified by
    /// `label`.
    pub fn new(label: &str) -> Self {
        Self {
            shape_label: label.to_owned(),
            errors: Vec::new(),
        }
    }

    /// Append an error to this result.
    pub fn add_error(&mut self, e: BrepCheckError) {
        self.errors.push(e);
    }

    /// Returns `true` when no errors have been recorded (the shape is valid).
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Number of errors recorded for this shape.
    pub fn nb_errors(&self) -> usize {
        self.errors.len()
    }

    /// Slice over all errors recorded for this shape.
    pub fn errors(&self) -> &[BrepCheckError] {
        &self.errors
    }

    /// The label that identifies this shape.
    pub fn shape_label(&self) -> &str {
        &self.shape_label
    }
}

// ---------------------------------------------------------------------------
// BrepCheckAnalyzer
// ---------------------------------------------------------------------------

/// Topology checker — accumulates per-shape [`BrepCheckResult`]s.
///
/// Mirrors OCCT's `BRepCheck_Analyzer`.  Call [`check_shape`] for each shape
/// to validate; the resulting [`BrepCheckResult`] is both returned and stored
/// inside the analyzer so that [`is_valid`] / [`nb_results`] reflect the full
/// session.
// occt: BRepCheck_Analyzer
#[derive(Debug, Default)]
pub struct BrepCheckAnalyzer {
    results: Vec<BrepCheckResult>,
}

impl BrepCheckAnalyzer {
    /// Create a new, empty analyzer with no results.
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate a single shape identified by `label`.
    ///
    /// When `valid` is `true` the returned result contains no errors.  When
    /// `valid` is `false` the result contains a single error with status
    /// [`BrepCheckStatus::Unknown`].  The result is also stored inside the
    /// analyzer.
    pub fn check_shape(&mut self, label: &str, valid: bool) -> BrepCheckResult {
        let mut result = BrepCheckResult::new(label);
        if !valid {
            result.add_error(BrepCheckError::new(BrepCheckStatus::Unknown, label));
        }
        self.results.push(result.clone());
        result
    }

    /// Store a pre-built result in the analyzer.
    pub fn add_result(&mut self, r: BrepCheckResult) {
        self.results.push(r);
    }

    /// Returns `true` when every stored result is valid (no errors in any
    /// shape).
    pub fn is_valid(&self) -> bool {
        self.results.iter().all(|r| r.is_valid())
    }

    /// Total number of results stored in the analyzer.
    pub fn nb_results(&self) -> usize {
        self.results.len()
    }
}

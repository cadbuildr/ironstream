// FILE: shape_checker.rs
// occt: BRepCheck_Analyzer, BRepCheck_Shell, BRepCheck_Result, ShapeAnalysis_CheckSmallFace

/// Detailed check status codes from BRepCheck.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CheckStatus {
    NoError,
    InvalidPointOnCurve,
    InvalidPointOnCurveOnSurface,
    InvalidPointOnSurface,
    No3DCurve,
    Multiple3DCurve,
    Invalid3DCurve,
    NoCurveOnSurface,
    InvalidCurveOnSurface,
    InvalidCurveOnClosedSurface,
    InvalidSameRangeFlag,
    InvalidSameParameterFlag,
    InvalidDegeneratedFlag,
    FreeEdge,
    InvalidMultiConnexity,
    InvalidRange,
    EmptyWire,
    RedundantEdge,
    SelfIntersectingWire,
    NoSurface,
    InvalidWire,
    RedundantWire,
    IntersectingWires,
    InvalidImbricationOfWires,
    EmptyShell,
    RedundantFace,
    UnorientableShape,
    NotClosed,
    NotConnected,
    SubshapeNotInShape,
    BadOrientation,
    BadOrientationOfSubshape,
    InvalidToleranceValue,
    EnclosedRegion,
    CheckFail,
}

impl Default for CheckStatus {
    fn default() -> Self { Self::NoError }
}

impl CheckStatus {
    pub fn is_error(&self) -> bool { *self != CheckStatus::NoError }
    pub fn message(&self) -> &'static str {
        match self {
            Self::NoError => "No error",
            Self::FreeEdge => "Free edge",
            Self::SelfIntersectingWire => "Self-intersecting wire",
            Self::NotClosed => "Not closed",
            Self::NotConnected => "Not connected",
            Self::EmptyShell => "Empty shell",
            Self::InvalidRange => "Invalid range",
            Self::InvalidToleranceValue => "Invalid tolerance value",
            _ => "Check error",
        }
    }
}

/// A result entry: a status code with optional shape reference.
#[derive(Clone, Debug)]
pub struct CheckResult {
    pub status: CheckStatus,
    pub shape_id: Option<u32>,
    pub sub_shape_id: Option<u32>,
}

impl CheckResult {
    pub fn ok() -> Self { Self { status: CheckStatus::NoError, shape_id: None, sub_shape_id: None } }
    pub fn error(status: CheckStatus, shape_id: u32) -> Self {
        Self { status, shape_id: Some(shape_id), sub_shape_id: None }
    }
    pub fn is_ok(&self) -> bool { !self.status.is_error() }
}

// occt: BRepCheck_Result — accumulates check results for a single shape
#[derive(Clone, Debug, Default)]
pub struct BrepCheckResult {
    pub shape_id: u32,
    pub context_results: Vec<CheckResult>,
    pub uncontextual_results: Vec<CheckResult>,
}

impl BrepCheckResult {
    pub fn new(shape_id: u32) -> Self {
        Self { shape_id, ..Default::default() }
    }

    pub fn add(&mut self, result: CheckResult) {
        if result.sub_shape_id.is_some() {
            self.context_results.push(result);
        } else {
            self.uncontextual_results.push(result);
        }
    }

    pub fn is_valid(&self) -> bool {
        self.context_results.iter().all(|r| r.is_ok()) &&
        self.uncontextual_results.iter().all(|r| r.is_ok())
    }

    pub fn statuses(&self) -> Vec<CheckStatus> {
        self.context_results.iter().chain(self.uncontextual_results.iter())
            .map(|r| r.status)
            .collect()
    }

    pub fn nb_errors(&self) -> usize {
        self.context_results.iter().chain(self.uncontextual_results.iter())
            .filter(|r| r.status.is_error())
            .count()
    }
}

// occt: BRepCheck_Shell — checks that a shell is valid (closed, connected, orientable)
#[derive(Clone, Debug)]
pub struct BrepCheckShell {
    pub shell_id: u32,
    pub is_closed: bool,
    pub is_orientable: bool,
    pub is_connected: bool,
    pub status: CheckStatus,
}

impl BrepCheckShell {
    pub fn new(shell_id: u32) -> Self {
        Self {
            shell_id,
            is_closed: false,
            is_orientable: true,
            is_connected: true,
            status: CheckStatus::NoError,
        }
    }

    pub fn set_closed(&mut self, v: bool) { self.is_closed = v; }
    pub fn set_orientable(&mut self, v: bool) {
        self.is_orientable = v;
        if !v { self.status = CheckStatus::UnorientableShape; }
    }
    pub fn set_connected(&mut self, v: bool) {
        self.is_connected = v;
        if !v { self.status = CheckStatus::NotConnected; }
    }

    pub fn is_valid(&self) -> bool { !self.status.is_error() && self.is_orientable && self.is_connected }
    pub fn closed(&self) -> bool { self.is_closed }
    pub fn orientable(&self) -> bool { self.is_orientable }
}

// occt: BRepCheck_Analyzer — top-level checker for a shape tree
#[derive(Clone, Debug, Default)]
pub struct BrepCheckAnalyzer {
    pub shape_id: u32,
    pub results: Vec<BrepCheckResult>,
    pub is_valid: bool,
}

impl BrepCheckAnalyzer {
    pub fn new(shape_id: u32) -> Self {
        Self { shape_id, results: Vec::new(), is_valid: true }
    }

    pub fn perform(&mut self, strict: bool) {
        // Stub: all valid unless a result was pre-loaded with errors
        let has_errors = self.results.iter().any(|r| !r.is_valid());
        if strict && has_errors {
            self.is_valid = false;
        } else {
            self.is_valid = !has_errors;
        }
    }

    pub fn is_valid_shape(&self) -> bool { self.is_valid }

    pub fn add_result(&mut self, result: BrepCheckResult) {
        self.results.push(result);
    }

    pub fn result(&self, shape_id: u32) -> Option<&BrepCheckResult> {
        self.results.iter().find(|r| r.shape_id == shape_id)
    }

    pub fn nb_results(&self) -> usize { self.results.len() }
}

/// Classification of a small face.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmallFaceKind {
    NotSmall,
    Spot,        // degenerated to a point
    Strip,       // degenerated to a curve (narrow strip)
    SmallArea,
}

impl Default for SmallFaceKind { fn default() -> Self { Self::NotSmall } }

// occt: ShapeAnalysis_CheckSmallFace — identifies degenerate/tiny faces
#[derive(Clone, Debug)]
pub struct CheckSmallFace {
    pub tolerance: f64,
    pub spot_tol: f64,
    pub strip_tol: f64,
}

impl Default for CheckSmallFace {
    fn default() -> Self { Self { tolerance: 1e-7, spot_tol: 1e-7, strip_tol: 1e-5 } }
}

impl CheckSmallFace {
    pub fn new() -> Self { Self::default() }
    pub fn with_tolerance(tol: f64) -> Self { Self { tolerance: tol, spot_tol: tol, strip_tol: tol * 100.0 } }

    pub fn is_spot_face(&self, face_area: f64) -> bool {
        face_area < self.spot_tol * self.spot_tol * std::f64::consts::PI
    }

    pub fn is_strip_face(&self, face_area: f64, face_perimeter: f64) -> bool {
        if face_perimeter < 1e-15 { return false; }
        let ratio = face_area / (face_perimeter * face_perimeter);
        ratio < self.strip_tol
    }

    pub fn classify(&self, face_area: f64, face_perimeter: f64) -> SmallFaceKind {
        if self.is_spot_face(face_area) {
            SmallFaceKind::Spot
        } else if self.is_strip_face(face_area, face_perimeter) {
            SmallFaceKind::Strip
        } else if face_area < self.tolerance * self.tolerance {
            SmallFaceKind::SmallArea
        } else {
            SmallFaceKind::NotSmall
        }
    }

    pub fn strip_tolerance(&self) -> f64 { self.strip_tol }
    pub fn spot_tolerance(&self) -> f64 { self.spot_tol }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_status_messages() {
        assert_eq!(CheckStatus::NoError.message(), "No error");
        assert!(!CheckStatus::NoError.is_error());
        assert!(CheckStatus::FreeEdge.is_error());
    }

    #[test]
    fn brep_check_result_accumulate() {
        let mut r = BrepCheckResult::new(1);
        r.add(CheckResult::ok());
        r.add(CheckResult::error(CheckStatus::FreeEdge, 5));
        assert!(!r.is_valid());
        assert_eq!(r.nb_errors(), 1);
    }

    #[test]
    fn brep_check_shell() {
        let mut sh = BrepCheckShell::new(10);
        sh.set_closed(true);
        assert!(sh.closed());
        sh.set_orientable(false);
        assert!(!sh.is_valid());
        assert_eq!(sh.status, CheckStatus::UnorientableShape);
    }

    #[test]
    fn brep_check_analyzer_valid() {
        let mut ana = BrepCheckAnalyzer::new(1);
        ana.perform(false);
        assert!(ana.is_valid_shape());
    }

    #[test]
    fn brep_check_analyzer_with_errors() {
        let mut ana = BrepCheckAnalyzer::new(1);
        let mut r = BrepCheckResult::new(2);
        r.add(CheckResult::error(CheckStatus::NotClosed, 2));
        ana.add_result(r);
        ana.perform(true);
        assert!(!ana.is_valid_shape());
    }

    #[test]
    fn small_face_classifier() {
        let csf = CheckSmallFace::new();
        assert_eq!(csf.classify(0.0, 0.0), SmallFaceKind::Spot);
        let large_area = 100.0;
        let large_perim = 40.0;
        assert_eq!(csf.classify(large_area, large_perim), SmallFaceKind::NotSmall);
    }
}

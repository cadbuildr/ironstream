// FILE: shape_upgrade.rs
//
// Pure-Rust zero-dependency stubs modelled after OCCT ShapeUpgrade classes.
// No external crates are used; only std.

// ---------------------------------------------------------------------------
// ShapeUpgradeStatus
// ---------------------------------------------------------------------------

// occt: ShapeExtend_Status
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeUpgradeStatus {
    Ok,
    Done,
    Fail,
    Warn,
}

// ---------------------------------------------------------------------------
// UnifySameDomain
// ---------------------------------------------------------------------------

// occt: ShapeUpgrade_UnifySameDomain
pub struct UnifySameDomain {
    pub shape: String,
    pub unify_faces: bool,
    pub unify_edges: bool,
    pub concat_b_splines: bool,
    result_shape: String,
    is_built: bool,
}

impl UnifySameDomain {
    pub fn new(shape: &str) -> Self {
        Self {
            shape: shape.to_owned(),
            unify_faces: true,
            unify_edges: true,
            concat_b_splines: false,
            result_shape: shape.to_owned(),
            is_built: false,
        }
    }

    pub fn set_unify_faces(&mut self, b: bool) {
        self.unify_faces = b;
    }

    pub fn set_unify_edges(&mut self, b: bool) {
        self.unify_edges = b;
    }

    /// Perform the unification algorithm.
    /// After calling build(), shape() returns the unified result.
    pub fn build(&mut self) {
        // Stub: result is the input shape (no actual geometry kernel).
        self.result_shape = self.shape.clone();
        self.is_built = true;
    }

    /// Return the result shape after build().
    pub fn shape(&self) -> &str {
        &self.result_shape
    }

    /// Return true when the sub-shape `s` was generated (i.e. is present)
    /// in the history of the unification.  Stub always returns false.
    pub fn history_is_generated(&self, _s: &str) -> bool {
        false
    }
}

// ---------------------------------------------------------------------------
// ShapeDivide
// ---------------------------------------------------------------------------

// occt: ShapeUpgrade_ShapeDivide
pub struct ShapeDivide {
    pub shape: String,
    pub tolerance: f64,
    result: String,
    performed: bool,
}

impl ShapeDivide {
    pub fn new(shape: &str, tol: f64) -> Self {
        Self {
            shape: shape.to_owned(),
            tolerance: tol,
            result: String::new(),
            performed: false,
        }
    }

    /// Execute the divide algorithm.
    /// Returns true on success (stub always succeeds).
    pub fn perform(&mut self) -> bool {
        // Stub: result equals the input shape.
        self.result = self.shape.clone();
        self.performed = true;
        true
    }

    /// Return the result shape string.
    /// Returns an empty string if perform() has not been called yet.
    pub fn result(&self) -> String {
        self.result.clone()
    }
}

// ---------------------------------------------------------------------------
// ShapeUpgradeUnifySameDomain  (legacy name kept for backward compatibility)
// ---------------------------------------------------------------------------

// occt: ShapeUpgrade_UnifySameDomain
pub struct ShapeUpgradeUnifySameDomain {
    unify_faces: bool,
    unify_edges: bool,
    concat_b_splines: bool,
    angular_tolerance: f64,
    linear_tolerance: f64,
    is_done: bool,
}

impl ShapeUpgradeUnifySameDomain {
    pub fn new() -> Self {
        Self {
            unify_faces: true,
            unify_edges: true,
            concat_b_splines: true,
            angular_tolerance: 1e-8,
            linear_tolerance: 1e-8,
            is_done: false,
        }
    }

    pub fn set_unify_faces(&mut self, v: bool) {
        self.unify_faces = v;
    }

    pub fn set_unify_edges(&mut self, v: bool) {
        self.unify_edges = v;
    }

    pub fn set_concat_b_splines(&mut self, v: bool) {
        self.concat_b_splines = v;
    }

    pub fn set_angular_tolerance(&mut self, t: f64) {
        self.angular_tolerance = t;
    }

    pub fn set_linear_tolerance(&mut self, t: f64) {
        self.linear_tolerance = t;
    }

    pub fn build(&mut self) {
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn unify_faces(&self) -> bool {
        self.unify_faces
    }

    pub fn unify_edges(&self) -> bool {
        self.unify_edges
    }
}

impl Default for ShapeUpgradeUnifySameDomain {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// ShapeUpgradeShapeTolerances
// ---------------------------------------------------------------------------

// occt-ref: ShapeUpgrade_ShapeTolerances
pub struct ShapeUpgradeShapeTolerances {
    tolerance: f64,
    mode: u8,
    is_done: bool,
}

impl ShapeUpgradeShapeTolerances {
    pub fn new(tolerance: f64) -> Self {
        Self {
            tolerance,
            mode: 3,
            is_done: false,
        }
    }

    pub fn set_mode(&mut self, mode: u8) {
        self.mode = mode;
    }

    pub fn mode(&self) -> u8 {
        self.mode
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn perform(&mut self) {
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

// ---------------------------------------------------------------------------
// ShapeUpgradeShapeDivide  (legacy name kept for backward compatibility)
// ---------------------------------------------------------------------------

// occt: ShapeUpgrade_ShapeDivide
pub struct ShapeUpgradeShapeDivide {
    precision: f64,
    max_tolerance: f64,
    min_tolerance: f64,
    is_done: bool,
}

impl ShapeUpgradeShapeDivide {
    pub fn new(precision: f64) -> Self {
        Self {
            precision,
            max_tolerance: 1.0,
            min_tolerance: 1e-7,
            is_done: false,
        }
    }

    pub fn set_max_tolerance(&mut self, t: f64) {
        self.max_tolerance = t;
    }

    pub fn set_min_tolerance(&mut self, t: f64) {
        self.min_tolerance = t;
    }

    pub fn perform(&mut self, _surface_segments: u32) -> ShapeUpgradeStatus {
        self.is_done = true;
        ShapeUpgradeStatus::Done
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn status(&self) -> ShapeUpgradeStatus {
        if self.is_done {
            ShapeUpgradeStatus::Done
        } else {
            ShapeUpgradeStatus::Ok
        }
    }
}

// ---------------------------------------------------------------------------
// Free function
// ---------------------------------------------------------------------------

/// Convenience wrapper: create a [`UnifySameDomain`] for `shape`, call
/// [`UnifySameDomain::build`], and return the resulting shape string.
pub fn unify_shape(shape: &str) -> String {
    let mut u = UnifySameDomain::new(shape);
    u.build();
    u.shape().to_owned()
}

// ---------------------------------------------------------------------------
// Internal unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- UnifySameDomain ---

    #[test]
    fn test_unify_same_domain_new_defaults() {
        let u = UnifySameDomain::new("box");
        assert_eq!(u.shape, "box");
        assert!(u.unify_faces);
        assert!(u.unify_edges);
        assert!(!u.concat_b_splines);
    }

    #[test]
    fn test_unify_same_domain_build_returns_shape() {
        let mut u = UnifySameDomain::new("cylinder");
        u.build();
        assert_eq!(u.shape(), "cylinder");
    }

    #[test]
    fn test_unify_same_domain_set_flags() {
        let mut u = UnifySameDomain::new("s");
        u.set_unify_faces(false);
        u.set_unify_edges(false);
        assert!(!u.unify_faces);
        assert!(!u.unify_edges);
    }

    #[test]
    fn test_unify_same_domain_history_is_generated() {
        let mut u = UnifySameDomain::new("s");
        u.build();
        assert!(!u.history_is_generated("s"));
    }

    // --- ShapeDivide ---

    #[test]
    fn test_shape_divide_new() {
        let sd = ShapeDivide::new("cone", 1e-4);
        assert_eq!(sd.shape, "cone");
        assert!((sd.tolerance - 1e-4).abs() < 1e-15);
        assert_eq!(sd.result(), "");
    }

    #[test]
    fn test_shape_divide_perform_true() {
        let mut sd = ShapeDivide::new("cone", 1e-4);
        assert!(sd.perform());
    }

    #[test]
    fn test_shape_divide_result_after_perform() {
        let mut sd = ShapeDivide::new("cone", 1e-4);
        sd.perform();
        assert_eq!(sd.result(), "cone");
    }

    // --- unify_shape ---

    #[test]
    fn test_unify_shape_fn() {
        let out = unify_shape("sphere");
        assert_eq!(out, "sphere");
    }

    // --- legacy ShapeUpgradeUnifySameDomain ---

    #[test]
    fn test_legacy_unify_new_defaults() {
        let u = ShapeUpgradeUnifySameDomain::new();
        assert!(u.unify_faces());
        assert!(u.unify_edges());
        assert!(!u.is_done());
    }

    #[test]
    fn test_legacy_unify_build_sets_done() {
        let mut u = ShapeUpgradeUnifySameDomain::new();
        u.build();
        assert!(u.is_done());
    }

    #[test]
    fn test_legacy_unify_set_unify_faces_false() {
        let mut u = ShapeUpgradeUnifySameDomain::new();
        u.set_unify_faces(false);
        assert!(!u.unify_faces());
        assert!(u.unify_edges());
    }

    #[test]
    fn test_legacy_unify_set_unify_edges_false() {
        let mut u = ShapeUpgradeUnifySameDomain::new();
        u.set_unify_edges(false);
        assert!(!u.unify_edges());
    }

    #[test]
    fn test_legacy_unify_tolerances() {
        let mut u = ShapeUpgradeUnifySameDomain::new();
        u.set_angular_tolerance(1e-4);
        u.set_linear_tolerance(1e-5);
        assert!((u.angular_tolerance - 1e-4).abs() < 1e-15);
        assert!((u.linear_tolerance - 1e-5).abs() < 1e-15);
    }

    // --- ShapeUpgradeShapeTolerances ---

    #[test]
    fn test_tolerances_new() {
        let st = ShapeUpgradeShapeTolerances::new(1e-3);
        assert!((st.tolerance() - 1e-3).abs() < 1e-15);
        assert_eq!(st.mode(), 3);
        assert!(!st.is_done());
    }

    #[test]
    fn test_tolerances_perform_sets_done() {
        let mut st = ShapeUpgradeShapeTolerances::new(1e-6);
        st.perform();
        assert!(st.is_done());
    }

    #[test]
    fn test_tolerances_set_mode() {
        let mut st = ShapeUpgradeShapeTolerances::new(1e-6);
        st.set_mode(1);
        assert_eq!(st.mode(), 1);
    }

    // --- legacy ShapeUpgradeShapeDivide ---

    #[test]
    fn test_legacy_divide_new_defaults() {
        let sd = ShapeUpgradeShapeDivide::new(1e-4);
        assert!(!sd.is_done());
        assert_eq!(sd.status(), ShapeUpgradeStatus::Ok);
    }

    #[test]
    fn test_legacy_divide_perform_returns_done() {
        let mut sd = ShapeUpgradeShapeDivide::new(1e-4);
        let result = sd.perform(8);
        assert_eq!(result, ShapeUpgradeStatus::Done);
        assert!(sd.is_done());
        assert_eq!(sd.status(), ShapeUpgradeStatus::Done);
    }

    #[test]
    fn test_legacy_divide_set_tolerances() {
        let mut sd = ShapeUpgradeShapeDivide::new(1e-4);
        sd.set_max_tolerance(0.5);
        sd.set_min_tolerance(1e-9);
        assert!((sd.max_tolerance - 0.5).abs() < 1e-15);
        assert!((sd.min_tolerance - 1e-9).abs() < 1e-20);
    }

    // --- ShapeUpgradeStatus ---

    #[test]
    fn test_status_enum_copy() {
        let s = ShapeUpgradeStatus::Done;
        let s2 = s;
        assert_eq!(s, s2);
    }

    #[test]
    fn test_status_enum_all_distinct() {
        assert_ne!(ShapeUpgradeStatus::Ok, ShapeUpgradeStatus::Done);
        assert_ne!(ShapeUpgradeStatus::Done, ShapeUpgradeStatus::Fail);
        assert_ne!(ShapeUpgradeStatus::Fail, ShapeUpgradeStatus::Warn);
    }

    #[test]
    fn test_legacy_unify_default_trait() {
        let u = ShapeUpgradeUnifySameDomain::default();
        assert!(u.unify_faces());
        assert!(u.unify_edges());
    }
}

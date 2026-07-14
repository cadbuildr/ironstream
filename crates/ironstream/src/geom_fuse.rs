//! `geom_fuse` — higher-level geometry boolean operations, mirroring OCCT's
//! `BRepAlgoAPI_Fuse` / `BRepAlgoAPI_Cut` / `BRepAlgoAPI_Common` API.
//!
//! This is a pure-Rust, zero-dependency stub. Shape operands are represented as
//! opaque `String` labels (e.g. a serialised BREP tag or a debug name) so that
//! the builder API can be exercised without a live geometry kernel.  When the
//! real kernel is wired in, replace the `String` operands with the appropriate
//! `TopoDS_Shape` / `Solid` type and fill in `build()` with the BSP / OCCT
//! dispatch.

/// Which Boolean set-operation to perform.
///
/// Maps 1-to-1 onto the three primary OCCT Boolean-API classes.
// occt-ref: BRepAlgoAPI_Fuse // / BRepAlgoAPI_Cut / BRepAlgoAPI_Common
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoolOp {
    /// A ∪ B — union of two shapes.
    // occt-ref: BRepAlgoAPI_Fuse
    Fuse,
    /// A \ B — subtract B from A.
    // occt-ref: BRepAlgoAPI_Cut
    Cut,
    /// A ∩ B — intersection of two shapes.
    // occt-ref: BRepAlgoAPI_Common
    Common,
}

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

/// Builder mirroring `BRepAlgoAPI_Fuse` / `BRepAlgoAPI_Cut` / `BRepAlgoAPI_Common`.
///
/// Construct via one of the static factory methods, optionally refine with
/// [`with_fuzzy`](BoolOpBuilder::with_fuzzy), then call
/// [`build`](BoolOpBuilder::build) to obtain the result shape label and
/// [`is_done`](BoolOpBuilder::is_done) to check success.
// occt-ref: BRepAlgoAPI_Fuse
// occt-ref: BRepAlgoAPI_Cut
#[derive(Clone, Debug)]
pub struct BoolOpBuilder {
    /// First operand shape (label / serialised tag).
    pub shape1: String,
    /// Second operand shape (label / serialised tag).
    pub shape2: String,
    /// Which Boolean operation to apply.
    pub op: BoolOp,
    /// Fuzzy tolerance passed to the underlying algorithm (≥ 0.0).
    /// Mirrors `BRepAlgoAPI_BooleanOperation::SetFuzzyValue`.
    pub fuzzy_value: f64,

    // Internal state --------------------------------------------------------
    done: bool,
}

impl BoolOpBuilder {
    // ------------------------------------------------------------------
    // Private constructor
    // ------------------------------------------------------------------

    fn new(s1: &str, s2: &str, op: BoolOp) -> Self {
        Self {
            shape1: s1.to_owned(),
            shape2: s2.to_owned(),
            op,
            fuzzy_value: 0.0,
            done: false,
        }
    }

    // ------------------------------------------------------------------
    // Factory methods (mirror the three OCCT classes)
    // ------------------------------------------------------------------

    /// Create a **fuse** (union, A ∪ B) builder.
    ///
    /// # occt: BRepAlgoAPI_Fuse
    // occt-ref: BRepAlgoAPI_Fuse
    pub fn fuse(s1: &str, s2: &str) -> Self {
        Self::new(s1, s2, BoolOp::Fuse)
    }

    /// Create a **cut** (subtraction, A \ B) builder.
    ///
    /// # occt: BRepAlgoAPI_Cut
    // occt-ref: BRepAlgoAPI_Cut
    pub fn cut(s1: &str, s2: &str) -> Self {
        Self::new(s1, s2, BoolOp::Cut)
    }

    /// Create a **common** (intersection, A ∩ B) builder.
    ///
    /// # occt: BRepAlgoAPI_Common
    // occt-ref: BRepAlgoAPI_Common
    pub fn common(s1: &str, s2: &str) -> Self {
        Self::new(s1, s2, BoolOp::Common)
    }

    // ------------------------------------------------------------------
    // Configuration
    // ------------------------------------------------------------------

    /// Set the fuzzy-matching tolerance.
    ///
    /// Mirrors `BRepAlgoAPI_BooleanOperation::SetFuzzyValue`. Values ≤ 0.0
    /// are clamped to 0.0 (disabled).
    pub fn with_fuzzy(mut self, fuzz: f64) -> Self {
        self.fuzzy_value = if fuzz > 0.0 { fuzz } else { 0.0 };
        self
    }

    // ------------------------------------------------------------------
    // Execution
    // ------------------------------------------------------------------

    /// Execute the Boolean operation and return the result shape label.
    ///
    /// The stub encodes the operation and operands into a human-readable tag
    /// so that callers can assert on the result without a live kernel.
    /// Replace this body with the real dispatch when the kernel is available.
    ///
    /// Mirrors `BRepAlgoAPI_BooleanOperation::Build`.
    pub fn build(&mut self) -> String {
        let op_tag = match self.op {
            BoolOp::Fuse   => "fuse",
            BoolOp::Cut    => "cut",
            BoolOp::Common => "common",
        };
        self.done = true;
        if self.fuzzy_value > 0.0 {
            format!(
                "{}({}, {}; fuzzy={})",
                op_tag, self.shape1, self.shape2, self.fuzzy_value
            )
        } else {
            format!("{}({}, {})", op_tag, self.shape1, self.shape2)
        }
    }

    /// Returns `true` after a successful [`build`](BoolOpBuilder::build) call.
    ///
    /// Mirrors `BRepAlgoAPI_BooleanOperation::IsDone`.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ---------------------------------------------------------------------------
// Convenience free functions (mirror the one-shot OCCT helper pattern)
// ---------------------------------------------------------------------------

/// Fuse (union) two shapes and return the result label.
///
/// Equivalent to `BoolOpBuilder::fuse(s1, s2).build()`.
// occt-ref: BRepAlgoAPI_Fuse
pub fn fuse(s1: &str, s2: &str) -> String {
    BoolOpBuilder::fuse(s1, s2).build()
}

/// Cut (subtract) `s2` from `s1` and return the result label.
///
/// Equivalent to `BoolOpBuilder::cut(s1, s2).build()`.
// occt-ref: BRepAlgoAPI_Cut
pub fn cut(s1: &str, s2: &str) -> String {
    BoolOpBuilder::cut(s1, s2).build()
}

/// Intersect (common) two shapes and return the result label.
///
/// Equivalent to `BoolOpBuilder::common(s1, s2).build()`.
// occt-ref: BRepAlgoAPI_Common
pub fn common(s1: &str, s2: &str) -> String {
    BoolOpBuilder::common(s1, s2).build()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- BoolOp enum ---------------------------------------------------------

    #[test]
    fn bool_op_variants_debug() {
        assert_eq!(format!("{:?}", BoolOp::Fuse),   "Fuse");
        assert_eq!(format!("{:?}", BoolOp::Cut),    "Cut");
        assert_eq!(format!("{:?}", BoolOp::Common), "Common");
    }

    #[test]
    fn bool_op_equality() {
        assert_eq!(BoolOp::Fuse, BoolOp::Fuse);
        assert_ne!(BoolOp::Fuse, BoolOp::Cut);
    }

    // -- BoolOpBuilder construction ------------------------------------------

    #[test]
    fn builder_fuse_stores_operands() {
        let b = BoolOpBuilder::fuse("box1", "box2");
        assert_eq!(b.shape1, "box1");
        assert_eq!(b.shape2, "box2");
        assert_eq!(b.op, BoolOp::Fuse);
    }

    #[test]
    fn builder_cut_stores_operands() {
        let b = BoolOpBuilder::cut("solid_a", "solid_b");
        assert_eq!(b.op, BoolOp::Cut);
    }

    #[test]
    fn builder_common_stores_operands() {
        let b = BoolOpBuilder::common("s1", "s2");
        assert_eq!(b.op, BoolOp::Common);
    }

    #[test]
    fn builder_fuzzy_default_zero() {
        let b = BoolOpBuilder::fuse("a", "b");
        assert_eq!(b.fuzzy_value, 0.0);
    }

    #[test]
    fn builder_with_fuzzy_sets_value() {
        let b = BoolOpBuilder::cut("a", "b").with_fuzzy(1e-5);
        assert!((b.fuzzy_value - 1e-5).abs() < 1e-15);
    }

    #[test]
    fn builder_with_fuzzy_negative_clamped_to_zero() {
        let b = BoolOpBuilder::fuse("a", "b").with_fuzzy(-3.0);
        assert_eq!(b.fuzzy_value, 0.0);
    }

    // -- is_done before and after build -------------------------------------

    #[test]
    fn builder_not_done_before_build() {
        let b = BoolOpBuilder::fuse("a", "b");
        assert!(!b.is_done());
    }

    #[test]
    fn builder_done_after_build() {
        let mut b = BoolOpBuilder::fuse("a", "b");
        let _ = b.build();
        assert!(b.is_done());
    }

    // -- build return value -------------------------------------------------

    #[test]
    fn builder_fuse_build_result_contains_operands() {
        let mut b = BoolOpBuilder::fuse("sphere", "cube");
        let result = b.build();
        assert!(result.contains("sphere"));
        assert!(result.contains("cube"));
        assert!(result.contains("fuse"));
    }

    #[test]
    fn builder_cut_build_result_contains_operands() {
        let mut b = BoolOpBuilder::cut("body", "hole");
        let result = b.build();
        assert!(result.contains("body"));
        assert!(result.contains("hole"));
        assert!(result.contains("cut"));
    }

    #[test]
    fn builder_common_build_result_contains_operands() {
        let mut b = BoolOpBuilder::common("p", "q");
        let result = b.build();
        assert!(result.contains("common"));
    }

    #[test]
    fn builder_build_with_fuzzy_includes_fuzzy_in_result() {
        let mut b = BoolOpBuilder::fuse("a", "b").with_fuzzy(0.001);
        let result = b.build();
        assert!(result.contains("fuzzy"));
    }

    #[test]
    fn builder_build_without_fuzzy_omits_fuzzy_tag() {
        let mut b = BoolOpBuilder::fuse("a", "b");
        let result = b.build();
        assert!(!result.contains("fuzzy"));
    }

    // -- free functions -----------------------------------------------------

    #[test]
    fn free_fuse_returns_nonempty() {
        let r = fuse("s1", "s2");
        assert!(!r.is_empty());
    }

    #[test]
    fn free_cut_returns_nonempty() {
        let r = cut("s1", "s2");
        assert!(!r.is_empty());
    }

    #[test]
    fn free_common_returns_nonempty() {
        let r = common("s1", "s2");
        assert!(!r.is_empty());
    }

    #[test]
    fn free_fuse_contains_fuse_tag() {
        let r = fuse("alpha", "beta");
        assert!(r.contains("fuse"));
        assert!(r.contains("alpha"));
        assert!(r.contains("beta"));
    }

    #[test]
    fn free_cut_contains_cut_tag() {
        let r = cut("alpha", "beta");
        assert!(r.contains("cut"));
    }

    #[test]
    fn free_common_contains_common_tag() {
        let r = common("alpha", "beta");
        assert!(r.contains("common"));
    }
}

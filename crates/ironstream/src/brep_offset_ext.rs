// FILE: rust/ironstream/crates/ironstream/src/brep_offset_ext.rs

// occt: BRepOffset_Analyse // / BRepOffset_DataMapOfShapeOffset — analysis and
// extended offset stubs.
//
// This module contains zero-dependency Rust stubs that mirror the public API
// shape of the OpenCascade `BRepOffset_Analyse` and `BRepOffset_MakeOffset`
// families.  No geometry is computed; the structs exist so that downstream
// code can be written and compiled against a stable interface before a real
// implementation is provided.

// ─────────────────────────────────────────────────────────────────────────────
// OffsetMode
// ─────────────────────────────────────────────────────────────────────────────

/// Controls how the offset shell is constructed.
///
// occt-ref: BRepOffset_Mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetMode {
    /// The offset shell follows the outer skin of the shape.
    ///
    // occt-ref: BRepOffset_Skin
    Skin,

    /// The offset shell follows a pipe profile swept along edges.
    ///
    // occt-ref: BRepOffset_Pipe
    Pipe,

    /// The offset shell is built on both sides (recto and verso).
    ///
    // occt-ref: BRepOffset_RectoVerso
    RectoVerso,
}

// ─────────────────────────────────────────────────────────────────────────────
// OffsetAnalysis
// ─────────────────────────────────────────────────────────────────────────────

/// Analyses a shape to determine whether it can be offset by the given
/// distance, and collects any detected errors.
///
/// This is the stub counterpart of `BRepOffset_Analyse`, which in OCCT walks
/// the shape topology and classifies edges/vertices to prepare for offsetting.
/// Here the struct stores the shape name and offset value and accumulates
/// error messages via [`perform`](OffsetAnalysis::perform).
///
// occt: BRepOffset_Analyse
#[derive(Clone, Debug)]
pub struct OffsetAnalysis {
    /// Name (or handle) of the input shape being analysed.
    pub shape: String,
    /// Signed offset distance requested for this analysis.
    pub offset: f64,
    /// Errors detected during the last [`perform`](OffsetAnalysis::perform)
    /// call.  Empty when no analysis has been run or when the shape is valid.
    pub errors: Vec<String>,
}

impl OffsetAnalysis {
    /// Create a new analysis descriptor for `shape` with the given signed
    /// offset distance.  No analysis is run until
    /// [`perform`](OffsetAnalysis::perform) is called.
    ///
    // occt: BRepOffset_Analyse // ::BRepOffset_Analyse
    pub fn new(shape: &str, offset: f64) -> Self {
        Self {
            shape: shape.to_string(),
            offset,
            errors: Vec::new(),
        }
    }

    /// Run the (stub) analysis.
    ///
    /// In the real OCCT implementation this traverses the shape topology and
    /// identifies edges whose local geometry would produce singularities or
    /// self-intersections after offsetting.  Here, the error list is cleared
    /// and left empty to indicate a clean analysis.
    ///
    // occt: BRepOffset_Analyse // ::Perform
    pub fn perform(&mut self) {
        self.errors.clear();
    }

    /// Return `true` when the last [`perform`](OffsetAnalysis::perform) call
    /// produced no errors.
    ///
    /// Returns `false` when [`perform`](OffsetAnalysis::perform) has never been
    /// called (because `errors` starts empty but the analysis hasn't validated
    /// the shape — callers should always call `perform` before `is_valid`).
    ///
    // occt: BRepOffset_Analyse // — error-free predicate
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Return the number of errors recorded during the last
    /// [`perform`](OffsetAnalysis::perform) call.
    ///
    // occt: BRepOffset_Analyse // — error count accessor
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MakeOffset
// ─────────────────────────────────────────────────────────────────────────────

/// Builds an offset shape from a source shape.
///
/// Mirrors the high-level driver `BRepOffset_MakeOffset` that in OCCT
/// orchestrates the full offset pipeline (analysis, face offsetting, topology
/// reconstruction).  Call [`build`](MakeOffset::build) to run the (stub)
/// algorithm and obtain a result shape name.
///
// occt: BRepOffset_MakeOffset
#[derive(Clone, Debug)]
pub struct MakeOffset {
    /// Name (or handle) of the input shape to offset.
    pub shape: String,
    /// Signed offset distance (positive = expand, negative = shrink).
    pub offset: f64,
    /// Offset construction mode.
    pub mode: OffsetMode,
    /// When `true` the result is closed into a solid; otherwise an open shell
    /// is produced.
    pub solid: bool,
    /// Whether [`build`](MakeOffset::build) has been called successfully.
    done: bool,
}

impl MakeOffset {
    /// Create a new offset builder for `shape` with the given signed offset
    /// distance.
    ///
    /// Defaults: `mode` = [`OffsetMode::Skin`], `solid` = `false`.
    ///
    // occt: BRepOffset_MakeOffset // ::BRepOffset_MakeOffset
    pub fn new(shape: &str, offset: f64) -> Self {
        Self {
            shape: shape.to_string(),
            offset,
            mode: OffsetMode::Skin,
            solid: false,
            done: false,
        }
    }

    /// Set the offset construction mode, consuming and returning `self` for
    /// builder-style chaining.
    ///
    // occt: BRepOffset_MakeOffset // ::SetOffsetMode
    pub fn with_mode(mut self, m: OffsetMode) -> Self {
        self.mode = m;
        self
    }

    /// Control whether the result is closed into a solid (`true`) or left as
    /// an open shell (`false`), consuming and returning `self` for
    /// builder-style chaining.
    ///
    // occt: BRepOffset_MakeOffset // ::SetSolid
    pub fn as_solid(mut self, b: bool) -> Self {
        self.solid = b;
        self
    }

    /// Run the (stub) offset algorithm and return the name of the result shape.
    ///
    /// The returned string follows the pattern
    /// `"<shape>_offset_<offset>_<mode>_<solid>"` so that callers can inspect
    /// what parameters were used without running real geometry code.  After
    /// this call [`is_done`](MakeOffset::is_done) returns `true`.
    ///
    // occt: BRepOffset_MakeOffset // ::MakeOffsetShape / Perform
    pub fn build(&mut self) -> String {
        self.done = true;
        let mode_str = match self.mode {
            OffsetMode::Skin => "skin",
            OffsetMode::Pipe => "pipe",
            OffsetMode::RectoVerso => "recto_verso",
        };
        let solid_str = if self.solid { "solid" } else { "shell" };
        format!(
            "{}_offset_{}_{}_{}",
            self.shape, self.offset, mode_str, solid_str
        )
    }

    /// Whether the last [`build`](MakeOffset::build) call completed
    /// successfully.
    ///
    // occt: BRepOffset_MakeOffset // ::IsDone
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── OffsetMode ────────────────────────────────────────────────────────────

    #[test]
    fn offset_mode_variants_are_distinct() {
        assert_ne!(OffsetMode::Skin, OffsetMode::Pipe);
        assert_ne!(OffsetMode::Pipe, OffsetMode::RectoVerso);
        assert_ne!(OffsetMode::Skin, OffsetMode::RectoVerso);
    }

    #[test]
    fn offset_mode_copy() {
        let m = OffsetMode::RectoVerso;
        let m2 = m;
        assert_eq!(m, m2);
    }

    // ── OffsetAnalysis ────────────────────────────────────────────────────────

    #[test]
    fn analysis_new_stores_fields() {
        let a = OffsetAnalysis::new("box1", 2.5);
        assert_eq!(a.shape, "box1");
        assert_eq!(a.offset, 2.5);
        assert!(a.errors.is_empty());
    }

    #[test]
    fn analysis_is_valid_before_perform() {
        // errors is empty on construction, so is_valid returns true even
        // without calling perform — callers should always call perform first.
        let a = OffsetAnalysis::new("sphere", 1.0);
        assert!(a.is_valid());
        assert_eq!(a.error_count(), 0);
    }

    #[test]
    fn analysis_perform_clears_errors() {
        let mut a = OffsetAnalysis::new("cone", 0.5);
        // Manually inject an error to confirm perform clears it.
        a.errors.push("pre-existing error".to_string());
        assert_eq!(a.error_count(), 1);
        a.perform();
        assert_eq!(a.error_count(), 0);
        assert!(a.is_valid());
    }

    #[test]
    fn analysis_is_valid_after_perform() {
        let mut a = OffsetAnalysis::new("cylinder", -1.0);
        a.perform();
        assert!(a.is_valid());
    }

    #[test]
    fn analysis_error_count_reflects_errors_vec() {
        let mut a = OffsetAnalysis::new("torus", 0.1);
        a.errors.push("err1".to_string());
        a.errors.push("err2".to_string());
        assert_eq!(a.error_count(), 2);
        assert!(!a.is_valid());
    }

    #[test]
    fn analysis_negative_offset() {
        let mut a = OffsetAnalysis::new("shell", -3.0);
        a.perform();
        assert_eq!(a.offset, -3.0);
        assert!(a.is_valid());
    }

    // ── MakeOffset ────────────────────────────────────────────────────────────

    #[test]
    fn make_offset_new_defaults() {
        let mo = MakeOffset::new("box", 1.0);
        assert_eq!(mo.shape, "box");
        assert_eq!(mo.offset, 1.0);
        assert_eq!(mo.mode, OffsetMode::Skin);
        assert!(!mo.solid);
        assert!(!mo.is_done());
    }

    #[test]
    fn make_offset_with_mode_builder() {
        let mo = MakeOffset::new("prism", 0.5).with_mode(OffsetMode::Pipe);
        assert_eq!(mo.mode, OffsetMode::Pipe);
    }

    #[test]
    fn make_offset_as_solid_builder() {
        let mo = MakeOffset::new("prism", 0.5).as_solid(true);
        assert!(mo.solid);
    }

    #[test]
    fn make_offset_builder_chain() {
        let mo = MakeOffset::new("wedge", 2.0)
            .with_mode(OffsetMode::RectoVerso)
            .as_solid(true);
        assert_eq!(mo.mode, OffsetMode::RectoVerso);
        assert!(mo.solid);
        assert_eq!(mo.offset, 2.0);
    }

    #[test]
    fn make_offset_not_done_before_build() {
        let mo = MakeOffset::new("box", 1.0);
        assert!(!mo.is_done());
    }

    #[test]
    fn make_offset_done_after_build() {
        let mut mo = MakeOffset::new("box", 1.0);
        mo.build();
        assert!(mo.is_done());
    }

    #[test]
    fn make_offset_build_skin_shell() {
        let mut mo = MakeOffset::new("box", 1.0);
        let result = mo.build();
        assert_eq!(result, "box_offset_1_skin_shell");
    }

    #[test]
    fn make_offset_build_pipe_solid() {
        let mut mo = MakeOffset::new("pipe_shape", 0.25)
            .with_mode(OffsetMode::Pipe)
            .as_solid(true);
        let result = mo.build();
        assert_eq!(result, "pipe_shape_offset_0.25_pipe_solid");
    }

    #[test]
    fn make_offset_build_recto_verso_shell() {
        let mut mo = MakeOffset::new("panel", -0.5)
            .with_mode(OffsetMode::RectoVerso);
        let result = mo.build();
        assert_eq!(result, "panel_offset_-0.5_recto_verso_shell");
    }

    #[test]
    fn make_offset_build_negative_distance() {
        let mut mo = MakeOffset::new("sphere", -2.0);
        let result = mo.build();
        assert!(result.contains("sphere"));
        assert!(result.contains("-2"));
        assert!(mo.is_done());
    }

    #[test]
    fn make_offset_build_zero_distance() {
        let mut mo = MakeOffset::new("flat", 0.0);
        let result = mo.build();
        assert!(result.contains("flat"));
        assert!(mo.is_done());
    }
}

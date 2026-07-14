// FILE: src/geom_loft.rs
//! `GeomLoft` — lofted surface through cross-section wires, mirroring
//! OpenCascade's `BRepOffsetAPI_ThruSections` package.
//!
//! Provides:
//!
//! - [`LoftSection`]   — one loft cross-section (wire or edge ring).
//! - [`LoftParams`]    — construction parameters (solid/ruled/precision).
//! - [`LoftResult`]    — query object holding face/edge counts after build.
//! - [`ThruSections`]  — the loft builder itself.
//!
//! All implementations are self-contained and contain **no unsafe code**.

// occt-ref: BRepOffsetAPI_ThruSections // (package boundary)

// ---------------------------------------------------------------------------
// LoftSection
// ---------------------------------------------------------------------------

/// One cross-section fed into a loft operation.
///
/// Each section carries a human-readable label, the number of poles (control
/// points) on its profile curve, whether it represents a closed wire (vs.\ an
/// open edge), and the parameter position along the lofting direction.
// occt-note: one loft cross-section
#[derive(Clone, Debug)]
pub struct LoftSection {
    /// Descriptive label (e.g. `"section_0"`).
    label: String,
    /// Number of B-spline poles on the profile curve.
    nb_poles: usize,
    /// `true` if the section is a closed wire; `false` for an open edge.
    is_wire: bool,
    /// Parameter along the lofting spine (typically 0.0 … 1.0).
    parameter: f64,
}

impl LoftSection {
    /// Create a new `LoftSection`.
    ///
    /// The section is initialised as an open edge (`is_wire = false`).
    ///
    /// # Arguments
    /// * `label`    — human-readable identifier.
    /// * `nb_poles` — number of control poles on the profile (must be >= 2).
    /// * `parameter`— position along the loft spine (e.g. 0.0 for the first
    ///                section, 1.0 for the last).
    pub fn new(label: &str, nb_poles: usize, parameter: f64) -> Self {
        Self {
            label: label.to_owned(),
            nb_poles,
            is_wire: false,
            parameter,
        }
    }

    /// Returns the section label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the number of control poles on the profile.
    pub fn nb_poles(&self) -> usize {
        self.nb_poles
    }

    /// Returns the parameter position along the lofting spine.
    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    /// Returns `true` if this section is a closed wire.
    pub fn is_wire(&self) -> bool {
        self.is_wire
    }

    /// Set whether this section is a closed wire (`true`) or an open edge
    /// (`false`).
    pub fn set_wire(&mut self, wire: bool) {
        self.is_wire = wire;
    }
}

// ---------------------------------------------------------------------------
// LoftParams
// ---------------------------------------------------------------------------

/// Construction parameters for a [`ThruSections`] loft.
///
/// Mirrors the flags accepted by `BRepOffsetAPI_ThruSections`.
// occt-ref: BRepOffsetAPI_ThruSections // params
#[derive(Clone, Debug)]
pub struct LoftParams {
    /// If `true` the result is capped to form a solid; otherwise a shell.
    is_solid: bool,
    /// If `true` faces between consecutive sections are ruled (planar ruling);
    /// otherwise they are smoothly blended.
    is_ruled: bool,
    /// 3D precision threshold used when approximating intermediate surfaces.
    pres3d: f64,
}

impl LoftParams {
    /// Create default loft parameters:
    /// * `solid  = false`
    /// * `ruled  = false`
    /// * `pres3d = 1e-6`
    pub fn new() -> Self {
        Self {
            is_solid: false,
            is_ruled: false,
            pres3d: 1.0e-6,
        }
    }

    /// Set whether the result should be a solid (`true`) or shell (`false`).
    pub fn set_solid(&mut self, solid: bool) {
        self.is_solid = solid;
    }

    /// Set whether faces should be ruled surfaces (`true`) or smooth blends
    /// (`false`).
    pub fn set_ruled(&mut self, ruled: bool) {
        self.is_ruled = ruled;
    }

    /// Returns `true` if the loft produces a solid.
    pub fn is_solid(&self) -> bool {
        self.is_solid
    }

    /// Returns `true` if inter-section faces are ruled.
    pub fn is_ruled(&self) -> bool {
        self.is_ruled
    }

    /// Returns the 3D precision threshold.
    pub fn pres3d(&self) -> f64 {
        self.pres3d
    }
}

impl Default for LoftParams {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// LoftResult
// ---------------------------------------------------------------------------

/// Holds topology counts produced by a completed loft build.
///
/// After [`ThruSections::build`] succeeds, callers query face and edge counts
/// through this object.
// occt-note: loft result
#[derive(Clone, Debug)]
pub struct LoftResult {
    /// Whether the build has completed successfully.
    is_done: bool,
    /// Number of lateral faces in the lofted shape.
    nb_faces: usize,
    /// Number of edges in the lofted shape.
    nb_edges: usize,
}

impl LoftResult {
    /// Construct an empty, not-done result.
    pub fn new() -> Self {
        Self {
            is_done: false,
            nb_faces: 0,
            nb_edges: 0,
        }
    }

    /// Record a successful build with the given topology counts.
    ///
    /// Sets `is_done` to `true`.
    pub fn build(&mut self, nb_faces: usize, nb_edges: usize) {
        self.is_done = true;
        self.nb_faces = nb_faces;
        self.nb_edges = nb_edges;
    }

    /// Returns `true` if the loft build completed successfully.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Returns the number of lateral faces.
    pub fn nb_faces(&self) -> usize {
        self.nb_faces
    }

    /// Returns the number of edges.
    pub fn nb_edges(&self) -> usize {
        self.nb_edges
    }
}

impl Default for LoftResult {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// ThruSections
// ---------------------------------------------------------------------------

/// Loft builder that produces a shell or solid by skinning through a sequence
/// of cross-section profiles.
///
/// Mirrors `BRepOffsetAPI_ThruSections`.  Sections must be added in spine
/// order (ascending `parameter`) before calling [`build`].
///
/// # Face and edge count formula
///
/// Between each pair of consecutive sections one lateral face is created per
/// pole of the profile:
/// ```text
/// nb_faces = (nb_sections − 1) × nb_poles_of_first_section
/// nb_edges = (nb_sections − 1) × nb_poles_of_first_section
///          + nb_sections × nb_poles_of_first_section
/// ```
/// When no sections have been added the counts remain zero.
// occt-ref: BRepOffsetAPI_ThruSections
#[derive(Clone, Debug)]
pub struct ThruSections {
    /// Ordered list of cross-section profiles.
    sections: Vec<LoftSection>,
    /// Build parameters.
    params: LoftParams,
    /// Build result (populated by [`build`]).
    result: LoftResult,
}

impl ThruSections {
    /// Create a new loft builder with the given parameters.
    pub fn new(params: LoftParams) -> Self {
        Self {
            sections: Vec::new(),
            params,
            result: LoftResult::new(),
        }
    }

    /// Append a cross-section to the loft.
    ///
    /// Sections should be added in ascending `parameter` order.
    pub fn add_section(&mut self, s: LoftSection) {
        self.sections.push(s);
        // Invalidate any previous result.
        self.result = LoftResult::new();
    }

    /// Returns the number of sections currently registered.
    pub fn nb_sections(&self) -> usize {
        self.sections.len()
    }

    /// Build the lofted shape.
    ///
    /// Computes face and edge counts from the registered sections and stores
    /// them in the internal [`LoftResult`].  Requires at least two sections;
    /// if fewer sections are present the build is a no-op and `is_done`
    /// remains `false`.
    ///
    /// # Face count
    /// `nb_faces = (nb_sections − 1) × nb_poles`  where `nb_poles` is taken
    /// from the first section (all sections are assumed to have matching
    /// profiles).
    ///
    /// # Edge count
    /// Lateral edges: one per face column plus one extra longitudinal edge per
    /// section boundary:
    /// `nb_edges = nb_faces + nb_sections × nb_poles`
    pub fn build(&mut self) {
        let n = self.sections.len();
        if n < 2 {
            return;
        }
        let nb_poles = self.sections[0].nb_poles();
        let nb_faces = (n - 1) * nb_poles;
        let nb_edges = nb_faces + n * nb_poles;
        self.result.build(nb_faces, nb_edges);
    }

    /// Returns `true` if [`build`] completed successfully.
    pub fn is_done(&self) -> bool {
        self.result.is_done()
    }

    /// Returns a reference to the build result.
    pub fn result(&self) -> &LoftResult {
        &self.result
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // LoftSection
    // -----------------------------------------------------------------------

    #[test]
    fn loft_section_new_defaults() {
        let s = LoftSection::new("sec0", 4, 0.0);
        assert_eq!(s.label(), "sec0");
        assert_eq!(s.nb_poles(), 4);
        assert!((s.parameter() - 0.0).abs() < f64::EPSILON);
        assert!(!s.is_wire());
    }

    #[test]
    fn loft_section_set_wire() {
        let mut s = LoftSection::new("wire_sec", 6, 0.5);
        assert!(!s.is_wire());
        s.set_wire(true);
        assert!(s.is_wire());
        s.set_wire(false);
        assert!(!s.is_wire());
    }

    #[test]
    fn loft_section_parameter_stored() {
        let s = LoftSection::new("s", 3, 0.75);
        assert!((s.parameter() - 0.75).abs() < 1.0e-15);
    }

    #[test]
    fn loft_section_clone_is_independent() {
        let mut original = LoftSection::new("orig", 5, 0.1);
        let mut cloned = original.clone();
        cloned.set_wire(true);
        assert!(!original.is_wire(), "clone mutation should not affect original");
    }

    // -----------------------------------------------------------------------
    // LoftParams
    // -----------------------------------------------------------------------

    #[test]
    fn loft_params_defaults() {
        let p = LoftParams::new();
        assert!(!p.is_solid());
        assert!(!p.is_ruled());
        assert!((p.pres3d() - 1.0e-6).abs() < 1.0e-15);
    }

    #[test]
    fn loft_params_set_solid() {
        let mut p = LoftParams::new();
        p.set_solid(true);
        assert!(p.is_solid());
        p.set_solid(false);
        assert!(!p.is_solid());
    }

    #[test]
    fn loft_params_set_ruled() {
        let mut p = LoftParams::new();
        p.set_ruled(true);
        assert!(p.is_ruled());
    }

    #[test]
    fn loft_params_default_trait() {
        let p: LoftParams = Default::default();
        assert!(!p.is_solid());
        assert!(!p.is_ruled());
    }

    // -----------------------------------------------------------------------
    // LoftResult
    // -----------------------------------------------------------------------

    #[test]
    fn loft_result_initial_state() {
        let r = LoftResult::new();
        assert!(!r.is_done());
        assert_eq!(r.nb_faces(), 0);
        assert_eq!(r.nb_edges(), 0);
    }

    #[test]
    fn loft_result_build_sets_done() {
        let mut r = LoftResult::new();
        r.build(6, 12);
        assert!(r.is_done());
        assert_eq!(r.nb_faces(), 6);
        assert_eq!(r.nb_edges(), 12);
    }

    #[test]
    fn loft_result_build_zero_counts() {
        let mut r = LoftResult::new();
        r.build(0, 0);
        assert!(r.is_done());
        assert_eq!(r.nb_faces(), 0);
        assert_eq!(r.nb_edges(), 0);
    }

    #[test]
    fn loft_result_default_trait() {
        let r: LoftResult = Default::default();
        assert!(!r.is_done());
    }

    // -----------------------------------------------------------------------
    // ThruSections
    // -----------------------------------------------------------------------

    #[test]
    fn thru_sections_empty_not_done() {
        let ts = ThruSections::new(LoftParams::new());
        assert_eq!(ts.nb_sections(), 0);
        assert!(!ts.is_done());
    }

    #[test]
    fn thru_sections_add_section_increments_count() {
        let mut ts = ThruSections::new(LoftParams::new());
        ts.add_section(LoftSection::new("s0", 4, 0.0));
        assert_eq!(ts.nb_sections(), 1);
        ts.add_section(LoftSection::new("s1", 4, 1.0));
        assert_eq!(ts.nb_sections(), 2);
    }

    #[test]
    fn thru_sections_build_one_section_not_done() {
        let mut ts = ThruSections::new(LoftParams::new());
        ts.add_section(LoftSection::new("s0", 4, 0.0));
        ts.build();
        assert!(!ts.is_done(), "need at least 2 sections to loft");
    }

    #[test]
    fn thru_sections_build_two_sections() {
        let mut ts = ThruSections::new(LoftParams::new());
        ts.add_section(LoftSection::new("s0", 4, 0.0));
        ts.add_section(LoftSection::new("s1", 4, 1.0));
        ts.build();
        assert!(ts.is_done());
        // nb_faces = (2-1)*4 = 4
        assert_eq!(ts.result().nb_faces(), 4);
        // nb_edges = 4 + 2*4 = 12
        assert_eq!(ts.result().nb_edges(), 12);
    }

    #[test]
    fn thru_sections_build_three_sections() {
        let mut ts = ThruSections::new(LoftParams::new());
        ts.add_section(LoftSection::new("s0", 6, 0.0));
        ts.add_section(LoftSection::new("s1", 6, 0.5));
        ts.add_section(LoftSection::new("s2", 6, 1.0));
        ts.build();
        assert!(ts.is_done());
        // nb_faces = (3-1)*6 = 12
        assert_eq!(ts.result().nb_faces(), 12);
        // nb_edges = 12 + 3*6 = 30
        assert_eq!(ts.result().nb_edges(), 30);
    }

    #[test]
    fn thru_sections_add_section_invalidates_result() {
        let mut ts = ThruSections::new(LoftParams::new());
        ts.add_section(LoftSection::new("s0", 4, 0.0));
        ts.add_section(LoftSection::new("s1", 4, 1.0));
        ts.build();
        assert!(ts.is_done());
        // Adding another section should reset is_done.
        ts.add_section(LoftSection::new("s2", 4, 2.0));
        assert!(!ts.is_done(), "result should be invalidated after add_section");
    }

    #[test]
    fn thru_sections_build_respects_nb_poles() {
        let mut ts = ThruSections::new(LoftParams::new());
        // Square cross-section: 4 poles per section.
        ts.add_section(LoftSection::new("bot", 4, 0.0));
        ts.add_section(LoftSection::new("mid", 4, 0.5));
        ts.add_section(LoftSection::new("top", 4, 1.0));
        ts.build();
        let r = ts.result();
        assert_eq!(r.nb_faces(), 8);  // (3-1)*4
        assert_eq!(r.nb_edges(), 20); // 8 + 3*4
    }

    #[test]
    fn thru_sections_solid_params_preserved() {
        let mut p = LoftParams::new();
        p.set_solid(true);
        p.set_ruled(true);
        let mut ts = ThruSections::new(p);
        ts.add_section(LoftSection::new("s0", 3, 0.0));
        ts.add_section(LoftSection::new("s1", 3, 1.0));
        ts.build();
        assert!(ts.is_done());
        // Face/edge formula is the same regardless of solid/ruled flags.
        assert_eq!(ts.result().nb_faces(), 3); // (2-1)*3
    }

    #[test]
    fn thru_sections_result_reference_matches_is_done() {
        let mut ts = ThruSections::new(LoftParams::new());
        ts.add_section(LoftSection::new("a", 5, 0.0));
        ts.add_section(LoftSection::new("b", 5, 1.0));
        ts.build();
        assert_eq!(ts.is_done(), ts.result().is_done());
    }

    #[test]
    fn thru_sections_wire_section_flag() {
        let mut ts = ThruSections::new(LoftParams::new());
        let mut s0 = LoftSection::new("wire0", 8, 0.0);
        s0.set_wire(true);
        let mut s1 = LoftSection::new("wire1", 8, 1.0);
        s1.set_wire(true);
        ts.add_section(s0);
        ts.add_section(s1);
        ts.build();
        assert!(ts.is_done());
        assert_eq!(ts.result().nb_faces(), 8); // (2-1)*8
    }
}

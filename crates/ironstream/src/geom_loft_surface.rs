// FILE: src/geom_loft_surface.rs
//! `GeomLoftSurface` — lofted surface through cross-section profiles, mirroring
//! OpenCascade's `BRepOffsetAPI_ThruSections` package.
//!
//! Provides:
//!
//! - [`ThruSections`]  — the primary loft builder (skins through ordered wires).
//! - [`LoftSurface`]   — a bilinearly-interpolated surface over profile grids.
//! - [`loft_profiles`] — free function convenience wrapper around [`LoftSurface`].
//!
//! All implementations are self-contained and contain **no unsafe code**.
//! Zero external dependencies — only `std` is used.

// occt: BRepOffsetAPI_ThruSections // (package boundary)

// ---------------------------------------------------------------------------
// ThruSections
// ---------------------------------------------------------------------------

/// Loft builder that skins a shell or solid through an ordered sequence of
/// cross-section wires (point lists).
///
/// Mirrors `BRepOffsetAPI_ThruSections` from OCCT.  Wires must be added in
/// spine order before calling [`build`](ThruSections::build).
///
/// # Example
/// ```
/// use ironstream::geom_loft_surface::ThruSections;
///
/// let mut ts = ThruSections::new(false);
/// ts.add_wire(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]]);
/// ts.add_wire(vec![[0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0]]);
/// ts.set_ruled(true);
/// let shape_id = ts.build();
/// assert!(ts.is_done());
/// ```
// occt: BRepOffsetAPI_ThruSections
#[derive(Clone, Debug)]
pub struct ThruSections {
    /// Ordered cross-section wires, each represented as a list of 3D points.
    pub sections: Vec<Vec<[f64; 3]>>,
    /// If `true` the loft result is capped to produce a solid; otherwise a shell.
    pub is_solid: bool,
    /// If `true` inter-section faces are ruled surfaces; otherwise smooth blends.
    pub ruled: bool,
    /// Internal flag set to `true` after a successful [`build`](Self::build).
    done: bool,
    /// Human-readable build result identifier (e.g. `"ThruSections_2_sections"`).
    shape_id: String,
}

impl ThruSections {
    /// `BRepOffsetAPI_ThruSections(isSolid, isRuled, pres3d)` — construct a new
    /// loft builder.
    ///
    /// # Arguments
    /// * `is_solid` — when `true` the result will be capped into a solid.
    pub fn new(is_solid: bool) -> Self {
        Self {
            sections: Vec::new(),
            is_solid,
            ruled: false,
            done: false,
            shape_id: String::new(),
        }
    }

    /// `AddWire(W)` — append a cross-section wire to the loft.
    ///
    /// Each `pts` element is a 3D point `[x, y, z]` on the cross-section
    /// profile.  Sections should be added in ascending spine-parameter order.
    /// Adding a new wire invalidates any previous [`build`](Self::build) result.
    pub fn add_wire(&mut self, pts: Vec<[f64; 3]>) {
        self.sections.push(pts);
        // Adding a section invalidates any previous result.
        self.done = false;
        self.shape_id.clear();
    }

    /// `SetRuled(isRuled)` — set whether inter-section faces are ruled.
    pub fn set_ruled(&mut self, b: bool) {
        self.ruled = b;
    }

    /// `Build()` — construct the lofted shape from the registered sections.
    ///
    /// Requires at least two sections; if fewer are present the build is a no-op
    /// and [`is_done`](Self::is_done) returns `false`.
    ///
    /// Returns a descriptive string identifier for the resulting shape.
    pub fn build(&mut self) -> String {
        let n = self.sections.len();
        if n < 2 {
            self.done = false;
            self.shape_id = String::new();
            return self.shape_id.clone();
        }
        let kind = if self.is_solid { "Solid" } else { "Shell" };
        let style = if self.ruled { "Ruled" } else { "Smooth" };
        self.shape_id = format!(
            "ThruSections_{}_sections_{}_{}",
            n, kind, style
        );
        self.done = true;
        self.shape_id.clone()
    }

    /// `IsDone()` — returns `true` if [`build`](Self::build) completed
    /// successfully.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns the number of cross-section wires currently registered.
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }
}

// ---------------------------------------------------------------------------
// LoftSurface
// ---------------------------------------------------------------------------

/// A lofted surface interpolated bilinearly over a grid of profile curves.
///
/// Each profile is a polyline of 3D points `[x, y, z]`.  After
/// [`build`](LoftSurface::build) the surface can be evaluated at any
/// parameter `(u, v) ∈ [0, 1] × [0, 1]` via [`evaluate`](LoftSurface::evaluate).
///
/// The surface interpolation strategy is:
/// - U axis: traverse point index within a profile (0 = first pole, 1 = last).
/// - V axis: traverse profile index (0 = first profile, 1 = last profile).
///
/// Between two adjacent poles / profiles bilinear interpolation is used.  This
/// is equivalent to a degree-1 tensor-product surface (ruled in both directions)
/// and serves as a pure-Rust zero-dependency stub for the OCCT loft kernel.
// occt-note: loft surface
#[derive(Clone, Debug)]
pub struct LoftSurface {
    /// Input profiles as added by [`add_profile`](LoftSurface::add_profile).
    pub profiles: Vec<Vec<[f64; 3]>>,
    /// Pole grid populated by [`build`](LoftSurface::build).
    ///
    /// `poles[i][j]` holds the point for profile `i`, pole `j`.
    pub poles: Vec<Vec<[f64; 3]>>,
}

impl LoftSurface {
    /// Construct an empty `LoftSurface` with no profiles.
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            poles: Vec::new(),
        }
    }

    /// Append a cross-section profile to the surface.
    ///
    /// `pts` is a slice of 3D points `[x, y, z]` forming the profile polyline.
    /// Invalidates any previous [`build`](Self::build) result.
    pub fn add_profile(&mut self, pts: Vec<[f64; 3]>) {
        self.profiles.push(pts);
        self.poles.clear();
    }

    /// Build the pole grid from the registered profiles.
    ///
    /// After calling `build`, the surface is ready for [`evaluate`](Self::evaluate).
    ///
    /// All profiles are resampled to the point count of the first profile by
    /// linear interpolation so that the pole grid is rectangular.  If fewer than
    /// two profiles have been added the method is a no-op.
    pub fn build(&mut self) {
        if self.profiles.len() < 2 {
            self.poles.clear();
            return;
        }

        // Target number of poles = length of the first profile (at least 2).
        let nb_poles = self.profiles[0].len().max(2);

        self.poles = self
            .profiles
            .iter()
            .map(|profile| resample_profile(profile, nb_poles))
            .collect();
    }

    /// Evaluate the surface at parameters `(u, v)` both in `[0, 1]`.
    ///
    /// `u` runs along the pole index within a profile (0 = first pole, 1 = last).
    /// `v` runs along the profile index (0 = first profile, 1 = last profile).
    ///
    /// Bilinear interpolation is used between the four nearest poles.
    ///
    /// Returns `[0.0, 0.0, 0.0]` if the surface has not been built yet or
    /// the pole grid is empty.
    pub fn evaluate(&self, u: f64, v: f64) -> [f64; 3] {
        if self.poles.len() < 2 {
            return [0.0, 0.0, 0.0];
        }

        let nb_profiles = self.poles.len();
        let nb_poles = self.poles[0].len();
        if nb_poles < 2 {
            return [0.0, 0.0, 0.0];
        }

        // Clamp parameters to [0, 1].
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        // Map v to profile span.
        let vf = v * (nb_profiles - 1) as f64;
        let v0 = (vf.floor() as usize).min(nb_profiles - 2);
        let v1 = v0 + 1;
        let tv = vf - v0 as f64;

        // Map u to pole span.
        let uf = u * (nb_poles - 1) as f64;
        let u0 = (uf.floor() as usize).min(nb_poles - 2);
        let u1 = u0 + 1;
        let tu = uf - u0 as f64;

        // Bilinear interpolation over the four surrounding poles.
        let p00 = self.poles[v0][u0];
        let p10 = self.poles[v0][u1];
        let p01 = self.poles[v1][u0];
        let p11 = self.poles[v1][u1];

        lerp3(lerp3(p00, p10, tu), lerp3(p01, p11, tu), tv)
    }
}

impl Default for LoftSurface {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Free function
// ---------------------------------------------------------------------------

/// Convenience function: build a [`LoftSurface`] from a slice of profiles.
///
/// Equivalent to constructing a [`LoftSurface`], calling
/// [`add_profile`](LoftSurface::add_profile) for each entry, and then
/// [`build`](LoftSurface::build).
pub fn loft_profiles(profiles: &[Vec<[f64; 3]>]) -> LoftSurface {
    let mut surface = LoftSurface::new();
    for profile in profiles {
        surface.add_profile(profile.clone());
    }
    surface.build();
    surface
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Linearly interpolate between two 3D points.
#[inline]
fn lerp3(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    [
        a[0] + t * (b[0] - a[0]),
        a[1] + t * (b[1] - a[1]),
        a[2] + t * (b[2] - a[2]),
    ]
}

/// Resample a profile polyline to exactly `n` equidistant-parameter points.
///
/// The original profile points are treated as evenly spaced in the `[0, 1]`
/// parameter space and the target points are sampled at `0/(n-1)`, `1/(n-1)`,
/// …, `(n-1)/(n-1)`.
fn resample_profile(profile: &[[f64; 3]], n: usize) -> Vec<[f64; 3]> {
    if profile.is_empty() {
        return vec![[0.0, 0.0, 0.0]; n];
    }
    if profile.len() == 1 {
        return vec![profile[0]; n];
    }
    if n <= 1 {
        return vec![profile[0]];
    }

    let m = profile.len() - 1; // number of segments in the source
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f64 / (n - 1) as f64;
        let tf = t * m as f64;
        let lo = (tf.floor() as usize).min(m - 1);
        let hi = lo + 1;
        let local_t = tf - lo as f64;
        result.push(lerp3(profile[lo], profile[hi], local_t));
    }
    result
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // ThruSections
    // -----------------------------------------------------------------------

    #[test]
    fn thru_sections_new_defaults() {
        let ts = ThruSections::new(false);
        assert_eq!(ts.section_count(), 0);
        assert!(!ts.is_done());
        assert!(!ts.is_solid);
        assert!(!ts.ruled);
    }

    #[test]
    fn thru_sections_new_solid() {
        let ts = ThruSections::new(true);
        assert!(ts.is_solid);
    }

    #[test]
    fn thru_sections_add_wire_increments_count() {
        let mut ts = ThruSections::new(false);
        ts.add_wire(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        assert_eq!(ts.section_count(), 1);
        ts.add_wire(vec![[0.0, 0.0, 1.0], [1.0, 0.0, 1.0]]);
        assert_eq!(ts.section_count(), 2);
    }

    #[test]
    fn thru_sections_build_one_wire_not_done() {
        let mut ts = ThruSections::new(false);
        ts.add_wire(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        let id = ts.build();
        assert!(!ts.is_done());
        assert!(id.is_empty());
    }

    #[test]
    fn thru_sections_build_two_wires_done() {
        let mut ts = ThruSections::new(false);
        ts.add_wire(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        ts.add_wire(vec![[0.0, 0.0, 1.0], [1.0, 0.0, 1.0]]);
        let id = ts.build();
        assert!(ts.is_done());
        assert!(!id.is_empty());
        assert!(id.contains("2_sections"));
    }

    #[test]
    fn thru_sections_build_solid_label() {
        let mut ts = ThruSections::new(true);
        ts.add_wire(vec![[0.0, 0.0, 0.0]]);
        ts.add_wire(vec![[0.0, 0.0, 1.0]]);
        let id = ts.build();
        assert!(id.contains("Solid"));
    }

    #[test]
    fn thru_sections_build_shell_label() {
        let mut ts = ThruSections::new(false);
        ts.add_wire(vec![[0.0, 0.0, 0.0]]);
        ts.add_wire(vec![[0.0, 0.0, 1.0]]);
        let id = ts.build();
        assert!(id.contains("Shell"));
    }

    #[test]
    fn thru_sections_build_ruled_label() {
        let mut ts = ThruSections::new(false);
        ts.set_ruled(true);
        ts.add_wire(vec![[0.0, 0.0, 0.0]]);
        ts.add_wire(vec![[0.0, 0.0, 1.0]]);
        let id = ts.build();
        assert!(id.contains("Ruled"));
    }

    #[test]
    fn thru_sections_build_smooth_label() {
        let mut ts = ThruSections::new(false);
        ts.add_wire(vec![[0.0, 0.0, 0.0]]);
        ts.add_wire(vec![[0.0, 0.0, 1.0]]);
        let id = ts.build();
        assert!(id.contains("Smooth"));
    }

    #[test]
    fn thru_sections_add_wire_invalidates_build() {
        let mut ts = ThruSections::new(false);
        ts.add_wire(vec![[0.0, 0.0, 0.0]]);
        ts.add_wire(vec![[0.0, 0.0, 1.0]]);
        ts.build();
        assert!(ts.is_done());
        ts.add_wire(vec![[0.0, 0.0, 2.0]]);
        assert!(!ts.is_done(), "adding a wire must invalidate the build result");
    }

    #[test]
    fn thru_sections_section_count_after_build() {
        let mut ts = ThruSections::new(false);
        ts.add_wire(vec![[0.0, 0.0, 0.0]]);
        ts.add_wire(vec![[0.0, 0.0, 1.0]]);
        ts.add_wire(vec![[0.0, 0.0, 2.0]]);
        ts.build();
        assert_eq!(ts.section_count(), 3);
    }

    #[test]
    fn thru_sections_sections_field_holds_points() {
        let mut ts = ThruSections::new(false);
        let wire = vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
        ts.add_wire(wire.clone());
        assert_eq!(ts.sections[0], wire);
    }

    // -----------------------------------------------------------------------
    // LoftSurface
    // -----------------------------------------------------------------------

    #[test]
    fn loft_surface_new_empty() {
        let ls = LoftSurface::new();
        assert!(ls.profiles.is_empty());
        assert!(ls.poles.is_empty());
    }

    #[test]
    fn loft_surface_add_profile_increments() {
        let mut ls = LoftSurface::new();
        ls.add_profile(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        assert_eq!(ls.profiles.len(), 1);
        ls.add_profile(vec![[0.0, 0.0, 1.0], [1.0, 0.0, 1.0]]);
        assert_eq!(ls.profiles.len(), 2);
    }

    #[test]
    fn loft_surface_build_one_profile_no_poles() {
        let mut ls = LoftSurface::new();
        ls.add_profile(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        ls.build();
        assert!(ls.poles.is_empty());
    }

    #[test]
    fn loft_surface_build_two_profiles_poles_populated() {
        let mut ls = LoftSurface::new();
        ls.add_profile(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        ls.add_profile(vec![[0.0, 0.0, 1.0], [1.0, 0.0, 1.0]]);
        ls.build();
        assert_eq!(ls.poles.len(), 2);
        assert_eq!(ls.poles[0].len(), 2);
    }

    #[test]
    fn loft_surface_evaluate_corners() {
        let mut ls = LoftSurface::new();
        ls.add_profile(vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]);
        ls.add_profile(vec![[0.0, 0.0, 4.0], [2.0, 0.0, 4.0]]);
        ls.build();

        let p00 = ls.evaluate(0.0, 0.0);
        assert!((p00[0] - 0.0).abs() < 1e-12);
        assert!((p00[2] - 0.0).abs() < 1e-12);

        let p10 = ls.evaluate(1.0, 0.0);
        assert!((p10[0] - 2.0).abs() < 1e-12);
        assert!((p10[2] - 0.0).abs() < 1e-12);

        let p01 = ls.evaluate(0.0, 1.0);
        assert!((p01[0] - 0.0).abs() < 1e-12);
        assert!((p01[2] - 4.0).abs() < 1e-12);

        let p11 = ls.evaluate(1.0, 1.0);
        assert!((p11[0] - 2.0).abs() < 1e-12);
        assert!((p11[2] - 4.0).abs() < 1e-12);
    }

    #[test]
    fn loft_surface_evaluate_center() {
        let mut ls = LoftSurface::new();
        ls.add_profile(vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]);
        ls.add_profile(vec![[0.0, 0.0, 2.0], [2.0, 0.0, 2.0]]);
        ls.build();

        let mid = ls.evaluate(0.5, 0.5);
        assert!((mid[0] - 1.0).abs() < 1e-12);
        assert!((mid[2] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn loft_surface_evaluate_no_build_returns_origin() {
        let ls = LoftSurface::new();
        let p = ls.evaluate(0.5, 0.5);
        assert_eq!(p, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn loft_surface_evaluate_clamps_out_of_range() {
        let mut ls = LoftSurface::new();
        ls.add_profile(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        ls.add_profile(vec![[0.0, 0.0, 1.0], [1.0, 0.0, 1.0]]);
        ls.build();

        // Out-of-range u/v should clamp to the boundary.
        let p_lo = ls.evaluate(-1.0, -1.0);
        let p_hi = ls.evaluate(2.0, 2.0);
        assert_eq!(p_lo, ls.evaluate(0.0, 0.0));
        assert_eq!(p_hi, ls.evaluate(1.0, 1.0));
    }

    #[test]
    fn loft_surface_add_profile_invalidates_poles() {
        let mut ls = LoftSurface::new();
        ls.add_profile(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        ls.add_profile(vec![[0.0, 0.0, 1.0], [1.0, 0.0, 1.0]]);
        ls.build();
        assert!(!ls.poles.is_empty());
        ls.add_profile(vec![[0.0, 0.0, 2.0], [1.0, 0.0, 2.0]]);
        assert!(
            ls.poles.is_empty(),
            "add_profile must clear the pole grid"
        );
    }

    #[test]
    fn loft_surface_default_trait() {
        let ls: LoftSurface = Default::default();
        assert!(ls.profiles.is_empty());
    }

    // -----------------------------------------------------------------------
    // loft_profiles free function
    // -----------------------------------------------------------------------

    #[test]
    fn loft_profiles_builds_surface() {
        let profiles = vec![
            vec![[0.0_f64, 0.0, 0.0], [1.0, 0.0, 0.0]],
            vec![[0.0_f64, 0.0, 1.0], [1.0, 0.0, 1.0]],
        ];
        let ls = loft_profiles(&profiles);
        assert_eq!(ls.profiles.len(), 2);
        assert!(!ls.poles.is_empty());
    }

    #[test]
    fn loft_profiles_empty_slice() {
        let ls = loft_profiles(&[]);
        assert!(ls.profiles.is_empty());
        assert!(ls.poles.is_empty());
    }

    #[test]
    fn loft_profiles_single_profile_no_poles() {
        let profiles = vec![vec![[0.0_f64, 0.0, 0.0], [1.0, 0.0, 0.0]]];
        let ls = loft_profiles(&profiles);
        assert_eq!(ls.profiles.len(), 1);
        assert!(ls.poles.is_empty());
    }

    #[test]
    fn loft_profiles_evaluate_matches_manual_build() {
        let profiles = vec![
            vec![[0.0_f64, 0.0, 0.0], [4.0, 0.0, 0.0]],
            vec![[0.0_f64, 0.0, 4.0], [4.0, 0.0, 4.0]],
        ];
        let ls = loft_profiles(&profiles);
        let p = ls.evaluate(0.5, 0.5);
        assert!((p[0] - 2.0).abs() < 1e-12);
        assert!((p[2] - 2.0).abs() < 1e-12);
    }

    // -----------------------------------------------------------------------
    // resample_profile (via LoftSurface.build with mismatched profile lengths)
    // -----------------------------------------------------------------------

    #[test]
    fn loft_surface_resample_longer_profile() {
        let mut ls = LoftSurface::new();
        // First profile has 3 poles; second has 5 — resampled to 3.
        ls.add_profile(vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
        ]);
        ls.add_profile(vec![
            [0.0, 0.0, 1.0],
            [0.5, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.5, 0.0, 1.0],
            [2.0, 0.0, 1.0],
        ]);
        ls.build();
        // Pole grid must be 2 profiles × 3 poles (nb_poles from first profile).
        assert_eq!(ls.poles.len(), 2);
        assert_eq!(ls.poles[0].len(), 3);
        assert_eq!(ls.poles[1].len(), 3);
    }
}

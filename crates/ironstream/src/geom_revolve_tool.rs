// FILE: src/geom_revolve_tool.rs
//! `BRepPrimAPI_MakeRevol` / `BRepFeat_MakeRevol` builder and feature types —
//! zero-dependency Rust stub mirroring OpenCascade's revolve-tool API.
//!
//! This module provides:
//!
//! * [`RevolTool`] — a builder that mirrors `BRepPrimAPI_MakeRevol`, accepting a
//!   source shape, revolution axis, optional axis origin, and sweep angle, then
//!   producing a descriptive shape token and an estimated volume.
//!
//! * [`FeatRevol`] — a feature-level builder that mirrors `BRepFeat_MakeRevol`,
//!   taking a face identifier and axis direction to grow or cut a feature on an
//!   existing solid.
//!
//! * [`revolve`] — a one-shot convenience function for the common case where the
//!   caller just needs the resulting shape token.
//!
//! No tessellation or geometric computation is performed here; all shape fields
//! are descriptive string tokens.  Only `std` is used — no external crates.

use std::f64::consts::PI;

// ─────────────────────────────── RevolTool ──────────────────────────────────

// occt: BRepPrimAPI_MakeRevol
/// Builder for a solid of revolution created from a named profile shape.
///
/// Mirrors the constructor and query surface of `BRepPrimAPI_MakeRevol`:
/// supply a source shape identifier, a revolution axis direction, an optional
/// axis-point origin, and a sweep angle; then call [`build`](RevolTool::build)
/// to obtain the resulting shape token.
///
/// The builder follows the OCCT pattern where the axis point defaults to the
/// origin and can be overridden with [`at`](RevolTool::at).
///
/// # Examples
///
/// ```
/// use ironstream::geom_revolve_tool::RevolTool;
/// use std::f64::consts::PI;
///
/// let result = RevolTool::new("profile_0", [0.0, 0.0, 1.0], 2.0 * PI)
///     .at([1.0, 0.0, 0.0])
///     .build();
/// assert!(!result.is_empty());
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct RevolTool {
    /// Identifier of the source profile shape to be revolved.
    pub shape: String,
    /// Unit direction vector of the revolution axis.
    pub axis: [f64; 3],
    /// Origin point through which the revolution axis passes.
    pub axis_pt: [f64; 3],
    /// Sweep angle in radians.  Values are kept as-is; the builder clamps
    /// them to `[0, 2π]` internally when computing the volume estimate.
    pub angle: f64,
}

impl RevolTool {
    /// Construct a revolution builder with the axis origin at the world origin.
    ///
    /// # Arguments
    /// * `shape` — identifier of the source profile shape.
    /// * `axis`  — direction of the revolution axis (stored as-is).
    /// * `angle` — sweep angle in radians.
    // occt: BRepPrimAPI_MakeRevol::BRepPrimAPI_MakeRevol
    pub fn new(shape: &str, axis: [f64; 3], angle: f64) -> Self {
        Self {
            shape: shape.to_owned(),
            axis,
            axis_pt: [0.0, 0.0, 0.0],
            angle,
        }
    }

    /// Override the axis origin point and return `self` for builder chaining.
    ///
    /// Mirrors the `gp_Ax1` axis-with-location overload of
    /// `BRepPrimAPI_MakeRevol` where the revolution axis passes through an
    /// arbitrary point rather than the world origin.
    ///
    /// # Arguments
    /// * `pt` — point through which the revolution axis passes.
    // occt: BRepPrimAPI_MakeRevol (gp_Ax1 axis-point overload)
    pub fn at(mut self, pt: [f64; 3]) -> Self {
        self.axis_pt = pt;
        self
    }

    /// Build the revolved shape and return a descriptive shape token.
    ///
    /// Returns an empty string when the source shape identifier is empty or the
    /// sweep angle is effectively zero (within `1e-12` of zero).
    ///
    /// Mirrors `BRepPrimAPI_MakeRevol::Build()` followed by
    /// `BRepPrimAPI_MakeRevol::Shape()`.
    // occt: BRepPrimAPI_MakeRevol::Shape
    pub fn build(&self) -> String {
        if self.shape.is_empty() || self.angle.abs() < 1e-12 {
            return String::new();
        }
        format!(
            "revol(shape={}, axis=[{:.6},{:.6},{:.6}], pt=[{:.6},{:.6},{:.6}], angle={:.6})",
            self.shape,
            self.axis[0],
            self.axis[1],
            self.axis[2],
            self.axis_pt[0],
            self.axis_pt[1],
            self.axis_pt[2],
            self.angle,
        )
    }

    /// Return `true` when the builder is in a valid, buildable state.
    ///
    /// Mirrors `BRepPrimAPI_MakeRevol::IsDone()`: the shape identifier must be
    /// non-empty and the sweep angle must be non-zero.
    // occt: BRepPrimAPI_MakeRevol::IsDone
    pub fn is_done(&self) -> bool {
        !self.shape.is_empty() && self.angle.abs() >= 1e-12
    }

    /// Return an estimated swept volume for the revolution.
    ///
    /// The volume estimate is computed as `angle * r² / 2` where `r` is the
    /// Euclidean distance from the axis origin to the axis origin itself plus
    /// one unit (stub radius = 1.0 because no real geometry is available).
    /// For a full revolution this reduces to `π * r²` — the area swept by
    /// a unit-radius disk.
    ///
    /// Returns `0.0` when [`is_done`](RevolTool::is_done) is `false`.
    ///
    /// This is a stub approximation; a real implementation would integrate the
    /// actual cross-sectional area from the BRep topology.
    // occt: BRepPrimAPI_MakeRevol (volume query, no direct OCCT equivalent —
    //       actual volumes come from BRepGProp_VolumeProperties)
    pub fn volume(&self) -> f64 {
        if !self.is_done() {
            return 0.0;
        }
        // Stub: treat the profile as a unit-area cross section at radius 1.
        // V ≈ angle * (stub_radius^2) / 2  (Pappus centroid theorem, r=1)
        let clamped_angle = self.angle.clamp(0.0, 2.0 * PI).abs();
        clamped_angle * 0.5
    }
}

// ─────────────────────────────── FeatRevol ──────────────────────────────────

// occt: BRepFeat_MakeRevol
/// Feature-level revolution builder operating on an existing solid face.
///
/// Mirrors `BRepFeat_MakeRevol`: takes a face identifier and an axis direction,
/// then produces a shape token representing the feature revolution (boss or
/// pocket) applied to the base solid.
///
/// The `direction` flag controls whether the revolution adds material (`true`,
/// analogous to a boss) or removes it (`false`, analogous to a pocket), similar
/// to the `Fuse` parameter of `BRepFeat_MakeRevol`.
///
/// # Examples
///
/// ```
/// use ironstream::geom_revolve_tool::FeatRevol;
///
/// let result = FeatRevol::new("top_face", [0.0, 1.0, 0.0]).build();
/// assert!(!result.is_empty());
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct FeatRevol {
    /// Identifier of the base face from which the feature revolution starts.
    pub face: String,
    /// Direction vector of the revolution axis.
    pub axis: [f64; 3],
    /// `true` to fuse (add material), `false` to cut (remove material).
    ///
    /// Mirrors the `Fuse` integer flag in `BRepFeat_MakeRevol` (1 = fuse,
    /// 0 = cut).
    pub direction: bool,
}

impl FeatRevol {
    /// Construct a feature revolution builder that fuses material by default.
    ///
    /// # Arguments
    /// * `face` — identifier of the source face.
    /// * `axis` — revolution axis direction (stored as-is).
    ///
    /// `direction` defaults to `true` (fuse / add material).
    // occt: BRepFeat_MakeRevol::BRepFeat_MakeRevol
    pub fn new(face: &str, axis: [f64; 3]) -> Self {
        Self {
            face: face.to_owned(),
            axis,
            direction: true,
        }
    }

    /// Build the feature revolution and return a descriptive shape token.
    ///
    /// Returns an empty string when the face identifier is empty.
    ///
    /// Mirrors `BRepFeat_MakeRevol::Perform()` combined with the subsequent
    /// `Shape()` query.
    // occt: BRepFeat_MakeRevol::Shape
    pub fn build(&self) -> String {
        if self.face.is_empty() {
            return String::new();
        }
        let op = if self.direction { "fuse" } else { "cut" };
        format!(
            "feat_revol(face={}, axis=[{:.6},{:.6},{:.6}], op={})",
            self.face,
            self.axis[0],
            self.axis[1],
            self.axis[2],
            op,
        )
    }

    /// Return `true` when the builder is in a valid, buildable state.
    ///
    /// Mirrors `BRepFeat_MakeRevol::IsDone()`: the face identifier must be
    /// non-empty.
    // occt: BRepFeat_MakeRevol::IsDone
    pub fn is_done(&self) -> bool {
        !self.face.is_empty()
    }
}

// ─────────────────────────────── revolve ────────────────────────────────────

/// Convenience function: revolve a named shape around `axis` by `angle` radians
/// and return the resulting shape token.
///
/// This is a thin wrapper around [`RevolTool`] that covers the common one-shot
/// usage pattern where the caller supplies a shape, an axis and an angle and
/// immediately wants the resulting token.  The axis origin defaults to the
/// world origin.
///
/// Returns an empty string when `shape` is empty or `angle` is effectively
/// zero.
///
/// # Arguments
/// * `shape` — identifier of the source profile shape.
/// * `axis`  — revolution axis direction.
/// * `angle` — sweep angle in radians.
///
/// # Examples
///
/// ```
/// use ironstream::geom_revolve_tool::revolve;
/// use std::f64::consts::PI;
///
/// let token = revolve("wire_0", [0.0, 0.0, 1.0], PI);
/// assert!(!token.is_empty());
/// ```
// occt: BRepPrimAPI_MakeRevol (free-function convenience wrapper)
pub fn revolve(shape: &str, axis: [f64; 3], angle: f64) -> String {
    RevolTool::new(shape, axis, angle).build()
}

// ─────────────────────────────── Unit tests ─────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── RevolTool ────────────────────────────────────────────────────────────

    #[test]
    fn revol_tool_new_defaults() {
        let t = RevolTool::new("profile_0", [0.0, 0.0, 1.0], PI);
        assert_eq!(t.shape, "profile_0");
        assert_eq!(t.axis, [0.0, 0.0, 1.0]);
        assert_eq!(t.axis_pt, [0.0, 0.0, 0.0]);
        assert!((t.angle - PI).abs() < f64::EPSILON);
    }

    #[test]
    fn revol_tool_at_overrides_axis_pt() {
        let t = RevolTool::new("p", [1.0, 0.0, 0.0], PI / 2.0)
            .at([3.0, 4.0, 5.0]);
        assert_eq!(t.axis_pt, [3.0, 4.0, 5.0]);
    }

    #[test]
    fn revol_tool_is_done_valid() {
        let t = RevolTool::new("solid", [0.0, 1.0, 0.0], PI);
        assert!(t.is_done());
    }

    #[test]
    fn revol_tool_is_done_empty_shape() {
        let t = RevolTool::new("", [0.0, 0.0, 1.0], PI);
        assert!(!t.is_done());
    }

    #[test]
    fn revol_tool_is_done_zero_angle() {
        let t = RevolTool::new("solid", [0.0, 0.0, 1.0], 0.0);
        assert!(!t.is_done());
    }

    #[test]
    fn revol_tool_build_returns_nonempty_for_valid_input() {
        let token = RevolTool::new("wire_0", [0.0, 0.0, 1.0], PI).build();
        assert!(!token.is_empty());
    }

    #[test]
    fn revol_tool_build_returns_empty_for_empty_shape() {
        let token = RevolTool::new("", [0.0, 0.0, 1.0], PI).build();
        assert!(token.is_empty());
    }

    #[test]
    fn revol_tool_build_returns_empty_for_zero_angle() {
        let token = RevolTool::new("wire_0", [0.0, 0.0, 1.0], 0.0).build();
        assert!(token.is_empty());
    }

    #[test]
    fn revol_tool_build_contains_shape_name() {
        let token = RevolTool::new("my_profile", [0.0, 0.0, 1.0], PI).build();
        assert!(token.contains("my_profile"));
    }

    #[test]
    fn revol_tool_volume_valid() {
        let t = RevolTool::new("solid", [0.0, 0.0, 1.0], 2.0 * PI);
        let v = t.volume();
        // For angle = 2π: volume stub = π * 0.5 ≈ 1.5708
        assert!(v > 0.0);
        assert!((v - PI).abs() < 1e-10);
    }

    #[test]
    fn revol_tool_volume_zero_when_not_done() {
        let t = RevolTool::new("", [0.0, 0.0, 1.0], PI);
        assert!((t.volume()).abs() < f64::EPSILON);
    }

    #[test]
    fn revol_tool_volume_half_turn() {
        let t = RevolTool::new("solid", [0.0, 0.0, 1.0], PI);
        let v = t.volume();
        // angle = π: volume stub = π/2
        assert!((v - PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn revol_tool_clone_is_independent() {
        let t = RevolTool::new("s", [0.0, 0.0, 1.0], PI);
        let mut u = t.clone();
        u.angle = 0.0;
        assert!(t.is_done(), "clone mutation must not affect original");
    }

    // ── FeatRevol ────────────────────────────────────────────────────────────

    #[test]
    fn feat_revol_new_defaults() {
        let f = FeatRevol::new("top_face", [0.0, 1.0, 0.0]);
        assert_eq!(f.face, "top_face");
        assert_eq!(f.axis, [0.0, 1.0, 0.0]);
        assert!(f.direction, "default direction must be fuse (true)");
    }

    #[test]
    fn feat_revol_is_done_valid() {
        let f = FeatRevol::new("face_0", [1.0, 0.0, 0.0]);
        assert!(f.is_done());
    }

    #[test]
    fn feat_revol_is_done_empty_face() {
        let f = FeatRevol::new("", [0.0, 0.0, 1.0]);
        assert!(!f.is_done());
    }

    #[test]
    fn feat_revol_build_nonempty_for_valid_face() {
        let token = FeatRevol::new("face_A", [0.0, 0.0, 1.0]).build();
        assert!(!token.is_empty());
    }

    #[test]
    fn feat_revol_build_empty_for_empty_face() {
        let token = FeatRevol::new("", [0.0, 0.0, 1.0]).build();
        assert!(token.is_empty());
    }

    #[test]
    fn feat_revol_build_contains_face_name() {
        let token = FeatRevol::new("my_face", [0.0, 1.0, 0.0]).build();
        assert!(token.contains("my_face"));
    }

    #[test]
    fn feat_revol_build_fuse_direction() {
        let mut f = FeatRevol::new("face_0", [0.0, 0.0, 1.0]);
        f.direction = true;
        let token = f.build();
        assert!(token.contains("fuse"));
    }

    #[test]
    fn feat_revol_build_cut_direction() {
        let mut f = FeatRevol::new("face_0", [0.0, 0.0, 1.0]);
        f.direction = false;
        let token = f.build();
        assert!(token.contains("cut"));
    }

    #[test]
    fn feat_revol_clone_is_independent() {
        let f = FeatRevol::new("face_0", [0.0, 0.0, 1.0]);
        let mut g = f.clone();
        g.face = String::new();
        assert!(f.is_done(), "clone mutation must not affect original");
    }

    // ── revolve ──────────────────────────────────────────────────────────────

    #[test]
    fn revolve_basic_returns_nonempty() {
        let token = revolve("wire_0", [0.0, 0.0, 1.0], PI);
        assert!(!token.is_empty());
    }

    #[test]
    fn revolve_empty_shape_returns_empty() {
        let token = revolve("", [0.0, 0.0, 1.0], PI);
        assert!(token.is_empty());
    }

    #[test]
    fn revolve_zero_angle_returns_empty() {
        let token = revolve("wire_0", [0.0, 0.0, 1.0], 0.0);
        assert!(token.is_empty());
    }

    #[test]
    fn revolve_full_turn() {
        let token = revolve("profile_A", [0.0, 1.0, 0.0], 2.0 * PI);
        assert!(token.contains("profile_A"));
    }

    #[test]
    fn revolve_axis_direction_does_not_affect_validity() {
        let t1 = revolve("p", [1.0, 0.0, 0.0], PI / 4.0);
        let t2 = revolve("p", [0.0, 1.0, 0.0], PI / 4.0);
        assert!(!t1.is_empty());
        assert!(!t2.is_empty());
    }
}

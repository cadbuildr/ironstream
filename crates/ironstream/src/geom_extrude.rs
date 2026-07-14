// FILE: src/geom_extrude.rs
//! `BRepPrimAPI_MakePrism` / `BRepFeat_MakePrism` parameter and result types —
//! zero-dependency Rust stub mirroring OpenCascade's linear-extrusion API.
//!
//! These types model the *inputs* to a prism/extrusion operation
//! ([`ExtrudeParams`]), the *outputs* ([`ExtrudeResult`]), a feature-level
//! builder ([`FeatExtrude`]) that mirrors `BRepFeat_MakePrism`, and a
//! convenience free function ([`extrude_profile`]).
//!
//! No tessellation or geometric computation is performed inside this module;
//! the shape field in [`ExtrudeResult`] is a descriptive string token that a
//! downstream stage can consume.  All implementations are `#[no_std]`-friendly
//! with respect to allocator choice — only `std::string::String` and primitive
//! arithmetic are used.

// ─────────────────────────────── ExtrudeParams ──────────────────────────────

// occt-ref: BRepPrimAPI_MakePrism
/// Parameters describing a linear extrusion (prism) operation.
///
/// Mirrors the constructor arguments of `BRepPrimAPI_MakePrism`: the source
/// profile, the extrusion direction (unit vector), the extrusion length, and
/// an optional flag for symmetric (both-directions) extrusion.
///
/// # Examples
///
/// ```
/// use ironstream::geom_extrude::ExtrudeParams;
///
/// let params = ExtrudeParams::new("sketch_0", [0.0, 0.0, 1.0], 10.0)
///     .both_directions(true);
/// assert!(params.both_dir);
/// assert!((params.length - 10.0).abs() < f64::EPSILON);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct ExtrudeParams {
    /// Identifier of the source profile (e.g. a sketch or wire name).
    pub profile: String,
    /// Unit direction vector of the extrusion.
    pub direction: [f64; 3],
    /// Extrusion length (always positive; sign is encoded by `direction`).
    pub length: f64,
    /// If `true` the profile is extruded symmetrically in both directions by
    /// `length / 2` on each side, mirroring `BRepPrimAPI_MakePrism` with the
    /// `Copy` flag.
    pub both_dir: bool,
}

impl ExtrudeParams {
    /// Construct extrusion parameters from a profile identifier, direction and
    /// length.
    ///
    /// `both_dir` defaults to `false`.
    ///
    /// # Arguments
    /// * `profile`   — profile identifier string.
    /// * `dir`       — extrusion direction (need not be normalised; stored
    ///                 as-is, normalisation is the caller's responsibility).
    /// * `len`       — extrusion length (absolute value is used).
    // occt-ref: BRepPrimAPI_MakePrism // ::BRepPrimAPI_MakePrism
    pub fn new(profile: &str, dir: [f64; 3], len: f64) -> Self {
        Self {
            profile: profile.to_owned(),
            direction: dir,
            length: len.abs(),
            both_dir: false,
        }
    }

    /// Set the symmetric extrusion flag and return `self` for builder-style
    /// chaining.
    ///
    /// When `b` is `true` the profile is extruded in both the positive and
    /// negative direction by `length / 2` each.
    // occt-ref: BRepPrimAPI_MakePrism // (Copy / both-directions flag)
    pub fn both_directions(mut self, b: bool) -> Self {
        self.both_dir = b;
        self
    }
}

// ─────────────────────────────── ExtrudeResult ──────────────────────────────

// occt-ref: BRepPrimAPI_MakePrism
/// Result produced by a linear extrusion build step.
///
/// Mirrors the shape-level output of `BRepPrimAPI_MakePrism::Shape()`:
/// a descriptive shape token and the computed (or estimated) volume.
///
/// A result is considered valid when `shape` is non-empty and `volume` is
/// strictly positive.
///
/// # Examples
///
/// ```
/// use ironstream::geom_extrude::ExtrudeResult;
///
/// let res = ExtrudeResult::new("solid_prism_0", 42.0);
/// assert!(res.is_valid());
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct ExtrudeResult {
    /// Descriptive token for the resulting solid shape (e.g. `"solid_prism_0"`).
    pub shape: String,
    /// Signed volume of the resulting shape.  Positive for outward-oriented
    /// solids, zero or negative indicates an invalid result.
    pub volume: f64,
}

impl ExtrudeResult {
    /// Construct an extrusion result from a shape token and volume.
    // occt-ref: BRepPrimAPI_MakePrism // ::Shape / IsDone
    pub fn new(shape: &str, volume: f64) -> Self {
        Self {
            shape: shape.to_owned(),
            volume,
        }
    }

    /// Return `true` when the result represents a geometrically valid solid.
    ///
    /// Validity requires that the shape token is non-empty **and** the volume
    /// is strictly positive, mirroring `BRepPrimAPI_MakePrism::IsDone()`.
    // occt-ref: BRepPrimAPI_MakePrism // ::IsDone
    pub fn is_valid(&self) -> bool {
        !self.shape.is_empty() && self.volume > 0.0
    }
}

// ─────────────────────────────── FeatExtrude ────────────────────────────────

// occt-ref: BRepFeat_MakePrism
/// Feature-level linear extrusion builder.
///
/// Mirrors `BRepFeat_MakePrism`: takes a face identifier, an extrusion
/// direction and a depth, then produces an [`ExtrudeResult`] via [`build`].
///
/// # Examples
///
/// ```
/// use ironstream::geom_extrude::FeatExtrude;
///
/// let result = FeatExtrude::new("top_face", [0.0, 0.0, 1.0], 5.0).build();
/// assert!(result.is_valid());
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct FeatExtrude {
    /// Identifier of the base face from which the extrusion starts.
    pub face: String,
    /// Extrusion direction vector.
    pub direction: [f64; 3],
    /// Extrusion depth (always positive).
    pub depth: f64,
}

impl FeatExtrude {
    /// Construct a feature extrusion builder.
    ///
    /// # Arguments
    /// * `face`  — identifier of the source face.
    /// * `dir`   — extrusion direction (stored as-is).
    /// * `depth` — extrusion depth; absolute value is used.
    // occt-ref: BRepFeat_MakePrism // ::BRepFeat_MakePrism
    pub fn new(face: &str, dir: [f64; 3], depth: f64) -> Self {
        Self {
            face: face.to_owned(),
            direction: dir,
            depth: depth.abs(),
        }
    }

    /// Build and return the extrusion result.
    ///
    /// Computes an estimated volume as the product of the face area proxy
    /// (derived from the face name hash, always 1.0 in this stub) and the
    /// extrusion depth, then returns an [`ExtrudeResult`].  A zero depth
    /// yields an invalid result.
    ///
    /// Mirrors `BRepFeat_MakePrism::Perform()` + `Shape()`.
    // occt-ref: BRepFeat_MakePrism // ::Shape
    pub fn build(self) -> ExtrudeResult {
        if self.depth < 1e-12 || self.face.is_empty() {
            return ExtrudeResult::new("", 0.0);
        }
        // Stub volume: unit face area × depth.  A real implementation would
        // integrate the actual face area from the BRep topology.
        let volume = self.depth;
        let shape = format!("feat_prism({})", self.face);
        ExtrudeResult::new(&shape, volume)
    }
}

// ─────────────────────────────── extrude_profile ────────────────────────────

/// Convenience function: extrude a named profile along `dir` by `len` and
/// return the resulting [`ExtrudeResult`].
///
/// Mirrors the one-shot usage pattern of `BRepPrimAPI_MakePrism` where the
/// caller supplies shape + vector and immediately queries the resulting solid.
///
/// # Arguments
/// * `profile` — profile identifier.
/// * `dir`     — extrusion direction.
/// * `len`     — extrusion length; absolute value is used internally.
///
/// # Examples
///
/// ```
/// use ironstream::geom_extrude::extrude_profile;
///
/// let res = extrude_profile("sketch_A", [0.0, 0.0, 1.0], 8.0);
/// assert!(res.is_valid());
/// assert!((res.volume - 8.0).abs() < 1e-12);
/// ```
// occt-ref: BRepPrimAPI_MakePrism // (free-function convenience wrapper)
pub fn extrude_profile(profile: &str, dir: [f64; 3], len: f64) -> ExtrudeResult {
    let len = len.abs();
    if profile.is_empty() || len < 1e-12 {
        return ExtrudeResult::new("", 0.0);
    }
    // Stub volume: unit cross-section area × length.
    let shape = format!("prism({})", profile);
    let _ = dir; // direction stored in shape token in a real implementation
    ExtrudeResult::new(&shape, len)
}

// ─────────────────────────────── Unit tests ─────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── ExtrudeParams ────────────────────────────────────────────────────────

    #[test]
    fn extrude_params_new_defaults() {
        let p = ExtrudeParams::new("wire_0", [1.0, 0.0, 0.0], 5.0);
        assert_eq!(p.profile, "wire_0");
        assert_eq!(p.direction, [1.0, 0.0, 0.0]);
        assert!((p.length - 5.0).abs() < f64::EPSILON);
        assert!(!p.both_dir);
    }

    #[test]
    fn extrude_params_length_is_absolute() {
        let p = ExtrudeParams::new("w", [0.0, 0.0, 1.0], -3.0);
        assert!((p.length - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn extrude_params_both_directions_builder() {
        let p = ExtrudeParams::new("profile", [0.0, 1.0, 0.0], 4.0)
            .both_directions(true);
        assert!(p.both_dir);
    }

    #[test]
    fn extrude_params_both_directions_false() {
        let p = ExtrudeParams::new("p", [1.0, 0.0, 0.0], 1.0)
            .both_directions(false);
        assert!(!p.both_dir);
    }

    #[test]
    fn extrude_params_clone_is_independent() {
        let p = ExtrudeParams::new("s", [0.0, 0.0, 1.0], 2.0);
        let mut q = p.clone();
        q.both_dir = true;
        assert!(!p.both_dir, "clone mutation must not affect original");
    }

    // ── ExtrudeResult ────────────────────────────────────────────────────────

    #[test]
    fn extrude_result_new_stores_fields() {
        let r = ExtrudeResult::new("solid_0", 12.5);
        assert_eq!(r.shape, "solid_0");
        assert!((r.volume - 12.5).abs() < f64::EPSILON);
    }

    #[test]
    fn extrude_result_is_valid_positive_volume() {
        let r = ExtrudeResult::new("solid_0", 1.0);
        assert!(r.is_valid());
    }

    #[test]
    fn extrude_result_is_invalid_zero_volume() {
        let r = ExtrudeResult::new("solid_0", 0.0);
        assert!(!r.is_valid());
    }

    #[test]
    fn extrude_result_is_invalid_negative_volume() {
        let r = ExtrudeResult::new("solid_0", -5.0);
        assert!(!r.is_valid());
    }

    #[test]
    fn extrude_result_is_invalid_empty_shape() {
        let r = ExtrudeResult::new("", 10.0);
        assert!(!r.is_valid());
    }

    #[test]
    fn extrude_result_is_invalid_empty_shape_and_zero_volume() {
        let r = ExtrudeResult::new("", 0.0);
        assert!(!r.is_valid());
    }

    // ── FeatExtrude ──────────────────────────────────────────────────────────

    #[test]
    fn feat_extrude_new_stores_fields() {
        let f = FeatExtrude::new("face_top", [0.0, 0.0, 1.0], 7.0);
        assert_eq!(f.face, "face_top");
        assert_eq!(f.direction, [0.0, 0.0, 1.0]);
        assert!((f.depth - 7.0).abs() < f64::EPSILON);
    }

    #[test]
    fn feat_extrude_depth_is_absolute() {
        let f = FeatExtrude::new("face", [1.0, 0.0, 0.0], -3.0);
        assert!((f.depth - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn feat_extrude_build_valid_result() {
        let result = FeatExtrude::new("face_0", [0.0, 0.0, 1.0], 5.0).build();
        assert!(result.is_valid());
        assert!((result.volume - 5.0).abs() < f64::EPSILON);
        assert_eq!(result.shape, "feat_prism(face_0)");
    }

    #[test]
    fn feat_extrude_build_zero_depth_invalid() {
        let result = FeatExtrude::new("face_0", [0.0, 0.0, 1.0], 0.0).build();
        assert!(!result.is_valid());
    }

    #[test]
    fn feat_extrude_build_empty_face_invalid() {
        let result = FeatExtrude::new("", [0.0, 0.0, 1.0], 4.0).build();
        assert!(!result.is_valid());
    }

    // ── extrude_profile ──────────────────────────────────────────────────────

    #[test]
    fn extrude_profile_basic() {
        let res = extrude_profile("sketch_A", [0.0, 0.0, 1.0], 8.0);
        assert!(res.is_valid());
        assert!((res.volume - 8.0).abs() < 1e-12);
        assert_eq!(res.shape, "prism(sketch_A)");
    }

    #[test]
    fn extrude_profile_length_is_absolute() {
        let res = extrude_profile("sketch_B", [0.0, 1.0, 0.0], -6.0);
        assert!(res.is_valid());
        assert!((res.volume - 6.0).abs() < 1e-12);
    }

    #[test]
    fn extrude_profile_empty_profile_invalid() {
        let res = extrude_profile("", [0.0, 0.0, 1.0], 5.0);
        assert!(!res.is_valid());
    }

    #[test]
    fn extrude_profile_zero_length_invalid() {
        let res = extrude_profile("sketch_C", [1.0, 0.0, 0.0], 0.0);
        assert!(!res.is_valid());
    }

    #[test]
    fn extrude_profile_direction_does_not_affect_volume() {
        let r1 = extrude_profile("p", [1.0, 0.0, 0.0], 3.0);
        let r2 = extrude_profile("p", [0.0, 1.0, 0.0], 3.0);
        assert!((r1.volume - r2.volume).abs() < 1e-12);
    }
}

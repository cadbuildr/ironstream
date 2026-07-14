// FILE: src/geom_revol.rs
//! `BRepPrimAPI_MakeRevol` parameter and result types — zero-dependency Rust
//! port of OpenCascade's revolution construction API.
//!
//! These types carry the *inputs* to a revolution operation ([`RevolParams`]),
//! the *outputs* ([`RevolResult`]), and a builder that ties them together
//! ([`MakeRevol`]).  No actual tessellation is performed here; geometry is
//! delegated to the caller via `nb_profile_edges`.

use std::f64::consts::TAU;

// ─────────────────────────────── RevolParams ────────────────────────────────

// occt-ref: BRepPrimAPI_MakeRevol // params
/// Parameters describing a revolution operation around an axis.
///
/// Mirrors the constructor arguments of `BRepPrimAPI_MakeRevol`:
/// axis direction, axis location (origin), sweep angle, and whether the
/// revolution is a full 360° sweep.
#[derive(Clone, Debug, PartialEq)]
pub struct RevolParams {
    /// Unit direction of the revolution axis.
    pub axis: [f64; 3],
    /// Origin point of the revolution axis.
    pub axis_origin: [f64; 3],
    /// Sweep angle in radians.  Clamped to `[0, 2π]` during construction.
    pub angle: f64,
    /// Whether this is a full 360° revolution.
    pub is_full_revolution: bool,
}

impl RevolParams {
    /// Construct revolution parameters from axis direction, axis origin and
    /// sweep angle (in radians).
    ///
    /// If `angle` is within floating-point tolerance of `2π` the revolution is
    /// automatically marked as full.
    pub fn new(axis: [f64; 3], axis_origin: [f64; 3], angle: f64) -> Self {
        let angle = angle.clamp(0.0, TAU);
        let is_full_revolution = (angle - TAU).abs() < 1e-10;
        Self {
            axis,
            axis_origin,
            angle,
            is_full_revolution,
        }
    }

    /// Mark the revolution as a full 360° sweep and set the angle to `2π`.
    pub fn set_full_revolution(&mut self) {
        self.is_full_revolution = true;
        self.angle = TAU;
    }

    /// Whether this revolution sweeps a full circle.
    pub fn is_full_revolution(&self) -> bool {
        self.is_full_revolution
    }

    /// Unit direction of the revolution axis.
    pub fn axis(&self) -> [f64; 3] {
        self.axis
    }

    /// Origin point of the revolution axis.
    pub fn axis_origin(&self) -> [f64; 3] {
        self.axis_origin
    }

    /// Sweep angle in radians.
    pub fn angle(&self) -> f64 {
        self.angle
    }
}

// ─────────────────────────────── RevolResult ────────────────────────────────

// occt-note: revol result
/// Result produced by a revolution build step.
///
/// Mirrors the shape-topology output of `BRepPrimAPI_MakeRevol::Build()`:
/// whether the operation succeeded, and the face/edge counts of the resulting
/// solid.
#[derive(Clone, Debug, PartialEq)]
pub struct RevolResult {
    /// Whether the build succeeded.
    pub is_done: bool,
    /// Number of faces in the revolved shape.
    pub nb_faces: usize,
    /// Number of edges in the revolved shape.
    pub nb_edges: usize,
}

impl RevolResult {
    /// Construct an empty (not-done) result.
    pub fn new() -> Self {
        Self {
            is_done: false,
            nb_faces: 0,
            nb_edges: 0,
        }
    }

    /// Record a successful build with the given face and edge counts.
    pub fn build(&mut self, nb_faces: usize, nb_edges: usize) {
        self.is_done = true;
        self.nb_faces = nb_faces;
        self.nb_edges = nb_edges;
    }

    /// Whether the build succeeded.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Number of faces in the revolved shape.
    pub fn nb_faces(&self) -> usize {
        self.nb_faces
    }

    /// Number of edges in the revolved shape.
    pub fn nb_edges(&self) -> usize {
        self.nb_edges
    }
}

impl Default for RevolResult {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────── MakeRevol ──────────────────────────────────

// occt-ref: BRepPrimAPI_MakeRevol
/// Builder for a solid of revolution.
///
/// Mirrors `BRepPrimAPI_MakeRevol`: takes [`RevolParams`] and computes topology
/// counts from the profile edge count and the sweep angle.
///
/// ## Face / edge count derivation
///
/// For a profile with `n` edges revolved by angle `θ`:
///
/// * Each profile edge sweeps out one lateral face → **`n` lateral faces**.
/// * Partial revolution (`θ < 2π`): two end caps (start and stop profiles) →
///   **`n + 2` total faces**, **`3·n + 1` total edges**.
/// * Full revolution (`θ = 2π`): no end caps → **`n` total faces** (lateral
///   only, profile is closed), **`2·n` total edges** (top/bottom rim circles +
///   lateral sweeps merge).
///
/// This mirrors the topology OCCT produces for a simple revolution of a planar
/// wire.
pub struct MakeRevol {
    /// Revolution parameters (axis, origin, angle).
    pub params: RevolParams,
    /// Result populated after [`build`](MakeRevol::build) is called.
    pub result: RevolResult,
}

impl MakeRevol {
    /// Construct the builder from [`RevolParams`].
    pub fn new(params: RevolParams) -> Self {
        Self {
            params,
            result: RevolResult::new(),
        }
    }

    /// Build the revolved shape for a profile with `nb_profile_edges` edges.
    ///
    /// Populates [`result`](MakeRevol::result) with computed face and edge counts.
    pub fn build(&mut self, nb_profile_edges: usize) {
        let n = nb_profile_edges;
        let (nb_faces, nb_edges) = if self.params.is_full_revolution() {
            // Full revolution: lateral faces only, profile seam closes.
            // faces = n lateral faces
            // edges = n sweep edges (one per profile edge swept) + n rim edges
            //         (top + bottom seam, but they merge in a full circle) = 2*n
            (n, 2 * n)
        } else {
            // Partial revolution: add start and end cap faces.
            // faces = n lateral + 2 caps
            // edges = n lateral sweep edges + 2 * n profile edges (start/stop)
            //         + 1 seam edge pair (top arc + bottom arc) = 3*n + 1 is
            //         a conventional OCCT count for a simple open wire.
            (n + 2, 3 * n + 1)
        };
        self.result.build(nb_faces, nb_edges);
    }

    /// Whether the build has completed successfully.
    pub fn is_done(&self) -> bool {
        self.result.is_done()
    }

    /// Borrow the result.
    pub fn result(&self) -> &RevolResult {
        &self.result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn params_new_stores_fields() {
        let p = RevolParams::new([0.0, 0.0, 1.0], [1.0, 2.0, 3.0], PI);
        assert_eq!(p.axis(), [0.0, 0.0, 1.0]);
        assert_eq!(p.axis_origin(), [1.0, 2.0, 3.0]);
        assert!((p.angle() - PI).abs() < 1e-15);
        assert!(!p.is_full_revolution());
    }

    #[test]
    fn params_full_revolution_auto_detect() {
        let p = RevolParams::new([0.0, 1.0, 0.0], [0.0; 3], TAU);
        assert!(p.is_full_revolution());
        assert!((p.angle() - TAU).abs() < 1e-15);
    }

    #[test]
    fn params_set_full_revolution() {
        let mut p = RevolParams::new([1.0, 0.0, 0.0], [0.0; 3], PI / 2.0);
        assert!(!p.is_full_revolution());
        p.set_full_revolution();
        assert!(p.is_full_revolution());
        assert!((p.angle() - TAU).abs() < 1e-15);
    }

    #[test]
    fn result_default_not_done() {
        let r = RevolResult::new();
        assert!(!r.is_done());
        assert_eq!(r.nb_faces(), 0);
        assert_eq!(r.nb_edges(), 0);
    }

    #[test]
    fn result_build_sets_done() {
        let mut r = RevolResult::new();
        r.build(6, 12);
        assert!(r.is_done());
        assert_eq!(r.nb_faces(), 6);
        assert_eq!(r.nb_edges(), 12);
    }

    #[test]
    fn make_revol_partial_counts() {
        let params = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], PI / 3.0);
        let mut builder = MakeRevol::new(params);
        assert!(!builder.is_done());
        builder.build(4);
        assert!(builder.is_done());
        // partial: faces = 4 + 2 = 6, edges = 3*4 + 1 = 13
        assert_eq!(builder.result().nb_faces(), 6);
        assert_eq!(builder.result().nb_edges(), 13);
    }

    #[test]
    fn make_revol_full_counts() {
        let params = RevolParams::new([0.0, 0.0, 1.0], [0.0; 3], TAU);
        let mut builder = MakeRevol::new(params);
        builder.build(4);
        // full: faces = 4, edges = 2*4 = 8
        assert_eq!(builder.result().nb_faces(), 4);
        assert_eq!(builder.result().nb_edges(), 8);
    }

    #[test]
    fn make_revol_exposes_params() {
        let params = RevolParams::new([0.0, 1.0, 0.0], [5.0, 0.0, 0.0], PI / 4.0);
        let builder = MakeRevol::new(params.clone());
        assert_eq!(builder.params.axis(), params.axis());
        assert_eq!(builder.params.axis_origin(), params.axis_origin());
        assert!((builder.params.angle() - PI / 4.0).abs() < 1e-15);
    }
}

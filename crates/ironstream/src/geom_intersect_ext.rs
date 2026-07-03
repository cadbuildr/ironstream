// FILE: src/geom_intersect_ext.rs
//! Extended surface/shape intersection stubs.
//!
//! Models the public API of two OCCT intersection subsystems:
//!
//! * [`ShapeIntersector`] — fires a ray at a named shape and collects 3-D hit
//!   points. Mirrors `IntCurvesFace_ShapeIntersector`.
//! * [`PatchIntersection`] — computes the intersection curve / point cloud of
//!   two parametric patches. Mirrors `IntPatch_Intersection`.
//!
//! Both types are pure stubs: no actual geometry kernel is linked; the
//! `perform` methods store synthetic results that downstream code can replace
//! with real computations.

// ─────────────────────────────────────────────────────────────────────────────
// ShapeIntersector
// ─────────────────────────────────────────────────────────────────────────────

/// Ray-vs-shape intersector.
///
/// Stores a shape handle (represented here as a `String` label), a positional
/// tolerance, and the 3-D intersection points accumulated after [`perform`] is
/// called.
///
// occt: IntCurvesFace_ShapeIntersector
#[derive(Debug, Clone)]
pub struct ShapeIntersector {
    /// Identifier / label of the target shape.
    pub shape: String,
    /// Positional tolerance used during intersection tests.
    pub tolerance: f64,
    /// Collected 3-D intersection points (world coordinates).
    pub points: Vec<[f64; 3]>,
    /// Whether the last `perform` call completed without error.
    done: bool,
}

impl ShapeIntersector {
    /// Construct a new intersector bound to `shape` with the given tolerance.
    ///
    /// No intersection is performed at construction time.
    pub fn new(shape: &str, tol: f64) -> Self {
        Self {
            shape: shape.to_owned(),
            tolerance: tol,
            points: Vec::new(),
            done: false,
        }
    }

    /// Shoot a ray from `pt` along direction `dir` and collect hits.
    ///
    /// In this stub the ray origin itself is recorded as the sole hit when
    /// `dir` is non-zero; real implementations replace this with actual
    /// geometric computations.
    pub fn perform(&mut self, pt: [f64; 3], dir: [f64; 3]) {
        self.points.clear();
        self.done = false;

        let len_sq = dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2];
        if len_sq < 1e-20 {
            // degenerate direction — nothing to intersect
            self.done = true;
            return;
        }

        // Stub: record the ray origin as the single intersection hit.
        self.points.push(pt);
        self.done = true;
    }

    /// Number of intersection points found by the last `perform` call.
    pub fn nb_points(&self) -> usize {
        self.points.len()
    }

    /// Return the `i`-th intersection point (0-based), or `None` if out of
    /// range.
    pub fn point(&self, i: usize) -> Option<[f64; 3]> {
        self.points.get(i).copied()
    }

    /// Returns `true` if the last `perform` call completed successfully.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PatchIntersection
// ─────────────────────────────────────────────────────────────────────────────

/// Surface-patch intersection solver.
///
/// Each intersection point is represented as a pair of 2-D parameter
/// coordinates: `([u1, v1], [u2, v2])` for the two input surfaces.
///
// occt: IntPatch_Intersection
#[derive(Debug, Clone)]
pub struct PatchIntersection {
    /// Parameter-pair intersection points: `(params_on_s1, params_on_s2)`.
    pub points: Vec<([f64; 2], [f64; 2])>,
    /// Positional tolerance used during the solve.
    pub tolerance: f64,
    /// Whether the last `perform` call completed.
    done: bool,
}

impl PatchIntersection {
    /// Construct a new patch intersector with the given tolerance.
    pub fn new(tol: f64) -> Self {
        Self {
            points: Vec::new(),
            tolerance: tol,
            done: false,
        }
    }

    /// Compute the intersection between two named patches `s1` and `s2`.
    ///
    /// In this stub, if both names are non-empty a single synthetic
    /// intersection point at `(0.5, 0.5)` on each surface is recorded.
    pub fn perform(&mut self, s1: &str, s2: &str) {
        self.points.clear();
        self.done = false;

        if !s1.is_empty() && !s2.is_empty() {
            // Stub: one synthetic intersection at the parametric mid-point of
            // both surfaces.
            self.points.push(([0.5, 0.5], [0.5, 0.5]));
        }

        self.done = true;
    }

    /// Number of intersection points found by the last `perform` call.
    pub fn nb_points(&self) -> usize {
        self.points.len()
    }

    /// Returns `true` if the last `perform` call completed successfully.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Free function: ray_shape_intersect
// ─────────────────────────────────────────────────────────────────────────────

/// Shoot a ray from `origin` along `dir` at `shape` and return all 3-D
/// intersection points.
///
/// Convenience wrapper around [`ShapeIntersector`] using a default tolerance
/// of `1e-7`.
pub fn ray_shape_intersect(origin: [f64; 3], dir: [f64; 3], shape: &str) -> Vec<[f64; 3]> {
    let mut intersector = ShapeIntersector::new(shape, 1e-7);
    intersector.perform(origin, dir);
    intersector.points.clone()
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_intersector_basic() {
        let mut si = ShapeIntersector::new("Box_1", 1e-6);
        assert_eq!(si.nb_points(), 0);
        assert!(!si.is_done());

        si.perform([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert!(si.is_done());
        assert_eq!(si.nb_points(), 1);
        assert_eq!(si.point(0), Some([0.0, 0.0, 0.0]));
        assert_eq!(si.point(1), None);
    }

    #[test]
    fn shape_intersector_degenerate_direction() {
        let mut si = ShapeIntersector::new("Sphere_1", 1e-6);
        si.perform([1.0, 2.0, 3.0], [0.0, 0.0, 0.0]);
        assert!(si.is_done());
        assert_eq!(si.nb_points(), 0);
    }

    #[test]
    fn patch_intersection_basic() {
        let mut pi = PatchIntersection::new(1e-6);
        assert_eq!(pi.nb_points(), 0);

        pi.perform("Plane_1", "Cylinder_1");
        assert!(pi.is_done());
        assert_eq!(pi.nb_points(), 1);
        let (p1, p2) = pi.points[0];
        assert_eq!(p1, [0.5, 0.5]);
        assert_eq!(p2, [0.5, 0.5]);
    }

    #[test]
    fn patch_intersection_empty_surface() {
        let mut pi = PatchIntersection::new(1e-6);
        pi.perform("", "Cylinder_1");
        assert!(pi.is_done());
        assert_eq!(pi.nb_points(), 0);
    }

    #[test]
    fn ray_shape_intersect_fn() {
        let hits = ray_shape_intersect([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], "Solid_1");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0], [0.0, 0.0, 0.0]);
    }

    #[test]
    fn ray_shape_intersect_degenerate() {
        let hits = ray_shape_intersect([1.0, 2.0, 3.0], [0.0, 0.0, 0.0], "Solid_1");
        assert!(hits.is_empty());
    }
}

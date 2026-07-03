// FILE: brep_proximity.rs
//! `BRepProximity` — shape-to-shape distance and proximity queries.
//!
//! # Scope
//!
//! * [`DistShapeShape`] — computes the minimum distance between two B-Rep
//!   shapes and enumerates the closest point pairs (solutions).  Models
//!   `BRepExtrema_DistShapeShape` from OpenCascade.
//!
//! * [`ShapeProximity`] — determines which sub-shapes (identified by integer
//!   indices) of two B-Rep shapes are within a given tolerance of each other,
//!   producing "overlap" index sets.  Models `BRepExtrema_ShapeProximity` from
//!   OpenCascade.
//!
//! # Notes
//!
//! This is a **stub** implementation.  `perform()` populates the result fields
//! with placeholder / identity values so that callers can compile and wire up
//! the API without a full geometric solver being present.  Replace the body of
//! `perform()` in each type with a real algorithm when geometry data is
//! available.
//!
//! No external crates are used; only `std`.

// ─────────────────────────────────────────────────────────────────────────────
// DistShapeShape
// ─────────────────────────────────────────────────────────────────────────────

/// Computes the minimum distance between two B-Rep shapes and collects every
/// pair of closest points (one on each shape) that achieves that minimum.
///
/// Each solution is stored as a pair of 3-D points `([f64; 3], [f64; 3])`,
/// where the first element is the point on `shape1` and the second is the
/// point on `shape2`.
///
/// Mirrors `BRepExtrema_DistShapeShape` from OpenCascade's `BRepExtrema`
/// package.
// occt: BRepExtrema_DistShapeShape
#[derive(Clone, Debug)]
pub struct DistShapeShape {
    /// Name / identifier of the first shape.
    pub shape1: String,
    /// Name / identifier of the second shape.
    pub shape2: String,
    /// Minimum distance between the two shapes, available after [`perform`]
    /// has been called and [`done`] is `true`.
    ///
    /// [`perform`]: DistShapeShape::perform
    /// [`done`]: DistShapeShape::done
    pub distance: f64,
    /// `true` once [`perform`] has finished successfully.
    ///
    /// [`perform`]: DistShapeShape::perform
    pub done: bool,
    /// All closest-point pairs `(pt_on_shape1, pt_on_shape2)`.
    pub solutions: Vec<([f64; 3], [f64; 3])>,
}

impl DistShapeShape {
    /// Construct a new solver for the two named shapes.
    ///
    /// [`perform`] must be called before any result accessor is used.
    ///
    /// [`perform`]: DistShapeShape::perform
    pub fn new(s1: &str, s2: &str) -> Self {
        Self {
            shape1: s1.to_owned(),
            shape2: s2.to_owned(),
            distance: f64::INFINITY,
            done: false,
            solutions: Vec::new(),
        }
    }

    /// Run the distance computation.
    ///
    /// After this call [`is_done`] returns `true` and the result accessors
    /// ([`value`], [`nb_solution`], [`point_on_shape1`], [`point_on_shape2`])
    /// are valid.
    ///
    /// **Stub behaviour**: inserts one dummy solution with both points at the
    /// origin and distance `0.0`.  Replace with a real geometric solver.
    ///
    /// [`is_done`]: DistShapeShape::is_done
    /// [`value`]: DistShapeShape::value
    /// [`nb_solution`]: DistShapeShape::nb_solution
    /// [`point_on_shape1`]: DistShapeShape::point_on_shape1
    /// [`point_on_shape2`]: DistShapeShape::point_on_shape2
    pub fn perform(&mut self) {
        // TODO: replace stub with real BRepExtrema_DistShapeShape algorithm.
        self.solutions.clear();
        // Stub: one solution — both points at the origin, distance = 0.
        self.solutions.push(([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]));
        self.distance = 0.0;
        self.done = true;
    }

    /// Returns `true` if [`perform`] completed successfully.
    ///
    /// [`perform`]: DistShapeShape::perform
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns the number of closest-point solution pairs found.
    ///
    /// # Panics
    ///
    /// Does not panic, but returns `0` if [`perform`] has not been called.
    pub fn nb_solution(&self) -> usize {
        self.solutions.len()
    }

    /// Returns the point on `shape1` for solution index `i` (0-based).
    ///
    /// # Panics
    ///
    /// Panics if `i >= nb_solution()`.
    pub fn point_on_shape1(&self, i: usize) -> [f64; 3] {
        self.solutions[i].0
    }

    /// Returns the point on `shape2` for solution index `i` (0-based).
    ///
    /// # Panics
    ///
    /// Panics if `i >= nb_solution()`.
    pub fn point_on_shape2(&self, i: usize) -> [f64; 3] {
        self.solutions[i].1
    }

    /// Returns the minimum distance between the two shapes.
    ///
    /// Valid only after a successful [`perform`] call.
    ///
    /// [`perform`]: DistShapeShape::perform
    pub fn value(&self) -> f64 {
        self.distance
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ShapeProximity
// ─────────────────────────────────────────────────────────────────────────────

/// Identifies which sub-shape elements (faces, indexed by an integer id) of
/// two B-Rep shapes are within `tolerance` distance of each other.
///
/// After [`perform`] is called, [`overlaps_1`] and [`overlaps_2`] return the
/// sets of element indices from `shape1` and `shape2` respectively that
/// participate in at least one overlapping pair.
///
/// Mirrors `BRepExtrema_ShapeProximity` from OpenCascade's `BRepExtrema`
/// package.
///
/// [`perform`]: ShapeProximity::perform
/// [`overlaps_1`]: ShapeProximity::overlaps_1
/// [`overlaps_2`]: ShapeProximity::overlaps_2
// occt: BRepExtrema_ShapeProximity
#[derive(Clone, Debug)]
pub struct ShapeProximity {
    /// Name / identifier of the first shape.
    pub shape1: String,
    /// Name / identifier of the second shape.
    pub shape2: String,
    /// Distance tolerance: two sub-shape elements are considered overlapping
    /// when their minimum separation is ≤ this value.
    pub tolerance: f64,

    /// Indices of elements in `shape1` that overlap with `shape2`.
    overlaps1: Vec<u32>,
    /// Indices of elements in `shape2` that overlap with `shape1`.
    overlaps2: Vec<u32>,
}

impl ShapeProximity {
    /// Construct a new proximity checker for the two named shapes with the
    /// given tolerance.
    ///
    /// [`perform`] must be called before the overlap accessors are used.
    ///
    /// [`perform`]: ShapeProximity::perform
    pub fn new(s1: &str, s2: &str, tol: f64) -> Self {
        Self {
            shape1: s1.to_owned(),
            shape2: s2.to_owned(),
            tolerance: tol,
            overlaps1: Vec::new(),
            overlaps2: Vec::new(),
        }
    }

    /// Run the proximity computation.
    ///
    /// After this call [`overlaps_1`] and [`overlaps_2`] return the
    /// overlapping element index sets.
    ///
    /// **Stub behaviour**: returns empty overlap sets (no elements overlap).
    /// Replace with a real BVH / BRepExtrema_ShapeProximity-style algorithm.
    ///
    /// [`overlaps_1`]: ShapeProximity::overlaps_1
    /// [`overlaps_2`]: ShapeProximity::overlaps_2
    pub fn perform(&mut self) {
        // TODO: replace stub with real BRepExtrema_ShapeProximity algorithm.
        self.overlaps1.clear();
        self.overlaps2.clear();
        // Stub: no overlapping elements.
    }

    /// Returns the indices of elements in `shape1` that are within
    /// [`tolerance`] of some element in `shape2`.
    ///
    /// Valid only after [`perform`] has been called.
    ///
    /// [`tolerance`]: ShapeProximity::tolerance
    /// [`perform`]: ShapeProximity::perform
    pub fn overlaps_1(&self) -> Vec<u32> {
        self.overlaps1.clone()
    }

    /// Returns the indices of elements in `shape2` that are within
    /// [`tolerance`] of some element in `shape1`.
    ///
    /// Valid only after [`perform`] has been called.
    ///
    /// [`tolerance`]: ShapeProximity::tolerance
    /// [`perform`]: ShapeProximity::perform
    pub fn overlaps_2(&self) -> Vec<u32> {
        self.overlaps2.clone()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dist_shape_shape_not_done_before_perform() {
        let d = DistShapeShape::new("box1", "box2");
        assert!(!d.is_done());
        assert_eq!(d.nb_solution(), 0);
        assert!(d.value().is_infinite());
    }

    #[test]
    fn dist_shape_shape_done_after_perform() {
        let mut d = DistShapeShape::new("box1", "box2");
        d.perform();
        assert!(d.is_done());
        assert_eq!(d.nb_solution(), 1);
        assert_eq!(d.value(), 0.0);
        assert_eq!(d.point_on_shape1(0), [0.0, 0.0, 0.0]);
        assert_eq!(d.point_on_shape2(0), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn shape_proximity_empty_overlaps_stub() {
        let mut p = ShapeProximity::new("sphere1", "sphere2", 1e-6);
        p.perform();
        assert!(p.overlaps_1().is_empty());
        assert!(p.overlaps_2().is_empty());
    }

    #[test]
    fn shape_proximity_fields() {
        let p = ShapeProximity::new("a", "b", 0.5);
        assert_eq!(p.shape1, "a");
        assert_eq!(p.shape2, "b");
        assert!((p.tolerance - 0.5).abs() < f64::EPSILON);
    }
}

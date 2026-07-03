// FILE: rust/ironstream/crates/ironstream/src/brep_pattern.rs
//! `BRepFeat` pattern types — linear, circular, mirror and scale patterns,
//! mirroring OpenCascade's `BRepFeat_MakePrism` (linear) and
//! `BRepFeat_MakeRevol` (circular) pattern semantics.
//!
//! # Overview
//!
//! * [`PatternType`]           — discriminant for which kind of pattern is stored.
//! * [`LinearPatternParams`]   — parameters for a linear (prismatic) pattern.
//! * [`CircularPatternParams`] — parameters for a circular (revolving) pattern.
//! * [`PatternResult`]         — collects the computed 4×4 homogeneous transforms
//!                               for every instance produced by a pattern.
//!
//! All arithmetic uses only `std`. No third-party crates are required.

use std::f64::consts::PI;

// ─────────────────────────────────────────────────────────────────────────────
// PatternType
// ─────────────────────────────────────────────────────────────────────────────

/// Discriminant for the kind of BRepFeat pattern being applied.
///
/// Maps to `BRepFeat_ModeType` / the sweep kind in OCCT's `BRepFeat` package.
// occt: pattern type — enum Linear, Circular, Mirror, Scale
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatternType {
    /// Linear (prismatic) pattern along a direction vector.
    Linear,
    /// Circular (revolving) pattern about an axis.
    Circular,
    /// Mirror pattern about a plane (normal stored externally).
    Mirror,
    /// Uniform-scale pattern about an origin point.
    Scale,
}

// ─────────────────────────────────────────────────────────────────────────────
// LinearPatternParams
// ─────────────────────────────────────────────────────────────────────────────

/// Parameters controlling a linear (prismatic) array pattern.
///
/// The pattern places `count` instances separated by `pitch` along `direction`.
/// Instance `i=0` is the original; instances `i=1..count-1` are translated by
/// `i * pitch` along the (normalised) `direction`.
// occt: BRepFeat_MakePrism linear pattern — fields: direction [f64;3], count u32, pitch f64
#[derive(Clone, Copy, Debug)]
pub struct LinearPatternParams {
    direction: [f64; 3],
    count: u32,
    pitch: f64,
}

impl LinearPatternParams {
    /// Create linear pattern parameters.
    ///
    /// * `direction` — pattern direction vector (need not be unit-length; it is
    ///   normalised internally before computing transforms).
    /// * `count`     — total number of instances, including the original (≥ 1).
    /// * `pitch`     — centre-to-centre spacing between consecutive instances,
    ///   in the same units as the model.
    // occt: BRepFeat_MakePrism linear pattern
    pub fn new(direction: [f64; 3], count: u32, pitch: f64) -> Self {
        Self {
            direction,
            count,
            pitch,
        }
    }

    /// Return the raw (possibly non-unit) direction vector as stored.
    pub fn direction(&self) -> [f64; 3] {
        self.direction
    }

    /// Total number of instances (including the seed at index 0).
    pub fn count(&self) -> u32 {
        self.count
    }

    /// Centre-to-centre spacing between consecutive instances.
    pub fn pitch(&self) -> f64 {
        self.pitch
    }

    /// Total span of the pattern: `(count - 1) * pitch`.
    ///
    /// Returns `0.0` when `count` is 0 or 1.
    pub fn total_length(&self) -> f64 {
        if self.count == 0 {
            return 0.0;
        }
        (self.count.saturating_sub(1) as f64) * self.pitch
    }

    /// Return the unit direction, or `[1.0, 0.0, 0.0]` if the stored vector
    /// has zero length.
    fn unit_direction(&self) -> [f64; 3] {
        let [x, y, z] = self.direction;
        let len = (x * x + y * y + z * z).sqrt();
        if len < 1e-12 {
            [1.0, 0.0, 0.0]
        } else {
            [x / len, y / len, z / len]
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CircularPatternParams
// ─────────────────────────────────────────────────────────────────────────────

/// Parameters controlling a circular (revolving) array pattern.
///
/// The pattern places `count` instances evenly distributed over `angle`
/// (or `2π` when `is_full_rotation` is set) about `axis`.
// occt: BRepFeat_MakeRevol circular pattern — fields: axis [f64;3], count u32, angle f64, is_full_rotation bool
#[derive(Clone, Copy, Debug)]
pub struct CircularPatternParams {
    axis: [f64; 3],
    count: u32,
    angle: f64,
    is_full_rotation: bool,
}

impl CircularPatternParams {
    /// Create circular pattern parameters.
    ///
    /// * `axis`  — rotation axis direction (need not be unit-length).
    /// * `count` — total number of instances (≥ 1).
    /// * `angle` — total angular span in radians. Use `2*PI` for a full turn.
    // occt: BRepFeat_MakeRevol circular pattern
    pub fn new(axis: [f64; 3], count: u32, angle: f64) -> Self {
        let full = (angle.abs() - 2.0 * PI).abs() < 1e-9 || angle.abs() >= 2.0 * PI;
        Self {
            axis,
            count,
            angle,
            is_full_rotation: full,
        }
    }

    /// Override the full-rotation flag.
    ///
    /// When `true`, the angular step between instances is always `2π / count`
    /// and the last instance coincides with the first (closed ring).
    pub fn set_full_rotation(&mut self, full: bool) {
        self.is_full_rotation = full;
    }

    /// Whether the pattern covers a full `2π` revolution.
    pub fn is_full_rotation(&self) -> bool {
        self.is_full_rotation
    }

    /// Rotation axis direction vector (raw, not necessarily unit-length).
    pub fn axis(&self) -> [f64; 3] {
        self.axis
    }

    /// Total number of instances.
    pub fn count(&self) -> u32 {
        self.count
    }

    /// Total angular span in radians as supplied at construction.
    pub fn angle(&self) -> f64 {
        self.angle
    }

    /// Effective angular step between consecutive instances.
    ///
    /// For a full rotation the step is `2π / count`; otherwise it is
    /// `angle / (count - 1)` (or `0` when `count <= 1`).
    fn step_angle(&self) -> f64 {
        if self.count <= 1 {
            return 0.0;
        }
        if self.is_full_rotation {
            2.0 * PI / self.count as f64
        } else {
            self.angle / (self.count - 1) as f64
        }
    }

    /// Return the unit axis, or `[0.0, 0.0, 1.0]` when stored vector is zero.
    fn unit_axis(&self) -> [f64; 3] {
        let [x, y, z] = self.axis;
        let len = (x * x + y * y + z * z).sqrt();
        if len < 1e-12 {
            [0.0, 0.0, 1.0]
        } else {
            [x / len, y / len, z / len]
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4×4 matrix helpers (row-major, homogeneous)
// ─────────────────────────────────────────────────────────────────────────────

/// Identity 4×4 matrix (row-major homogeneous).
fn identity4() -> [[f64; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// Build a 4×4 translation matrix for displacement `(tx, ty, tz)`.
fn translation4(tx: f64, ty: f64, tz: f64) -> [[f64; 4]; 4] {
    [
        [1.0, 0.0, 0.0, tx],
        [0.0, 1.0, 0.0, ty],
        [0.0, 0.0, 1.0, tz],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// Build a 4×4 rotation matrix about unit axis `(ux, uy, uz)` by `theta`
/// radians (Rodrigues / right-hand rule).
fn rotation4(ux: f64, uy: f64, uz: f64, theta: f64) -> [[f64; 4]; 4] {
    let c = theta.cos();
    let s = theta.sin();
    let t = 1.0 - c;
    [
        [
            t * ux * ux + c,
            t * ux * uy - s * uz,
            t * ux * uz + s * uy,
            0.0,
        ],
        [
            t * ux * uy + s * uz,
            t * uy * uy + c,
            t * uy * uz - s * ux,
            0.0,
        ],
        [
            t * ux * uz - s * uy,
            t * uy * uz + s * ux,
            t * uz * uz + c,
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

// ─────────────────────────────────────────────────────────────────────────────
// PatternResult
// ─────────────────────────────────────────────────────────────────────────────

/// Collects the per-instance transforms produced by a [`PatternType`] operation.
///
/// Each transform is a 4×4 row-major homogeneous matrix. Index `0` always
/// holds the identity (the original / seed instance).
// occt: pattern shape results — fields: pattern_type PatternType, nb_instances u32, transforms Vec<[[f64;4];4]>, is_done bool
#[derive(Clone, Debug)]
pub struct PatternResult {
    /// Which kind of pattern produced this result.
    pattern_type: PatternType,
    /// Number of instances actually placed (equals `transforms.len()` after
    /// a successful `perform_*` call).
    nb_instances: u32,
    /// Per-instance homogeneous transforms (row-major, 4×4).
    transforms: Vec<[[f64; 4]; 4]>,
    /// `true` after a successful `perform_linear` or `perform_circular`.
    is_done: bool,
}

impl PatternResult {
    /// Create an empty, not-done result for the given pattern kind.
    // occt: pattern shape results
    pub fn new(pattern_type: PatternType) -> Self {
        Self {
            pattern_type,
            nb_instances: 0,
            transforms: Vec::new(),
            is_done: false,
        }
    }

    /// `true` after a successful `perform_linear` or `perform_circular` call.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Number of instances in the result (0 until `perform_*` succeeds).
    pub fn nb_instances(&self) -> u32 {
        self.nb_instances
    }

    /// Compute and store transforms for a linear pattern.
    ///
    /// Instance `i` is translated by `i * pitch` along the (normalised)
    /// direction. Instance `0` is always the identity.
    ///
    /// Mirrors `BRepFeat_MakePrism` linear array semantics.
    // occt: BRepFeat_MakePrism linear pattern
    pub fn perform_linear(&mut self, params: &LinearPatternParams) {
        self.transforms.clear();
        self.is_done = false;
        self.nb_instances = 0;

        let count = params.count as usize;
        if count == 0 {
            return;
        }

        let [ux, uy, uz] = params.unit_direction();
        let p = params.pitch;

        for i in 0..count {
            let d = i as f64 * p;
            self.transforms.push(translation4(ux * d, uy * d, uz * d));
        }

        self.nb_instances = count as u32;
        self.is_done = true;
    }

    /// Compute and store transforms for a circular pattern.
    ///
    /// Instance `i` is rotated by `i * step_angle` about the axis.
    /// Instance `0` is always the identity.
    ///
    /// Mirrors `BRepFeat_MakeRevol` circular array semantics.
    // occt: BRepFeat_MakeRevol circular pattern
    pub fn perform_circular(&mut self, params: &CircularPatternParams) {
        self.transforms.clear();
        self.is_done = false;
        self.nb_instances = 0;

        let count = params.count as usize;
        if count == 0 {
            return;
        }

        let [ux, uy, uz] = params.unit_axis();
        let step = params.step_angle();

        for i in 0..count {
            let theta = i as f64 * step;
            self.transforms.push(rotation4(ux, uy, uz, theta));
        }

        self.nb_instances = count as u32;
        self.is_done = true;
    }

    /// Return the homogeneous 4×4 transform for instance `i`.
    ///
    /// Returns the identity matrix when `i >= nb_instances`.
    pub fn transform(&self, i: usize) -> [[f64; 4]; 4] {
        self.transforms.get(i).copied().unwrap_or_else(identity4)
    }

    /// The pattern kind stored in this result.
    pub fn pattern_type(&self) -> PatternType {
        self.pattern_type
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn near(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
    }

    // ── PatternType ───────────────────────────────────────────────────────────

    #[test]
    fn pattern_type_variants_are_copy() {
        let t = PatternType::Linear;
        let t2 = t; // Copy
        assert_eq!(t, t2);
    }

    // ── LinearPatternParams ───────────────────────────────────────────────────

    #[test]
    fn linear_params_accessors() {
        let p = LinearPatternParams::new([1.0, 0.0, 0.0], 5, 2.0);
        assert_eq!(p.count(), 5);
        near(p.pitch(), 2.0, 1e-12);
        near(p.total_length(), 8.0, 1e-12); // (5-1)*2
        assert_eq!(p.direction(), [1.0, 0.0, 0.0]);
    }

    #[test]
    fn linear_total_length_single_instance() {
        let p = LinearPatternParams::new([0.0, 1.0, 0.0], 1, 5.0);
        near(p.total_length(), 0.0, 1e-12);
    }

    #[test]
    fn linear_total_length_zero_count() {
        let p = LinearPatternParams::new([0.0, 0.0, 1.0], 0, 3.0);
        near(p.total_length(), 0.0, 1e-12);
    }

    // ── CircularPatternParams ─────────────────────────────────────────────────

    #[test]
    fn circular_params_accessors() {
        let p = CircularPatternParams::new([0.0, 0.0, 1.0], 4, PI / 2.0);
        assert_eq!(p.count(), 4);
        near(p.angle(), PI / 2.0, 1e-12);
        assert_eq!(p.axis(), [0.0, 0.0, 1.0]);
        // angle != 2pi, so not full rotation
        assert!(!p.is_full_rotation());
    }

    #[test]
    fn circular_full_rotation_detected_from_angle() {
        let p = CircularPatternParams::new([0.0, 0.0, 1.0], 6, 2.0 * PI);
        assert!(p.is_full_rotation());
    }

    #[test]
    fn circular_set_full_rotation() {
        let mut p = CircularPatternParams::new([0.0, 0.0, 1.0], 4, PI / 2.0);
        assert!(!p.is_full_rotation());
        p.set_full_rotation(true);
        assert!(p.is_full_rotation());
    }
}

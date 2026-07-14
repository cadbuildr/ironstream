// FILE: rust/ironstream/crates/ironstream/src/geom_helix.rs
//! Helix (screw) curve helpers — a faithful reproduction of the helix geometry
//! used in OpenCascade's `BRepBuilderAPI` helix construction
//! (`src/ModelingAlgo/TKPrim/BRepPrimAPI/BRepPrimAPI_MakeRevol` and related).
//!
//! A right-handed helix with radius `r`, pitch `p` (axial advance per turn),
//! total height `h`, and start angle `a0` is parameterised by `t ∈ [0, 1]`:
//!
//! ```text
//! n = h / p                          (number of turns)
//! x(t) =  r * cos(2π·n·t + a0)
//! y(t) =  r * sin(2π·n·t + a0)      (negated for left-handed)
//! z(t) =  h * t
//! ```
//!
//! Zero third-party dependencies — only `std`.

use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// HelixParams
// ---------------------------------------------------------------------------

/// Parameters that define a helix / spiral curve.
///
/// occt-note: helix parameters used in BRepBuilderAPI — fields: radius f64, pitch f64,
/// height f64, start_angle f64, left_handed bool
// occt-note: helix parameters used in BRepBuilderAPI
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HelixParams {
    /// Radius of the helix (distance from the axis).
    radius: f64,
    /// Axial advance per full turn (pitch).
    pitch: f64,
    /// Total axial height of the helix.
    height: f64,
    /// Start angle in radians (offset at t = 0).
    start_angle: f64,
    /// `true` for a left-handed (counterclockwise-ascending) helix.
    left_handed: bool,
}

impl HelixParams {
    /// Construct new helix parameters with `start_angle = 0` and right-handed
    /// orientation.
    ///
    /// # Panics
    /// Panics if `radius <= 0`, `pitch <= 0`, or `height <= 0`.
    pub fn new(radius: f64, pitch: f64, height: f64) -> Self {
        assert!(radius > 0.0, "HelixParams: radius must be > 0");
        assert!(pitch > 0.0, "HelixParams: pitch must be > 0");
        assert!(height > 0.0, "HelixParams: height must be > 0");
        Self {
            radius,
            pitch,
            height,
            start_angle: 0.0,
            left_handed: false,
        }
    }

    /// Set the start angle (radians).
    pub fn set_start_angle(&mut self, angle: f64) {
        self.start_angle = angle;
    }

    /// Set the handedness. `true` → left-handed, `false` → right-handed.
    pub fn set_left_handed(&mut self, left_handed: bool) {
        self.left_handed = left_handed;
    }

    /// Radius of the helix.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Pitch (axial advance per turn).
    pub fn pitch(&self) -> f64 {
        self.pitch
    }

    /// Total axial height.
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Number of turns: `height / pitch`.
    pub fn nb_turns(&self) -> f64 {
        self.height / self.pitch
    }

    /// `true` if the helix is left-handed.
    pub fn is_left_handed(&self) -> bool {
        self.left_handed
    }
}

// ---------------------------------------------------------------------------
// HelixCurve
// ---------------------------------------------------------------------------

/// A sampled helix (screw) curve stored as a sequence of 3-D poles.
///
/// occt-note: helix (screw curve) — mirrors the discretised 3-D polyline that OCCT
/// builds when sweeping a profile along a helix spine in `BRepBuilderAPI`.
// occt-note: helix (screw curve)
#[derive(Clone, Debug)]
pub struct HelixCurve {
    /// Parameters used to construct this curve.
    params: HelixParams,
    /// Requested number of poles (as supplied to `new`).
    nb_poles: u32,
    /// Uniformly-sampled points in parameter `t ∈ [0, 1]`.
    poles: Vec<[f64; 3]>,
}

impl HelixCurve {
    /// Construct a new `HelixCurve` by sampling `nb_poles` points evenly along
    /// the helix (parameter `t ∈ [0, 1]`).
    ///
    /// # Panics
    /// Panics if `nb_poles < 2`.
    pub fn new(params: HelixParams, nb_poles: u32) -> Self {
        assert!(nb_poles >= 2, "HelixCurve: nb_poles must be >= 2");

        let n = nb_poles as usize;
        let mut poles = Vec::with_capacity(n);
        for i in 0..n {
            let t = (i as f64) / ((n - 1) as f64);
            poles.push(Self::evaluate_with_params(&params, t));
        }

        Self { params, nb_poles, poles }
    }

    /// Evaluate the helix at parameter `t ∈ [0, 1]`.
    ///
    /// Returns `[r·cos(2π·n·t + a0), r·sin(2π·n·t + a0)·sign, h·t]`
    /// where `sign = -1` for left-handed and `+1` for right-handed,
    /// `n = nb_turns`, `a0 = start_angle`, `h = height`.
    pub fn evaluate(&self, t: f64) -> [f64; 3] {
        Self::evaluate_with_params(&self.params, t)
    }

    /// Number of sampled poles.
    pub fn nb_poles(&self) -> usize {
        self.poles.len()
    }

    /// Return the `i`-th sampled pole.
    ///
    /// # Panics
    /// Panics if `i >= self.nb_poles()`.
    pub fn pole(&self, i: usize) -> [f64; 3] {
        self.poles[i]
    }

    /// Analytical arc length of the helix: `sqrt((2π·r·n)² + h²)`.
    ///
    /// This is the exact closed-form result, not a chord-length approximation.
    pub fn arc_length(&self) -> f64 {
        let r = self.params.radius;
        let n = self.params.nb_turns();
        let h = self.params.height;
        let circumference = 2.0 * PI * r * n;
        circumference.hypot(h)
    }

    // Internal helper shared by `new` and `evaluate`.
    fn evaluate_with_params(p: &HelixParams, t: f64) -> [f64; 3] {
        let n = p.nb_turns();
        let theta = 2.0 * PI * n * t + p.start_angle;
        let y_sign = if p.left_handed { -1.0 } else { 1.0 };
        [
            p.radius * theta.cos(),
            p.radius * theta.sin() * y_sign,
            p.height * t,
        ]
    }
}

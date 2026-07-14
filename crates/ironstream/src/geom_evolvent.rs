//! Discretisation stubs for curves by deflection criteria.
//!
//! Models after OpenCascade's `GCPnts_UniformDeflection` and
//! `GCPnts_TangentialDeflection`.  This file contains zero-dependency,
//! pure-Rust stubs that mirror the public API of those OCCT classes without
//! binding to the C++ library.
//!
//! An *evolvent* (involute) of a circle with base radius `r` is traced by the
//! end of a taut string unwound from the circle:
//!
//! ```text
//! P(t) = [r*(cos(t) + t*sin(t)),  r*(sin(t) - t*cos(t))]
//! ```
//!
//! Zero third-party dependencies — only `std`.

// ---------------------------------------------------------------------------
// UniformDeflection
// ---------------------------------------------------------------------------

/// Discretises a curve so that the chordal deflection between successive
/// sample points does not exceed a given tolerance.
///
// occt-ref: GCPnts_UniformDeflection
#[derive(Clone, Debug)]
pub struct UniformDeflection {
    /// Parameter values at each sampled point.
    pub params: Vec<f64>,
    /// 3-D coordinates at each sampled point.
    pub points: Vec<[f64; 3]>,
    /// `true` when the algorithm has produced a valid result.
    pub done: bool,
}

impl UniformDeflection {
    /// Build a uniform-deflection discretisation of `curve`.
    ///
    /// This stub treats `curve` as a name tag only; it samples the unit
    /// parametric interval `[0, 1]` so that successive chord lengths stay
    /// at or below `deflection`.  Points are placed on the unit line
    /// `(t, 0, 0)` as a placeholder for real curve evaluation.
    ///
    /// # Arguments
    /// * `curve`      — identifies the curve (stub: not evaluated).
    /// * `deflection` — maximum allowed chordal deviation (> 0).
    /// * `tol`        — parametric tolerance; samples closer than `tol`
    ///                  apart are merged.
    pub fn new(curve: &str, deflection: f64, tol: f64) -> Self {
        let _ = curve;

        if deflection <= 0.0 || tol < 0.0 {
            return Self {
                params: Vec::new(),
                points: Vec::new(),
                done: false,
            };
        }

        // Stub: sample [0, 1] with step = deflection, clamped by tol.
        let step = deflection.max(tol);
        let n_intervals = (1.0_f64 / step).ceil() as usize;
        let nb = n_intervals + 1;

        let mut params = Vec::with_capacity(nb);
        let mut points = Vec::with_capacity(nb);

        for i in 0..nb {
            let t = (i as f64) / (n_intervals as f64);
            params.push(t);
            // Stub geometry: point lies on the unit X-axis at parameter t.
            points.push([t, 0.0, 0.0]);
        }

        Self {
            params,
            points,
            done: true,
        }
    }

    /// Number of sampled points produced by the algorithm.
    pub fn nb_points(&self) -> usize {
        self.params.len()
    }

    /// Return the parameter value at the `i`-th sample (0-based).
    ///
    /// # Panics
    /// Panics if `i >= self.nb_points()`.
    pub fn parameter(&self, i: usize) -> f64 {
        self.params[i]
    }

    /// Return the 3-D point at the `i`-th sample (0-based).
    ///
    /// # Panics
    /// Panics if `i >= self.nb_points()`.
    pub fn value(&self, i: usize) -> [f64; 3] {
        self.points[i]
    }

    /// Return `true` when the algorithm completed successfully.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ---------------------------------------------------------------------------
// TangentialDeflection
// ---------------------------------------------------------------------------

/// Discretises a curve so that both the angular deviation between successive
/// tangent directions and the chordal deviation remain within given bounds.
///
// occt: GCPnts_TangentialDeflection
#[derive(Clone, Debug)]
pub struct TangentialDeflection {
    /// Parameter values at each sampled point.
    pub params: Vec<f64>,
    /// 3-D coordinates at each sampled point.
    pub points: Vec<[f64; 3]>,
    /// `true` when the algorithm has produced a valid result.
    pub done: bool,
}

impl TangentialDeflection {
    /// Build a tangential-deflection discretisation of `curve`.
    ///
    /// This stub samples the unit interval `[0, 1]` using the smaller of
    /// `angle_def` and `chord_def` as the step size.  Points are placed on
    /// the unit line `(t, 0, 0)` as a placeholder for real curve evaluation.
    ///
    /// # Arguments
    /// * `curve`     — identifies the curve (stub: not evaluated).
    /// * `angle_def` — maximum angular deflection between tangents (radians).
    /// * `chord_def` — maximum chordal deflection.
    pub fn new(curve: &str, angle_def: f64, chord_def: f64) -> Self {
        let _ = curve;

        if angle_def <= 0.0 || chord_def <= 0.0 {
            return Self {
                params: Vec::new(),
                points: Vec::new(),
                done: false,
            };
        }

        let step = angle_def.min(chord_def);
        let n_intervals = (1.0_f64 / step).ceil() as usize;
        let nb = n_intervals + 1;

        let mut params = Vec::with_capacity(nb);
        let mut points = Vec::with_capacity(nb);

        for i in 0..nb {
            let t = (i as f64) / (n_intervals as f64);
            params.push(t);
            points.push([t, 0.0, 0.0]);
        }

        Self {
            params,
            points,
            done: true,
        }
    }

    /// Number of sampled points produced by the algorithm.
    pub fn nb_points(&self) -> usize {
        self.params.len()
    }

    /// Return the parameter value at the `i`-th sample (0-based).
    ///
    /// # Panics
    /// Panics if `i >= self.nb_points()`.
    pub fn parameter(&self, i: usize) -> f64 {
        self.params[i]
    }

    /// Return the 3-D point at the `i`-th sample (0-based).
    ///
    /// # Panics
    /// Panics if `i >= self.nb_points()`.
    pub fn value(&self, i: usize) -> [f64; 3] {
        self.points[i]
    }

    /// Return `true` when the algorithm completed successfully.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Append an extra sample point at `param` with coordinates `pt`.
    ///
    /// Mirrors `GCPnts_TangentialDeflection::AddPoint` which lets callers
    /// inject specific parameters (e.g. discontinuities) into the sequence.
    pub fn add_point(&mut self, param: f64, pt: [f64; 3]) {
        // Insert in sorted order so params remains monotone.
        let pos = self.params.partition_point(|&p| p < param);
        self.params.insert(pos, param);
        self.points.insert(pos, pt);
    }
}

// ---------------------------------------------------------------------------
// Free function
// ---------------------------------------------------------------------------

/// Sample `curve` by deflection, returning at least `n_min` 3-D points.
///
/// The function uses [`UniformDeflection`] under the hood and then
/// guarantees that at least `n_min` points are present by re-sampling with
/// a finer deflection when necessary.
///
/// # Arguments
/// * `curve`      — identifies the curve (stub: not evaluated).
/// * `deflection` — maximum chordal deviation between successive points.
/// * `n_min`      — minimum number of points to return (>= 1).
pub fn sample_by_deflection(curve: &str, deflection: f64, n_min: usize) -> Vec<[f64; 3]> {
    let n_min = n_min.max(1);

    let ud = UniformDeflection::new(curve, deflection, 0.0);
    if ud.is_done() && ud.nb_points() >= n_min {
        return ud.points;
    }

    // Re-sample with a finer step to meet the n_min requirement.
    // We need at least n_min points spanning [0, 1], so we need
    // (n_min - 1) intervals, meaning step = 1 / (n_min - 1).
    let step = if n_min <= 1 {
        1.0
    } else {
        1.0 / ((n_min - 1) as f64)
    };

    let finer_deflection = step.min(deflection.abs().max(f64::MIN_POSITIVE));
    let ud2 = UniformDeflection::new(curve, finer_deflection, 0.0);
    if ud2.is_done() {
        ud2.points
    } else {
        // Fallback: uniform spacing with exactly n_min points.
        (0..n_min)
            .map(|i| {
                let t = if n_min == 1 {
                    0.0
                } else {
                    (i as f64) / ((n_min - 1) as f64)
                };
                [t, 0.0, 0.0]
            })
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Involute (evolvent) of a circle
// ---------------------------------------------------------------------------

/// Compute the 2-D involute point at parameter `t` on the circle with base
/// radius `r`.
///
/// ```text
/// P(t) = [r*(cos(t) + t*sin(t)),  r*(sin(t) - t*cos(t))]
/// ```
pub fn involute_point(r: f64, t: f64) -> [f64; 2] {
    [
        r * (t.cos() + t * t.sin()),
        r * (t.sin() - t * t.cos()),
    ]
}

/// Compute the 2-D tangent vector of the involute at parameter `t`.
///
/// ```text
/// P'(t) = [r*t*cos(t),  r*t*sin(t)]
/// ```
pub fn involute_tangent(r: f64, t: f64) -> [f64; 2] {
    [r * t * t.cos(), r * t * t.sin()]
}

/// Parameters for constructing an [`InvoluteCurve`].
#[derive(Clone, Debug)]
pub struct InvoluteParams {
    base_radius: f64,
    start_angle: f64,
    end_angle: f64,
    nb_poles: u32,
}

impl InvoluteParams {
    /// Create a new parameter block.
    ///
    /// # Panics
    /// * `base_radius <= 0`
    /// * `nb_poles < 2`
    /// * `start_angle == end_angle`
    pub fn new(base_radius: f64, start_angle: f64, end_angle: f64, nb_poles: u32) -> Self {
        assert!(base_radius > 0.0, "base_radius must be > 0");
        assert!(nb_poles >= 2, "nb_poles must be >= 2");
        assert!(
            (start_angle - end_angle).abs() > f64::EPSILON,
            "start_angle and end_angle must differ"
        );
        Self { base_radius, start_angle, end_angle, nb_poles }
    }

    pub fn base_radius(&self) -> f64 { self.base_radius }
    pub fn start_angle(&self) -> f64 { self.start_angle }
    pub fn end_angle(&self) -> f64 { self.end_angle }
    pub fn nb_poles(&self) -> u32 { self.nb_poles }
}

/// Discretised involute (evolvent) of a circle, stored as a sequence of 2-D
/// poles uniformly spaced in parameter.
///
// occt-ref: Geom_Curve // (involute specialisation)
#[derive(Clone, Debug)]
pub struct InvoluteCurve {
    base_radius: f64,
    poles: Vec<[f64; 2]>,
}

impl InvoluteCurve {
    /// Build an involute curve from the given [`InvoluteParams`].
    pub fn new(params: InvoluteParams) -> Self {
        let r  = params.base_radius;
        let t0 = params.start_angle;
        let t1 = params.end_angle;
        let n  = params.nb_poles as usize;

        let mut poles = Vec::with_capacity(n);
        for i in 0..n {
            let t = t0 + (t1 - t0) * (i as f64) / ((n - 1) as f64);
            poles.push(involute_point(r, t));
        }

        Self { base_radius: r, poles }
    }

    /// Number of poles (sampled positions).
    pub fn nb_poles(&self) -> usize { self.poles.len() }

    /// Return the `i`-th pole (0-based).
    pub fn pole(&self, i: usize) -> [f64; 2] { self.poles[i] }

    /// Evaluate the involute at an arbitrary parameter `t`.
    pub fn evaluate(&self, t: f64) -> [f64; 2] {
        involute_point(self.base_radius, t)
    }

    /// Approximate arc length as the sum of chord lengths between consecutive
    /// poles.
    pub fn arc_length(&self) -> f64 {
        self.poles.windows(2).map(|w| {
            let dx = w[1][0] - w[0][0];
            let dy = w[1][1] - w[0][1];
            dx.hypot(dy)
        }).sum()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_deflection_basic() {
        let ud = UniformDeflection::new("line", 0.5, 0.0);
        assert!(ud.is_done());
        // deflection 0.5 over [0,1] → ceil(1/0.5)=2 intervals → 3 points
        assert_eq!(ud.nb_points(), 3);
        assert!((ud.parameter(0) - 0.0).abs() < 1e-12);
        assert!((ud.parameter(1) - 0.5).abs() < 1e-12);
        assert!((ud.parameter(2) - 1.0).abs() < 1e-12);
        assert_eq!(ud.value(0), [0.0, 0.0, 0.0]);
        assert_eq!(ud.value(2), [1.0, 0.0, 0.0]);
    }

    #[test]
    fn uniform_deflection_is_done() {
        let ud = UniformDeflection::new("arc", 0.1, 1e-7);
        assert!(ud.is_done());
    }

    #[test]
    fn uniform_deflection_zero_deflection_not_done() {
        let ud = UniformDeflection::new("arc", 0.0, 0.0);
        assert!(!ud.is_done());
        assert_eq!(ud.nb_points(), 0);
    }

    #[test]
    fn tangential_deflection_basic() {
        let td = TangentialDeflection::new("circle", 0.5, 0.25);
        assert!(td.is_done());
        // effective step = min(0.5, 0.25) = 0.25 → ceil(1/0.25)=4 intervals → 5 points
        assert_eq!(td.nb_points(), 5);
        assert!((td.parameter(0) - 0.0).abs() < 1e-12);
        assert!((td.parameter(4) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn tangential_deflection_add_point() {
        let mut td = TangentialDeflection::new("curve", 0.5, 0.5);
        // Initial: ceil(1/0.5)=2 intervals → 3 points at 0, 0.5, 1
        let before = td.nb_points();
        td.add_point(0.25, [0.25, 0.0, 0.0]);
        assert_eq!(td.nb_points(), before + 1);
        // params must remain sorted
        for w in td.params.windows(2) {
            assert!(w[0] <= w[1]);
        }
    }

    #[test]
    fn tangential_deflection_add_point_at_boundary() {
        let mut td = TangentialDeflection::new("spline", 0.5, 0.5);
        td.add_point(1.0, [1.0, 0.1, 0.0]);
        // 1.0 already present; inserted at partition_point so order preserved
        let ones: Vec<_> = td.params.iter().filter(|&&p| (p - 1.0).abs() < 1e-12).collect();
        assert!(!ones.is_empty());
    }

    #[test]
    fn tangential_deflection_not_done_zero_defs() {
        let td = TangentialDeflection::new("x", 0.0, 0.1);
        assert!(!td.is_done());
        let td2 = TangentialDeflection::new("x", 0.1, 0.0);
        assert!(!td2.is_done());
    }

    #[test]
    fn sample_by_deflection_meets_n_min() {
        let pts = sample_by_deflection("curve", 0.5, 10);
        assert!(pts.len() >= 10);
    }

    #[test]
    fn sample_by_deflection_coarse_deflection() {
        // deflection larger than span → still at least n_min=1 point
        let pts = sample_by_deflection("line", 10.0, 1);
        assert!(!pts.is_empty());
    }

    #[test]
    fn sample_by_deflection_zero_n_min() {
        // n_min clamped to 1
        let pts = sample_by_deflection("line", 0.5, 0);
        assert!(!pts.is_empty());
    }
}

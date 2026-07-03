// FILE: rust/ironstream/crates/ironstream/src/geom_interp.rs
//! `geom_interp` — zero-dependency B-spline interpolation stubs mirroring
//! OpenCascade's `GeomAPI_Interpolate` and `Geom2dAPI_Interpolate`.
//!
//! Both types follow the classic OCCT two-phase pattern:
//! 1. Construct with `new()` – no computation is performed.
//! 2. Call `load()` to run the interpolation stub.
//!
//! Tangent constraints for 3-D curves may be set before `load()` via
//! `with_tangents()`.
//!
//! The 2-D evaluator (`Interpolate2d::evaluate`) uses uniform-parameter
//! piecewise-linear blending as a placeholder for the full B-spline kernel.
//!
//! No unsafe code; no third-party dependencies.

// ───────────────────────────── Interpolate3d ─────────────────────────────────

/// B-spline interpolation through an ordered set of 3-D points.
///
/// Lightweight stub mirroring the interface of `GeomAPI_Interpolate`.
/// `load()` fits a clamped cubic B-spline (degree 3) through every supplied
/// point using chord-length parameterisation and stores the resulting control
/// poles and knot vector.
// occt: GeomAPI_Interpolate
#[derive(Clone, Debug)]
pub struct Interpolate3d {
    /// Input point sequence (at least two points required by `load()`).
    pub points: Vec<[f64; 3]>,
    /// When `true` the curve closes on itself (first point == last point in
    /// the parameterisation).
    pub periodic: bool,
    /// Point-coincidence tolerance used during construction.
    pub tolerance: f64,
    /// Resulting B-spline control poles (filled after a successful `load()`).
    pub result_poles: Vec<[f64; 3]>,
    /// Resulting B-spline knot vector (filled after a successful `load()`).
    pub result_knots: Vec<f64>,
    /// `true` after a successful call to `load()`.
    pub done: bool,
    /// Optional start tangent (set via `with_tangents`).
    tangent_start: Option<[f64; 3]>,
    /// Optional end tangent (set via `with_tangents`).
    tangent_end: Option<[f64; 3]>,
}

impl Interpolate3d {
    /// Construct a new interpolator.
    ///
    /// No computation is performed; call `load()` when ready.
    ///
    /// `tolerance` defaults to `1e-7`.
    pub fn new(pts: Vec<[f64; 3]>, periodic: bool) -> Self {
        Self {
            points: pts,
            periodic,
            tolerance: 1.0e-7,
            result_poles: Vec::new(),
            result_knots: Vec::new(),
            done: false,
            tangent_start: None,
            tangent_end: None,
        }
    }

    /// Attach end tangent constraints before calling `load()`.
    ///
    /// `t1` is the tangent at the first point; `t2` is the tangent at the
    /// last point.  Both vectors are stored normalised.
    ///
    /// Returns `&mut Self` for method chaining.
    pub fn with_tangents(&mut self, t1: [f64; 3], t2: [f64; 3]) -> &mut Self {
        self.tangent_start = Some(normalize3(t1));
        self.tangent_end = Some(normalize3(t2));
        self
    }

    /// Run the interpolation stub.
    ///
    /// Requires at least two distinct points.  On success `done` is set to
    /// `true`, `result_poles` contains the control polygon (identical to the
    /// input points for this stub — a placeholder for the full tridiagonal
    /// solve), and `result_knots` contains the chord-length parameter values
    /// clamped into a B-spline knot vector of degree 3.
    ///
    /// If `periodic` is set, a closing knot equal to the total chord length
    /// is appended.
    ///
    /// When end tangents have been set via `with_tangents`, two phantom ghost
    /// poles displaced by the tangents are inserted at the head and tail of
    /// `result_poles` (mirroring the OCCT end-condition encoding).
    pub fn load(&mut self) {
        self.done = false;
        self.result_poles.clear();
        self.result_knots.clear();

        let n = self.points.len();
        if n < 2 {
            return;
        }

        // ── chord-length parameterisation ──────────────────────────────────
        let mut params = Vec::with_capacity(n);
        params.push(0.0_f64);
        for i in 1..n {
            let d = dist3(self.points[i - 1], self.points[i]);
            params.push(params[i - 1] + d.max(self.tolerance));
        }
        let total = *params.last().unwrap();

        // ── build clamped knot vector (degree 3) ──────────────────────────
        //   Clamp: first and last knots have multiplicity 4.
        //   Interior: one knot per interior point.
        let mut knots = Vec::new();
        // leading clamp
        for _ in 0..4 {
            knots.push(0.0_f64);
        }
        // interior
        for i in 1..n - 1 {
            knots.push(params[i] / total);
        }
        // trailing clamp
        for _ in 0..4 {
            knots.push(1.0_f64);
        }

        if self.periodic {
            knots.push(total);
        }

        // ── control poles (stub: equal to input points) ───────────────────
        //   A production implementation solves the cubic B-spline tridiagonal
        //   system here.  The ghost-pole end-condition encoding is applied when
        //   tangents are present.
        let mut poles: Vec<[f64; 3]> = self.points.clone();

        if let Some(t) = self.tangent_start {
            let p0 = poles[0];
            let ghost = [p0[0] + t[0], p0[1] + t[1], p0[2] + t[2]];
            poles.insert(0, ghost);
        }
        if let Some(t) = self.tangent_end {
            let pk = *poles.last().unwrap();
            let ghost = [pk[0] + t[0], pk[1] + t[1], pk[2] + t[2]];
            poles.push(ghost);
        }

        self.result_poles = poles;
        self.result_knots = knots;
        self.done = true;
    }

    /// Whether `load()` completed successfully.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Number of control poles in the result (0 if not done).
    pub fn nb_poles(&self) -> usize {
        self.result_poles.len()
    }
}

// ───────────────────────────── Interpolate2d ─────────────────────────────────

/// B-spline interpolation through an ordered set of 2-D points.
///
/// Lightweight stub mirroring the interface of `Geom2dAPI_Interpolate`.
/// `load()` stores the input points verbatim as the control polygon.
/// `evaluate(t)` blends between consecutive poles using a uniform parameter
/// in [0, 1] as a placeholder for the full B-spline evaluation kernel.
// occt: Geom2dAPI_Interpolate
#[derive(Clone, Debug)]
pub struct Interpolate2d {
    /// Input point sequence (at least two points required by `load()`).
    pub points: Vec<[f64; 2]>,
    /// When `true` the curve closes on itself.
    pub periodic: bool,
    /// `true` after a successful call to `load()`.
    pub done: bool,
    /// Internal chord-length parameter values (one per input point).
    params: Vec<f64>,
}

impl Interpolate2d {
    /// Construct a new 2-D interpolator.
    ///
    /// No computation is performed; call `load()` when ready.
    pub fn new(pts: Vec<[f64; 2]>, periodic: bool) -> Self {
        Self {
            points: pts,
            periodic,
            done: false,
            params: Vec::new(),
        }
    }

    /// Run the interpolation stub.
    ///
    /// Requires at least two distinct points.  Builds the chord-length
    /// parameter table used by `evaluate`.  On success `done` is `true`.
    pub fn load(&mut self) {
        self.done = false;
        self.params.clear();

        let n = self.points.len();
        if n < 2 {
            return;
        }

        let mut params = Vec::with_capacity(n);
        params.push(0.0_f64);
        for i in 1..n {
            let d = dist2(self.points[i - 1], self.points[i]);
            params.push(params[i - 1] + d.max(1.0e-14));
        }

        if self.periodic {
            // close the loop: add chord back to the start
            let extra = dist2(*self.points.last().unwrap(), self.points[0]);
            params.push(params[n - 1] + extra.max(1.0e-14));
        }

        self.params = params;
        self.done = true;
    }

    /// Whether `load()` completed successfully.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Evaluate the interpolating curve at parameter `t` ∈ [0, 1].
    ///
    /// Uses piecewise-linear blending over the chord-length parameter table
    /// built by `load()`.  Returns `[0.0, 0.0]` if the interpolator has not
    /// been successfully loaded or if the point list is empty.
    ///
    /// For periodic curves `t` is wrapped into [0, 1) before evaluation.
    pub fn evaluate(&self, t: f64) -> [f64; 2] {
        if !self.done || self.points.is_empty() {
            return [0.0, 0.0];
        }

        let n = self.points.len();
        if n == 1 {
            return self.points[0];
        }

        let total = *self.params.last().unwrap();
        if total == 0.0 {
            return self.points[0];
        }

        // map t ∈ [0,1] → s ∈ [0, total]
        let t_clamped = if self.periodic {
            t.rem_euclid(1.0)
        } else {
            t.clamp(0.0, 1.0)
        };
        let s = t_clamped * total;

        // find the segment [params[i], params[i+1]) containing s
        let seg_count = self.params.len() - 1;
        let mut idx = seg_count - 1; // default to last segment
        for i in 0..seg_count {
            if s <= self.params[i + 1] {
                idx = i;
                break;
            }
        }

        let p0 = self.points[idx % n];
        let p1 = self.points[(idx + 1) % n];

        let span = self.params[idx + 1] - self.params[idx];
        let u = if span > 0.0 {
            (s - self.params[idx]) / span
        } else {
            0.0
        };

        [
            p0[0] + u * (p1[0] - p0[0]),
            p0[1] + u * (p1[1] - p0[1]),
        ]
    }
}

// ───────────────────────────── free function ─────────────────────────────────

/// Convenience wrapper: construct and immediately load a 3-D interpolator.
///
/// Returns an `Interpolate3d` with `done == true` on success.
pub fn interpolate_3d(pts: &[[f64; 3]], periodic: bool) -> Interpolate3d {
    let mut interp = Interpolate3d::new(pts.to_vec(), periodic);
    interp.load();
    interp
}

// ───────────────────────────── helpers ───────────────────────────────────────

#[inline]
fn dist3(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];
    let dz = b[2] - a[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

#[inline]
fn dist2(a: [f64; 2], b: [f64; 2]) -> f64 {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];
    (dx * dx + dy * dy).sqrt()
}

/// Normalise a 3-D vector; returns the original if its length is below `1e-14`.
#[inline]
fn normalize3(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1.0e-14 {
        v
    } else {
        [v[0] / len, v[1] / len, v[2] / len]
    }
}

// ───────────────────────────── tests ─────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolate3d_two_points() {
        let pts = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
        let mut interp = Interpolate3d::new(pts, false);
        interp.load();
        assert!(interp.is_done());
        assert_eq!(interp.nb_poles(), 2);
        assert!(!interp.result_knots.is_empty());
    }

    #[test]
    fn interpolate3d_with_tangents() {
        let pts = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [2.0, 0.0, 0.0]];
        let mut interp = Interpolate3d::new(pts, false);
        interp.with_tangents([1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
        interp.load();
        assert!(interp.is_done());
        // ghost poles add 2 extra
        assert_eq!(interp.nb_poles(), 5);
    }

    #[test]
    fn interpolate3d_too_few_points() {
        let mut interp = Interpolate3d::new(vec![[0.0, 0.0, 0.0]], false);
        interp.load();
        assert!(!interp.is_done());
    }

    #[test]
    fn interpolate3d_free_function() {
        let pts = [[0.0, 0.0, 0.0], [1.0, 2.0, 3.0]];
        let result = interpolate_3d(&pts, false);
        assert!(result.is_done());
        assert_eq!(result.nb_poles(), 2);
    }

    #[test]
    fn interpolate2d_evaluate_endpoints() {
        let pts = vec![[0.0, 0.0], [4.0, 0.0]];
        let mut interp = Interpolate2d::new(pts, false);
        interp.load();
        assert!(interp.is_done());

        let start = interp.evaluate(0.0);
        let end = interp.evaluate(1.0);
        assert!((start[0] - 0.0).abs() < 1e-10);
        assert!((end[0] - 4.0).abs() < 1e-10);
    }

    #[test]
    fn interpolate2d_midpoint() {
        let pts = vec![[0.0, 0.0], [2.0, 0.0]];
        let mut interp = Interpolate2d::new(pts, false);
        interp.load();
        let mid = interp.evaluate(0.5);
        assert!((mid[0] - 1.0).abs() < 1e-10);
        assert!(mid[1].abs() < 1e-10);
    }

    #[test]
    fn interpolate2d_not_done() {
        let interp = Interpolate2d::new(vec![[0.0, 0.0], [1.0, 1.0]], false);
        // load() not called
        assert!(!interp.is_done());
        let val = interp.evaluate(0.5);
        assert_eq!(val, [0.0, 0.0]);
    }

    #[test]
    fn interpolate2d_periodic_wraps() {
        let pts = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]];
        let mut interp = Interpolate2d::new(pts, true);
        interp.load();
        assert!(interp.is_done());
        // t slightly above 1.0 should not panic and should wrap cleanly
        let _ = interp.evaluate(1.001);
    }
}

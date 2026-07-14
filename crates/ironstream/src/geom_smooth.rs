// FILE: rust/ironstream/crates/ironstream/src/geom_smooth.rs
//! `geom_smooth` — pure-Rust stubs mirroring OpenCascade's
//! `GeomSmooth_Array1OfBSplKnotMult` / `ShapeSmoothing` family.
//!
//! These types implement the two main smoothing patterns from OCCT:
//!
//! * [`SmoothParams`]   — shared parameter block (continuity, tolerance, …).
//! * [`CurveSmooth`]    — pole-based B-spline curve smoother.
//! * [`SurfaceSmooth`]  — grid-of-poles B-spline surface smoother.
//! * [`smooth_polyline`] — stand-alone iterative Laplacian polyline smoother.
//!
//! No unsafe code; no third-party dependencies.

// ─────────────────────────── SmoothParams ────────────────────────────────────

/// Shared smoothing parameters controlling continuity, tolerance, degree and
/// segment limits.
///
/// Maps to the constructor arguments accepted by the `ShapeSmoothing` family
/// of OCCT algorithms.
// occt-ref: ShapeSmoothing
#[derive(Clone, Debug, PartialEq)]
pub struct SmoothParams {
    /// Required geometric continuity order (0 = C0, 1 = C1, 2 = C2, …).
    pub continuity: u32,
    /// Maximum allowed deviation from the input geometry.
    pub tolerance: f64,
    /// Maximum polynomial degree of the output B-spline.
    pub max_degree: u32,
    /// Maximum number of B-spline segments (knot spans).
    pub max_segments: u32,
}

impl SmoothParams {
    /// Create a [`SmoothParams`] with the given values.
    pub fn new(continuity: u32, tolerance: f64, max_degree: u32, max_segments: u32) -> Self {
        Self {
            continuity,
            tolerance,
            max_degree,
            max_segments,
        }
    }

    /// Sensible defaults matching OCCT's typical `ShapeSmoothing` constructor:
    /// C1 continuity, tolerance 1e-4, degree 3, 16 segments.
    pub fn default() -> Self {
        Self {
            continuity: 1,
            tolerance: 1.0e-4,
            max_degree: 3,
            max_segments: 16,
        }
    }

    /// Return a copy of `self` with `continuity` replaced by `c`.
    pub fn with_continuity(mut self, c: u32) -> Self {
        self.continuity = c;
        self
    }
}

// ─────────────────────────── CurveSmooth ─────────────────────────────────────

/// B-spline curve smoother operating on a sequence of 3-D control poles.
///
/// Mirrors the interface of `GeomSmooth_Array1OfBSplKnotMult` / curve variant:
/// accumulate points with [`add_point`][CurveSmooth::add_point], then call
/// [`smooth`][CurveSmooth::smooth] to obtain the smoothed pole sequence, or
/// [`evaluate`][CurveSmooth::evaluate] to sample the underlying piecewise-linear
/// curve at parameter `t ∈ [0, 1]`.
// occt-ref: ShapeSmoothing
#[derive(Clone, Debug)]
pub struct CurveSmooth {
    /// Current control poles.
    pub poles: Vec<[f64; 3]>,
    /// B-spline degree (informational; smoothing is Laplacian-based).
    pub degree: u32,
}

impl CurveSmooth {
    /// Construct an empty smoother with the given B-spline `degree`.
    pub fn new(degree: u32) -> Self {
        Self {
            poles: Vec::new(),
            degree,
        }
    }

    /// Append a 3-D point `p` to the pole sequence.
    pub fn add_point(&mut self, p: [f64; 3]) {
        self.poles.push(p);
    }

    /// Return a smoothed copy of the accumulated poles.
    ///
    /// The algorithm performs `params.max_segments.min(64)` passes of
    /// constrained Laplacian averaging.  The first and last poles are pinned
    /// (interpolated), matching OCCT's clamped-knot convention.  `tolerance`
    /// is used as a convergence guard: if the maximum pole displacement in a
    /// pass falls below `params.tolerance` the loop terminates early.
    pub fn smooth(&self, params: &SmoothParams) -> Vec<[f64; 3]> {
        if self.poles.len() < 2 {
            return self.poles.clone();
        }

        let passes = params.max_segments.min(64) as usize;
        let mut pts = self.poles.clone();
        let n = pts.len();

        for _ in 0..passes {
            let prev = pts.clone();
            let mut max_disp: f64 = 0.0;

            for i in 1..n - 1 {
                let smoothed = midpoint(prev[i - 1], prev[i + 1]);
                let disp = dist(smoothed, prev[i]);
                max_disp = max_disp.max(disp);
                pts[i] = smoothed;
            }

            if max_disp < params.tolerance {
                break;
            }
        }

        pts
    }

    /// Evaluate the piecewise-linear interpolant of the current poles at
    /// parameter `t ∈ [0, 1]`.
    ///
    /// `t = 0.0` returns the first pole; `t = 1.0` returns the last.
    pub fn evaluate(&self, t: f64) -> [f64; 3] {
        let n = self.poles.len();
        if n == 0 {
            return [0.0, 0.0, 0.0];
        }
        if n == 1 {
            return self.poles[0];
        }

        let t = t.clamp(0.0, 1.0);
        let scaled = t * (n - 1) as f64;
        let lo = (scaled as usize).min(n - 2);
        let frac = scaled - lo as f64;

        let a = self.poles[lo];
        let b = self.poles[lo + 1];
        [
            a[0] + frac * (b[0] - a[0]),
            a[1] + frac * (b[1] - a[1]),
            a[2] + frac * (b[2] - a[2]),
        ]
    }
}

// ─────────────────────────── SurfaceSmooth ───────────────────────────────────

/// B-spline surface smoother operating on a rectangular grid of 3-D poles.
///
/// Mirrors the `ShapeSmoothing` surface variant from OCCT.  Add rows of poles
/// with [`add_row`][SurfaceSmooth::add_row], then call
/// [`smooth`][SurfaceSmooth::smooth] to obtain a smoothed grid.
// occt-ref: ShapeSmoothing
#[derive(Clone, Debug)]
pub struct SurfaceSmooth {
    /// Grid of control poles indexed as `poles[row][col]`.
    pub poles: Vec<Vec<[f64; 3]>>,
    /// B-spline degree in the U (row) direction.
    pub u_degree: u32,
    /// B-spline degree in the V (column) direction.
    pub v_degree: u32,
}

impl SurfaceSmooth {
    /// Construct an empty surface smoother with U-degree `ud` and V-degree `vd`.
    pub fn new(ud: u32, vd: u32) -> Self {
        Self {
            poles: Vec::new(),
            u_degree: ud,
            v_degree: vd,
        }
    }

    /// Append a complete row of poles.
    pub fn add_row(&mut self, row: Vec<[f64; 3]>) {
        self.poles.push(row);
    }

    /// Return a smoothed copy of the pole grid.
    ///
    /// The algorithm performs `params.max_segments.min(64)` passes of
    /// constrained Laplacian averaging across both the U and V directions.
    /// Border rows and columns are pinned.  Early termination applies when the
    /// maximum pole displacement falls below `params.tolerance`.
    pub fn smooth(&self, params: &SmoothParams) -> Vec<Vec<[f64; 3]>> {
        let rows = self.poles.len();
        if rows < 2 {
            return self.poles.clone();
        }
        let cols = self.poles[0].len();
        if cols < 2 {
            return self.poles.clone();
        }

        let passes = params.max_segments.min(64) as usize;
        let mut grid = self.poles.clone();

        for _ in 0..passes {
            let prev = grid.clone();
            let mut max_disp: f64 = 0.0;

            for r in 1..rows - 1 {
                if prev[r].len() < cols || prev[r - 1].len() < cols || prev[r + 1].len() < cols {
                    continue;
                }
                for c in 1..cols - 1 {
                    // Average the four axis-aligned neighbours.
                    let avg = avg4(
                        prev[r - 1][c],
                        prev[r + 1][c],
                        prev[r][c - 1],
                        prev[r][c + 1],
                    );
                    let disp = dist(avg, prev[r][c]);
                    max_disp = max_disp.max(disp);
                    grid[r][c] = avg;
                }
            }

            if max_disp < params.tolerance {
                break;
            }
        }

        grid
    }
}

// ─────────────────────────── smooth_polyline ─────────────────────────────────

/// Iteratively smooth a polyline using constrained Laplacian averaging.
///
/// Each of the `iterations` passes replaces every interior vertex with the
/// midpoint of its two neighbours.  The first and last vertices are always
/// pinned so the endpoints do not move.
pub fn smooth_polyline(pts: &[[f64; 3]], iterations: u32) -> Vec<[f64; 3]> {
    let n = pts.len();
    if n < 3 {
        return pts.to_vec();
    }

    let mut result = pts.to_vec();

    for _ in 0..iterations {
        let prev = result.clone();
        for i in 1..n - 1 {
            result[i] = midpoint(prev[i - 1], prev[i + 1]);
        }
    }

    result
}

// ─────────────────────────── Internal helpers ─────────────────────────────────

#[inline]
fn midpoint(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        (a[0] + b[0]) * 0.5,
        (a[1] + b[1]) * 0.5,
        (a[2] + b[2]) * 0.5,
    ]
}

#[inline]
fn avg4(a: [f64; 3], b: [f64; 3], c: [f64; 3], d: [f64; 3]) -> [f64; 3] {
    [
        (a[0] + b[0] + c[0] + d[0]) * 0.25,
        (a[1] + b[1] + c[1] + d[1]) * 0.25,
        (a[2] + b[2] + c[2] + d[2]) * 0.25,
    ]
}

#[inline]
fn dist(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

// ─────────────────────────── Tests ───────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smooth_params_default_values() {
        let p = SmoothParams::default();
        assert_eq!(p.continuity, 1);
        assert_eq!(p.max_degree, 3);
        assert_eq!(p.max_segments, 16);
    }

    #[test]
    fn smooth_params_with_continuity() {
        let p = SmoothParams::default().with_continuity(2);
        assert_eq!(p.continuity, 2);
        assert_eq!(p.max_degree, 3); // unchanged
    }

    #[test]
    fn smooth_params_new() {
        let p = SmoothParams::new(0, 1e-3, 5, 32);
        assert_eq!(p.continuity, 0);
        assert!((p.tolerance - 1e-3).abs() < 1e-15);
        assert_eq!(p.max_degree, 5);
        assert_eq!(p.max_segments, 32);
    }

    #[test]
    fn curve_smooth_endpoints_pinned() {
        let mut cs = CurveSmooth::new(3);
        cs.add_point([0.0, 0.0, 0.0]);
        cs.add_point([1.0, 10.0, 0.0]); // outlier
        cs.add_point([2.0, 0.0, 0.0]);

        let params = SmoothParams::new(1, 1e-6, 3, 8);
        let smoothed = cs.smooth(&params);

        assert_eq!(smoothed.len(), 3);
        // Endpoints must not move.
        assert_eq!(smoothed[0], [0.0, 0.0, 0.0]);
        assert_eq!(smoothed[2], [2.0, 0.0, 0.0]);
        // Middle point should move toward the midpoint of neighbours.
        assert!(smoothed[1][1] < 10.0);
    }

    #[test]
    fn curve_smooth_single_point_passthrough() {
        let mut cs = CurveSmooth::new(3);
        cs.add_point([1.0, 2.0, 3.0]);
        let params = SmoothParams::default();
        let out = cs.smooth(&params);
        assert_eq!(out, vec![[1.0, 2.0, 3.0]]);
    }

    #[test]
    fn curve_smooth_evaluate_endpoints() {
        let mut cs = CurveSmooth::new(3);
        cs.add_point([0.0, 0.0, 0.0]);
        cs.add_point([1.0, 1.0, 0.0]);
        cs.add_point([2.0, 0.0, 0.0]);

        let start = cs.evaluate(0.0);
        let end = cs.evaluate(1.0);
        assert_eq!(start, [0.0, 0.0, 0.0]);
        assert_eq!(end, [2.0, 0.0, 0.0]);
    }

    #[test]
    fn curve_smooth_evaluate_midpoint() {
        let mut cs = CurveSmooth::new(3);
        cs.add_point([0.0, 0.0, 0.0]);
        cs.add_point([2.0, 0.0, 0.0]);
        let mid = cs.evaluate(0.5);
        assert!((mid[0] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn surface_smooth_endpoints_pinned() {
        let mut ss = SurfaceSmooth::new(3, 3);
        ss.add_row(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]]);
        ss.add_row(vec![[0.0, 1.0, 0.0], [1.0, 10.0, 0.0], [2.0, 1.0, 0.0]]);
        ss.add_row(vec![[0.0, 2.0, 0.0], [1.0, 2.0, 0.0], [2.0, 2.0, 0.0]]);

        let params = SmoothParams::new(1, 1e-6, 3, 4);
        let out = ss.smooth(&params);

        // Corner poles must be unchanged.
        assert_eq!(out[0][0], [0.0, 0.0, 0.0]);
        assert_eq!(out[2][2], [2.0, 2.0, 0.0]);
        // Interior pole should have moved.
        assert!(out[1][1][1] < 10.0);
    }

    #[test]
    fn smooth_polyline_endpoints_pinned() {
        let pts = vec![
            [0.0, 0.0, 0.0],
            [1.0, 10.0, 0.0],
            [2.0, 10.0, 0.0],
            [3.0, 0.0, 0.0],
        ];
        let out = smooth_polyline(&pts, 10);
        assert_eq!(out.len(), 4);
        assert_eq!(out[0], [0.0, 0.0, 0.0]);
        assert_eq!(out[3], [3.0, 0.0, 0.0]);
    }

    #[test]
    fn smooth_polyline_short_passthrough() {
        let pts = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]];
        let out = smooth_polyline(&pts, 5);
        assert_eq!(out, pts);
    }
}

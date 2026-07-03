// FILE: extrema.rs
//! Port of OCCT's `Extrema` package — minimum/maximum distance algorithms
//! between geometric entities (curve–curve, curve–surface, surface–surface).
//!
//! Classes implemented:
//! - `ExtAlgo`       — algorithm selector (gradient vs. tree)
//! - `ExtCC`         — extrema between two 3D curves
//! - `ExtCS`         — extrema between a 3D curve and a surface
//! - `ExtSS`         — extrema between two parametric surfaces
//! - `POnCurv`       — a point on a curve together with its parameter
//! - `POnSurf`       — a point on a surface together with its (u, v) parameters

use crate::gp::Pnt;
use crate::geom_adaptor_curve::GeomAdaptorCurve;
use crate::geom_adaptor_surface::GeomAdaptorSurface;

// ────────────────────────────────────────────────────────────────────────────
// ExtAlgo
// ────────────────────────────────────────────────────────────────────────────

/// Chooses the internal strategy used by the Extrema solvers.
///
/// OCCT distinguishes between a gradient-descent approach and a tree/grid
/// approach; the distinction matters for performance but not for correctness.
// occt: Extrema_ExtAlgo
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtAlgo {
    /// Gradient-based approach (Newton-like descent). Faster on smooth inputs.
    Grad,
    /// Tree/grid-based approach. More robust for non-convex or multi-modal
    /// distance functions.
    Tree,
}

impl Default for ExtAlgo {
    fn default() -> Self {
        ExtAlgo::Grad
    }
}

// ────────────────────────────────────────────────────────────────────────────
// POnCurv
// ────────────────────────────────────────────────────────────────────────────

/// A solution point on a curve: the 3D position plus the parameter value.
// occt: Extrema_POnCurv
#[derive(Clone, Copy, Debug)]
pub struct POnCurv {
    /// Curve parameter at the solution point.
    pub parameter: f64,
    /// 3D position.
    pub point: Pnt,
}

impl POnCurv {
    /// Construct directly from a parameter and a 3D point.
    pub fn new(parameter: f64, point: Pnt) -> Self {
        Self { parameter, point }
    }

    /// The curve parameter.
    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    /// The 3D position.
    pub fn point(&self) -> Pnt {
        self.point
    }
}

// ────────────────────────────────────────────────────────────────────────────
// POnSurf
// ────────────────────────────────────────────────────────────────────────────

/// A solution point on a surface: the 3D position plus the (u, v) parameters.
// occt: Extrema_POnSurf
#[derive(Clone, Copy, Debug)]
pub struct POnSurf {
    /// First surface parameter.
    pub u: f64,
    /// Second surface parameter.
    pub v: f64,
    /// 3D position.
    pub point: Pnt,
}

impl POnSurf {
    /// Construct directly from (u, v) and a 3D point.
    pub fn new(u: f64, v: f64, point: Pnt) -> Self {
        Self { u, v, point }
    }

    /// Returns (u, v) together.
    pub fn parameters(&self) -> (f64, f64) {
        (self.u, self.v)
    }

    /// The 3D position.
    pub fn point(&self) -> Pnt {
        self.point
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Internal numeric helpers
// ────────────────────────────────────────────────────────────────────────────

/// Evaluate a curve point as `Pnt`.
fn curve_pnt(c: &GeomAdaptorCurve, t: f64) -> Pnt {
    let p = c.value(t);
    Pnt::new(p[0], p[1], p[2])
}

/// Evaluate a surface point as `Pnt`.
fn surf_pnt(s: &GeomAdaptorSurface, u: f64, v: f64) -> Pnt {
    let p = s.value(u, v);
    Pnt::new(p[0], p[1], p[2])
}

/// Uniformly sample `n` values in `[lo, hi]`.
fn linspace(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    if n <= 1 {
        return vec![(lo + hi) * 0.5];
    }
    (0..n)
        .map(|i| lo + (hi - lo) * i as f64 / (n - 1) as f64)
        .collect()
}

/// Golden-section search for the minimum of f on [a, b] to within `tol`.
fn golden_min<F: Fn(f64) -> f64>(mut a: f64, mut b: f64, tol: f64, f: &F) -> f64 {
    const PHI: f64 = 0.618_033_988_749_895;
    let mut c = b - PHI * (b - a);
    let mut d = a + PHI * (b - a);
    while (b - a).abs() > tol {
        if f(c) < f(d) {
            b = d;
        } else {
            a = c;
        }
        c = b - PHI * (b - a);
        d = a + PHI * (b - a);
    }
    (a + b) * 0.5
}

// ────────────────────────────────────────────────────────────────────────────
// ExtCC — curve–curve extrema
// ────────────────────────────────────────────────────────────────────────────

/// Extrema (minimum and maximum distance) between two 3D parametric curves.
///
/// The algorithm samples both curves on a regular grid and refines each
/// candidate minimum with golden-section search.  Mirrors the API of OCCT's
/// `Extrema_ExtCC`.
// occt: Extrema_ExtCC
pub struct ExtCC {
    results: Vec<(f64, POnCurv, POnCurv)>, // (squared-dist, point-on-C1, point-on-C2)
    done: bool,
    algo: ExtAlgo,
}

impl ExtCC {
    /// Number of samples per curve parameter range for the initial grid sweep.
    const SAMPLES: usize = 20;

    /// Construct and immediately perform the computation.
    pub fn new(
        c1: &GeomAdaptorCurve,
        c2: &GeomAdaptorCurve,
    ) -> Self {
        Self::with_algo(c1, c2, ExtAlgo::default())
    }

    /// Construct with an explicit algorithm choice.
    pub fn with_algo(
        c1: &GeomAdaptorCurve,
        c2: &GeomAdaptorCurve,
        algo: ExtAlgo,
    ) -> Self {
        let mut ec = ExtCC {
            results: Vec::new(),
            done: false,
            algo,
        };
        ec.compute(c1, c2);
        ec
    }

    fn compute(&mut self, c1: &GeomAdaptorCurve, c2: &GeomAdaptorCurve) {
        let t1s = linspace(c1.first_parameter(), c1.last_parameter(), Self::SAMPLES);
        let t2s = linspace(c2.first_parameter(), c2.last_parameter(), Self::SAMPLES);

        // Build a distance grid and locate all local minima.
        let n = t1s.len();
        let m = t2s.len();
        let mut grid = vec![vec![0.0f64; m]; n];
        for (i, &t1) in t1s.iter().enumerate() {
            let p1 = curve_pnt(c1, t1);
            for (j, &t2) in t2s.iter().enumerate() {
                grid[i][j] = p1.distance(curve_pnt(c2, t2));
            }
        }

        // Collect grid cells that are local minima in the 2-D sense.
        let mut candidates: Vec<(f64, f64)> = Vec::new();
        for i in 0..n {
            for j in 0..m {
                let d = grid[i][j];
                let left  = if i > 0 { grid[i-1][j] } else { d + 1.0 };
                let right = if i < n-1 { grid[i+1][j] } else { d + 1.0 };
                let up    = if j > 0 { grid[i][j-1] } else { d + 1.0 };
                let down  = if j < m-1 { grid[i][j+1] } else { d + 1.0 };
                if d <= left && d <= right && d <= up && d <= down {
                    candidates.push((t1s[i], t2s[j]));
                }
            }
        }
        if candidates.is_empty() {
            // Fall back: use the overall grid minimum.
            let mut best_i = 0;
            let mut best_j = 0;
            let mut best_d = grid[0][0];
            for ii in 0..n {
                for jj in 0..m {
                    if grid[ii][jj] < best_d {
                        best_d = grid[ii][jj];
                        best_i = ii;
                        best_j = jj;
                    }
                }
            }
            candidates.push((t1s[best_i], t2s[best_j]));
        }

        let dt1 = (c1.last_parameter() - c1.first_parameter()) / Self::SAMPLES as f64;
        let dt2 = (c2.last_parameter() - c2.first_parameter()) / Self::SAMPLES as f64;
        let refine_tol = 1e-9;

        for (s1, s2) in candidates {
            // Alternate single-variable refinement steps (coordinate descent).
            let mut t1 = s1;
            let mut t2 = s2;
            for _ in 0..8 {
                let a1 = (t1 - dt1).max(c1.first_parameter());
                let b1 = (t1 + dt1).min(c1.last_parameter());
                let t2_cap = t2;
                t1 = golden_min(a1, b1, refine_tol, &|u| {
                    curve_pnt(c1, u).distance(curve_pnt(c2, t2_cap))
                });

                let a2 = (t2 - dt2).max(c2.first_parameter());
                let b2 = (t2 + dt2).min(c2.last_parameter());
                let t1_cap = t1;
                t2 = golden_min(a2, b2, refine_tol, &|v| {
                    curve_pnt(c1, t1_cap).distance(curve_pnt(c2, v))
                });
            }

            let p1 = curve_pnt(c1, t1);
            let p2 = curve_pnt(c2, t2);
            let d2 = p1.distance(p2).powi(2);

            // Deduplicate against existing results.
            let dup = self.results.iter().any(|(od2, op1, _)| {
                (od2 - d2).abs() < 1e-12 && op1.point.distance(p1) < 1e-9
            });
            if !dup {
                self.results.push((
                    d2,
                    POnCurv::new(t1, p1),
                    POnCurv::new(t2, p2),
                ));
            }
        }

        // Sort by ascending squared distance so index 0 is the minimum.
        self.results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        self.done = true;
    }

    /// Returns `true` if the computation succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Number of extrema found.
    pub fn nb_ext(&self) -> usize {
        self.results.len()
    }

    /// Square distance of the `i`-th solution (1-based, OCCT convention).
    pub fn square_distance(&self, i: usize) -> f64 {
        self.results[i - 1].0
    }

    /// Points on C1 and C2 for the `i`-th solution (1-based).
    pub fn points(&self, i: usize) -> (POnCurv, POnCurv) {
        let (_, p1, p2) = self.results[i - 1];
        (p1, p2)
    }

    /// The minimum distance found (Euclidean, not squared).
    pub fn min_distance(&self) -> Option<f64> {
        self.results.first().map(|(d2, _, _)| d2.sqrt())
    }

    /// The algorithm in use.
    pub fn algo(&self) -> ExtAlgo {
        self.algo
    }
}

// ────────────────────────────────────────────────────────────────────────────
// ExtCS — curve–surface extrema
// ────────────────────────────────────────────────────────────────────────────

/// Extrema between a 3D parametric curve and a parametric surface.
///
/// A uniform grid over (t, u, v) is sampled to seed a coordinate-descent
/// refinement.  Mirrors OCCT's `Extrema_ExtCS`.
// occt: Extrema_ExtCS
pub struct ExtCS {
    results: Vec<(f64, POnCurv, POnSurf)>,
    done: bool,
    algo: ExtAlgo,
}

impl ExtCS {
    /// Samples per axis for the initial grid search.
    const CURVE_SAMP: usize = 16;
    const SURF_SAMP: usize = 8;

    /// Construct and compute.
    pub fn new(curve: &GeomAdaptorCurve, surf: &GeomAdaptorSurface) -> Self {
        Self::with_algo(curve, surf, ExtAlgo::default())
    }

    /// Construct with explicit algorithm selector.
    pub fn with_algo(
        curve: &GeomAdaptorCurve,
        surf: &GeomAdaptorSurface,
        algo: ExtAlgo,
    ) -> Self {
        let mut ec = ExtCS {
            results: Vec::new(),
            done: false,
            algo,
        };
        ec.compute(curve, surf);
        ec
    }

    fn compute(&mut self, curve: &GeomAdaptorCurve, surf: &GeomAdaptorSurface) {
        let ts = linspace(curve.first_parameter(), curve.last_parameter(), Self::CURVE_SAMP);
        let us = linspace(surf.u_first(), surf.u_last(), Self::SURF_SAMP);
        let vs = linspace(surf.v_first(), surf.v_last(), Self::SURF_SAMP);

        let nc = ts.len();
        let nu = us.len();
        let nv = vs.len();

        // 3D grid distances.
        let dist = |t: f64, u: f64, v: f64| {
            curve_pnt(curve, t).distance(surf_pnt(surf, u, v))
        };

        // Find all local minima in the (t,u,v) grid.
        let mut candidates: Vec<(f64, f64, f64)> = Vec::new();
        let mut best_d = f64::MAX;
        let mut best = (ts[0], us[0], vs[0]);

        for &t in &ts {
            for &u in &us {
                for &v in &vs {
                    let d = dist(t, u, v);
                    if d < best_d {
                        best_d = d;
                        best = (t, u, v);
                    }
                }
            }
        }
        candidates.push(best);

        // Additionally find any cell that is a local grid minimum along each
        // curve parameter slice.
        for ic in 0..nc {
            for iu in 0..nu {
                for iv in 0..nv {
                    let d = dist(ts[ic], us[iu], vs[iv]);
                    let neighbors_larger = [
                        if ic > 0 { dist(ts[ic-1], us[iu], vs[iv]) } else { d + 1.0 },
                        if ic+1 < nc { dist(ts[ic+1], us[iu], vs[iv]) } else { d + 1.0 },
                        if iu > 0 { dist(ts[ic], us[iu-1], vs[iv]) } else { d + 1.0 },
                        if iu+1 < nu { dist(ts[ic], us[iu+1], vs[iv]) } else { d + 1.0 },
                        if iv > 0 { dist(ts[ic], us[iu], vs[iv-1]) } else { d + 1.0 },
                        if iv+1 < nv { dist(ts[ic], us[iu], vs[iv+1]) } else { d + 1.0 },
                    ];
                    if neighbors_larger.iter().all(|&nd| nd >= d) {
                        candidates.push((ts[ic], us[iu], vs[iv]));
                    }
                }
            }
        }

        let dt = (curve.last_parameter() - curve.first_parameter()) / Self::CURVE_SAMP as f64;
        let du = (surf.u_last() - surf.u_first()) / Self::SURF_SAMP as f64;
        let dv = (surf.v_last() - surf.v_first()) / Self::SURF_SAMP as f64;
        let tol = 1e-9;

        for (st, su, sv) in candidates {
            let mut t = st;
            let mut u = su;
            let mut v = sv;

            for _ in 0..10 {
                // Refine t.
                let at = (t - dt).max(curve.first_parameter());
                let bt = (t + dt).min(curve.last_parameter());
                let uc = u; let vc = v;
                t = golden_min(at, bt, tol, &|tt| dist(tt, uc, vc));

                // Refine u.
                let au = (u - du).max(surf.u_first());
                let bu = (u + du).min(surf.u_last());
                let tc = t; let vc2 = v;
                u = golden_min(au, bu, tol, &|uu| dist(tc, uu, vc2));

                // Refine v.
                let av = (v - dv).max(surf.v_first());
                let bv = (v + dv).min(surf.v_last());
                let tc2 = t; let uc2 = u;
                v = golden_min(av, bv, tol, &|vv| dist(tc2, uc2, vv));
            }

            let cp = curve_pnt(curve, t);
            let sp = surf_pnt(surf, u, v);
            let d2 = cp.distance(sp).powi(2);

            let dup = self.results.iter().any(|(od2, op, _)| {
                (od2 - d2).abs() < 1e-12 && op.point.distance(cp) < 1e-9
            });
            if !dup {
                self.results.push((
                    d2,
                    POnCurv::new(t, cp),
                    POnSurf::new(u, v, sp),
                ));
            }
        }

        self.results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        self.done = true;
    }

    /// Returns `true` if computation succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Number of extrema found.
    pub fn nb_ext(&self) -> usize {
        self.results.len()
    }

    /// Squared distance for the `i`-th solution (1-based).
    pub fn square_distance(&self, i: usize) -> f64 {
        self.results[i - 1].0
    }

    /// Points for the `i`-th solution (1-based): (curve point, surface point).
    pub fn points(&self, i: usize) -> (POnCurv, POnSurf) {
        let (_, pc, ps) = self.results[i - 1];
        (pc, ps)
    }

    /// Minimum distance (Euclidean).
    pub fn min_distance(&self) -> Option<f64> {
        self.results.first().map(|(d2, _, _)| d2.sqrt())
    }

    /// Algorithm in use.
    pub fn algo(&self) -> ExtAlgo {
        self.algo
    }
}

// ────────────────────────────────────────────────────────────────────────────
// ExtSS — surface–surface extrema
// ────────────────────────────────────────────────────────────────────────────

/// Extrema between two parametric surfaces.
///
/// A uniform (u1, v1, u2, v2) grid seeds a coordinate-descent refinement.
/// Mirrors OCCT's `Extrema_ExtSS`.
// occt: Extrema_ExtSS
pub struct ExtSS {
    results: Vec<(f64, POnSurf, POnSurf)>,
    done: bool,
    algo: ExtAlgo,
}

impl ExtSS {
    /// Samples per surface-parameter axis for the initial sweep.
    const SAMP: usize = 8;

    /// Construct and compute.
    pub fn new(s1: &GeomAdaptorSurface, s2: &GeomAdaptorSurface) -> Self {
        Self::with_algo(s1, s2, ExtAlgo::default())
    }

    /// Construct with explicit algorithm selector.
    pub fn with_algo(
        s1: &GeomAdaptorSurface,
        s2: &GeomAdaptorSurface,
        algo: ExtAlgo,
    ) -> Self {
        let mut es = ExtSS {
            results: Vec::new(),
            done: false,
            algo,
        };
        es.compute(s1, s2);
        es
    }

    fn compute(&mut self, s1: &GeomAdaptorSurface, s2: &GeomAdaptorSurface) {
        let u1s = linspace(s1.u_first(), s1.u_last(), Self::SAMP);
        let v1s = linspace(s1.v_first(), s1.v_last(), Self::SAMP);
        let u2s = linspace(s2.u_first(), s2.u_last(), Self::SAMP);
        let v2s = linspace(s2.v_first(), s2.v_last(), Self::SAMP);

        let dist = |u1: f64, v1: f64, u2: f64, v2: f64| {
            surf_pnt(s1, u1, v1).distance(surf_pnt(s2, u2, v2))
        };

        // Find the overall global minimum across the 4-D grid.
        let mut best_d = f64::MAX;
        let mut best = (u1s[0], v1s[0], u2s[0], v2s[0]);
        for &u1 in &u1s {
            for &v1 in &v1s {
                for &u2 in &u2s {
                    for &v2 in &v2s {
                        let d = dist(u1, v1, u2, v2);
                        if d < best_d {
                            best_d = d;
                            best = (u1, v1, u2, v2);
                        }
                    }
                }
            }
        }

        let mut candidates = vec![best];

        // Also find local minima in 2-D slices (fix s2 params, search s1 and vice versa).
        for &u2 in &u2s {
            for &v2 in &v2s {
                let mut local_best_d = f64::MAX;
                let mut local_best = (u1s[0], v1s[0]);
                for &u1 in &u1s {
                    for &v1 in &v1s {
                        let d = dist(u1, v1, u2, v2);
                        if d < local_best_d {
                            local_best_d = d;
                            local_best = (u1, v1);
                        }
                    }
                }
                if local_best_d < best_d * 2.0 {
                    candidates.push((local_best.0, local_best.1, u2, v2));
                }
            }
        }

        let du1 = (s1.u_last() - s1.u_first()) / Self::SAMP as f64;
        let dv1 = (s1.v_last() - s1.v_first()) / Self::SAMP as f64;
        let du2 = (s2.u_last() - s2.u_first()) / Self::SAMP as f64;
        let dv2 = (s2.v_last() - s2.v_first()) / Self::SAMP as f64;
        let tol = 1e-9;

        for (su1, sv1, su2, sv2) in candidates {
            let mut u1 = su1; let mut v1 = sv1;
            let mut u2 = su2; let mut v2 = sv2;

            for _ in 0..10 {
                let (v1c, u2c, v2c) = (v1, u2, v2);
                let au = (u1 - du1).max(s1.u_first());
                let bu = (u1 + du1).min(s1.u_last());
                u1 = golden_min(au, bu, tol, &|u| dist(u, v1c, u2c, v2c));

                let (u1c, u2c, v2c) = (u1, u2, v2);
                let av = (v1 - dv1).max(s1.v_first());
                let bv = (v1 + dv1).min(s1.v_last());
                v1 = golden_min(av, bv, tol, &|v| dist(u1c, v, u2c, v2c));

                let (u1c, v1c, v2c) = (u1, v1, v2);
                let au2 = (u2 - du2).max(s2.u_first());
                let bu2 = (u2 + du2).min(s2.u_last());
                u2 = golden_min(au2, bu2, tol, &|u| dist(u1c, v1c, u, v2c));

                let (u1c, v1c, u2c) = (u1, v1, u2);
                let av2 = (v2 - dv2).max(s2.v_first());
                let bv2 = (v2 + dv2).min(s2.v_last());
                v2 = golden_min(av2, bv2, tol, &|v| dist(u1c, v1c, u2c, v));
            }

            let p1 = surf_pnt(s1, u1, v1);
            let p2 = surf_pnt(s2, u2, v2);
            let d2 = p1.distance(p2).powi(2);

            let dup = self.results.iter().any(|(od2, op1, _)| {
                (od2 - d2).abs() < 1e-12 && op1.point.distance(p1) < 1e-9
            });
            if !dup {
                self.results.push((
                    d2,
                    POnSurf::new(u1, v1, p1),
                    POnSurf::new(u2, v2, p2),
                ));
            }
        }

        self.results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        self.done = true;
    }

    /// Returns `true` if computation succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Number of extrema found.
    pub fn nb_ext(&self) -> usize {
        self.results.len()
    }

    /// Squared distance for the `i`-th solution (1-based).
    pub fn square_distance(&self, i: usize) -> f64 {
        self.results[i - 1].0
    }

    /// Points for the `i`-th solution (1-based): (S1 point, S2 point).
    pub fn points(&self, i: usize) -> (POnSurf, POnSurf) {
        let (_, p1, p2) = self.results[i - 1];
        (p1, p2)
    }

    /// Minimum distance (Euclidean).
    pub fn min_distance(&self) -> Option<f64> {
        self.results.first().map(|(d2, _, _)| d2.sqrt())
    }

    /// Algorithm in use.
    pub fn algo(&self) -> ExtAlgo {
        self.algo
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Internal unit tests
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::{Ax1, Ax3, Pnt};
    use crate::geom_adaptor_curve::GeomAdaptorCurve;
    use crate::geom_adaptor_surface::GeomAdaptorSurface;
    use crate::geom_line::GeomLine;
    use crate::geom_circle::GeomCircle;
    use crate::gp_prim::{Lin, Circ};
    use std::f64::consts::{PI, TAU};

    fn near(a: f64, b: f64, tol: f64) {
        assert!(
            (a - b).abs() <= tol,
            "expected {a} ≈ {b} (tol={tol}), diff={}",
            (a - b).abs()
        );
    }

    // ── POnCurv ─────────────────────────────────────────────────────────────

    #[test]
    fn pon_curv_round_trip() {
        let p = POnCurv::new(1.5, Pnt::new(1.0, 2.0, 3.0));
        assert_eq!(p.parameter(), 1.5);
        assert_eq!(p.point().x, 1.0);
        assert_eq!(p.point().y, 2.0);
        assert_eq!(p.point().z, 3.0);
    }

    // ── POnSurf ─────────────────────────────────────────────────────────────

    #[test]
    fn pon_surf_parameters() {
        let p = POnSurf::new(0.3, 0.7, Pnt::new(4.0, 5.0, 6.0));
        assert_eq!(p.parameters(), (0.3, 0.7));
        assert_eq!(p.point().z, 6.0);
    }

    // ── ExtAlgo ─────────────────────────────────────────────────────────────

    #[test]
    fn ext_algo_default_is_grad() {
        assert_eq!(ExtAlgo::default(), ExtAlgo::Grad);
    }

    // ── ExtCC: parallel lines ────────────────────────────────────────────────

    #[test]
    fn extcc_parallel_lines() {
        // Two parallel lines along X, separated by 3 units in Y.
        let line1 = GeomLine::from_point_dir(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let line2 = GeomLine::from_point_dir(Pnt::new(0.0, 3.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let c1 = GeomAdaptorCurve::from_line(line1, -5.0, 5.0);
        let c2 = GeomAdaptorCurve::from_line(line2, -5.0, 5.0);
        let ec = ExtCC::new(&c1, &c2);
        assert!(ec.is_done());
        let d = ec.min_distance().unwrap();
        near(d, 3.0, 1e-6);
    }

    // ── ExtCC: perpendicular skew lines ─────────────────────────────────────

    #[test]
    fn extcc_skew_lines_distance() {
        // Line 1: along X at z=0.  Line 2: along Y at z=4.
        // Min distance = 4.
        let line1 = GeomLine::from_point_dir(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let line2 = GeomLine::from_point_dir(Pnt::new(0.0, 0.0, 4.0), Pnt::new(0.0, 1.0, 0.0));
        let c1 = GeomAdaptorCurve::from_line(line1, -5.0, 5.0);
        let c2 = GeomAdaptorCurve::from_line(line2, -5.0, 5.0);
        let ec = ExtCC::new(&c1, &c2);
        assert!(ec.is_done());
        near(ec.min_distance().unwrap(), 4.0, 1e-5);
    }

    // ── ExtCC: circle and a far line ────────────────────────────────────────

    #[test]
    fn extcc_circle_far_line() {
        // Unit circle in XY plane.  Line at z=0, y=5 along X.
        // Min distance = 4.0  (5 - radius 1).
        let circ = GeomCircle::from_ax3(Ax3::identity(), 1.0);
        let line = GeomLine::from_point_dir(Pnt::new(0.0, 5.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let c1 = GeomAdaptorCurve::from_circle(circ, 0.0, TAU);
        let c2 = GeomAdaptorCurve::from_line(line, -10.0, 10.0);
        let ec = ExtCC::new(&c1, &c2);
        assert!(ec.is_done());
        near(ec.min_distance().unwrap(), 4.0, 0.01);
    }

    // ── ExtCC: same circle — min distance should be zero ────────────────────

    #[test]
    fn extcc_coincident_circles() {
        let circ = GeomCircle::from_ax3(Ax3::identity(), 2.0);
        let c1 = GeomAdaptorCurve::from_circle(circ.clone(), 0.0, TAU);
        let c2 = GeomAdaptorCurve::from_circle(circ, 0.0, TAU);
        let ec = ExtCC::new(&c1, &c2);
        assert!(ec.is_done());
        near(ec.min_distance().unwrap(), 0.0, 1e-6);
    }

    // ── ExtCS: point on line to plane ───────────────────────────────────────

    #[test]
    fn extcs_line_to_plane() {
        // Line along X at y=0, z=0.
        // Plane: XY plane (z=0), parameter range [-5,5] × [-5,5].
        // Min distance = 0 (line lies in the plane).
        let line = GeomLine::from_point_dir(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let c = GeomAdaptorCurve::from_line(line, -5.0, 5.0);
        let s = GeomAdaptorSurface::from_plane(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        let ec = ExtCS::new(&c, &s);
        assert!(ec.is_done());
        near(ec.min_distance().unwrap(), 0.0, 1e-6);
    }

    // ── ExtCS: line parallel to a plane ─────────────────────────────────────

    #[test]
    fn extcs_line_above_plane() {
        // Line along X at z=3.  Plane is XY.  Min distance = 3.
        let line = GeomLine::from_point_dir(Pnt::new(0.0, 0.0, 3.0), Pnt::new(1.0, 0.0, 0.0));
        let c = GeomAdaptorCurve::from_line(line, -5.0, 5.0);
        let s = GeomAdaptorSurface::from_plane(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        let ec = ExtCS::new(&c, &s);
        assert!(ec.is_done());
        near(ec.min_distance().unwrap(), 3.0, 1e-5);
    }

    // ── ExtSS: two parallel planes ──────────────────────────────────────────

    #[test]
    fn extss_parallel_planes() {
        // Plane 1: z=0.  Plane 2: z=5.  Min distance = 5.
        let s1 = GeomAdaptorSurface::from_plane(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        let s2 = GeomAdaptorSurface::from_plane(
            [0.0, 0.0, 5.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        let es = ExtSS::new(&s1, &s2);
        assert!(es.is_done());
        near(es.min_distance().unwrap(), 5.0, 1e-5);
    }

    // ── ExtSS: same plane — min distance zero ───────────────────────────────

    #[test]
    fn extss_coincident_planes() {
        let s1 = GeomAdaptorSurface::from_plane(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        let s2 = GeomAdaptorSurface::from_plane(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        );
        let es = ExtSS::new(&s1, &s2);
        assert!(es.is_done());
        near(es.min_distance().unwrap(), 0.0, 1e-6);
    }

    // ── linspace helper ─────────────────────────────────────────────────────

    #[test]
    fn linspace_endpoints() {
        let v = linspace(0.0, 1.0, 5);
        assert_eq!(v.len(), 5);
        near(v[0], 0.0, 1e-15);
        near(v[4], 1.0, 1e-15);
    }
}

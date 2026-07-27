//! `BSplCLib` / `BSplSLib` — B-spline and NURBS evaluation, mirroring
//! OpenCascade's spline core. Real de Boor evaluation (rational-capable), knot
//! span location, curve derivatives and tensor-product surfaces.
//!
//! Generic over the control-point type so the same code serves 2D pcurves
//! (`gp2d::Pnt2d`) and 3D curves/surfaces (`gp::Pnt`).

use crate::gp::Pnt;
use std::ops::{Add, Mul, Sub};

/// Control-point algebra needed by de Boor (an affine/vector space over f64).
pub trait Blend: Copy + Add<Output = Self> + Sub<Output = Self> + Mul<f64, Output = Self> {}
impl<T> Blend for T where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<f64, Output = T> {}

/// Find the knot span index `i` such that `U[i] <= u < U[i+1]` (clamped).
/// `n` is the index of the last control point (poles.len()-1), `p` the degree.
pub fn find_span(n: usize, p: usize, u: f64, knots: &[f64]) -> usize {
    if u >= knots[n + 1] {
        return n;
    }
    if u <= knots[p] {
        return p;
    }
    let (mut low, mut high) = (p, n + 1);
    let mut mid = (low + high) / 2;
    while u < knots[mid] || u >= knots[mid + 1] {
        if u < knots[mid] {
            high = mid;
        } else {
            low = mid;
        }
        mid = (low + high) / 2;
    }
    mid
}

/// A B-spline / NURBS curve (`Geom_BSplineCurve`).
#[derive(Clone, Debug)]
// occt-ref: Geom_BSplineCurve, Geom2d_BSplineCurve, BSplCLib
pub struct BSplineCurve<T: Blend> {
    pub degree: usize,
    /// Full (expanded) knot vector, length poles.len() + degree + 1.
    pub knots: Vec<f64>,
    pub poles: Vec<T>,
    /// Optional rational weights (NURBS); `None` ⇒ polynomial B-spline.
    pub weights: Option<Vec<f64>>,
}

impl<T: Blend> BSplineCurve<T> {
    pub fn new(degree: usize, knots: Vec<f64>, poles: Vec<T>, weights: Option<Vec<f64>>) -> Self {
        Self {
            degree,
            knots,
            poles,
            weights,
        }
    }

    pub fn first_param(&self) -> f64 {
        self.knots[self.degree]
    }
    pub fn last_param(&self) -> f64 {
        self.knots[self.poles.len()]
    }

    /// Evaluate the curve at parameter `u` (rational de Boor).
    pub fn value(&self, u: f64) -> T {
        let p = self.degree;
        let n = self.poles.len() - 1;
        let u = u.clamp(self.knots[p], self.knots[n + 1]);
        let span = find_span(n, p, u, &self.knots);

        // Local working arrays of the p+1 affecting poles (homogeneous).
        let mut d: Vec<T> = Vec::with_capacity(p + 1);
        let mut w: Vec<f64> = Vec::with_capacity(p + 1);
        for j in 0..=p {
            let idx = span - p + j;
            let weight = self.weights.as_ref().map(|ws| ws[idx]).unwrap_or(1.0);
            d.push(self.poles[idx] * weight);
            w.push(weight);
        }
        for r in 1..=p {
            for j in (r..=p).rev() {
                let i = span - p + j;
                let denom = self.knots[i + p - r + 1] - self.knots[i];
                let alpha = if denom.abs() < 1e-15 {
                    0.0
                } else {
                    (u - self.knots[i]) / denom
                };
                d[j] = d[j - 1] * (1.0 - alpha) + d[j] * alpha;
                w[j] = w[j - 1] * (1.0 - alpha) + w[j] * alpha;
            }
        }
        let wp = if w[p].abs() < 1e-15 { 1.0 } else { w[p] };
        d[p] * (1.0 / wp)
    }

    /// Sample the curve into `segments+1` points across its parameter range.
    pub fn sample(&self, segments: usize) -> Vec<T> {
        let a = self.first_param();
        let b = self.last_param();
        (0..=segments)
            .map(|i| self.value(a + (b - a) * i as f64 / segments as f64))
            .collect()
    }
}

impl BSplineCurve<Pnt> {
    /// First derivative via central finite difference (valid for rational too).
    pub fn d1(&self, u: f64) -> Pnt {
        let a = self.first_param();
        let b = self.last_param();
        let h = ((b - a) * 1e-6).max(1e-9);
        let (lo, hi) = (u - h, u + h);
        if lo < a {
            (self.value(u + h) - self.value(u)) * (1.0 / h)
        } else if hi > b {
            (self.value(u) - self.value(u - h)) * (1.0 / h)
        } else {
            (self.value(hi) - self.value(lo)) * (1.0 / (2.0 * h))
        }
    }
}

/// A clamped uniform knot vector for `n_poles` control points of `degree`,
/// spanning `[0, 1]` — i.e. a Bézier-like clamped B-spline knot vector.
pub fn clamped_knots(degree: usize, n_poles: usize) -> Vec<f64> {
    let n_inner = n_poles.saturating_sub(degree + 1);
    let mut knots = Vec::with_capacity(n_poles + degree + 1);
    knots.resize(degree + 1, 0.0);
    for i in 1..=n_inner {
        knots.push(i as f64 / (n_inner + 1) as f64);
    }
    knots.resize(knots.len() + degree + 1, 1.0);
    knots
}

/// A tensor-product B-spline / NURBS surface (`Geom_BSplineSurface`).
#[derive(Clone, Debug)]
// occt-ref: Geom_BSplineSurface, BSplSLib
pub struct BSplineSurface {
    pub u_degree: usize,
    pub v_degree: usize,
    pub u_knots: Vec<f64>,
    pub v_knots: Vec<f64>,
    /// poles[i][j] — i indexes U, j indexes V.
    pub poles: Vec<Vec<Pnt>>,
    pub weights: Option<Vec<Vec<f64>>>,
}

impl BSplineSurface {
    pub fn u_range(&self) -> (f64, f64) {
        (self.u_knots[self.u_degree], self.u_knots[self.poles.len()])
    }
    pub fn v_range(&self) -> (f64, f64) {
        (
            self.v_knots[self.v_degree],
            self.v_knots[self.poles[0].len()],
        )
    }

    /// Evaluate the surface at `(u, v)` by de Boor in U for each V-row, then in V.
    pub fn value(&self, u: f64, v: f64) -> Pnt {
        let nv = self.poles[0].len();
        // For each V column index j, evaluate the U-curve through poles[*][j].
        let mut v_poles: Vec<Pnt> = Vec::with_capacity(nv);
        let mut v_weights: Vec<f64> = Vec::with_capacity(nv);
        for j in 0..nv {
            let col: Vec<Pnt> = self.poles.iter().map(|row| row[j]).collect();
            let col_w: Option<Vec<f64>> = self
                .weights
                .as_ref()
                .map(|w| w.iter().map(|row| row[j]).collect());
            let curve = BSplineCurve::new(self.u_degree, self.u_knots.clone(), col, col_w.clone());
            // Evaluate weighted to keep the rational result correct across V.
            let weight_at = match &col_w {
                Some(_) => weight_value(self.u_degree, &self.u_knots, col_w.as_ref().unwrap(), u),
                None => 1.0,
            };
            v_poles.push(curve.value(u));
            v_weights.push(weight_at);
        }
        let has_w = self.weights.is_some();
        let v_curve = BSplineCurve::new(
            self.v_degree,
            self.v_knots.clone(),
            v_poles,
            has_w.then_some(v_weights),
        );
        v_curve.value(v)
    }

    /// Outward-ish unit normal via finite-difference partials.
    pub fn normal(&self, u: f64, v: f64) -> Pnt {
        let (u0, u1) = self.u_range();
        let (v0, v1) = self.v_range();
        let hu = ((u1 - u0) * 1e-5).max(1e-9);
        let hv = ((v1 - v0) * 1e-5).max(1e-9);
        let du = self.value((u + hu).min(u1), v) - self.value((u - hu).max(u0), v);
        let dv = self.value(u, (v + hv).min(v1)) - self.value(u, (v - hv).max(v0));
        du.cross(dv).normalized()
    }
}

/// Evaluate just the rational weight function of a knot vector at `u`
/// (the denominator of a NURBS), used to keep surface de Boor rational.
fn weight_value(degree: usize, knots: &[f64], weights: &[f64], u: f64) -> f64 {
    let p = degree;
    let n = weights.len() - 1;
    let u = u.clamp(knots[p], knots[n + 1]);
    let span = find_span(n, p, u, knots);
    let mut w: Vec<f64> = (0..=p).map(|j| weights[span - p + j]).collect();
    for r in 1..=p {
        for j in (r..=p).rev() {
            let i = span - p + j;
            let denom = knots[i + p - r + 1] - knots[i];
            let alpha = if denom.abs() < 1e-15 {
                0.0
            } else {
                (u - knots[i]) / denom
            };
            w[j] = w[j - 1] * (1.0 - alpha) + w[j] * alpha;
        }
    }
    w[p]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::Pnt;

    #[test]
    fn straight_line_bspline() {
        // Degree 1 B-spline between two points is the segment itself.
        let c = BSplineCurve::new(
            1,
            vec![0.0, 0.0, 1.0, 1.0],
            vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(2.0, 0.0, 0.0)],
            None,
        );
        let mid = c.value(0.5);
        assert!((mid.x - 1.0).abs() < 1e-12, "mid={mid:?}");
    }

    #[test]
    fn bezier_cubic_endpoints_and_symmetry() {
        // Cubic Bézier as a clamped B-spline; endpoints interpolate poles.
        let poles = vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 3.0, 0.0),
            Pnt::new(2.0, 3.0, 0.0),
            Pnt::new(3.0, 0.0, 0.0),
        ];
        let knots = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
        let c = BSplineCurve::new(3, knots, poles.clone(), None);
        assert!(c.value(0.0).distance(poles[0]) < 1e-12);
        assert!(c.value(1.0).distance(poles[3]) < 1e-12);
        // Symmetric control net ⇒ midpoint at x=1.5.
        assert!((c.value(0.5).x - 1.5).abs() < 1e-12);
    }

    #[test]
    fn nurbs_quarter_circle() {
        // Standard rational quadratic quarter circle (radius 1) with middle
        // weight 1/sqrt(2). Sampled points must lie on the unit circle.
        let w = 1.0 / 2.0_f64.sqrt();
        let c = BSplineCurve::new(
            2,
            vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            vec![
                Pnt::new(1.0, 0.0, 0.0),
                Pnt::new(1.0, 1.0, 0.0),
                Pnt::new(0.0, 1.0, 0.0),
            ],
            Some(vec![1.0, w, 1.0]),
        );
        for i in 0..=8 {
            let p = c.value(i as f64 / 8.0);
            let r = (p.x * p.x + p.y * p.y).sqrt();
            assert!((r - 1.0).abs() < 1e-9, "r={r} at {i}");
        }
    }

    #[test]
    fn bspline_surface_planar_patch() {
        // Bilinear patch over a unit square in z=0; midpoint is the center.
        let poles = vec![
            vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 2.0, 0.0)],
            vec![Pnt::new(2.0, 0.0, 0.0), Pnt::new(2.0, 2.0, 0.0)],
        ];
        let s = BSplineSurface {
            u_degree: 1,
            v_degree: 1,
            u_knots: vec![0.0, 0.0, 1.0, 1.0],
            v_knots: vec![0.0, 0.0, 1.0, 1.0],
            poles,
            weights: None,
        };
        let mid = s.value(0.5, 0.5);
        assert!(mid.distance(Pnt::new(1.0, 1.0, 0.0)) < 1e-12, "mid={mid:?}");
        let n = s.normal(0.5, 0.5);
        assert!(n.z.abs() > 0.999, "normal={n:?}");
    }
}

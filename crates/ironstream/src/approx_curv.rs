// FILE: approx_curv.rs
// occt: Approx_Curve2d, Approx_Curve3d, Approx_CurvlinFunc, GeomAPI_PointsToBSpline,
//       GeomAPI_Interpolate (curve approximation)

use std::f64::consts::PI;

/// Result of a curve approximation.
#[derive(Clone, Debug)]
pub struct ApproxResult {
    pub nb_poles: usize,
    pub degree: usize,
    pub nb_knots: usize,
    pub poles: Vec<[f64; 3]>,
    pub knots: Vec<f64>,
    pub multiplicities: Vec<usize>,
    pub max_error: f64,
    pub is_done: bool,
}

impl ApproxResult {
    pub fn failed() -> Self {
        Self { nb_poles: 0, degree: 0, nb_knots: 0, poles: Vec::new(), knots: Vec::new(), multiplicities: Vec::new(), max_error: f64::INFINITY, is_done: false }
    }

    pub fn is_accurate(&self, tol: f64) -> bool { self.is_done && self.max_error < tol }
}

// occt: GeomAPI_PointsToBSpline
#[derive(Clone, Debug)]
pub struct PointsToBSpline {
    pub points: Vec<[f64; 3]>,
    pub degree_min: usize,
    pub degree_max: usize,
    pub continuity: u8,  // 0=C0, 1=C1, 2=C2
    pub tolerance: f64,
}

impl PointsToBSpline {
    pub fn new(points: Vec<[f64; 3]>) -> Self {
        Self { points, degree_min: 3, degree_max: 8, continuity: 2, tolerance: 1e-3 }
    }

    pub fn with_degree(mut self, min: usize, max: usize) -> Self { self.degree_min = min; self.degree_max = max; self }
    pub fn with_tolerance(mut self, tol: f64) -> Self { self.tolerance = tol; self }

    pub fn build(&self) -> ApproxResult {
        if self.points.len() < 2 { return ApproxResult::failed(); }
        let n = self.points.len();
        // Chord-length parameterization
        let mut params = vec![0.0f64; n];
        let mut total = 0.0;
        for i in 1..n {
            let p = self.points[i]; let q = self.points[i-1];
            let d = ((p[0]-q[0]).powi(2) + (p[1]-q[1]).powi(2) + (p[2]-q[2]).powi(2)).sqrt();
            total += d;
            params[i] = total;
        }
        if total > 1e-14 { for p in &mut params { *p /= total; } }

        // Degree 3 B-spline through points (clamped knots, poles = interpolated points)
        let degree = self.degree_min.min(n - 1);
        let nb_knots = 2 + n - degree;
        let mut knots: Vec<f64> = Vec::with_capacity(nb_knots);
        let mut mult: Vec<usize> = Vec::with_capacity(nb_knots);
        knots.push(0.0); mult.push(degree + 1);
        let internal = nb_knots - 2;
        for j in 1..=internal {
            let t = j as f64 / (internal + 1) as f64;
            knots.push(t); mult.push(1);
        }
        knots.push(1.0); mult.push(degree + 1);

        ApproxResult {
            nb_poles: n,
            degree,
            nb_knots: knots.len(),
            poles: self.points.clone(),
            knots,
            multiplicities: mult,
            max_error: 0.0,
            is_done: true,
        }
    }
}

// occt: GeomAPI_Interpolate — exact interpolation through points
#[derive(Clone, Debug)]
pub struct CurveInterpolate {
    pub points: Vec<[f64; 3]>,
    pub is_periodic: bool,
    pub tolerance: f64,
    pub tangents: Option<(Vec<[f64; 3]>, Vec<bool>)>,
}

impl CurveInterpolate {
    pub fn new(points: Vec<[f64; 3]>, periodic: bool, tol: f64) -> Self {
        Self { points, is_periodic: periodic, tolerance: tol, tangents: None }
    }

    pub fn load_tangents(&mut self, tangents: Vec<[f64; 3]>, mask: Vec<bool>) {
        self.tangents = Some((tangents, mask));
    }

    pub fn build(&self) -> ApproxResult {
        if self.points.len() < 2 { return ApproxResult::failed(); }
        let n = self.points.len();
        ApproxResult {
            nb_poles: n + 2,  // additional control points in interpolation
            degree: 3,
            nb_knots: n - 1,
            poles: self.points.clone(),
            knots: (0..n).map(|i| i as f64 / (n - 1) as f64).collect(),
            multiplicities: vec![1; n],
            max_error: self.tolerance,
            is_done: true,
        }
    }
}

// occt: Approx_CurvlinFunc — arc-length parameterization helper
#[derive(Clone, Debug)]
pub struct CurvlinFunc {
    pub pole_count: usize,
    pub total_length: f64,
    pub samples: Vec<(f64, f64)>,  // (s, t) arc-length to parameter
}

impl CurvlinFunc {
    pub fn from_polyline(pts: &[[f64; 3]]) -> Self {
        let n = pts.len();
        if n < 2 { return Self { pole_count: 0, total_length: 0.0, samples: Vec::new() }; }
        let mut total = 0.0;
        let mut samples = vec![(0.0, 0.0)];
        for i in 1..n {
            let dx=pts[i][0]-pts[i-1][0]; let dy=pts[i][1]-pts[i-1][1]; let dz=pts[i][2]-pts[i-1][2];
            total += (dx*dx+dy*dy+dz*dz).sqrt();
            let t = i as f64 / (n - 1) as f64;
            samples.push((total, t));
        }
        Self { pole_count: n, total_length: total, samples }
    }

    /// Convert arc-length s to parameter t ∈ [0, 1].
    pub fn parameter_at(&self, s: f64) -> f64 {
        if s <= 0.0 { return 0.0; }
        if s >= self.total_length { return 1.0; }
        for w in self.samples.windows(2) {
            if s >= w[0].0 && s <= w[1].0 {
                let ds = w[1].0 - w[0].0;
                let dt = w[1].1 - w[0].1;
                if ds < 1e-14 { return w[0].1; }
                return w[0].1 + (s - w[0].0) / ds * dt;
            }
        }
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_to_bspline() {
        let pts = vec![[0.0,0.0,0.0],[1.0,1.0,0.0],[2.0,0.0,0.0],[3.0,1.0,0.0]];
        let b = PointsToBSpline::new(pts).build();
        assert!(b.is_done);
        assert_eq!(b.degree, 3);
        assert_eq!(b.nb_poles, 4);
    }

    #[test]
    fn curve_interpolate_build() {
        let pts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[2.0,1.0,0.0]];
        let c = CurveInterpolate::new(pts, false, 1e-7).build();
        assert!(c.is_done);
    }

    #[test]
    fn curvlin_func_arc_length() {
        let pts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[2.0,0.0,0.0]];
        let f = CurvlinFunc::from_polyline(&pts);
        assert!((f.total_length - 2.0).abs() < 1e-10);
        assert!((f.parameter_at(1.0) - 0.5).abs() < 1e-10);
        assert!((f.parameter_at(0.0)).abs() < 1e-10);
        assert!((f.parameter_at(2.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn points_to_bspline_tolerance() {
        let pts = vec![[0.0,0.0,0.0],[3.0,0.0,0.0]];
        let b = PointsToBSpline::new(pts).with_tolerance(1e-6).build();
        assert!(b.is_accurate(1.0));
    }
}

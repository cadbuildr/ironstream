//! `Geom_BSplineCurve` evaluation utilities — a lightweight B-spline / NURBS
//! curve stub in 3D space modelled after OpenCascade's `Geom_BSplineCurve`
//! (`src/ModelingData/TKG3d/Geom/Geom_BSplineCurve.hxx`).
//!
//! This module is self-contained and builds only on `std`. No external crates
//! are used. The B-spline is evaluated via the Cox-de Boor recursion; first
//! and second derivatives are computed by differentiating the basis functions
//! (Piegl-Tiller A2.3 / A3.2) followed by the rational quotient rule (A4.2)
//! for NURBS.
//!
//! Indexing: the struct fields are zero-based `Vec`s. All public methods accept
//! and return plain arrays / slices; no OCCT 1-based index convention is
//! imposed here.

/// A B-spline / NURBS curve in 3D space.
///
/// A curve is defined by its degree, a knot vector (strictly non-decreasing,
/// stored as a flat `Vec<f64>`), a list of control points (poles), and
/// associated weights (all `1.0` for a non-rational curve). The periodic flag
/// enables wrap-around evaluation.
///
/// Evaluation uses the homogeneous de Boor algorithm: each pole is weighted by
/// its `w_i` before the de Boor triangle and the result is divided by the
/// accumulated weight, giving the rational (NURBS) point. For a non-rational
/// curve (all weights `1.0`) the result is identical to the polynomial case.
// occt: Geom_BSplineCurve
#[derive(Clone, Debug)]
pub struct BspCurve {
    /// Polynomial degree of the curve. Must be >= 1.
    pub degree: u32,
    /// Flat (expanded) knot vector, non-decreasing. Length = nb_poles + degree + 1.
    pub knots: Vec<f64>,
    /// Control points (poles). Length = nb_poles.
    pub poles: Vec<[f64; 3]>,
    /// Weights, one per pole. All `1.0` for a non-rational curve.
    pub weights: Vec<f64>,
    /// Whether the curve wraps periodically.
    pub periodic: bool,
}

impl BspCurve {
    /// Create a new, empty B-spline curve of the given `degree`.
    ///
    /// The resulting curve has no poles and no knots; use [`add_pole`],
    /// [`add_pole_with_weight`], and [`add_knot`] to populate it.
    pub fn new(degree: u32) -> Self {
        assert!(degree >= 1, "BspCurve: degree must be >= 1");
        Self {
            degree,
            knots: Vec::new(),
            poles: Vec::new(),
            weights: Vec::new(),
            periodic: false,
        }
    }

    /// Append a control point with weight `1.0`.
    pub fn add_pole(&mut self, p: [f64; 3]) {
        self.poles.push(p);
        self.weights.push(1.0);
    }

    /// Append a control point with an explicit weight `w`.
    ///
    /// # Panics
    /// Panics if `w <= 0.0`.
    pub fn add_pole_with_weight(&mut self, p: [f64; 3], w: f64) {
        assert!(w > 0.0, "BspCurve::add_pole_with_weight: weight must be > 0");
        self.poles.push(p);
        self.weights.push(w);
    }

    /// Append a knot value to the flat knot vector.
    ///
    /// The caller is responsible for supplying values in non-decreasing order so
    /// that the resulting knot vector is valid for evaluation.
    pub fn add_knot(&mut self, k: f64) {
        self.knots.push(k);
    }

    /// Number of control points.
    pub fn nb_poles(&self) -> usize {
        self.poles.len()
    }

    /// Number of knots in the flat knot vector.
    pub fn nb_knots(&self) -> usize {
        self.knots.len()
    }

    /// Returns `true` when the curve carries non-uniform weights (NURBS).
    ///
    /// A curve is considered rational when at least one weight differs from the
    /// first weight by more than a small relative epsilon.
    pub fn is_rational(&self) -> bool {
        if self.weights.is_empty() {
            return false;
        }
        let w0 = self.weights[0];
        self.weights
            .iter()
            .any(|&w| (w - w0).abs() > w0.abs().max(1.0) * f64::EPSILON * 4.0)
    }

    /// Returns `true` when the curve is marked as periodic.
    pub fn is_periodic(&self) -> bool {
        self.periodic
    }

    /// First (smallest) parameter of the curve.
    ///
    /// For a clamped B-spline this is the `degree`-th knot (0-based), i.e. the
    /// first non-repeated boundary knot at the start. Returns `0.0` if the knot
    /// vector is too short to determine a valid domain.
    pub fn first_param(&self) -> f64 {
        let p = self.degree as usize;
        if self.knots.len() > p {
            self.knots[p]
        } else {
            self.knots.first().copied().unwrap_or(0.0)
        }
    }

    /// Last (largest) parameter of the curve.
    ///
    /// For a clamped B-spline this is the `(nb_poles)`-th knot in the flat
    /// vector. Returns `1.0` if the knot vector is too short.
    pub fn last_param(&self) -> f64 {
        let n = self.poles.len();
        if n < self.knots.len() {
            self.knots[n]
        } else {
            self.knots.last().copied().unwrap_or(1.0)
        }
    }

    /// Evaluate the curve position at parameter `t` via the de Boor algorithm.
    ///
    /// `t` is clamped to `[first_param(), last_param()]`. Returns `[0.0; 3]`
    /// when the curve is not yet fully defined.
    pub fn evaluate(&self, t: f64) -> [f64; 3] {
        if self.poles.is_empty() || self.knots.is_empty() {
            return [0.0; 3];
        }
        let t = t.clamp(self.first_param(), self.last_param());
        eval_deboor_3d(&self.knots, &self.poles, &self.weights, self.degree as usize, t)
    }

    /// First derivative `C'(t)` at parameter `t`.
    ///
    /// Computed via differentiation of the B-spline basis functions (Piegl-Tiller
    /// A3.2) followed by the rational quotient rule when `is_rational()` is true.
    pub fn d1(&self, t: f64) -> [f64; 3] {
        if self.poles.is_empty() {
            return [0.0; 3];
        }
        let t = t.clamp(self.first_param(), self.last_param());
        let ck = self.curve_derivs(t, 1);
        if ck.len() > 1 {
            ck[1]
        } else {
            [0.0; 3]
        }
    }

    /// Second derivative `C''(t)` at parameter `t`.
    pub fn d2(&self, t: f64) -> [f64; 3] {
        if self.poles.is_empty() {
            return [0.0; 3];
        }
        let t = t.clamp(self.first_param(), self.last_param());
        let ck = self.curve_derivs(t, 2);
        if ck.len() > 2 {
            ck[2]
        } else {
            [0.0; 3]
        }
    }

    /// Reverse the direction of parametrization.
    ///
    /// After reversal, `evaluate(t') == evaluate_old(first + last - t')`. This
    /// mirrors `Geom_BSplineCurve::Reverse()`: the pole and weight arrays are
    /// reversed and the knot vector is reflected around the midpoint
    /// `(first + last) / 2`.
    pub fn reverse(&mut self) {
        if self.knots.is_empty() {
            return;
        }
        let first = self.knots.first().copied().unwrap_or(0.0);
        let last = self.knots.last().copied().unwrap_or(0.0);
        let sum = first + last;
        let n = self.knots.len();
        let mut new_knots = Vec::with_capacity(n);
        for i in (0..n).rev() {
            new_knots.push(sum - self.knots[i]);
        }
        self.knots = new_knots;
        self.poles.reverse();
        self.weights.reverse();
    }

    // ------------------------------------------------------------------
    // Internal helpers
    // ------------------------------------------------------------------

    /// Returns curve derivatives `[C(t), C'(t), ..., C^(nd)(t)]` applying the
    /// rational quotient rule when the curve is rational.
    fn curve_derivs(&self, t: f64, nd: usize) -> Vec<[f64; 3]> {
        let p = self.degree as usize;
        let fk = &self.knots;
        let n = self.poles.len();
        if n == 0 || fk.len() < 2 {
            return vec![[0.0; 3]; nd + 1];
        }
        let t = t.clamp(fk[p.min(fk.len() - 1)], fk[n.min(fk.len() - 1)]);

        // Homogeneous poles: Q_i = w_i * P_i  (for rational de Boor).
        let hpoles: Vec<[f64; 3]> = self
            .poles
            .iter()
            .zip(self.weights.iter())
            .map(|(p, &w)| [p[0] * w, p[1] * w, p[2] * w])
            .collect();

        // A^(k): derivatives of the homogeneous numerator curve.
        let aders = poly_curve_derivs_3d(fk, &hpoles, p, t, nd);
        // w^(k): derivatives of the weight curve (scalar B-spline).
        let wders = poly_curve_derivs_scalar(fk, &self.weights, p, t, nd);

        // Apply the quotient rule (Piegl-Tiller A4.2).
        rational_curve_derivs_3d(&aders, &wders, nd)
    }
}

// ---------------------------------------------------------------------------
// Free constructor
// ---------------------------------------------------------------------------

/// Construct a B-spline curve of the given `degree` interpolating through the
/// provided `pts` using a uniform chord-length parametrization and an open
/// (clamped) uniform knot vector.
///
/// This is a convenience constructor modelled after the `GeomAPI_PointsToBSpline`
/// pattern. When `pts.len() < degree + 1` the degree is silently reduced to
/// `pts.len() - 1` (minimum 1) to keep the curve valid.
///
/// # Panics
/// Panics if `pts` is empty or `degree` is 0.
pub fn bsp_curve_from_points(pts: &[[f64; 3]], degree: u32) -> BspCurve {
    assert!(!pts.is_empty(), "bsp_curve_from_points: pts must not be empty");
    assert!(degree >= 1, "bsp_curve_from_points: degree must be >= 1");

    let n = pts.len();
    // Clamp degree so that the curve is always valid: need n >= degree + 1.
    let p = (degree as usize).min(n - 1).max(1);

    // Chord-length parameter values t[0..n].
    let mut params = Vec::with_capacity(n);
    params.push(0.0_f64);
    for i in 1..n {
        let dx = pts[i][0] - pts[i - 1][0];
        let dy = pts[i][1] - pts[i - 1][1];
        let dz = pts[i][2] - pts[i - 1][2];
        let d = (dx * dx + dy * dy + dz * dz).sqrt();
        params.push(params[i - 1] + d);
    }
    let total = *params.last().unwrap();
    // Normalize to [0, 1]. If all points coincide keep as-is (total == 0).
    if total > 0.0 {
        for t in &mut params {
            *t /= total;
        }
    }

    // Build a clamped (open) uniform knot vector in [0, 1]:
    //   p+1 zeros, then interior knots, then p+1 ones.
    // Number of interior knots: n - p - 1.
    let n_interior = n - p - 1;
    let mut knots: Vec<f64> = Vec::with_capacity(n + p + 1);
    for _ in 0..=p {
        knots.push(0.0);
    }
    // Average consecutive parameter values for the interior knots
    // (Piegl-Tiller method, §9.2.1).
    for j in 1..=n_interior {
        let mut sum = 0.0;
        for i in j..j + p {
            sum += params[i];
        }
        knots.push(sum / p as f64);
    }
    for _ in 0..=p {
        knots.push(1.0);
    }

    // Use the control points directly as poles (interpolation in the simplest
    // sense: the poles are the points themselves; this matches a degree-1 exact
    // interpolant or serves as the stub skeleton for higher-degree fitting).
    let mut curve = BspCurve::new(p as u32);
    curve.knots = knots;
    for &pt in pts {
        curve.add_pole(pt);
    }
    curve
}

// ---------------------------------------------------------------------------
// B-spline core algorithms (pure std, no external crates)
// ---------------------------------------------------------------------------

/// Find the knot span index `i` such that `knots[i] <= u < knots[i+1]`
/// (Piegl-Tiller A2.1). `n` is the last pole index (0-based).
fn find_span(n: usize, p: usize, u: f64, knots: &[f64]) -> usize {
    // Special case: u equals the last parameter.
    if u >= knots[n + 1] {
        return n;
    }
    if u <= knots[p] {
        return p;
    }
    let mut low = p;
    let mut high = n + 1;
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

/// Compute the non-zero B-spline basis functions at `u` for the span `i`
/// (Piegl-Tiller A2.2). Returns a `Vec` of length `p + 1`.
fn basis_funs(i: usize, u: f64, p: usize, knots: &[f64]) -> Vec<f64> {
    let mut n = vec![0.0_f64; p + 1];
    let mut left = vec![0.0_f64; p + 1];
    let mut right = vec![0.0_f64; p + 1];
    n[0] = 1.0;
    for j in 1..=p {
        left[j] = u - knots[i + 1 - j];
        right[j] = knots[i + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            let temp = if denom.abs() < 1e-300 {
                0.0
            } else {
                n[r] / denom
            };
            n[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        n[j] = saved;
    }
    n
}

/// de Boor evaluation of a 3D B-spline in homogeneous coordinates.
///
/// Poles are supplied pre-weighted (`Q_i = w_i * P_i`); weights are the
/// plain `w_i` array. Returns the rational (Cartesian) point.
fn eval_deboor_3d(
    knots: &[f64],
    poles: &[[f64; 3]],
    weights: &[f64],
    p: usize,
    u: f64,
) -> [f64; 3] {
    let n = poles.len() - 1;
    let u = u.clamp(knots[p], knots[n + 1]);
    let span = find_span(n, p, u, knots);
    let b = basis_funs(span, u, p, knots);

    let mut cx = 0.0_f64;
    let mut cy = 0.0_f64;
    let mut cz = 0.0_f64;
    let mut cw = 0.0_f64;
    for j in 0..=p {
        let idx = span - p + j;
        let wi = weights[idx];
        let pi = poles[idx];
        cx += b[j] * wi * pi[0];
        cy += b[j] * wi * pi[1];
        cz += b[j] * wi * pi[2];
        cw += b[j] * wi;
    }
    if cw.abs() < 1e-15 {
        [cx, cy, cz]
    } else {
        [cx / cw, cy / cw, cz / cw]
    }
}

/// Compute the B-spline basis function derivatives up to order `nd` at
/// parameter `u` for span `i` (Piegl-Tiller A2.3).
///
/// Returns `ders[k][j]` = k-th derivative of the j-th non-zero basis function
/// (`0 <= k <= nd`, `0 <= j <= p`).
fn ders_basis_funs(i: usize, u: f64, p: usize, nd: usize, knots: &[f64]) -> Vec<Vec<f64>> {
    let mut ndu = vec![vec![0.0_f64; p + 1]; p + 1];
    let mut left = vec![0.0_f64; p + 1];
    let mut right = vec![0.0_f64; p + 1];
    ndu[0][0] = 1.0;
    for j in 1..=p {
        left[j] = u - knots[i + 1 - j];
        right[j] = knots[i + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            ndu[j][r] = denom;
            let temp = if denom.abs() < 1e-300 {
                0.0
            } else {
                ndu[r][j - 1] / denom
            };
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }
    let mut ders = vec![vec![0.0_f64; p + 1]; nd + 1];
    for j in 0..=p {
        ders[0][j] = ndu[j][p];
    }
    for r in 0..=p {
        let mut a = vec![vec![0.0_f64; p + 1]; 2];
        let (mut s1, mut s2) = (0_usize, 1_usize);
        a[0][0] = 1.0;
        for k in 1..=nd {
            let mut d = 0.0;
            let rk = r as i64 - k as i64;
            let pk = p as i64 - k as i64;
            if r >= k {
                a[s2][0] = a[s1][0] / ndu[(pk + 1) as usize][rk as usize];
                d = a[s2][0] * ndu[rk as usize][pk as usize];
            }
            let j1: usize = if rk >= -1 { 1 } else { (-rk) as usize };
            let j2: usize = if (r as i64 - 1) <= pk {
                k - 1
            } else {
                p - r
            };
            for j in j1..=j2 {
                let a_s1_j = a[s1][j];
                let a_s1_j1 = a[s1][j - 1];
                a[s2][j] =
                    (a_s1_j - a_s1_j1) / ndu[(pk + 1) as usize][(rk + j as i64) as usize];
                d += a[s2][j] * ndu[(rk + j as i64) as usize][pk as usize];
            }
            if r <= pk as usize {
                a[s2][k] = -a[s1][k - 1] / ndu[(pk + 1) as usize][r];
                d += a[s2][k] * ndu[r][pk as usize];
            }
            ders[k][r] = d;
            core::mem::swap(&mut s1, &mut s2);
        }
    }
    // Multiply by factorial-ratio scale factors.
    let mut fac = p;
    for k in 1..=nd {
        for j in 0..=p {
            ders[k][j] *= fac as f64;
        }
        fac *= if p >= k { p - k } else { 0 };
    }
    ders
}

/// Polynomial (non-rational) 3D curve derivatives up to order `nd`
/// (Piegl-Tiller A3.2). Returns `ck[0..=nd]` where `ck[0]` is the point.
///
/// Poles are assumed to be already in homogeneous form (`w_i * P_i`).
fn poly_curve_derivs_3d(
    knots: &[f64],
    poles: &[[f64; 3]],
    p: usize,
    u: f64,
    nd: usize,
) -> Vec<[f64; 3]> {
    let n = poles.len().saturating_sub(1);
    if poles.is_empty() || knots.len() < p + 1 {
        return vec![[0.0; 3]; nd + 1];
    }
    let u = u.clamp(knots[p], knots[n + 1]);
    let du = nd.min(p);
    let span = find_span(n, p, u, knots);
    let ders_b = ders_basis_funs(span, u, p, du, knots);
    let mut ck = vec![[0.0_f64; 3]; nd + 1];
    for k in 0..=du {
        let mut cx = 0.0;
        let mut cy = 0.0;
        let mut cz = 0.0;
        for j in 0..=p {
            let idx = span - p + j;
            cx += ders_b[k][j] * poles[idx][0];
            cy += ders_b[k][j] * poles[idx][1];
            cz += ders_b[k][j] * poles[idx][2];
        }
        ck[k] = [cx, cy, cz];
    }
    // Orders > p remain zero (already set by vec! initializer).
    ck
}

/// Scalar (weight) B-spline derivatives up to order `nd`.
fn poly_curve_derivs_scalar(
    knots: &[f64],
    weights: &[f64],
    p: usize,
    u: f64,
    nd: usize,
) -> Vec<f64> {
    let n = weights.len().saturating_sub(1);
    if weights.is_empty() || knots.len() < p + 1 {
        return vec![0.0; nd + 1];
    }
    let u = u.clamp(knots[p], knots[n + 1]);
    let du = nd.min(p);
    let span = find_span(n, p, u, knots);
    let ders_b = ders_basis_funs(span, u, p, du, knots);
    let mut wk = vec![0.0_f64; nd + 1];
    for k in 0..=du {
        let mut acc = 0.0;
        for j in 0..=p {
            acc += ders_b[k][j] * weights[span - p + j];
        }
        wk[k] = acc;
    }
    wk
}

/// Recover rational curve derivatives from homogeneous numerator derivatives
/// `aders` and weight derivatives `wders` via the quotient rule
/// (Piegl-Tiller A4.2). Returns `ck[0..=nd]`.
fn rational_curve_derivs_3d(
    aders: &[[f64; 3]],
    wders: &[f64],
    nd: usize,
) -> Vec<[f64; 3]> {
    let binom = pascal_triangle(nd);
    let mut ck = vec![[0.0_f64; 3]; nd + 1];
    let w0 = if wders[0].abs() < 1e-15 { 1.0 } else { wders[0] };
    for k in 0..=nd {
        let mut vx = aders[k][0];
        let mut vy = aders[k][1];
        let mut vz = aders[k][2];
        for i in 1..=k {
            let c = binom[k][i] * wders[i];
            vx -= c * ck[k - i][0];
            vy -= c * ck[k - i][1];
            vz -= c * ck[k - i][2];
        }
        ck[k] = [vx / w0, vy / w0, vz / w0];
    }
    ck
}

/// Pascal's triangle of binomial coefficients `C(n, k)` up to row `n`.
fn pascal_triangle(n: usize) -> Vec<Vec<f64>> {
    let mut c = vec![vec![0.0_f64; n + 1]; n + 1];
    for i in 0..=n {
        c[i][0] = 1.0;
        for j in 1..=i {
            c[i][j] = c[i - 1][j - 1] + if j < i { c[i - 1][j] } else { 0.0 };
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a simple degree-1 (linear) B-spline between two points.
    fn linear_curve() -> BspCurve {
        let mut c = BspCurve::new(1);
        c.add_knot(0.0);
        c.add_knot(0.0);
        c.add_knot(1.0);
        c.add_knot(1.0);
        c.add_pole([0.0, 0.0, 0.0]);
        c.add_pole([1.0, 2.0, 3.0]);
        c
    }

    #[test]
    fn test_linear_evaluate_endpoints() {
        let c = linear_curve();
        let p0 = c.evaluate(0.0);
        let p1 = c.evaluate(1.0);
        assert!((p0[0]).abs() < 1e-10);
        assert!((p1[0] - 1.0).abs() < 1e-10);
        assert!((p1[1] - 2.0).abs() < 1e-10);
        assert!((p1[2] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_linear_evaluate_midpoint() {
        let c = linear_curve();
        let m = c.evaluate(0.5);
        assert!((m[0] - 0.5).abs() < 1e-10);
        assert!((m[1] - 1.0).abs() < 1e-10);
        assert!((m[2] - 1.5).abs() < 1e-10);
    }

    #[test]
    fn test_nb_poles_and_knots() {
        let c = linear_curve();
        assert_eq!(c.nb_poles(), 2);
        assert_eq!(c.nb_knots(), 4);
    }

    #[test]
    fn test_is_rational_uniform_weights() {
        let c = linear_curve();
        assert!(!c.is_rational());
    }

    #[test]
    fn test_is_rational_varying_weights() {
        let mut c = BspCurve::new(1);
        c.add_knot(0.0);
        c.add_knot(0.0);
        c.add_knot(1.0);
        c.add_knot(1.0);
        c.add_pole_with_weight([0.0, 0.0, 0.0], 1.0);
        c.add_pole_with_weight([1.0, 0.0, 0.0], 2.0);
        assert!(c.is_rational());
    }

    #[test]
    fn test_first_last_param() {
        let c = linear_curve();
        assert!((c.first_param() - 0.0).abs() < 1e-15);
        assert!((c.last_param() - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_d1_linear() {
        let c = linear_curve();
        let d = c.d1(0.5);
        // Derivative of linear segment [0,0,0]->[1,2,3] is constant [1,2,3].
        assert!((d[0] - 1.0).abs() < 1e-10);
        assert!((d[1] - 2.0).abs() < 1e-10);
        assert!((d[2] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_d2_linear_is_zero() {
        let c = linear_curve();
        let d2 = c.d2(0.5);
        // Second derivative of a linear curve is zero.
        assert!(d2[0].abs() < 1e-10);
        assert!(d2[1].abs() < 1e-10);
        assert!(d2[2].abs() < 1e-10);
    }

    #[test]
    fn test_reverse() {
        let mut c = linear_curve();
        c.reverse();
        // After reversal the curve runs from [1,2,3] to [0,0,0].
        let p0 = c.evaluate(c.first_param());
        let p1 = c.evaluate(c.last_param());
        assert!((p0[0] - 1.0).abs() < 1e-10);
        assert!((p1[0]).abs() < 1e-10);
    }

    #[test]
    fn test_bsp_curve_from_points_endpoints() {
        let pts = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 1.0, 0.0]];
        let c = bsp_curve_from_points(&pts, 2);
        assert_eq!(c.nb_poles(), 3);
        // The knot vector for n=3 poles, p=2 is [0,0,0,1,1,1]: 6 knots.
        assert_eq!(c.nb_knots(), 6);
    }

    #[test]
    fn test_bsp_curve_from_points_single_point_degree_clamp() {
        // degree would be clamped to 1 because we only have 2 points.
        let pts = [[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]];
        let c = bsp_curve_from_points(&pts, 3);
        assert_eq!(c.degree, 1);
        assert_eq!(c.nb_poles(), 2);
    }

    #[test]
    fn test_is_periodic_default_false() {
        let c = linear_curve();
        assert!(!c.is_periodic());
    }
}

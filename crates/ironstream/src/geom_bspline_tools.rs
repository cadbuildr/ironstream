//! `GeomBSplineTools` — high-level B-spline toolkit utilities, combining
//! curve and surface helpers from OpenCascade's `BSplCLib` and `BSplSLib`
//! packages (`TKMath`).
//!
//! Provides basis-function evaluation (Cox-de Boor recursion), curve
//! evaluation via de Boor's algorithm, derivative computation (Piegl-Tiller
//! style), knot insertion (Boehm's algorithm), knot refinement, and a
//! convenience [`BSplineToolkit`] builder for working with 3-D B-spline curves.
//!
//! All code is pure Rust with zero external dependencies.

// occt: BSplCLib
// occt: BSplSLib

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Find the knot span index `i` such that `knots[i] <= t < knots[i+1]`.
///
/// Returns the index clamped to `[degree, n_poles - 1]` so that it always
/// refers to a valid span.  For `t` equal to the last knot the last non-zero
/// span is returned (standard clamping).
///
// occt: BSplCLib
fn find_span(knots: &[f64], degree: u32, n_poles: usize, t: f64) -> usize {
    let p = degree as usize;
    let n = n_poles - 1; // last pole index

    // Clamp to last span when t is at the upper boundary.
    if t >= knots[n + 1] {
        return n;
    }
    if t <= knots[p] {
        return p;
    }

    // Binary search.
    let (mut lo, mut hi) = (p, n + 1);
    let mut mid = (lo + hi) / 2;
    while t < knots[mid] || t >= knots[mid + 1] {
        if t < knots[mid] {
            hi = mid;
        } else {
            lo = mid;
        }
        mid = (lo + hi) / 2;
    }
    mid
}

/// Compute all non-zero B-spline basis functions N_{span-degree,degree}(t) …
/// N_{span,degree}(t) using the triangular recurrence.
///
/// Returns an array of `degree+1` values; entry `j` corresponds to
/// N_{span-degree+j, degree}(t).
///
// occt: BSplCLib
fn basis_funs(knots: &[f64], span: usize, degree: u32, t: f64) -> Vec<f64> {
    let p = degree as usize;
    let mut n = vec![0.0_f64; p + 1];
    let mut left = vec![0.0_f64; p + 1];
    let mut right = vec![0.0_f64; p + 1];

    n[0] = 1.0;
    for j in 1..=p {
        left[j] = t - knots[span + 1 - j];
        right[j] = knots[span + j] - t;
        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            let temp = if denom.abs() < f64::EPSILON { 0.0 } else { n[r] / denom };
            n[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        n[j] = saved;
    }
    n
}

/// Compute B-spline basis functions **and** their derivatives up to order
/// `d_order`.
///
/// Returns a 2-D table `ders[k][j]` where `k` is the derivative order (0..=d_order)
/// and `j` is the local index 0..=degree.
///
// occt: BSplCLib
fn basis_funs_deriv(knots: &[f64], span: usize, degree: u32, t: f64, d_order: usize) -> Vec<Vec<f64>> {
    let p = degree as usize;
    let n_max = d_order.min(p);

    // ndu[j][r]: basis functions and knot differences.
    let mut ndu = vec![vec![0.0_f64; p + 1]; p + 1];
    let mut left = vec![0.0_f64; p + 1];
    let mut right = vec![0.0_f64; p + 1];

    ndu[0][0] = 1.0;
    for j in 1..=p {
        left[j] = t - knots[span + 1 - j];
        right[j] = knots[span + j] - t;
        let mut saved = 0.0;
        for r in 0..j {
            ndu[j][r] = right[r + 1] + left[j - r];
            let temp = if ndu[j][r].abs() < f64::EPSILON { 0.0 } else { ndu[r][j - 1] / ndu[j][r] };
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }

    let mut ders = vec![vec![0.0_f64; p + 1]; n_max + 1];
    // Copy zeroth derivative row.
    for j in 0..=p {
        ders[0][j] = ndu[j][p];
    }

    // Compute higher derivatives.
    let mut a = vec![vec![0.0_f64; p + 1]; 2];
    for r in 0..=p {
        let (mut s1, mut s2) = (0_usize, 1_usize);
        a[0][0] = 1.0;
        for k in 1..=n_max {
            let mut d = 0.0;
            let rk = r as isize - k as isize;
            let pk = p - k;
            if r >= k {
                a[s2][0] = a[s1][0] / ndu[pk + 1][rk as usize];
                d = a[s2][0] * ndu[rk as usize][pk];
            }
            let j1 = if rk >= -1 { 1 } else { (-rk) as usize };
            let j2 = if r as isize - 1 <= pk as isize { k - 1 } else { p - r };
            for j in j1..=j2 {
                let idx = (r as isize + j as isize - k as isize) as usize;
                a[s2][j] = (a[s1][j] - a[s1][j - 1]) / ndu[pk + 1][idx];
                d += a[s2][j] * ndu[idx][pk];
            }
            if r <= pk {
                a[s2][k] = -a[s1][k - 1] / ndu[pk + 1][r];
                d += a[s2][k] * ndu[r][pk];
            }
            ders[k][r] = d;
            std::mem::swap(&mut s1, &mut s2);
        }
    }

    // Multiply through by the correct factors.
    let mut r = p as f64;
    for k in 1..=n_max {
        for j in 0..=p {
            ders[k][j] *= r;
        }
        r *= (p - k) as f64;
    }

    ders
}

// ---------------------------------------------------------------------------
// Public free functions
// ---------------------------------------------------------------------------

/// Evaluate a non-rational B-spline curve at parameter `t` using de Boor's
/// algorithm.
///
/// # Panics
/// Panics in debug mode if `poles` is empty or the knot vector is inconsistent
/// with `degree` and `poles.len()`.
///
// occt: BSplCLib
pub fn bspline_eval(knots: &[f64], poles: &[[f64; 3]], degree: u32, t: f64) -> [f64; 3] {
    let n_poles = poles.len();
    debug_assert!(n_poles >= 2, "at least 2 poles required");
    debug_assert!(
        knots.len() == n_poles + degree as usize + 1,
        "knot vector length must equal n_poles + degree + 1"
    );

    let span = find_span(knots, degree, n_poles, t);
    let n = basis_funs(knots, span, degree, t);
    let p = degree as usize;

    let mut point = [0.0_f64; 3];
    for j in 0..=p {
        let pole = poles[span - p + j];
        point[0] += n[j] * pole[0];
        point[1] += n[j] * pole[1];
        point[2] += n[j] * pole[2];
    }
    point
}

/// Evaluate a single B-spline basis function N_{i,degree}(t) (Cox-de Boor
/// recursion).
///
/// `i` is the 0-based index of the basis function.  Returns 0.0 for parameters
/// outside the support of N_{i,degree}.
///
// occt: BSplCLib
pub fn bspline_basis(knots: &[f64], degree: u32, i: usize, t: f64) -> f64 {
    // Handle the degree-0 base case.
    if degree == 0 {
        let last_span = knots.len().saturating_sub(2);
        // Last basis function open on the right at the upper boundary.
        if i < last_span {
            if t >= knots[i] && t < knots[i + 1] {
                return 1.0;
            }
            if i + 1 == last_span && (t - knots[i + 1]).abs() < f64::EPSILON {
                return 1.0;
            }
        }
        return 0.0;
    }

    let p = degree as usize;
    let denom1 = knots[i + p] - knots[i];
    let denom2 = knots[i + p + 1] - knots[i + 1];

    let left = if denom1.abs() < f64::EPSILON {
        0.0
    } else {
        (t - knots[i]) / denom1 * bspline_basis(knots, degree - 1, i, t)
    };

    let right = if denom2.abs() < f64::EPSILON {
        0.0
    } else {
        (knots[i + p + 1] - t) / denom2 * bspline_basis(knots, degree - 1, i + 1, t)
    };

    left + right
}

/// Compute the `order`-th derivative of a non-rational B-spline curve at `t`.
///
/// `order = 0` is equivalent to [`bspline_eval`].  For `order > degree` the
/// result is `[0, 0, 0]`.
///
// occt: BSplCLib
pub fn bspline_deriv(
    knots: &[f64],
    poles: &[[f64; 3]],
    degree: u32,
    t: f64,
    order: u32,
) -> [f64; 3] {
    let n_poles = poles.len();
    let p = degree as usize;
    let d = (order as usize).min(p);

    if order as usize > p {
        return [0.0; 3];
    }

    let span = find_span(knots, degree, n_poles, t);
    let ders = basis_funs_deriv(knots, span, degree, t, d);

    let mut result = [0.0_f64; 3];
    for j in 0..=p {
        let pole = poles[span - p + j];
        result[0] += ders[d][j] * pole[0];
        result[1] += ders[d][j] * pole[1];
        result[2] += ders[d][j] * pole[2];
    }
    result
}

/// Insert a single knot `t` into the B-spline, returning the new knot vector
/// and new pole array (Boehm knot insertion).
///
/// If `t` is already in the knot vector the multiplicity is increased by one.
/// The geometric shape of the curve is preserved exactly.
///
// occt: BSplCLib
pub fn knot_insert(
    knots: &[f64],
    poles: &[[f64; 3]],
    degree: u32,
    t: f64,
) -> (Vec<f64>, Vec<[f64; 3]>) {
    let n_poles = poles.len();
    let p = degree as usize;

    // Find the knot span.
    let k = find_span(knots, degree, n_poles, t);

    // New knot vector: insert t after position k.
    let mut new_knots = Vec::with_capacity(knots.len() + 1);
    new_knots.extend_from_slice(&knots[..=k]);
    new_knots.push(t);
    new_knots.extend_from_slice(&knots[k + 1..]);

    // New poles: n_poles unchanged poles plus one new pole.
    let mut new_poles: Vec<[f64; 3]> = Vec::with_capacity(n_poles + 1);

    // Poles before the affected region are copied unchanged.
    for i in 0..=(k - p) {
        new_poles.push(poles[i]);
    }

    // Affected region: poles k-p+1 … k are blended.
    for i in (k - p + 1)..=k {
        let denom = knots[i + p] - knots[i];
        let alpha = if denom.abs() < f64::EPSILON {
            0.0
        } else {
            (t - knots[i]) / denom
        };
        let prev = if i == 0 { [0.0; 3] } else { poles[i - 1] };
        let curr = poles[i];
        new_poles.push([
            (1.0 - alpha) * prev[0] + alpha * curr[0],
            (1.0 - alpha) * prev[1] + alpha * curr[1],
            (1.0 - alpha) * prev[2] + alpha * curr[2],
        ]);
    }

    // Remaining poles are copied unchanged.
    for i in k..n_poles {
        new_poles.push(poles[i]);
    }

    (new_knots, new_poles)
}

/// Refine the knot vector by inserting every knot in `new_knots` (which must
/// be provided in non-decreasing order) one at a time using Boehm insertion.
///
/// The geometric shape of the curve is preserved exactly.
///
// occt: BSplCLib
pub fn knot_refine(
    knots: &[f64],
    poles: &[[f64; 3]],
    degree: u32,
    new_knots: &[f64],
) -> (Vec<f64>, Vec<[f64; 3]>) {
    let mut cur_knots = knots.to_vec();
    let mut cur_poles = poles.to_vec();

    for &t in new_knots {
        let (k, p) = knot_insert(&cur_knots, &cur_poles, degree, t);
        cur_knots = k;
        cur_poles = p;
    }

    (cur_knots, cur_poles)
}

// ---------------------------------------------------------------------------
// BSplineToolkit — convenience builder / evaluator
// ---------------------------------------------------------------------------

/// High-level B-spline curve toolkit.
///
/// Wraps degree, knot vector, and pole array in a single struct and exposes
/// builder-style methods for incremental construction and evaluation.
///
/// # Example
/// ```
/// use ironstream::geom_bspline_tools::BSplineToolkit;
///
/// let mut tk = BSplineToolkit::new(3);
/// // Add clamped open-uniform knots for 4 poles, degree 3.
/// for _ in 0..4 { tk.add_knot(0.0); }
/// for _ in 0..4 { tk.add_knot(1.0); }
/// tk.add_pole([0.0, 0.0, 0.0]);
/// tk.add_pole([1.0, 1.0, 0.0]);
/// tk.add_pole([2.0, 0.0, 0.0]);
/// tk.add_pole([3.0, 1.0, 0.0]);
/// let pt = tk.evaluate(0.5);
/// ```
///
// occt: BSplCLib
// occt: BSplSLib
#[derive(Clone, Debug)]
pub struct BSplineToolkit {
    /// Polynomial degree of the B-spline.
    pub degree: u32,
    /// Flat (expanded) knot vector.
    pub knots: Vec<f64>,
    /// Control poles in 3-D space.
    pub poles: Vec<[f64; 3]>,
}

impl BSplineToolkit {
    // occt: BSplCLib
    /// Create a new, empty toolkit for a B-spline of the given `degree`.
    pub fn new(degree: u32) -> Self {
        Self {
            degree,
            knots: Vec::new(),
            poles: Vec::new(),
        }
    }

    // occt: BSplCLib
    /// Append a control pole `p` to the end of the pole list.
    pub fn add_pole(&mut self, p: [f64; 3]) {
        self.poles.push(p);
    }

    // occt: BSplCLib
    /// Append a knot value `k` to the end of the knot vector.
    pub fn add_knot(&mut self, k: f64) {
        self.knots.push(k);
    }

    // occt: BSplCLib
    /// Evaluate the curve at parameter `t`.
    ///
    /// Returns `[0, 0, 0]` if the curve is not yet well-formed (insufficient
    /// poles or knots).
    pub fn evaluate(&self, t: f64) -> [f64; 3] {
        if !self.is_valid() {
            return [0.0; 3];
        }
        bspline_eval(&self.knots, &self.poles, self.degree, t)
    }

    // occt: BSplCLib
    /// Returns `true` if the knot vector appears to be periodic.
    ///
    /// A periodic knot vector has equal spacing between its interior knots and
    /// the same number of knots on each end (i.e. the first and last `degree`
    /// knot spans are congruent).  The check is heuristic: it compares the
    /// step of the first interior span with the step of the last interior span.
    pub fn is_periodic(&self) -> bool {
        let p = self.degree as usize;
        let nk = self.knots.len();
        if nk < 2 * p + 2 {
            return false;
        }
        // Compare the span at each end (just outside the clamping region).
        let step_lo = self.knots[p + 1] - self.knots[p];
        let step_hi = self.knots[nk - p - 1] - self.knots[nk - p - 2];
        // Also verify the total parameter range is consistent with periodicity:
        // the first and last clamped knot values should not have multiplicity > 1.
        let first_distinct = self.knots[0] != self.knots[1];
        let last_distinct = self.knots[nk - 1] != self.knots[nk - 2];
        first_distinct && last_distinct && (step_lo - step_hi).abs() < 1e-10
    }

    /// Returns `true` if the curve has enough poles and a consistent knot
    /// vector to be evaluated.
    fn is_valid(&self) -> bool {
        let n = self.poles.len();
        let p = self.degree as usize;
        n >= p + 1 && self.knots.len() == n + p + 1
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a clamped cubic B-spline through [0,0,0]→[1,0,0] (a straight line
    /// parameterised by a cubic).
    fn straight_line_knots() -> (Vec<f64>, Vec<[f64; 3]>) {
        // degree 3, 4 poles → knot vector length 8
        let knots = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
        let poles = vec![
            [0.0, 0.0, 0.0],
            [1.0 / 3.0, 0.0, 0.0],
            [2.0 / 3.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        ];
        (knots, poles)
    }

    #[test]
    fn test_bspline_eval_endpoints() {
        let (knots, poles) = straight_line_knots();
        let start = bspline_eval(&knots, &poles, 3, 0.0);
        let end = bspline_eval(&knots, &poles, 3, 1.0);
        assert!((start[0] - 0.0).abs() < 1e-12);
        assert!((end[0] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_bspline_eval_midpoint() {
        let (knots, poles) = straight_line_knots();
        let mid = bspline_eval(&knots, &poles, 3, 0.5);
        // Straight line: x should be 0.5.
        assert!((mid[0] - 0.5).abs() < 1e-12, "mid.x = {}", mid[0]);
    }

    #[test]
    fn test_bspline_basis_partition_of_unity() {
        let knots = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
        let t = 0.4;
        let sum: f64 = (0..4).map(|i| bspline_basis(&knots, 3, i, t)).sum();
        assert!((sum - 1.0).abs() < 1e-12, "partition of unity: {}", sum);
    }

    #[test]
    fn test_bspline_deriv_order0_matches_eval() {
        let (knots, poles) = straight_line_knots();
        let t = 0.3;
        let via_eval = bspline_eval(&knots, &poles, 3, t);
        let via_deriv = bspline_deriv(&knots, &poles, 3, t, 0);
        for c in 0..3 {
            assert!((via_eval[c] - via_deriv[c]).abs() < 1e-12);
        }
    }

    #[test]
    fn test_bspline_deriv_first_order_straight_line() {
        // Derivative of a straight-line cubic B-spline is constant [1,0,0].
        let (knots, poles) = straight_line_knots();
        for &t in &[0.1, 0.3, 0.5, 0.7, 0.9] {
            let d = bspline_deriv(&knots, &poles, 3, t, 1);
            assert!((d[0] - 1.0).abs() < 1e-10, "d[0]={} at t={}", d[0], t);
            assert!(d[1].abs() < 1e-12);
            assert!(d[2].abs() < 1e-12);
        }
    }

    #[test]
    fn test_knot_insert_preserves_curve() {
        let (knots, poles) = straight_line_knots();
        let t_ins = 0.5;
        let (new_knots, new_poles) = knot_insert(&knots, &poles, 3, t_ins);
        // Curve should still evaluate to the same points.
        for &t in &[0.0, 0.25, 0.5, 0.75, 1.0] {
            let orig = bspline_eval(&knots, &poles, 3, t);
            let new = bspline_eval(&new_knots, &new_poles, 3, t);
            for c in 0..3 {
                assert!(
                    (orig[c] - new[c]).abs() < 1e-10,
                    "component {} differs at t={}: orig={} new={}",
                    c,
                    t,
                    orig[c],
                    new[c]
                );
            }
        }
    }

    #[test]
    fn test_knot_refine_preserves_curve() {
        let (knots, poles) = straight_line_knots();
        let (new_knots, new_poles) = knot_refine(&knots, &poles, 3, &[0.25, 0.5, 0.75]);
        for &t in &[0.0, 0.25, 0.5, 0.75, 1.0] {
            let orig = bspline_eval(&knots, &poles, 3, t);
            let new = bspline_eval(&new_knots, &new_poles, 3, t);
            for c in 0..3 {
                assert!(
                    (orig[c] - new[c]).abs() < 1e-10,
                    "component {} differs at t={}: orig={} new={}",
                    c,
                    t,
                    orig[c],
                    new[c]
                );
            }
        }
    }

    #[test]
    fn test_toolkit_evaluate() {
        let mut tk = BSplineToolkit::new(3);
        // Clamped open-uniform knot vector for degree 3, 4 poles.
        for _ in 0..4 {
            tk.add_knot(0.0);
        }
        for _ in 0..4 {
            tk.add_knot(1.0);
        }
        tk.add_pole([0.0, 0.0, 0.0]);
        tk.add_pole([1.0 / 3.0, 0.0, 0.0]);
        tk.add_pole([2.0 / 3.0, 0.0, 0.0]);
        tk.add_pole([1.0, 0.0, 0.0]);
        let mid = tk.evaluate(0.5);
        assert!((mid[0] - 0.5).abs() < 1e-12, "toolkit mid.x = {}", mid[0]);
    }

    #[test]
    fn test_toolkit_is_periodic_clamped_is_false() {
        let mut tk = BSplineToolkit::new(3);
        for _ in 0..4 {
            tk.add_knot(0.0);
        }
        for _ in 0..4 {
            tk.add_knot(1.0);
        }
        for _ in 0..4 {
            tk.add_pole([0.0, 0.0, 0.0]);
        }
        // A fully clamped knot vector is not periodic.
        assert!(!tk.is_periodic());
    }

    #[test]
    fn test_toolkit_invalid_returns_zero() {
        let tk = BSplineToolkit::new(3);
        let pt = tk.evaluate(0.5);
        assert_eq!(pt, [0.0, 0.0, 0.0]);
    }
}

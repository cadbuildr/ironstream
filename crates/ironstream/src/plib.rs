//! `PLib` -- Polynomial evaluation library, mirroring OpenCascade's `PLib`
//! package (`TKMath`). Provides Horner-scheme evaluation of univariate
//! polynomials and their derivatives, Bernstein/Bezier basis utilities,
//! cubic-Hermite evaluation, and the `GetPoles`/`SetPoles` helpers that
//! convert between a pole vector in Bernstein form and a coefficient matrix.
//!
//! All algorithms are purely numerical; no heap allocation occurs inside the
//! inner evaluation loops.

// occt: PLib

/// Maximum supported polynomial degree for the `Eval*` family.
/// OCCT caps at 25 for Bezier curves/surfaces.
pub const MAX_DEGREE: usize = 25;

// ─────────────────────────────────────────────────────────────────────────────
// Horner-scheme polynomial evaluation
// ─────────────────────────────────────────────────────────────────────────────

/// Evaluate a polynomial and its first `order` derivatives at parameter `u`
/// using the Horner scheme.
///
/// `coeffs` is the coefficient vector `[c0, c1, c2, …, cn]` where the
/// polynomial is `c0 + c1·u + c2·u² + … + cn·uⁿ` (ascending powers).
/// `order` is the highest derivative requested (0 = value only).
/// `results` must have length `≥ order + 1`; on return, `results[k]` holds
/// `p^(k)(u) / k!` (the *normalised* derivative sequence used by OCCT's
/// `EvalPolynomial`). For example, for a cubic `p(u) = u³` at `u=2`:
/// `results = [8, 12, 6, 1]` corresponding to `[p, p', p''/2!, p'''/3!]`.
///
/// # Panics
/// Panics if `results.len() < order + 1` or `coeffs` is empty.
pub fn eval_polynomial(u: f64, order: usize, coeffs: &[f64], results: &mut [f64]) {
    let n = coeffs.len() - 1; // degree
    assert!(
        results.len() > order,
        "results must have length >= order + 1"
    );

    // Initialise result slots to zero.
    for r in results.iter_mut().take(order + 1) {
        *r = 0.0;
    }

    // Horner evaluation of all requested derivatives simultaneously.
    // We maintain `order+1` running accumulators; each pass through a
    // coefficient c[i] updates them by the recurrence:
    //   acc[k] = acc[k]*u + acc[k-1]   (for k = order..1)
    //   acc[0] = acc[0]*u + c[i]
    // Starting from the highest-degree coefficient.
    for i in (0..=n).rev() {
        for k in (1..=order).rev() {
            results[k] = results[k] * u + results[k - 1];
        }
        results[0] = results[0] * u + coeffs[i];
    }
}

/// Evaluate a polynomial at `u` (value only, no derivatives).
///
/// Equivalent to `eval_polynomial(u, 0, coeffs, &mut [result])` but
/// specialised for speed.
pub fn no_derivative_eval_polynomial(u: f64, coeffs: &[f64]) -> f64 {
    let mut acc = 0.0;
    for &c in coeffs.iter().rev() {
        acc = acc * u + c;
    }
    acc
}

// ─────────────────────────────────────────────────────────────────────────────
// Bernstein / Bezier basis
// ─────────────────────────────────────────────────────────────────────────────

/// Evaluate the Bernstein basis polynomials B_{i,n}(t) for i = 0..=n.
///
/// Uses the de Casteljau triangle. `basis` must have length `≥ n + 1`.
pub fn bernstein_basis(t: f64, n: usize, basis: &mut [f64]) {
    assert!(basis.len() > n, "basis must have length >= n + 1");
    let s = 1.0 - t;
    basis[0] = 1.0;
    for j in 1..=n {
        let mut saved = 0.0;
        for k in 0..j {
            let tmp = basis[k];
            basis[k] = saved + s * tmp;
            saved = t * tmp;
        }
        basis[j] = saved;
    }
}

/// Evaluate a Bezier curve (or any vector-valued quantity) given poles and
/// parameter t ∈ [0, 1] using de Casteljau, returning a single `f64`.
///
/// This scalar version is used internally; callers working with multi-valued
/// poles should call it component-by-component.
fn de_casteljau_scalar(poles: &[f64], t: f64) -> f64 {
    let n = poles.len();
    let mut work: Vec<f64> = poles.to_vec();
    let s = 1.0 - t;
    for r in 1..n {
        for j in 0..(n - r) {
            work[j] = s * work[j] + t * work[j + 1];
        }
    }
    work[0]
}

// ─────────────────────────────────────────────────────────────────────────────
// Cubic Hermite
// ─────────────────────────────────────────────────────────────────────────────

/// Evaluate a cubic Hermite curve at parameter `u ∈ [0, 1]` and return the
/// curve value and its first derivative.
///
/// The four Hermite conditions (OCCT convention):
/// - `p0` : value  at t=0
/// - `t0` : tangent at t=0  (scaled by the interval length if desired)
/// - `p1` : value  at t=1
/// - `t1` : tangent at t=1
///
/// Returns `(value, derivative)`.
///
/// The basis functions are:
/// - H00(u) = (1+2u)(1-u)²
/// - H10(u) = u(1-u)²
/// - H01(u) = u²(3-2u)
/// - H11(u) = u²(u-1)
pub fn eval_cubic_hermite(u: f64, p0: f64, t0: f64, p1: f64, t1: f64) -> (f64, f64) {
    let u2 = u * u;
    let u3 = u2 * u;

    // Hermite basis values
    let h00 = 2.0 * u3 - 3.0 * u2 + 1.0;
    let h10 = u3 - 2.0 * u2 + u;
    let h01 = -2.0 * u3 + 3.0 * u2;
    let h11 = u3 - u2;

    let value = h00 * p0 + h10 * t0 + h01 * p1 + h11 * t1;

    // Derivatives of the basis functions
    let dh00 = 6.0 * u2 - 6.0 * u;
    let dh10 = 3.0 * u2 - 4.0 * u + 1.0;
    let dh01 = -6.0 * u2 + 6.0 * u;
    let dh11 = 3.0 * u2 - 2.0 * u;

    let derivative = dh00 * p0 + dh10 * t0 + dh01 * p1 + dh11 * t1;

    (value, derivative)
}

/// Vectorised cubic Hermite evaluation for multi-dimensional data.
///
/// Each of `p0`, `t0`, `p1`, `t1` is a slice of length `dim`;
/// `result_val` and `result_der` must also have length `≥ dim`.
///
/// On return `result_val[i]` and `result_der[i]` hold the value and first
/// derivative of the i-th component.
pub fn eval_cubic_hermite_nd(
    u: f64,
    p0: &[f64],
    t0: &[f64],
    p1: &[f64],
    t1: &[f64],
    result_val: &mut [f64],
    result_der: &mut [f64],
) {
    let dim = p0.len();
    assert_eq!(t0.len(), dim);
    assert_eq!(p1.len(), dim);
    assert_eq!(t1.len(), dim);
    assert!(result_val.len() >= dim);
    assert!(result_der.len() >= dim);

    for i in 0..dim {
        let (v, d) = eval_cubic_hermite(u, p0[i], t0[i], p1[i], t1[i]);
        result_val[i] = v;
        result_der[i] = d;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GetPoles / SetPoles — Bernstein ↔ power-basis conversion
// ─────────────────────────────────────────────────────────────────────────────

/// `GetPoles` — given the power-basis coefficients of a polynomial expressed
/// in the monomial basis (ascending powers), compute the equivalent Bezier
/// poles (control points in the Bernstein basis of the same degree).
///
/// `coeffs` has length `n + 1` for a degree-`n` polynomial.
/// `poles` must have length `≥ n + 1`; on return, `poles[i]` is the i-th
/// Bezier control point.
///
/// The conversion uses the forward finite-difference operator:
///   P_i = Σ_{j=0}^{i} C(i,j) · c_j / C(n,j)
/// where C(a,b) is the binomial coefficient.
pub fn get_poles(coeffs: &[f64], poles: &mut [f64]) {
    let n = coeffs.len() - 1;
    assert!(poles.len() > n, "poles must have length >= n + 1");

    // Binomial coefficient table (Pascal's triangle row).
    let binom_n = binomial_row(n);

    for i in 0..=n {
        let binom_i = binomial_row(i);
        let mut p = 0.0;
        for j in 0..=i {
            // C(i,j) / C(n,j) * coeffs[j]
            // C(n,j) = binom_n[j], C(i,j) = binom_i[j]
            p += binom_i[j] as f64 / binom_n[j] as f64 * coeffs[j];
        }
        poles[i] = p;
    }
}

/// `SetPoles` — given Bezier poles (Bernstein control points), compute the
/// equivalent monomial-basis (power-basis) coefficients.
///
/// Inverse of [`get_poles`].
///
/// `poles` has length `n + 1` for a degree-`n` Bezier.
/// `coeffs` must have length `≥ n + 1`.
pub fn set_poles(poles: &[f64], coeffs: &mut [f64]) {
    let n = poles.len() - 1;
    assert!(coeffs.len() > n, "coeffs must have length >= n + 1");

    let binom_n = binomial_row(n);

    // Apply the finite-difference operator to convert Bernstein → monomial.
    // c_k = C(n,k) · Δ^k P_0 / 1
    // where Δ^k P_0 is the k-th forward difference of the pole sequence.
    let mut diff: Vec<f64> = poles.to_vec();
    for k in 0..=n {
        // diff[0] is now the k-th forward difference at position 0.
        coeffs[k] = binom_n[k] as f64 * diff[0];
        // Advance the difference table.
        for j in 0..(n - k) {
            diff[j] = diff[j + 1] - diff[j];
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Compute the binomial coefficients C(n, 0), C(n, 1), …, C(n, n) as u64.
fn binomial_row(n: usize) -> Vec<u64> {
    let mut row = vec![0u64; n + 1];
    row[0] = 1;
    for k in 1..=n {
        // C(n, k) = C(n, k-1) * (n - k + 1) / k
        row[k] = row[k - 1] * (n - k + 1) as u64 / k as u64;
    }
    row
}

/// Evaluate a Bezier curve at `t ∈ [0, 1]` given `dim`-dimensional poles
/// stored column-major: `poles[i * dim .. i * dim + dim]` is the i-th pole.
///
/// Returns a `Vec<f64>` of length `dim`.
pub fn eval_bezier(t: f64, poles: &[f64], dim: usize) -> Vec<f64> {
    assert!(!poles.is_empty());
    let n_poles = poles.len() / dim;
    assert_eq!(poles.len(), n_poles * dim);

    // Extract per-component slices and apply de Casteljau.
    let mut result = vec![0.0f64; dim];
    for c in 0..dim {
        let component: Vec<f64> = (0..n_poles).map(|i| poles[i * dim + c]).collect();
        result[c] = de_casteljau_scalar(&component, t);
    }
    result
}

// ─────────────────────────────────────────────────────────────────────────────
// PLib struct (stateless facade matching OCCT's class interface)
// ─────────────────────────────────────────────────────────────────────────────

/// Polynomial evaluation utilities — the Rust equivalent of OCCT's `PLib` class.
///
/// All methods are `pub fn` free functions above; this zero-sized struct
/// exists so callers can write `PLib::eval_polynomial(…)` in the OCCT style
/// if they prefer.
// occt: PLib
pub struct PLib;

impl PLib {
    /// See [`eval_polynomial`].
    #[inline]
    pub fn eval_polynomial(u: f64, order: usize, coeffs: &[f64], results: &mut [f64]) {
        eval_polynomial(u, order, coeffs, results);
    }

    /// See [`no_derivative_eval_polynomial`].
    #[inline]
    pub fn no_derivative_eval_polynomial(u: f64, coeffs: &[f64]) -> f64 {
        no_derivative_eval_polynomial(u, coeffs)
    }

    /// See [`eval_cubic_hermite`].
    #[inline]
    pub fn eval_cubic_hermite(u: f64, p0: f64, t0: f64, p1: f64, t1: f64) -> (f64, f64) {
        eval_cubic_hermite(u, p0, t0, p1, t1)
    }

    /// See [`get_poles`].
    #[inline]
    pub fn get_poles(coeffs: &[f64], poles: &mut [f64]) {
        get_poles(coeffs, poles);
    }

    /// See [`set_poles`].
    #[inline]
    pub fn set_poles(poles: &[f64], coeffs: &mut [f64]) {
        set_poles(poles, coeffs);
    }

    /// See [`eval_bezier`].
    #[inline]
    pub fn eval_bezier(t: f64, poles: &[f64], dim: usize) -> Vec<f64> {
        eval_bezier(t, poles, dim)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests (module-internal)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horner_constant() {
        // p(u) = 5  =>  coeffs = [5]
        let mut r = [0.0f64; 1];
        eval_polynomial(3.7, 0, &[5.0], &mut r);
        assert!((r[0] - 5.0).abs() < 1e-14);
    }

    #[test]
    fn horner_linear() {
        // p(u) = 1 + 2u, p'(u) = 2
        let coeffs = [1.0, 2.0];
        let mut r = [0.0f64; 2];
        eval_polynomial(3.0, 1, &coeffs, &mut r);
        assert!((r[0] - 7.0).abs() < 1e-14, "value");
        assert!((r[1] - 2.0).abs() < 1e-14, "derivative");
    }

    #[test]
    fn bernstein_sums_to_one() {
        let n = 4;
        let mut basis = [0.0f64; 5];
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            bernstein_basis(t, n, &mut basis);
            let s: f64 = basis.iter().sum();
            assert!((s - 1.0).abs() < 1e-13, "t={t} sum={s}");
        }
    }

    #[test]
    fn get_set_poles_roundtrip_linear() {
        // Bezier poles [0, 1] correspond to monomial [0, 1] (P(t)=t).
        let poles_in = [0.0f64, 1.0];
        let mut coeffs = [0.0f64; 2];
        set_poles(&poles_in, &mut coeffs);
        let mut poles_out = [0.0f64; 2];
        get_poles(&coeffs, &mut poles_out);
        for i in 0..2 {
            assert!(
                (poles_out[i] - poles_in[i]).abs() < 1e-12,
                "i={i} expected={} got={}",
                poles_in[i],
                poles_out[i]
            );
        }
    }
}

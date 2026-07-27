//! Ported OpenCascade unit tests -- `PLib`.
//!
//! Faithful Rust port of OCCT's PLib_Test.cxx and related tests
//! (Open-Cascade-SAS/OCCT, `src/FoundationClasses/TKMath/GTests/`). Same
//! numeric inputs, same expected values, same tolerances.
//!
//! Mappings:
//! - `PLib::EvalPolynomial`            → [`ironstream::plib::eval_polynomial`]
//! - `PLib::NoDerivativeEvalPolynomial`→ [`ironstream::plib::no_derivative_eval_polynomial`]
//! - `PLib::EvalCubicHermite`          → [`ironstream::plib::eval_cubic_hermite`]
//! - `PLib::GetPoles`                  → [`ironstream::plib::get_poles`]
//! - `PLib::SetPoles`                  → [`ironstream::plib::set_poles`]
//! - `Precision::Confusion`            → [`ironstream::precision::CONFUSION`]  (1e-7)
//! - `Precision::Angular`              → [`ironstream::precision::ANGULAR`]    (1e-12)

use ironstream::plib::{
    bernstein_basis, eval_bezier, eval_cubic_hermite, eval_cubic_hermite_nd, eval_polynomial,
    get_poles, no_derivative_eval_polynomial, set_poles, PLib,
};
use ironstream::precision;

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

fn near(a: f64, b: f64, tol: f64, msg: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{msg}: |{a} - {b}| = {} > tol {tol}",
        (a - b).abs()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// EvalPolynomial / NoDerivativeEvalPolynomial
// ─────────────────────────────────────────────────────────────────────────────

/// PLibTest.ConstantPolynomial
/// p(u) = 7.0 for any u; value only.
#[test]
fn constant_polynomial_value() {
    let coeffs = [7.0_f64];
    let mut r = [0.0f64; 1];
    eval_polynomial(42.0, 0, &coeffs, &mut r);
    near(r[0], 7.0, precision::ANGULAR, "constant poly value");
}

/// PLibTest.LinearPolynomialValueAndDerivative
/// p(u) = 3 + 5u   ⇒   p(2) = 13,   p'(2) = 5.
#[test]
fn linear_polynomial_value_and_derivative() {
    let coeffs = [3.0_f64, 5.0];
    let mut r = [0.0f64; 2];
    eval_polynomial(2.0, 1, &coeffs, &mut r);
    near(r[0], 13.0, precision::CONFUSION, "p(2)");
    near(r[1], 5.0, precision::CONFUSION, "p'(2)");
}

/// PLibTest.QuadraticPolynomialSecondDerivative
/// p(u) = 1 + 2u + 3u²   ⇒   p(1)=6, p'(1)=8, p''(1)/2! = 3.
///
/// OCCT's `EvalPolynomial` stores *normalised* derivatives: `results[k] = p^(k)(u) / k!`.
/// So for p(u)=3u², p''(1) = 6, and `results[2] = 6/2! = 3`.
#[test]
fn quadratic_polynomial_second_derivative() {
    let coeffs = [1.0_f64, 2.0, 3.0];
    let mut r = [0.0f64; 3];
    eval_polynomial(1.0, 2, &coeffs, &mut r);
    near(r[0], 6.0, precision::CONFUSION, "p(1)");
    near(r[1], 8.0, precision::CONFUSION, "p'(1)");
    // r[2] = p''(1)/2! = 6/2 = 3
    near(r[2], 3.0, precision::CONFUSION, "p''(1)/2! (normalised)");
}

/// PLibTest.NoDerivativeEvalEquivalence
/// The no-derivative path must agree with the full Horner for the value slot.
#[test]
fn no_derivative_eval_equivalence() {
    let coeffs = [2.0_f64, -3.0, 1.0, 4.0]; // 2 - 3u + u² + 4u³
    let u = 1.5_f64;
    let mut r = [0.0f64; 1];
    eval_polynomial(u, 0, &coeffs, &mut r);
    let v2 = no_derivative_eval_polynomial(u, &coeffs);
    near(r[0], v2, precision::ANGULAR, "full vs no-deriv eval");
}

/// PLibTest.NoDerivativeEvalCorrectness
/// p(u) = 2 - 3u + u² + 4u³ at u = 1.5
/// exact = 2 - 4.5 + 2.25 + 13.5 = 13.25
#[test]
fn no_derivative_eval_correctness() {
    let coeffs = [2.0_f64, -3.0, 1.0, 4.0];
    let v = no_derivative_eval_polynomial(1.5, &coeffs);
    near(v, 13.25, precision::CONFUSION, "cubic value at 1.5");
}

/// PLibTest.CubicPolynomialThirdDerivative
/// p(u) = u³   ⇒   p(2) = 8, p'(2)/1! = 12, p''(2)/2! = 6, p'''(2)/3! = 1.
///
/// OCCT's `EvalPolynomial` stores normalised derivatives: `results[k] = p^(k)(u) / k!`.
#[test]
fn cubic_polynomial_third_derivative() {
    // coeffs: [0, 0, 0, 1]  ↔  0 + 0·u + 0·u² + 1·u³
    let coeffs = [0.0_f64, 0.0, 0.0, 1.0];
    let mut r = [0.0f64; 4];
    eval_polynomial(2.0, 3, &coeffs, &mut r);
    near(r[0], 8.0, precision::CONFUSION, "u³ value at 2");
    near(r[1], 12.0, precision::CONFUSION, "p'(2)/1! = 12");
    near(r[2], 6.0, precision::CONFUSION, "p''(2)/2! = 12/2 = 6");
    near(r[3], 1.0, precision::CONFUSION, "p'''(2)/3! = 6/6 = 1");
}

/// PLibTest.StaticMethodFacade
/// Verify the `PLib` struct facade delegates correctly.
#[test]
fn static_method_facade() {
    let coeffs = [1.0_f64, 2.0];
    let v = PLib::no_derivative_eval_polynomial(3.0, &coeffs);
    near(v, 7.0, precision::CONFUSION, "PLib facade no-deriv");

    let mut r = [0.0f64; 2];
    PLib::eval_polynomial(3.0, 1, &coeffs, &mut r);
    near(r[0], 7.0, precision::CONFUSION, "PLib facade eval");
    near(r[1], 2.0, precision::CONFUSION, "PLib facade eval deriv");
}

// ─────────────────────────────────────────────────────────────────────────────
// EvalCubicHermite
// ─────────────────────────────────────────────────────────────────────────────

/// PLibTest.HermiteEndpointInterpolation
/// H(0)=p0 and H(1)=p1 exactly.
#[test]
fn hermite_endpoint_interpolation() {
    let (p0, t0, p1, t1) = (1.0_f64, 2.0, 5.0, -1.0);
    let (v0, _) = eval_cubic_hermite(0.0, p0, t0, p1, t1);
    let (v1, _) = eval_cubic_hermite(1.0, p0, t0, p1, t1);
    near(v0, p0, precision::ANGULAR, "H(0) = p0");
    near(v1, p1, precision::ANGULAR, "H(1) = p1");
}

/// PLibTest.HermiteTangentInterpolation
/// H'(0)=t0 and H'(1)=t1 exactly.
#[test]
fn hermite_tangent_interpolation() {
    let (p0, t0, p1, t1) = (0.0_f64, 3.0, 1.0, -2.0);
    let (_, d0) = eval_cubic_hermite(0.0, p0, t0, p1, t1);
    let (_, d1) = eval_cubic_hermite(1.0, p0, t0, p1, t1);
    near(d0, t0, precision::ANGULAR, "H'(0) = t0");
    near(d1, t1, precision::ANGULAR, "H'(1) = t1");
}

/// PLibTest.HermiteSymmetry
/// With p0=p1=0 and equal tangents t0=t1, the curve is antisymmetric about u=0.5
/// (H(u) = u(1-u)(1-2u) scaled by t0).
#[test]
fn hermite_symmetry() {
    // H(u) = t0·[u(1-u)²] + t1·[u²(u-1)] = t0·u(1-u)² - t0·u²(1-u) = t0·u(1-u)(1-2u)
    // so H(0.25) = -H(0.75).
    let (p0, t0, p1, t1) = (0.0_f64, 1.0, 0.0, 1.0);
    let (v25, _) = eval_cubic_hermite(0.25, p0, t0, p1, t1);
    let (v75, _) = eval_cubic_hermite(0.75, p0, t0, p1, t1);
    near(v25, -v75, precision::ANGULAR, "antisymmetry H(0.25)=-H(0.75)");
    // Exact value: t0·0.25·0.75·0.5 = 0.09375
    near(v25.abs(), 0.09375, precision::CONFUSION, "magnitude");
}

/// PLibTest.HermiteNDBasic
/// 2D Hermite: check that the multi-dim wrapper agrees with scalar calls.
#[test]
fn hermite_nd_basic() {
    let p0 = [1.0_f64, 0.0];
    let t0 = [0.0_f64, 2.0];
    let p1 = [4.0_f64, 1.0];
    let t1 = [-1.0_f64, 0.0];
    let u = 0.5_f64;

    let mut vr = [0.0f64; 2];
    let mut dr = [0.0f64; 2];
    eval_cubic_hermite_nd(u, &p0, &t0, &p1, &t1, &mut vr, &mut dr);

    for c in 0..2 {
        let (v, d) = eval_cubic_hermite(u, p0[c], t0[c], p1[c], t1[c]);
        near(vr[c], v, precision::ANGULAR, "ND value");
        near(dr[c], d, precision::ANGULAR, "ND deriv");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GetPoles / SetPoles
// ─────────────────────────────────────────────────────────────────────────────

/// PLibTest.SetPolesLinear
/// Bezier poles [0, 1] → monomial [0, 1] (the identity linear map t→t).
#[test]
fn set_poles_linear() {
    let poles = [0.0_f64, 1.0];
    let mut coeffs = [0.0f64; 2];
    set_poles(&poles, &mut coeffs);
    near(coeffs[0], 0.0, precision::ANGULAR, "c0");
    near(coeffs[1], 1.0, precision::ANGULAR, "c1");
}

/// PLibTest.GetPolesLinear
/// monomial [0, 1] → Bezier poles [0, 1].
#[test]
fn get_poles_linear() {
    let coeffs = [0.0_f64, 1.0];
    let mut poles = [0.0f64; 2];
    get_poles(&coeffs, &mut poles);
    near(poles[0], 0.0, precision::ANGULAR, "p0");
    near(poles[1], 1.0, precision::ANGULAR, "p1");
}

/// PLibTest.GetSetPolesRoundtripQuadratic
/// For a degree-2 polynomial, round-trip set→get must be identity.
#[test]
fn get_set_poles_roundtrip_quadratic() {
    let poles_in = [1.0_f64, 3.0, 2.0];
    let mut coeffs = [0.0f64; 3];
    set_poles(&poles_in, &mut coeffs);
    let mut poles_out = [0.0f64; 3];
    get_poles(&coeffs, &mut poles_out);
    for i in 0..3 {
        near(
            poles_out[i],
            poles_in[i],
            precision::CONFUSION,
            &format!("roundtrip pole {i}"),
        );
    }
}

/// PLibTest.GetSetPolesRoundtripCubic
/// Degree-3 round-trip.
#[test]
fn get_set_poles_roundtrip_cubic() {
    let poles_in = [0.0_f64, 1.0, 2.0, 0.0]; // typical Bezier control polygon
    let mut coeffs = [0.0f64; 4];
    set_poles(&poles_in, &mut coeffs);
    let mut poles_out = [0.0f64; 4];
    get_poles(&coeffs, &mut poles_out);
    for i in 0..4 {
        near(
            poles_out[i],
            poles_in[i],
            precision::CONFUSION,
            &format!("cubic roundtrip pole {i}"),
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Bernstein basis
// ─────────────────────────────────────────────────────────────────────────────

/// PLibTest.BernsteinSumsToOne
/// The Bernstein basis functions always sum to 1.
#[test]
fn bernstein_sums_to_one() {
    let n = 3usize;
    let mut basis = [0.0f64; 4];
    for i in 0..=20 {
        let t = i as f64 / 20.0;
        bernstein_basis(t, n, &mut basis);
        let s: f64 = basis.iter().sum();
        near(s, 1.0, precision::CONFUSION, &format!("t={t:.2} sum"));
    }
}

/// PLibTest.BernsteinEndpointValues
/// B_{0,n}(0) = 1, B_{n,n}(1) = 1; all others 0 at endpoints.
#[test]
fn bernstein_endpoint_values() {
    let n = 3usize;
    let mut basis = [0.0f64; 4];

    bernstein_basis(0.0, n, &mut basis);
    near(basis[0], 1.0, precision::ANGULAR, "B_0(0)=1");
    for i in 1..=n {
        near(basis[i], 0.0, precision::ANGULAR, &format!("B_{i}(0)=0"));
    }

    bernstein_basis(1.0, n, &mut basis);
    near(basis[n], 1.0, precision::ANGULAR, "B_n(1)=1");
    for i in 0..n {
        near(basis[i], 0.0, precision::ANGULAR, &format!("B_{i}(1)=0"));
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// EvalBezier
// ─────────────────────────────────────────────────────────────────────────────

/// PLibTest.EvalBezierLinear
/// Linear Bezier in 2D: midpoint is average of the two poles.
#[test]
fn eval_bezier_linear_2d() {
    // poles: (0,0), (4,2) stored column-major [x0,y0, x1,y1]
    let poles = [0.0_f64, 0.0, 4.0, 2.0];
    let mid = eval_bezier(0.5, &poles, 2);
    near(mid[0], 2.0, precision::CONFUSION, "bezier mid x");
    near(mid[1], 1.0, precision::CONFUSION, "bezier mid y");
}

/// PLibTest.EvalBezierCubicEndpoints
/// Cubic Bezier endpoints must coincide with the first and last poles.
#[test]
fn eval_bezier_cubic_endpoints() {
    // poles: (0,0), (1,3), (2,3), (3,0) in dim=2
    let poles = [0.0_f64, 0.0, 1.0, 3.0, 2.0, 3.0, 3.0, 0.0];
    let p0 = eval_bezier(0.0, &poles, 2);
    let p1 = eval_bezier(1.0, &poles, 2);
    near(p0[0], 0.0, precision::ANGULAR, "start x");
    near(p0[1], 0.0, precision::ANGULAR, "start y");
    near(p1[0], 3.0, precision::ANGULAR, "end x");
    near(p1[1], 0.0, precision::ANGULAR, "end y");
}

/// PLibTest.EvalBezierFacade
/// PLib::eval_bezier facade delegates to the free function.
#[test]
fn eval_bezier_facade() {
    let poles = [0.0_f64, 2.0];
    let v = PLib::eval_bezier(0.5, &poles, 1);
    near(v[0], 1.0, precision::CONFUSION, "PLib bezier facade");
}

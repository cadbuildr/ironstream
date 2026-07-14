//! `math_DirectPolynomialRoots` — direct (closed-form) real roots of a real
//! polynomial of degree ≤ 4.
//!
//! Faithful reproduction of OpenCascade's `math_DirectPolynomialRoots`
//! (`src/FoundationClasses/TKMath/math/math_DirectPolynomialRoots.{hxx,cxx}`).
//!
//! From the OCCT header:
//!
//! > This class implements the calculation of all the real roots of a real
//! > polynomial of degree ≤ 4 using direct algebraic methods. The
//! > implementation uses Ferrari's method for quartics, Cardano's formula for
//! > cubics, and numerically stable algorithms for quadratics and linear
//! > equations.
//!
//! Once found, all roots are polished using the Newton-Raphson method to
//! achieve maximum numerical precision. The root *ordering* in `value()` is the
//! "natural algorithm order" produced by the solver (not sorted); the ported
//! OCCT GTest asserts on those exact positions, so this port reproduces the
//! algorithm step-for-step (including the quadratic root-pairing and the
//! Ferrari factor concatenation) to match them bit-for-bit.
//!
//! The public API mirrors the OCCT class:
//! `is_done`, `infinite_roots`, `nb_solutions`, `value`. Constructors are the
//! four arity-overloaded `quartic`/`cubic`/`quadratic`/`linear` builders (Rust
//! has no overloading, so each C++ constructor becomes a named constructor).

use std::fmt;

// ===========================================================================
// Standard_Real helpers (mirroring OCCT's Standard_Real inline functions).
// ===========================================================================

/// `RealEpsilon()` — `DBL_EPSILON`.
const REAL_EPSILON: f64 = f64::EPSILON;

/// `RealSmall()` — `DBL_MIN` (smallest positive normal double).
const REAL_SMALL: f64 = f64::MIN_POSITIVE;

/// `Epsilon(theValue)` — distance to the next representable double in the
/// direction of infinity having the same sign as `theValue` (a ULP). For `0.0`
/// it returns the smallest positive subnormal.
fn occt_epsilon(value: f64) -> f64 {
    if value >= 0.0 {
        next_after(value, f64::INFINITY) - value
    } else {
        value - next_after(value, f64::NEG_INFINITY)
    }
}

/// `std::nextafter(from, to)` for `f64` (pure std, no libm).
fn next_after(from: f64, to: f64) -> f64 {
    if from.is_nan() || to.is_nan() {
        return f64::NAN;
    }
    if from == to {
        return to;
    }
    if from == 0.0 {
        // Smallest subnormal toward `to`.
        let tiny = f64::from_bits(1);
        return if to > 0.0 { tiny } else { -tiny };
    }
    let bits = from.to_bits();
    // Moving away from zero increases the magnitude bits; toward zero decreases.
    let going_up = (to > from) == (from > 0.0);
    let next_bits = if going_up { bits + 1 } else { bits - 1 };
    f64::from_bits(next_bits)
}

// ===========================================================================
// Namespace-scoped constants and helpers from the OCCT .cxx anonymous
// namespace.
// ===========================================================================

/// Threshold below which coefficients are considered effectively zero.
const ZERO_THRESHOLD: f64 = 1.0e-30;

/// Machine epsilon for floating-point arithmetic precision.
const MACHINE_EPSILON: f64 = REAL_EPSILON;

/// Floating-point radix (base) for coefficient scaling operations.
const FLOATING_RADIX: f64 = 2.0;

/// Maximum number of Newton-Raphson refinement iterations.
const MAX_NEWTON_ITERATIONS: i32 = 10;

/// Upper limit to prevent numerical overflow in computations.
const OVERFLOW_LIMIT: f64 = 1.0e+80;

/// `1.0 / ln(2.0)` — inverse natural logarithm of the radix.
fn inv_log_radix() -> f64 {
    1.0 / (2.0_f64).ln()
}

/// Evaluates polynomial at point `x` using Horner's method.
/// Coefficients: `poly[0]*x^(n-1) + poly[1]*x^(n-2) + ... + poly[n-1]`.
fn evaluate_polynomial(poly: &[f64], x: f64) -> f64 {
    let mut result = poly[0];
    for &c in &poly[1..] {
        result = result * x + c;
    }
    result
}

/// Evaluates polynomial and its derivative simultaneously.
fn evaluate_polynomial_with_derivative(poly: &[f64], x: f64) -> (f64, f64) {
    let n = poly.len();
    let mut value = poly[0] * x + poly[1];
    let mut derivative = poly[0];

    for &c in poly.iter().take(n).skip(2) {
        derivative = derivative * x + value;
        value = value * x + c;
    }
    (value, derivative)
}

/// Refines a root using Newton-Raphson iterations.
fn refine_root(poly: &[f64], initial_guess: f64) -> f64 {
    let mut value = 0.0;
    let mut solution = initial_guess;
    let initial_value = evaluate_polynomial(poly, initial_guess);

    let mut iter = 1;
    while iter < MAX_NEWTON_ITERATIONS {
        let (v, derivative) = evaluate_polynomial_with_derivative(poly, solution);
        value = v;

        if derivative.abs() <= ZERO_THRESHOLD {
            break;
        }

        let delta = -value / derivative;

        if delta.abs() <= MACHINE_EPSILON * solution.abs() {
            break;
        }

        solution += delta;
        iter += 1;
    }

    // Return improved solution only if it's better.
    if value.abs() <= initial_value.abs() {
        solution
    } else {
        initial_guess
    }
}

/// Scales then Newton-refines every root of a polynomial in place.
fn scale_and_refine_all_roots(roots: &mut [f64], nb_roots: usize, scale_factor: f64, poly: &[f64]) {
    for r in roots.iter_mut().take(nb_roots) {
        *r *= scale_factor;
        *r = refine_root(poly, *r);
    }
}

/// Computes the base-2 exponent for coefficient scaling.
fn compute_base_exponent(value: f64) -> i32 {
    if value > 1.0 {
        (value.ln() * inv_log_radix()) as i32
    } else if value < -1.0 {
        (-(-value).ln() * inv_log_radix()) as i32
    } else {
        0
    }
}

/// Scaled coefficients, used to avoid overflow/underflow during the closed-form
/// solve. Mirrors OCCT's `ScaledCoefficients` struct.
struct ScaledCoefficients {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    #[allow(dead_code)]
    e: f64,
    /// Scale factor for converting roots back to the original coordinates.
    scale_factor: f64,
}

impl ScaledCoefficients {
    fn scale_quartic(the_a: f64, the_b: f64, the_c: f64, the_d: f64, the_e: f64) -> Self {
        let exp = compute_base_exponent(the_e) / 4;
        let scale_factor = FLOATING_RADIX.powi(exp);
        let sf2 = scale_factor * scale_factor;

        ScaledCoefficients {
            a: the_a / scale_factor,
            b: the_b / sf2,
            c: the_c / (sf2 * scale_factor),
            d: the_d / (sf2 * sf2),
            e: the_e / (sf2 * sf2),
            scale_factor,
        }
    }

    fn scale_cubic(the_a: f64, the_b: f64, the_c: f64, the_d: f64) -> Self {
        let exp = compute_base_exponent(the_d) / 3;
        let scale_factor = FLOATING_RADIX.powi(exp);
        let sf2 = scale_factor * scale_factor;

        ScaledCoefficients {
            a: the_a / scale_factor,
            b: the_b / sf2,
            c: the_c / (sf2 * scale_factor),
            d: the_d, // Not used for cubic, kept for consistency.
            e: 0.0,
            scale_factor,
        }
    }
}

/// Computes the special discriminant for `P < 0` cases (improved stability).
fn compute_special_discriminant(beta: f64, gamma: f64, del: f64, a1: f64) -> f64 {
    let sigma = beta * gamma / 3.0 - 2.0 * beta * beta * beta / 27.0;
    let psi = gamma * gamma * (4.0 * gamma - beta * beta) / 27.0;

    let d1 = if sigma >= 0.0 {
        sigma + 2.0 * (-a1).sqrt()
    } else {
        sigma - 2.0 * (-a1).sqrt()
    };

    let d2 = psi / d1;

    if (del - d1).abs() >= 18.0 * MACHINE_EPSILON * (del.abs() + d1.abs())
        && (del - d2).abs() >= 24.0 * MACHINE_EPSILON * (del.abs() + d2.abs())
    {
        return (del - d1) * (del - d2) / 4.0;
    }

    0.0
}

/// Solves a cubic with three distinct real roots via the trigonometric method.
fn solve_cubic_three_real_roots(
    beta: f64,
    gamma: f64,
    del: f64,
    p: f64,
    q: f64,
    discr: f64,
    roots: &mut [f64; 4],
) {
    if beta == 0.0 && q == 0.0 {
        // Special case: x^3 + Px = 0.
        roots[0] = (-p).sqrt();
        roots[1] = -roots[0];
        roots[2] = 0.0;
    } else {
        let sb = if beta >= 0.0 { 1.0 } else { -1.0 };
        let omega = (0.5 * q / (-discr).sqrt()).atan();
        let sp3 = (-p / 3.0).sqrt();
        let y1 = -2.0 * sb * sp3 * (std::f64::consts::PI / 6.0 - sb * omega / 3.0).cos();

        roots[0] = -beta / 3.0 + y1;

        if beta * q <= 0.0 {
            roots[1] = -beta / 3.0 + 2.0 * sp3 * (omega / 3.0).sin();
        } else {
            // Alternative formula for better accuracy.
            let dbg = del - beta * gamma;
            let sdbg = if dbg >= 0.0 { 1.0 } else { -1.0 };
            let den1 = 8.0 * beta * beta / 9.0 - 4.0 * beta * y1 / 3.0 - 2.0 * q / y1;
            let den2 = 2.0 * y1 * y1 - q / y1;
            roots[1] = dbg / den1 + sdbg * (-27.0 * discr).sqrt() / den2;
        }

        // Use Vieta's formula for the third root.
        roots[2] = -del / (roots[0] * roots[1]);
    }
}

/// Solves a cubic with one real root via Cardano's formula.
fn solve_cubic_one_real_root(beta: f64, del: f64, p: f64, q: f64, discr: f64, roots: &mut [f64; 4]) {
    let mut u = discr.sqrt() + (q / 2.0).abs();
    u = if u >= 0.0 {
        u.powf(1.0 / 3.0)
    } else {
        -(u.abs().powf(1.0 / 3.0))
    };

    let h = if p >= 0.0 {
        u * u + p / 3.0 + (p / u) * (p / u) / 9.0
    } else {
        u * q.abs() / (u * u - p / 3.0)
    };

    if beta * q >= 0.0 {
        if h.abs() <= REAL_SMALL && q.abs() <= REAL_SMALL {
            roots[0] = -beta / 3.0 - u + p / (3.0 * u);
        } else {
            roots[0] = -beta / 3.0 - q / h;
        }
    } else {
        roots[0] = -del / (beta * beta / 9.0 + h - beta * q / (3.0 * h));
    }
}

/// Solves a cubic with multiple roots (discriminant == 0).
fn solve_cubic_multiple_roots(
    beta: f64,
    gamma: f64,
    del: f64,
    p: f64,
    q: f64,
    roots: &mut [f64; 4],
    nb_roots: &mut usize,
) {
    *nb_roots = 3;
    let sq = if q >= 0.0 { 1.0 } else { -1.0 };
    let sp3 = (-p / 3.0).sqrt();

    if beta * q <= 0.0 {
        roots[0] = -beta / 3.0 + sq * sp3;
        roots[1] = roots[0];

        if beta * q == 0.0 {
            roots[2] = -beta / 3.0 - 2.0 * sq * sp3;
        } else {
            roots[2] = -del / (roots[0] * roots[1]);
        }
    } else {
        roots[0] = -gamma / (beta + 3.0 * sq * sp3);
        roots[1] = roots[0];
        roots[2] = -beta / 3.0 - 2.0 * sq * sp3;
    }
}

/// Determines whether a quartic should be reduced to a lower degree.
fn should_reduce_degree_quartic(the_a: f64, the_b: f64, the_c: f64, the_d: f64, the_e: f64) -> bool {
    if the_a.abs() <= ZERO_THRESHOLD {
        return true;
    }

    // Modified by jgv, 22.01.09 for numerical stability.
    let mut max_coeff = ZERO_THRESHOLD;
    max_coeff = max_coeff.max(the_b.abs());
    max_coeff = max_coeff.max(the_c.abs());
    max_coeff = max_coeff.max(the_d.abs());
    max_coeff = max_coeff.max(the_e.abs());

    if max_coeff > ZERO_THRESHOLD {
        max_coeff = occt_epsilon(100.0 * max_coeff);
    }

    if the_a.abs() <= max_coeff {
        let max_coeff1000 = 1000.0 * max_coeff;
        let mut with_a = false;

        if the_b.abs() > ZERO_THRESHOLD && the_b.abs() <= max_coeff1000 {
            with_a = true;
        }
        if the_c.abs() > ZERO_THRESHOLD && the_c.abs() <= max_coeff1000 {
            with_a = true;
        }
        if the_d.abs() > ZERO_THRESHOLD && the_d.abs() <= max_coeff1000 {
            with_a = true;
        }
        if the_e.abs() > ZERO_THRESHOLD && the_e.abs() <= max_coeff1000 {
            with_a = true;
        }

        return !with_a;
    }

    false
}

/// Factorization of a quartic into two quadratics via Ferrari's method.
struct QuarticFactorization {
    p1: f64,
    q1: f64,
    p2: f64,
    q2: f64,
}

/// Factors a quartic into two quadratics using Ferrari's method.
fn factor_quartic_via_ferrari(
    the_a: f64,
    the_b: f64,
    the_c: f64,
    the_d: f64,
    y0: f64,
) -> QuarticFactorization {
    // Compute discriminant and parameters.
    let discr = the_a * y0 * 0.5 - the_c;
    let sdiscr = if discr >= 0.0 { 1.0 } else { -1.0 };

    // Compute P0 and Q0 for the quadratic factors.
    let mut p0 = the_a * the_a * 0.25 - the_b + y0;
    p0 = if p0 < 0.0 { 0.0 } else { p0.sqrt() };

    let mut q0 = y0 * y0 * 0.25 - the_d;

    // Handle the case where Q0^2 is very close to zero more robustly.
    if q0.abs() < 10.0 * MACHINE_EPSILON {
        q0 = 0.0;
    } else {
        q0 = if q0 < 0.0 { 0.0 } else { q0.sqrt() };
    }

    // Form coefficients for the two quadratic equations.
    let ademi = the_a * 0.5;
    let ydemi = y0 * 0.5;
    let sdiscr_q0 = sdiscr * q0;

    let mut factors = QuarticFactorization {
        p1: ademi + p0,
        q1: ydemi + sdiscr_q0,
        p2: ademi - p0,
        q2: ydemi - sdiscr_q0,
    };

    // Clean up near-zero coefficients.
    let eps = 100.0 * MACHINE_EPSILON;

    if factors.p1.abs() <= eps {
        factors.p1 = 0.0;
    }
    if factors.p2.abs() <= eps {
        factors.p2 = 0.0;
    }
    if factors.q1.abs() <= eps {
        factors.q1 = 0.0;
    }
    if factors.q2.abs() <= eps {
        factors.q2 = 0.0;
    }

    factors
}

// ===========================================================================
// math_DirectPolynomialRoots
// ===========================================================================

// occt-ref: math_DirectPolynomialRoots
/// All real roots of a real polynomial of degree ≤ 4, found by direct
/// algebraic methods (Ferrari / Cardano / stable quadratic) and polished with
/// Newton-Raphson.
///
/// Construct with one of [`MathDirectPolynomialRoots::quartic`],
/// [`MathDirectPolynomialRoots::cubic`], [`MathDirectPolynomialRoots::quadratic`]
/// or [`MathDirectPolynomialRoots::linear`].
#[derive(Clone, Debug)]
pub struct MathDirectPolynomialRoots {
    /// Computation status flag.
    done: bool,
    /// Infinite-solutions flag (`0*x + 0 = 0`).
    infinite_status: bool,
    /// Number of real roots found (0..=4).
    nb_sol: usize,
    /// Storage for the computed real roots.
    roots: [f64; 4],
}

impl MathDirectPolynomialRoots {
    /// Computes all real roots of `A x^4 + B x^3 + C x^2 + D x + E = 0`
    /// via Ferrari's method.
    pub fn quartic(the_a: f64, the_b: f64, the_c: f64, the_d: f64, the_e: f64) -> Self {
        let mut s = MathDirectPolynomialRoots {
            done: true,
            infinite_status: false,
            nb_sol: 0,
            roots: [0.0; 4],
        };
        s.solve_quartic(the_a, the_b, the_c, the_d, the_e);
        s
    }

    /// Computes all real roots of `A x^3 + B x^2 + C x + D = 0`
    /// via Cardano's method.
    pub fn cubic(the_a: f64, the_b: f64, the_c: f64, the_d: f64) -> Self {
        let mut s = MathDirectPolynomialRoots {
            done: true,
            infinite_status: false,
            nb_sol: 0,
            roots: [0.0; 4],
        };
        s.solve_cubic(the_a, the_b, the_c, the_d);
        s
    }

    /// Computes all real roots of `A x^2 + B x + C = 0`
    /// via the numerically stable quadratic formula.
    pub fn quadratic(the_a: f64, the_b: f64, the_c: f64) -> Self {
        let mut s = MathDirectPolynomialRoots {
            done: true,
            infinite_status: false,
            nb_sol: 0,
            roots: [0.0; 4],
        };
        s.solve_quadratic(the_a, the_b, the_c);
        s
    }

    /// Computes the real root of the linear equation `A x + B = 0`.
    pub fn linear(the_a: f64, the_b: f64) -> Self {
        let mut s = MathDirectPolynomialRoots {
            done: true,
            infinite_status: false,
            nb_sol: 0,
            roots: [0.0; 4],
        };
        s.solve_linear(the_a, the_b);
        s
    }

    /// Returns true if the computations were successful.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns true if there is an infinity of roots (only `0*x + 0 = 0`).
    pub fn infinite_roots(&self) -> bool {
        self.infinite_status
    }

    /// Returns the number of distinct real roots found.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_InfiniteSolutions`) if there is an infinity
    /// of roots.
    pub fn nb_solutions(&self) -> usize {
        assert!(
            !self.infinite_status,
            "math_DirectPolynomialRoots: infinite solutions"
        );
        self.nb_sol
    }

    /// Returns the value of the `index`-th root (1-based), in the solver's
    /// natural ordering.
    ///
    /// # Panics
    /// Panics if there is an infinity of roots, or if `index` is out of
    /// `1..=nb_solutions()` (OCCT raises `StdFail_InfiniteSolutions` /
    /// `Standard_RangeError`).
    pub fn value(&self, index: usize) -> f64 {
        assert!(
            !self.infinite_status,
            "math_DirectPolynomialRoots: infinite solutions"
        );
        assert!(
            index >= 1 && index <= self.nb_sol,
            "math_DirectPolynomialRoots: index out of range"
        );
        self.roots[index - 1]
    }

    // -----------------------------------------------------------------------
    // Solve overloads (the C++ protected `Solve`).
    // -----------------------------------------------------------------------

    fn solve_quartic(&mut self, the_a: f64, the_b: f64, the_c: f64, the_d: f64, the_e: f64) {
        // Check for degree reduction.
        if should_reduce_degree_quartic(the_a, the_b, the_c, the_d, the_e) {
            self.solve_cubic(the_b, the_c, the_d, the_e);
            return;
        }

        // Normalize coefficients.
        let a = the_b / the_a;
        let b = the_c / the_a;
        let c = the_d / the_a;
        let d = the_e / the_a;

        // Scale coefficients to avoid overflow/underflow.
        // NOTE: OCCT passes `D` twice (theE = D) here; reproduced faithfully.
        let scaled = ScaledCoefficients::scale_quartic(a, b, c, d, d);

        // Solve Ferrari's resolvent cubic.
        let (y0, success) =
            solve_ferrari_resolvent(scaled.a, scaled.b, scaled.c, scaled.d);
        if !success {
            self.done = false;
            return;
        }

        // Factor into two quadratics.
        let factors = factor_quartic_via_ferrari(scaled.a, scaled.b, scaled.c, scaled.d, y0);

        // Solve first quadratic.
        let quad1 = MathDirectPolynomialRoots::quadratic(1.0, factors.p1, factors.q1);
        if !quad1.is_done() {
            self.done = false;
            return;
        }

        // Solve second quadratic.
        let quad2 = MathDirectPolynomialRoots::quadratic(1.0, factors.p2, factors.q2);
        if !quad2.is_done() {
            self.done = false;
            return;
        }

        // Collect all roots.
        self.nb_sol = quad1.nb_sol + quad2.nb_sol;
        let mut idx = 0;
        for i in 0..quad1.nb_sol {
            self.roots[idx] = quad1.roots[i];
            idx += 1;
        }
        for i in 0..quad2.nb_sol {
            self.roots[idx] = quad2.roots[i];
            idx += 1;
        }

        // Apply inverse scaling and Newton-Raphson refinement to all roots.
        let poly = [the_a, the_b, the_c, the_d, the_e];
        scale_and_refine_all_roots(&mut self.roots, self.nb_sol, scaled.scale_factor, &poly);
    }

    fn solve_cubic(&mut self, the_a: f64, the_b: f64, the_c: f64, the_d: f64) {
        // Check for degree reduction.
        if the_a.abs() <= ZERO_THRESHOLD {
            self.solve_quadratic(the_b, the_c, the_d);
            return;
        }

        // Normalize coefficients.
        let beta = the_b / the_a;
        let gamma = the_c / the_a;
        let del = the_d / the_a;

        // Scale to avoid overflow/underflow.
        let scaled = ScaledCoefficients::scale_cubic(beta, gamma, del, del);

        // Transform to depressed cubic: t^3 + Pt + Q = 0.
        let p1 = scaled.b;
        let p2 = -(scaled.a * scaled.a) / 3.0;
        let mut p = p1 + p2;
        let ep = 5.0 * MACHINE_EPSILON * (p1.abs() + p2.abs());
        if p.abs() <= ep {
            p = 0.0;
        }

        let q1 = scaled.c;
        let q2 = -scaled.a * scaled.b / 3.0;
        let q3 = 2.0 * (scaled.a * scaled.a * scaled.a) / 27.0;
        let mut q = q1 + q2 + q3;
        let eq = 10.0 * MACHINE_EPSILON * (q1.abs() + q2.abs() + q3.abs());
        if q.abs() <= eq {
            q = 0.0;
        }

        // Check for overflow.
        if p.abs() > OVERFLOW_LIMIT {
            self.done = false;
            return;
        }

        // Compute discriminant.
        let a1 = (p * p * p) / 27.0;
        let a2 = (q * q) / 4.0;
        let mut discr = a1 + a2;

        // Special handling for P < 0.
        if p < 0.0 {
            discr = compute_special_discriminant(scaled.a, scaled.b, scaled.c, a1);
        }

        // Solve based on discriminant.
        if discr < 0.0 {
            // Three distinct real roots.
            self.nb_sol = 3;
            solve_cubic_three_real_roots(
                scaled.a,
                scaled.b,
                scaled.c,
                p,
                q,
                discr,
                &mut self.roots,
            );
        } else if discr > 0.0 {
            // One real root.
            self.nb_sol = 1;
            solve_cubic_one_real_root(scaled.a, scaled.c, p, q, discr, &mut self.roots);
        } else {
            // Multiple roots.
            solve_cubic_multiple_roots(
                scaled.a,
                scaled.b,
                scaled.c,
                p,
                q,
                &mut self.roots,
                &mut self.nb_sol,
            );
        }

        // Apply inverse scaling and Newton-Raphson refinement to all roots.
        let poly = [the_a, the_b, the_c, the_d];
        scale_and_refine_all_roots(&mut self.roots, self.nb_sol, scaled.scale_factor, &poly);
    }

    fn solve_quadratic(&mut self, the_a: f64, the_b: f64, the_c: f64) {
        // Check for degree reduction.
        if the_a.abs() <= ZERO_THRESHOLD {
            self.solve_linear(the_b, the_c);
            return;
        }

        // Solve normalized quadratic x^2 + P*x + Q = 0.
        let p = the_b / the_a;
        let q = the_c / the_a;

        // Compute discriminant with error bounds.
        let eps_d = 3.0 * MACHINE_EPSILON * (p * p + (4.0 * q).abs());
        let mut discrim = p * p - 4.0 * q;

        if discrim.abs() <= eps_d {
            discrim = 0.0;
        }

        if discrim < 0.0 {
            // No real roots.
            self.nb_sol = 0;
        } else if discrim == 0.0 {
            // Double root.
            self.nb_sol = 2;
            let poly = [1.0, p, q];
            self.roots[0] = -0.5 * p;
            self.roots[0] = refine_root(&poly, self.roots[0]);
            self.roots[1] = self.roots[0];
        } else {
            // Two distinct real roots — numerically stable formula.
            self.nb_sol = 2;
            let poly = [1.0, p, q];
            if p > 0.0 {
                self.roots[0] = -(p + discrim.sqrt()) / 2.0;
            } else {
                self.roots[0] = -(p - discrim.sqrt()) / 2.0;
            }
            self.roots[0] = refine_root(&poly, self.roots[0]);
            self.roots[1] = q / self.roots[0];
            self.roots[1] = refine_root(&poly, self.roots[1]);
        }
    }

    fn solve_linear(&mut self, the_a: f64, the_b: f64) {
        if the_a.abs() <= ZERO_THRESHOLD {
            if the_b.abs() <= ZERO_THRESHOLD {
                // 0 = 0: infinite solutions.
                self.infinite_status = true;
                return;
            }
            // 0*x + B = 0: no solution.
            self.nb_sol = 0;
            return;
        }

        // Normal case: unique solution.
        self.nb_sol = 1;
        self.roots[0] = -the_b / the_a;
    }
}

/// Constructs and solves Ferrari's resolvent cubic, returning its largest root
/// and whether the inner cubic solve succeeded. Mirrors OCCT's
/// `SolveFerrariResolvent`.
fn solve_ferrari_resolvent(the_a: f64, the_b: f64, the_c: f64, the_d: f64) -> (f64, bool) {
    // Construct resolvent cubic: Y^3 + R3*Y^2 + S3*Y + T3 = 0.
    let r3 = -the_b;
    let s3 = the_a * the_c - 4.0 * the_d;
    let t3 = the_d * (4.0 * the_b - the_a * the_a) - the_c * the_c;

    let cubic = MathDirectPolynomialRoots::cubic(1.0, r3, s3, t3);

    if !cubic.is_done() {
        return (0.0, false);
    }

    // Choose the largest root for numerical stability.
    let mut y0 = cubic.value(1);
    for i in 2..=cubic.nb_solutions() {
        if cubic.value(i) > y0 {
            y0 = cubic.value(i);
        }
    }

    (y0, true)
}

impl fmt::Display for MathDirectPolynomialRoots {
    /// `math_DirectPolynomialRoots::Dump` — diagnostic dump used by `operator<<`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "math_DirectPolynomialRoots ")?;
        if !self.done {
            writeln!(f, " Not Done ")
        } else if self.infinite_status {
            writeln!(f, " Status = Infinity Roots ")
        } else {
            writeln!(f, " Status = Not Infinity Roots ")?;
            writeln!(f, " Number of solutions = {}", self.nb_sol)?;
            for i in 1..=self.nb_sol {
                writeln!(f, " Solution number {} = {}", i, self.roots[i - 1])?;
            }
            Ok(())
        }
    }
}

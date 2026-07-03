//! `math_TrigonometricFunctionRoots` — roots of the trigonometric equation
//!
//! ```text
//! a*cos(x)*cos(x) + 2*b*cos(x)*sin(x) + c*cos(x) + d*sin(x) + e = 0
//! ```
//!
//! Faithful reproduction of OpenCascade's `math_TrigonometricFunctionRoots`
//! (`src/FoundationClasses/TKMath/math/math_TrigonometricFunctionRoots.{hxx,cxx,lxx}`).
//!
//! The degree of the equation can be 4, 3 or 2. The class implements the same
//! resolution pipeline as OCCT:
//!
//! - special-case analytic reductions for the degenerate (linear / quadratic /
//!   sin-cos product) forms,
//! - a degree-4 polynomial path through the Weierstrass / half-angle
//!   substitution `t = tan(x/2)` solved by [`MathDirectPolynomialRoots`]
//!   (a faithful port of OCCT's `math_DirectPolynomialRoots`),
//! - Newton refinement of every accepted root via
//!   [`MathTrigonometricEquationFunction`] + [`MathNewtonFunctionRoot`]
//!   (ports of `math_TrigonometricEquationFunction` and
//!   `math_NewtonFunctionRoot`),
//! - the `PI` special case at the end of `Perform`.
//!
//! The supporting numerical helpers (`RealFirst`, `RealLast`, `RealEpsilon`,
//! `RealSmall`, `Epsilon`) are reproduced exactly so the control flow and the
//! produced root sets match OCCT bit-for-bit on the test inputs.

use std::fmt;

// ===========================================================================
// OCCT numeric primitives (Standard_Real / Precision helpers).
// ===========================================================================

/// `RealFirst()` — the most negative finite double.
pub const REAL_FIRST: f64 = -1.797_693_134_862_315_7e308;
/// `RealLast()` — the largest finite double.
pub const REAL_LAST: f64 = 1.797_693_134_862_315_7e308;
/// `RealEpsilon()` — `DBL_EPSILON`.
pub const REAL_EPSILON: f64 = f64::EPSILON; // 2.220446049250313e-16
/// `RealSmall()` — `DBL_MIN`, the smallest positive normal double.
pub const REAL_SMALL: f64 = f64::MIN_POSITIVE; // 2.2250738585072014e-308
/// `Precision::PConfusion()` — parametric coincidence tolerance.
pub const P_CONFUSION: f64 = 1.0e-9;

/// `Epsilon(theValue)` — OCCT's positive ULP of `theValue`.
///
/// OCCT defines this as the distance from `|theValue|` to the next
/// representable double above it (and returns a tiny positive number even at
/// zero, matching the OCCT/`frexp`-based implementation).
pub fn epsilon(value: f64) -> f64 {
    // OCCT implementation:
    //   res = abs(theValue);
    //   if (frexp(res, &exp) >= 0.5) exp++ ; else if == -0.5, etc.
    //   res = ldexp(1.0, exp - 53);
    // This is equivalent to nextafter(|x|, +inf) - |x| for normal values.
    let res = value.abs();
    if res == 0.0 {
        // ldexp(1.0, -1074) style smallest subnormal step; OCCT returns a tiny
        // positive number here. The next-up of 0 is the smallest subnormal.
        return f64::from_bits(1);
    }
    let up = next_up(res);
    up - res
}

/// Smallest double strictly greater than `x` (for finite non-negative `x`).
fn next_up(x: f64) -> f64 {
    if x.is_nan() || x == f64::INFINITY {
        return x;
    }
    let bits = x.to_bits();
    // For x >= 0 the bit pattern increases monotonically toward +inf.
    f64::from_bits(bits + 1)
}

// ===========================================================================
// math_DirectPolynomialRoots — real roots of polynomials up to degree 4.
// ===========================================================================

const ZERO_THRESHOLD: f64 = 1.0e-30;
const MACHINE_EPSILON: f64 = REAL_EPSILON;
const FLOATING_RADIX: f64 = 2.0;
const MAX_NEWTON_ITERATIONS: i32 = 10;
const OVERFLOW_LIMIT: f64 = 1.0e+80;

#[inline]
fn inv_log_radix() -> f64 {
    1.0 / (2.0f64).ln()
}

// Evaluates polynomial at point X using Horner's method.
fn evaluate_polynomial(n: usize, poly: &[f64], x: f64) -> f64 {
    let mut result = poly[0];
    for i in 1..n {
        result = result * x + poly[i];
    }
    result
}

// Evaluates polynomial and its derivative simultaneously.
fn evaluate_polynomial_with_derivative(n: usize, poly: &[f64], x: f64) -> (f64, f64) {
    let mut value = poly[0] * x + poly[1];
    let mut derivative = poly[0];
    for i in 2..n {
        derivative = derivative * x + value;
        value = value * x + poly[i];
    }
    (value, derivative)
}

// Refines root using Newton-Raphson iterations.
fn refine_root(n: usize, poly: &[f64], initial_guess: f64) -> f64 {
    let mut value = 0.0;
    let mut solution = initial_guess;
    let initial_value = evaluate_polynomial(n, poly, initial_guess);

    for _iter in 1..MAX_NEWTON_ITERATIONS {
        let (v, derivative) = evaluate_polynomial_with_derivative(n, poly, solution);
        value = v;
        if derivative.abs() <= ZERO_THRESHOLD {
            break;
        }
        let delta = -value / derivative;
        if delta.abs() <= MACHINE_EPSILON * solution.abs() {
            break;
        }
        solution += delta;
    }

    if value.abs() <= initial_value.abs() {
        solution
    } else {
        initial_guess
    }
}

// Computes base-2 exponent for coefficient scaling.
fn compute_base_exponent(value: f64) -> i32 {
    if value > 1.0 {
        (value.ln() * inv_log_radix()) as i32
    } else if value < -1.0 {
        (-(-value).ln() * inv_log_radix()) as i32
    } else {
        0
    }
}

#[derive(Clone, Copy, Default)]
struct ScaledCoefficients {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    #[allow(dead_code)]
    e: f64,
    scale_factor: f64,
}

impl ScaledCoefficients {
    fn scale_quartic(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64) {
        let exp = compute_base_exponent(e) / 4;
        self.scale_factor = FLOATING_RADIX.powi(exp);
        let sf = self.scale_factor;
        let sf2 = sf * sf;
        self.a = a / sf;
        self.b = b / sf2;
        self.c = c / (sf2 * sf);
        self.d = d / (sf2 * sf2);
        self.e = e / (sf2 * sf2);
    }

    fn scale_cubic(&mut self, a: f64, b: f64, c: f64, d: f64) {
        let exp = compute_base_exponent(d) / 3;
        self.scale_factor = FLOATING_RADIX.powi(exp);
        let sf = self.scale_factor;
        let sf2 = sf * sf;
        self.a = a / sf;
        self.b = b / sf2;
        self.c = c / (sf2 * sf);
        self.d = d; // unused for cubic, kept for parity
    }
}

// Computes special discriminant for P < 0 cases with improved numerical stability.
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

// Solves cubic with three real roots using the trigonometric method.
fn solve_cubic_three_real_roots(
    beta: f64,
    gamma: f64,
    del: f64,
    p: f64,
    q: f64,
    discr: f64,
    roots: &mut [f64],
) {
    if beta == 0.0 && q == 0.0 {
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
            let dbg = del - beta * gamma;
            let sdbg = if dbg >= 0.0 { 1.0 } else { -1.0 };
            let den1 = 8.0 * beta * beta / 9.0 - 4.0 * beta * y1 / 3.0 - 2.0 * q / y1;
            let den2 = 2.0 * y1 * y1 - q / y1;
            roots[1] = dbg / den1 + sdbg * (-27.0 * discr).sqrt() / den2;
        }
        roots[2] = -del / (roots[0] * roots[1]);
    }
}

// Solves cubic with one real root using Cardano's formula.
fn solve_cubic_one_real_root(beta: f64, del: f64, p: f64, q: f64, discr: f64, roots: &mut [f64]) {
    let mut u = discr.sqrt() + (q / 2.0).abs();
    u = if u >= 0.0 {
        u.powf(1.0 / 3.0)
    } else {
        -u.abs().powf(1.0 / 3.0)
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

// Solves cubic with multiple roots (discriminant == 0).
fn solve_cubic_multiple_roots(
    beta: f64,
    gamma: f64,
    del: f64,
    p: f64,
    q: f64,
    roots: &mut [f64],
    nb_roots: &mut i32,
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

// Determines if quartic should be reduced to lower degree.
fn should_reduce_degree_quartic(a: f64, b: f64, c: f64, d: f64, e: f64) -> bool {
    if a.abs() <= ZERO_THRESHOLD {
        return true;
    }
    let mut max_coeff = ZERO_THRESHOLD;
    max_coeff = max_coeff.max(b.abs());
    max_coeff = max_coeff.max(c.abs());
    max_coeff = max_coeff.max(d.abs());
    max_coeff = max_coeff.max(e.abs());

    if max_coeff > ZERO_THRESHOLD {
        max_coeff = epsilon(100.0 * max_coeff);
    }

    if a.abs() <= max_coeff {
        let max_coeff1000 = 1000.0 * max_coeff;
        let mut with_a = false;
        if b.abs() > ZERO_THRESHOLD && b.abs() <= max_coeff1000 {
            with_a = true;
        }
        if c.abs() > ZERO_THRESHOLD && c.abs() <= max_coeff1000 {
            with_a = true;
        }
        if d.abs() > ZERO_THRESHOLD && d.abs() <= max_coeff1000 {
            with_a = true;
        }
        if e.abs() > ZERO_THRESHOLD && e.abs() <= max_coeff1000 {
            with_a = true;
        }
        return !with_a;
    }
    false
}

// Constructs and solves Ferrari's resolvent cubic equation.
fn solve_ferrari_resolvent(a: f64, b: f64, c: f64, d: f64) -> Option<f64> {
    let r3 = -b;
    let s3 = a * c - 4.0 * d;
    let t3 = d * (4.0 * b - a * a) - c * c;

    let cubic_solver = MathDirectPolynomialRoots::cubic(1.0, r3, s3, t3);
    if !cubic_solver.is_done() {
        return None;
    }

    let mut y0 = cubic_solver.value(1);
    for i in 2..=cubic_solver.nb_solutions() {
        let v = cubic_solver.value(i);
        if v > y0 {
            y0 = v;
        }
    }
    Some(y0)
}

#[derive(Clone, Copy, Default)]
struct QuarticFactorization {
    p1: f64,
    q1: f64,
    p2: f64,
    q2: f64,
}

// Factors quartic into two quadratics using Ferrari's method.
fn factor_quartic_via_ferrari(a: f64, b: f64, _c: f64, d: f64, y0: f64) -> QuarticFactorization {
    let discr = a * y0 * 0.5 - _c;
    let sdiscr = if discr >= 0.0 { 1.0 } else { -1.0 };

    let mut p0 = a * a * 0.25 - b + y0;
    p0 = if p0 < 0.0 { 0.0 } else { p0.sqrt() };

    let mut q0 = y0 * y0 * 0.25 - d;
    if q0.abs() < 10.0 * MACHINE_EPSILON {
        q0 = 0.0;
    } else {
        q0 = if q0 < 0.0 { 0.0 } else { q0.sqrt() };
    }

    let ademi = a * 0.5;
    let ydemi = y0 * 0.5;
    let sdiscr_q0 = sdiscr * q0;

    let mut f = QuarticFactorization {
        p1: ademi + p0,
        q1: ydemi + sdiscr_q0,
        p2: ademi - p0,
        q2: ydemi - sdiscr_q0,
    };

    let eps = 100.0 * MACHINE_EPSILON;
    if f.p1.abs() <= eps {
        f.p1 = 0.0;
    }
    if f.p2.abs() <= eps {
        f.p2 = 0.0;
    }
    if f.q1.abs() <= eps {
        f.q1 = 0.0;
    }
    if f.q2.abs() <= eps {
        f.q2 = 0.0;
    }
    f
}

// occt: math_DirectPolynomialRoots
/// Real roots of a polynomial of degree 2, 3 or 4 — a faithful port of OCCT's
/// `math_DirectPolynomialRoots`. Roots are 1-indexed via [`value`](Self::value)
/// to match the original 1-based `Value(Index)` accessor.
#[derive(Clone, Debug)]
pub struct MathDirectPolynomialRoots {
    nb_sol: i32,
    roots: [f64; 4],
    infinite_status: bool,
    done: bool,
}

impl MathDirectPolynomialRoots {
    /// `math_DirectPolynomialRoots(A,B,C,D,E)` — quartic `A x^4 + ... + E`.
    pub fn quartic(a: f64, b: f64, c: f64, d: f64, e: f64) -> Self {
        let mut s = Self {
            nb_sol: 0,
            roots: [0.0; 4],
            infinite_status: false,
            done: true,
        };
        s.solve_quartic(a, b, c, d, e);
        s
    }

    /// `math_DirectPolynomialRoots(A,B,C,D)` — cubic `A x^3 + ... + D`.
    pub fn cubic(a: f64, b: f64, c: f64, d: f64) -> Self {
        let mut s = Self {
            nb_sol: 0,
            roots: [0.0; 4],
            infinite_status: false,
            done: true,
        };
        s.solve_cubic(a, b, c, d);
        s
    }

    /// `math_DirectPolynomialRoots(A,B,C)` — quadratic `A x^2 + B x + C`.
    pub fn quadratic(a: f64, b: f64, c: f64) -> Self {
        let mut s = Self {
            nb_sol: 0,
            roots: [0.0; 4],
            infinite_status: false,
            done: true,
        };
        s.solve_quadratic(a, b, c);
        s
    }

    /// `math_DirectPolynomialRoots(A,B)` — linear `A x + B`.
    pub fn linear(a: f64, b: f64) -> Self {
        let mut s = Self {
            nb_sol: 0,
            roots: [0.0; 4],
            infinite_status: false,
            done: true,
        };
        s.solve_linear(a, b);
        s
    }

    /// `IsDone()`.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// `InfiniteRoots()`.
    pub fn infinite_roots(&self) -> bool {
        self.infinite_status
    }

    /// `NbSolutions()`.
    pub fn nb_solutions(&self) -> i32 {
        self.nb_sol
    }

    /// `Value(Index)` — 1-indexed.
    pub fn value(&self, index: i32) -> f64 {
        self.roots[(index - 1) as usize]
    }

    fn solve_quartic(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64) {
        if should_reduce_degree_quartic(a, b, c, d, e) {
            self.solve_cubic(b, c, d, e);
            return;
        }

        let an = b / a;
        let bn = c / a;
        let cn = d / a;
        let dn = e / a;

        let mut scaled = ScaledCoefficients::default();
        scaled.scale_quartic(an, bn, cn, dn, dn);

        let y0 = match solve_ferrari_resolvent(scaled.a, scaled.b, scaled.c, scaled.d) {
            Some(v) => v,
            None => {
                self.done = false;
                return;
            }
        };

        let factors = factor_quartic_via_ferrari(scaled.a, scaled.b, scaled.c, scaled.d, y0);

        let quad1 = MathDirectPolynomialRoots::quadratic(1.0, factors.p1, factors.q1);
        if !quad1.is_done() {
            self.done = false;
            return;
        }
        let quad2 = MathDirectPolynomialRoots::quadratic(1.0, factors.p2, factors.q2);
        if !quad2.is_done() {
            self.done = false;
            return;
        }

        self.nb_sol = quad1.nb_solutions() + quad2.nb_solutions();
        let mut idx = 0usize;
        for i in 0..quad1.nb_solutions() as usize {
            self.roots[idx] = quad1.roots[i];
            idx += 1;
        }
        for i in 0..quad2.nb_solutions() as usize {
            self.roots[idx] = quad2.roots[i];
            idx += 1;
        }

        // Inverse scaling and Newton-Raphson refinement.
        let coeffs = [a, b, c, d, e];
        for i in 0..self.nb_sol as usize {
            self.roots[i] *= scaled.scale_factor;
            self.roots[i] = refine_root(5, &coeffs, self.roots[i]);
        }
    }

    fn solve_cubic(&mut self, a: f64, b: f64, c: f64, d: f64) {
        if a.abs() <= ZERO_THRESHOLD {
            self.solve_quadratic(b, c, d);
            return;
        }

        let beta = b / a;
        let gamma = c / a;
        let del = d / a;

        let mut scaled = ScaledCoefficients::default();
        scaled.scale_cubic(beta, gamma, del, del);

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

        if p.abs() > OVERFLOW_LIMIT {
            self.done = false;
            return;
        }

        let a1 = (p * p * p) / 27.0;
        let a2 = (q * q) / 4.0;
        let mut discr = a1 + a2;

        if p < 0.0 {
            discr = compute_special_discriminant(scaled.a, scaled.b, scaled.c, a1);
        }

        if discr < 0.0 {
            self.nb_sol = 3;
            solve_cubic_three_real_roots(
                scaled.a, scaled.b, scaled.c, p, q, discr, &mut self.roots,
            );
        } else if discr > 0.0 {
            self.nb_sol = 1;
            solve_cubic_one_real_root(scaled.a, scaled.c, p, q, discr, &mut self.roots);
        } else {
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

        let coeffs = [a, b, c, d];
        for i in 0..self.nb_sol as usize {
            self.roots[i] *= scaled.scale_factor;
            self.roots[i] = refine_root(4, &coeffs, self.roots[i]);
        }
    }

    fn solve_quadratic(&mut self, a: f64, b: f64, c: f64) {
        if a.abs() <= ZERO_THRESHOLD {
            self.solve_linear(b, c);
            return;
        }

        let p = b / a;
        let q = c / a;

        let eps_d = 3.0 * MACHINE_EPSILON * (p * p + (4.0 * q).abs());
        let mut discrim = p * p - 4.0 * q;
        if discrim.abs() <= eps_d {
            discrim = 0.0;
        }

        if discrim < 0.0 {
            self.nb_sol = 0;
        } else if discrim == 0.0 {
            self.nb_sol = 2;
            self.roots[0] = -0.5 * p;
            self.roots[0] = refine_root(3, &[1.0, p, q], self.roots[0]);
            self.roots[1] = self.roots[0];
        } else {
            self.nb_sol = 2;
            if p > 0.0 {
                self.roots[0] = -(p + discrim.sqrt()) / 2.0;
            } else {
                self.roots[0] = -(p - discrim.sqrt()) / 2.0;
            }
            self.roots[0] = refine_root(3, &[1.0, p, q], self.roots[0]);
            self.roots[1] = q / self.roots[0];
            self.roots[1] = refine_root(3, &[1.0, p, q], self.roots[1]);
        }
    }

    fn solve_linear(&mut self, a: f64, b: f64) {
        if a.abs() <= ZERO_THRESHOLD {
            if b.abs() <= ZERO_THRESHOLD {
                self.infinite_status = true;
                return;
            }
            self.nb_sol = 0;
            return;
        }
        self.nb_sol = 1;
        self.roots[0] = -b / a;
    }
}

// ===========================================================================
// math_TrigonometricEquationFunction — F(x) and F'(x) of the trig equation.
// ===========================================================================

// occt: math_TrigonometricEquationFunction
/// `a*cos^2(x) + 2*b*cos(x)*sin(x) + c*cos(x) + d*sin(x) + e` and its
/// derivative — a faithful port of OCCT's `math_TrigonometricEquationFunction`.
#[derive(Clone, Copy, Debug)]
pub struct MathTrigonometricEquationFunction {
    aa: f64,
    bb: f64,
    cc: f64,
    dd: f64,
    ee: f64,
}

impl MathTrigonometricEquationFunction {
    /// Constructor `(A,B,C,D,E)`.
    pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64) -> Self {
        Self {
            aa: a,
            bb: b,
            cc: c,
            dd: d,
            ee: e,
        }
    }

    /// `Value(X)` — F(X).
    pub fn value(&self, x: f64) -> f64 {
        let cn = x.cos();
        let sn = x.sin();
        cn * (self.aa * cn + (self.bb + self.bb) * sn + self.cc) + self.dd * sn + self.ee
    }

    /// `Derivative(X)` — F'(X).
    pub fn derivative(&self, x: f64) -> f64 {
        let cn = x.cos();
        let sn = x.sin();
        let mut d = -self.aa * cn * sn + self.bb * (cn * cn - sn * sn);
        d += d;
        d += -self.cc * sn + self.dd * cn;
        d
    }

    /// `Values(X)` — both F(X) and F'(X).
    pub fn values(&self, x: f64) -> (f64, f64) {
        let cn = x.cos();
        let sn = x.sin();
        let aacn = self.aa * cn;
        let bbsn = self.bb * sn;
        let f = aacn * cn + bbsn * (cn + cn) + self.cc * cn + self.dd * sn + self.ee;
        let mut d = -aacn * sn + self.bb * (cn * cn - sn * sn);
        d += d;
        d += -self.cc * sn + self.dd * cn;
        (f, d)
    }
}

// ===========================================================================
// math_NewtonFunctionRoot — Newton root finder over [A,B].
// ===========================================================================

// occt: math_NewtonFunctionRoot
/// Newton root finder for [`MathTrigonometricEquationFunction`], a faithful
/// port of OCCT's `math_NewtonFunctionRoot` (bounded variant).
#[derive(Clone, Copy, Debug)]
pub struct MathNewtonFunctionRoot {
    binf: f64,
    bsup: f64,
    epsilon_x: f64,
    epsilon_f: f64,
    itermax: i32,
    done: bool,
    x: f64,
}

impl MathNewtonFunctionRoot {
    /// `math_NewtonFunctionRoot(F, Guess, EpsX, EpsF, NbIterations)` — the
    /// unbounded constructor used by `math_TrigonometricFunctionRoots`, where
    /// `[Binf,Bsup] = [RealFirst(), RealLast()]`.
    pub fn new(
        f: &MathTrigonometricEquationFunction,
        guess: f64,
        eps_x: f64,
        eps_f: f64,
        nb_iterations: i32,
    ) -> Self {
        let mut s = Self {
            binf: REAL_FIRST,
            bsup: REAL_LAST,
            epsilon_x: eps_x,
            epsilon_f: eps_f,
            itermax: nb_iterations,
            done: false,
            x: REAL_LAST,
        };
        s.perform(f, guess);
        s
    }

    /// `IsDone()`.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// `Root()`.
    pub fn root(&self) -> f64 {
        self.x
    }

    fn perform(&mut self, f: &MathTrigonometricEquationFunction, guess: f64) {
        let mut best_x = self.x;
        let mut best_fx = REAL_LAST;

        let (aa, bb) = if self.binf < self.bsup {
            (self.binf, self.bsup)
        } else {
            (self.bsup, self.binf)
        };

        let mut dx = REAL_LAST;
        let mut fx = REAL_LAST;
        self.x = guess;
        let mut it = 1;

        while it <= self.itermax && (dx.abs() > self.epsilon_x || fx.abs() > self.epsilon_f) {
            let (fv, dfx) = f.values(self.x);
            fx = fv;

            let abs_fx = fx.abs();
            if abs_fx < best_fx {
                best_fx = abs_fx;
                best_x = self.x;
            }

            if dfx == 0.0 {
                self.done = false;
                it = self.itermax + 1;
            } else {
                dx = fx / dfx;
                self.x -= dx;
                if self.x <= aa {
                    self.x = aa;
                }
                if self.x >= bb {
                    self.x = bb;
                }
                it += 1;
            }
        }
        self.x = best_x;
        self.done = it <= self.itermax;
    }
}

// ===========================================================================
// math_TrigonometricFunctionRoots — the primary class.
// ===========================================================================

// occt: math_TrigonometricFunctionRoots
/// Solutions of `a*cos^2(x) + 2*b*cos(x)*sin(x) + c*cos(x) + d*sin(x) + e = 0`
/// in `[InfBound, SupBound]`. Faithful reproduction of OCCT's
/// `math_TrigonometricFunctionRoots`. The degree of the equation can be 4, 3
/// or 2 (`a`, `b` and `c` may be zero).
#[derive(Clone, Debug)]
pub struct MathTrigonometricFunctionRoots {
    nb_sol: i32,
    sol: [f64; 4],
    infinite_status: bool,
    done: bool,
}

impl MathTrigonometricFunctionRoots {
    /// Full constructor `(A,B,C,D,E,InfBound,SupBound)` — resolves the degree-4
    /// equation `a*cos^2 + 2b*cos*sin + c*cos + d*sin + e = 0`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        inf_bound: f64,
        sup_bound: f64,
    ) -> Self {
        let mut s = Self::empty();
        s.perform(a, b, c, d, e, inf_bound, sup_bound);
        s
    }

    /// Constructor `(D,E,InfBound,SupBound)` — resolves `d*sin(x) + e = 0`.
    pub fn new_sine(d: f64, e: f64, inf_bound: f64, sup_bound: f64) -> Self {
        let mut s = Self::empty();
        s.perform(0.0, 0.0, 0.0, d, e, inf_bound, sup_bound);
        s
    }

    /// Constructor `(C,D,E,InfBound,SupBound)` — resolves
    /// `c*cos(x) + d*sin(x) + e = 0`.
    pub fn new_cosine_sine(c: f64, d: f64, e: f64, inf_bound: f64, sup_bound: f64) -> Self {
        let mut s = Self::empty();
        s.perform(0.0, 0.0, c, d, e, inf_bound, sup_bound);
        s
    }

    fn empty() -> Self {
        Self {
            nb_sol: -1,
            sol: [0.0; 4],
            infinite_status: false,
            done: false,
        }
    }

    /// `IsDone()` — true if the computation succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// `InfiniteRoots()` — true if there is an infinity of roots.
    pub fn infinite_roots(&self) -> bool {
        self.infinite_status
    }

    /// `NbSolutions()` — number of solutions found.
    ///
    /// # Panics
    /// Panics (mirroring OCCT's `StdFail_NotDone` / `StdFail_InfiniteSolutions`)
    /// if not done, or if there is an infinity of solutions.
    pub fn nb_solutions(&self) -> i32 {
        assert!(!self.infinite_status, "InfiniteSolutions");
        assert!(self.done, "NotDone");
        self.nb_sol
    }

    /// `Value(Index)` — the 1-indexed solution.
    ///
    /// # Panics
    /// Panics if there is an infinity of solutions, if not done, or if
    /// `index > NbSolutions`.
    pub fn value(&self, index: i32) -> f64 {
        assert!(!self.infinite_status, "InfiniteSolutions");
        assert!(self.done, "NotDone");
        assert!(index <= self.nb_sol, "OutOfRange");
        self.sol[(index - 1) as usize]
    }

    #[allow(clippy::too_many_arguments)]
    fn perform(
        &mut self,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        inf_bound: f64,
        sup_bound: f64,
    ) {
        let pi = std::f64::consts::PI;

        let mut nzer: i32 = 0;
        let nit = 10;
        let mut zer = [0.0f64; 4]; // OCCT Zer(1..4)
        let tol1 = 1.0e-15;

        self.infinite_status = false;
        self.done = true;

        let eps = 1.5e-12;
        let depi = pi + pi;

        let my_borne_inf;
        let mut delta;
        let modv;

        if inf_bound <= REAL_FIRST && sup_bound >= REAL_LAST {
            my_borne_inf = 0.0;
            delta = depi;
            modv = 0.0;
        } else if sup_bound >= REAL_LAST {
            my_borne_inf = inf_bound;
            delta = depi;
            modv = my_borne_inf / depi;
        } else if inf_bound <= REAL_FIRST {
            my_borne_inf = sup_bound - depi;
            delta = depi;
            modv = my_borne_inf / depi;
        } else {
            my_borne_inf = inf_bound;
            delta = sup_bound - inf_bound;
            modv = inf_bound / depi;
            if (sup_bound - inf_bound) > depi {
                delta = depi;
            }
        }

        if a.abs() <= eps && b.abs() <= eps {
            if c.abs() <= eps {
                if d.abs() <= eps {
                    if e.abs() <= eps {
                        self.infinite_status = true;
                        return;
                    } else {
                        self.nb_sol = 0;
                        return;
                    }
                } else {
                    // d*sin(x) + e = 0
                    self.nb_sol = 0;
                    let aa = -e / d;
                    if aa.abs() > 1.0 {
                        return;
                    }
                    zer[0] = aa.asin();
                    zer[1] = pi - zer[0];
                    nzer = 2;
                    for i in 0..nzer as usize {
                        if zer[i] <= -eps {
                            zer[i] = depi - zer[i].abs();
                        }
                        zer[i] += modv.trunc() * depi;
                        let x = zer[i] - my_borne_inf;
                        if x > -epsilon(delta) && x < delta + epsilon(delta) {
                            self.sol[self.nb_sol as usize] = zer[i];
                            self.nb_sol += 1;
                        }
                    }
                }
                return;
            } else if d.abs() <= eps {
                // c*cos(x) + e = 0
                self.nb_sol = 0;
                let aa = -e / c;
                if aa.abs() > 1.0 {
                    return;
                }
                zer[0] = aa.acos();
                zer[1] = -zer[0];
                nzer = 2;
                for i in 0..nzer as usize {
                    if zer[i] <= -eps {
                        zer[i] = depi - zer[i].abs();
                    }
                    zer[i] += modv.trunc() * 2.0 * pi;
                    let x = zer[i] - my_borne_inf;
                    if x >= -epsilon(delta) && x <= delta + epsilon(delta) {
                        self.sol[self.nb_sol as usize] = zer[i];
                        self.nb_sol += 1;
                    }
                }
                return;
            } else {
                // Second-degree equation.
                let aa = e - c;
                let bb = 2.0 * d;
                let cc = e + c;

                let resol = MathDirectPolynomialRoots::quadratic(aa, bb, cc);
                if !resol.is_done() {
                    self.done = false;
                    return;
                } else if !resol.infinite_roots() {
                    nzer = resol.nb_solutions();
                    for i in 0..nzer as usize {
                        zer[i] = resol.value(i as i32 + 1);
                    }
                } else if resol.infinite_roots() {
                    self.infinite_status = true;
                    return;
                }
            }
        } else {
            // Two additional analytical cases.
            if a.abs() <= eps && e.abs() <= eps {
                if c.abs() <= eps {
                    // 2*B*sin*cos + D*sin = 0
                    nzer = 2;
                    zer[0] = 0.0;
                    zer[1] = pi;

                    let aa = -d / (b * 2.0);
                    if aa.abs() <= 1.0 + P_CONFUSION {
                        nzer = 4;
                        if aa >= 1.0 {
                            zer[2] = 0.0;
                            zer[3] = 0.0;
                        } else if aa <= -1.0 {
                            zer[2] = pi;
                            zer[3] = pi;
                        } else {
                            zer[2] = aa.acos();
                            zer[3] = depi - zer[2];
                        }
                    }

                    self.nb_sol = 0;
                    for i in 0..nzer as usize {
                        if zer[i] <= my_borne_inf - eps {
                            zer[i] += depi;
                        }
                        zer[i] += modv.trunc() * 2.0 * pi;
                        let x = zer[i] - my_borne_inf;
                        if x >= -P_CONFUSION && x <= delta + P_CONFUSION {
                            if zer[i] < inf_bound {
                                zer[i] = inf_bound;
                            }
                            if zer[i] > sup_bound {
                                zer[i] = sup_bound;
                            }
                            self.sol[self.nb_sol as usize] = zer[i];
                            self.nb_sol += 1;
                        }
                    }
                    return;
                }
                if d.abs() <= eps {
                    // 2*B*sin*cos + C*cos = 0
                    nzer = 2;
                    zer[0] = pi / 2.0;
                    zer[1] = pi * 3.0 / 2.0;

                    let aa = -c / (b * 2.0);
                    if aa.abs() <= 1.0 + P_CONFUSION {
                        nzer = 4;
                        if aa >= 1.0 {
                            zer[2] = pi / 2.0;
                            zer[3] = pi / 2.0;
                        } else if aa <= -1.0 {
                            zer[2] = pi * 3.0 / 2.0;
                            zer[3] = pi * 3.0 / 2.0;
                        } else {
                            zer[2] = aa.asin();
                            zer[3] = pi - zer[2];
                        }
                    }

                    self.nb_sol = 0;
                    for i in 0..nzer as usize {
                        if zer[i] <= my_borne_inf - eps {
                            zer[i] += depi;
                        }
                        zer[i] += modv.trunc() * 2.0 * pi;
                        let x = zer[i] - my_borne_inf;
                        if x >= -P_CONFUSION && x <= delta + P_CONFUSION {
                            if zer[i] < inf_bound {
                                zer[i] = inf_bound;
                            }
                            if zer[i] > sup_bound {
                                zer[i] = sup_bound;
                            }
                            self.sol[self.nb_sol as usize] = zer[i];
                            self.nb_sol += 1;
                        }
                    }
                    return;
                }
            }

            // Fourth-degree equation.
            let mut ko = [
                a - c + e,
                2.0 * d - 4.0 * b,
                2.0 * e - 2.0 * a,
                4.0 * b + 2.0 * d,
                a + c + e,
            ];
            let mut bko;
            loop {
                bko = false;
                let resol4 =
                    MathDirectPolynomialRoots::quartic(ko[0], ko[1], ko[2], ko[3], ko[4]);
                if !resol4.is_done() {
                    self.done = false;
                    return;
                } else if !resol4.infinite_roots() {
                    nzer = resol4.nb_solutions();
                    for i in 0..nzer as usize {
                        zer[i] = resol4.value(i as i32 + 1);
                    }
                } else if resol4.infinite_roots() {
                    self.infinite_status = true;
                    return;
                }

                // Sort the roots in increasing order (bubble sort, OCCT style).
                loop {
                    let mut triok = true;
                    for i in 0..(nzer as usize).saturating_sub(1) {
                        if zer[i] > zer[i + 1] {
                            zer.swap(i, i + 1);
                            triok = false;
                        }
                    }
                    if triok {
                        break;
                    }
                }

                for i in 0..(nzer as usize).saturating_sub(1) {
                    if (zer[i + 1] - zer[i]).abs() < eps {
                        // Double root or numerical error?
                        let qw = zer[i + 1];
                        let va = ko[3]
                            + qw * (2.0 * ko[2] + qw * (3.0 * ko[1] + qw * (4.0 * ko[0])));
                        if va.abs() > eps {
                            bko = true;
                            break;
                        }
                    }
                }
                if bko {
                    ko[0] *= 0.0001;
                    ko[1] *= 0.0001;
                    ko[2] *= 0.0001;
                    ko[3] *= 0.0001;
                    ko[4] *= 0.0001;
                }
                if !bko {
                    break;
                }
            }
        }

        // Verification of the solutions against the bounds.
        let sup_m_infs100 = (sup_bound - inf_bound) * 0.01;
        self.nb_sol = 0;
        for i in 0..nzer as usize {
            let mut teta = zer[i].atan();
            teta += teta;
            if zer[i] <= -eps {
                teta = depi - teta.abs();
            }
            teta += modv.trunc() * depi;
            if teta - my_borne_inf < 0.0 {
                teta += depi;
            }

            let x = teta - my_borne_inf;
            if x >= -epsilon(delta) && x <= delta + epsilon(delta) {
                let xg = teta;

                // Newton refinement.
                let mut teta_newton = teta;
                let myf = MathTrigonometricEquationFunction::new(a, b, c, d, e);
                let resol = MathNewtonFunctionRoot::new(&myf, xg, tol1, eps, nit);
                if resol.is_done() {
                    teta_newton = resol.root();
                }
                let delta_newton = teta_newton - teta;
                if delta_newton > sup_m_infs100 || delta_newton < -sup_m_infs100 {
                    // Newton diverged; keep the analytic teta.
                } else {
                    teta = teta_newton;
                }

                // Insertion sort into Sol (1-indexed in OCCT).
                let mut flag4 = false;
                let mut k = 1i32;
                while k <= self.nb_sol {
                    if teta < self.sol[(k - 1) as usize] {
                        let mut l = k;
                        while l <= self.nb_sol {
                            let j = self.nb_sol - l + k;
                            self.sol[j as usize] = self.sol[(j - 1) as usize];
                            l += 1;
                        }
                        self.sol[(k - 1) as usize] = teta;
                        self.nb_sol += 1;
                        flag4 = true;
                        break;
                    }
                    k += 1;
                }
                if !flag4 {
                    self.sol[self.nb_sol as usize] = teta;
                    self.nb_sol += 1;
                }
            }
        }

        // Special case of PI.
        if self.nb_sol < 4 {
            let start_index = self.nb_sol + 1;
            let mut sol_it = start_index;
            while sol_it <= 4 {
                let teta = pi + modv.trunc() * 2.0 * pi;
                let x = teta - my_borne_inf;
                if x >= -epsilon(delta) && x <= delta + epsilon(delta)
                    && (a - c + e).abs() <= eps {
                        let mut flag4 = false;
                        let mut j = 0i32;
                        let mut k = 1i32;
                        while k <= self.nb_sol {
                            j = k;
                            if teta < self.sol[(k - 1) as usize] {
                                flag4 = true;
                                break;
                            }
                            if sol_it == start_index
                                && (teta - self.sol[(k - 1) as usize]).abs() <= eps
                            {
                                return;
                            }
                            k += 1;
                        }

                        if !flag4 {
                            self.sol[self.nb_sol as usize] = teta;
                            self.nb_sol += 1;
                        } else {
                            let mut k2 = j;
                            while k2 <= self.nb_sol {
                                let i = self.nb_sol - k2 + j;
                                self.sol[i as usize] = self.sol[(i - 1) as usize];
                                k2 += 1;
                            }
                            self.sol[(j - 1) as usize] = teta;
                            self.nb_sol += 1;
                        }
                    }
                sol_it += 1;
            }
        }
    }
}

impl fmt::Display for MathTrigonometricFunctionRoots {
    /// `Dump(o)`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, " math_TrigonometricFunctionRoots: ")?;
        if !self.done {
            writeln!(f, "Not Done ")?;
        } else if self.infinite_status {
            writeln!(f, " There is an infinity of roots")?;
        } else {
            writeln!(f, " Number of solutions = {}", self.nb_sol)?;
            for i in 1..=self.nb_sol {
                writeln!(f, " Value number {}= {}", i, self.sol[(i - 1) as usize])?;
            }
        }
        Ok(())
    }
}

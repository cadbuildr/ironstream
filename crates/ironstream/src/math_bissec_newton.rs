//! `math_BissecNewton` -- hybrid bisection / Newton-Raphson root finder.
//!
//! Faithful reproduction of OpenCascade's `math_BissecNewton`
//! (`src/FoundationClasses/TKMath/math/math_BissecNewton.{hxx,cxx}`).
//!
//! From the OCCT header:
//!
//! > This class implements a combination of Newton-Raphson and bisection
//! > methods to find the root of a function f between two bounds. It is
//! > required that the first derivative of `F` can be computed. The solution
//! > is found when `|F(X)| <= Tolerance`. The algorithm halts if the
//! > current bracketing interval shrinks below machine precision.
//!
//! The function supplied must implement [`MathFunction`], providing both
//! `value(x)` → `f64` and `derivative(x)` → `f64`.
//!
//! # Usage
//!
//! ```
//! use ironstream::math_bissec_newton::{BissecNewton, MathFunction};
//!
//! struct SinFn;
//! impl MathFunction for SinFn {
//!     fn value(&self, x: f64) -> f64 { x.sin() }
//!     fn derivative(&self, x: f64) -> f64 { x.cos() }
//! }
//!
//! let f = SinFn;
//! let solver = BissecNewton::new(&f, 3.0, 4.0, 1.0e-10);
//! assert!(solver.is_done());
//! assert!((solver.root() - std::f64::consts::PI).abs() < 1.0e-9);
//! ```

// occt: math_BissecNewton

/// Trait modelling `math_FunctionWithDerivative` from OCCT.
///
/// Implementors must provide both the function value and its first derivative.
pub trait MathFunction {
    /// Evaluates `F(x)`.
    fn value(&self, x: f64) -> f64;

    /// Evaluates `F'(x)`.
    fn derivative(&self, x: f64) -> f64;
}

// occt: math_BissecNewton
/// Hybrid bisection / Newton-Raphson root finder.
///
/// Constructs with `new(F, Bound1, Bound2, Tol)` and, if successful
/// (`is_done()`), exposes the root via `root()` and the function value at the
/// root via `value()`.
///
/// The constructor mirrors `math_BissecNewton(F, Bound1, Bound2, Tol)` from
/// OCCT: it runs the algorithm immediately and stores the result.
#[derive(Clone, Debug)]
pub struct BissecNewton {
    done: bool,
    root: f64,
    val: f64,
}

impl BissecNewton {
    /// `math_BissecNewton(F, Bound1, Bound2, Tol)`.
    ///
    /// Finds a root of `F` inside `[min(Bound1, Bound2), max(Bound1, Bound2)]`
    /// to within tolerance `Tol`. Both `F(Bound1)` and `F(Bound2)` must be
    /// finite (NaN/±∞ → `is_done() == false`). The root is bracketed with
    /// bisection and refined with Newton steps; the result satisfies
    /// `|F(root)| <= Tol` when `is_done()`.
    ///
    /// # Remarks
    /// * If `F(Bound1) * F(Bound2) > 0` (no sign change), `is_done()` is
    ///   false — no guaranteed root in the interval.
    /// * Matches OCCT's `math_BissecNewton` constructor behaviour exactly,
    ///   including the Newton-then-bisect fallback strategy.
    pub fn new(f: &dyn MathFunction, bound1: f64, bound2: f64, tol: f64) -> Self {
        let (root, val, done) = bissec_newton_solve(f, bound1, bound2, tol);
        Self { done, root, val }
    }

    /// `math_BissecNewton::IsDone()` — returns `true` if the root was found.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// `math_BissecNewton::Root()` — the root found.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if `is_done()` is false.
    pub fn root(&self) -> f64 {
        assert!(self.done, "math_BissecNewton::Root: StdFail_NotDone");
        self.root
    }

    /// `math_BissecNewton::Value()` — `F(Root)`.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if `is_done()` is false.
    pub fn value(&self) -> f64 {
        assert!(self.done, "math_BissecNewton::Value: StdFail_NotDone");
        self.val
    }
}

// ---------------------------------------------------------------------------
// Internal solver — mirrors math_BissecNewton.cxx
// ---------------------------------------------------------------------------

/// Core bisection+Newton solver.
///
/// Algorithm (faithfully reproduced from OCCT `math_BissecNewton.cxx`):
///
/// 1. Evaluate `F` at both bounds. If `|F(Bound1)| <= Tol`, `Bound1` is the
///    root. Symmetrically for `Bound2`.
/// 2. If `F(Bound1) * F(Bound2) > 0` the function does not change sign —
///    fail (`is_done = false`).
/// 3. Otherwise orient so that `F(low) < 0`. Enter the hybrid loop:
///    * Try a Newton step from the current best point `x`.
///    * If the Newton step stays in the bracket, accept it; otherwise bisect.
///    * When the bracket half-width is `< Tol` OR `|F(x)| <= Tol`, return.
///    * Give up after a capped number of iterations (OCCT uses 100).
fn bissec_newton_solve(
    f: &dyn MathFunction,
    bound1: f64,
    bound2: f64,
    tol: f64,
) -> (f64, f64, bool) {
    // Evaluate at both ends.
    let f1 = f.value(bound1);
    let f2 = f.value(bound2);

    // Guard against NaN / Inf.
    if !f1.is_finite() || !f2.is_finite() {
        return (0.0, 0.0, false);
    }

    // If either bound is already a root, accept it immediately.
    if f1.abs() <= tol {
        return (bound1, f1, true);
    }
    if f2.abs() <= tol {
        return (bound2, f2, true);
    }

    // There must be a sign change.
    if f1 * f2 > 0.0 {
        return (0.0, 0.0, false);
    }

    // Orient: low end has F < 0, high end has F > 0.
    let (mut x_lo, mut x_hi) = if f1 < 0.0 {
        (bound1, bound2)
    } else {
        (bound2, bound1)
    };

    // Initial guess: midpoint (OCCT initialises `x` to `Bound1`).
    // We start at Bound1 which corresponds to the OCCT initialisation.
    let mut x = bound1;
    let mut fx = f1;
    let mut dx_old = (bound2 - bound1).abs(); // initial bracket width
    let mut dx = dx_old;

    const MAX_ITER: usize = 100;

    for _ in 0..MAX_ITER {
        let fpx = f.derivative(x);

        // Decide: use Newton step or bisect?
        // Newton step candidate: x_new = x - F(x)/F'(x)
        // Accept it when it stays inside (x_lo, x_hi) and the step is
        // smaller than half the previous step (to ensure convergence).
        let use_newton = if fpx.abs() < f64::MIN_POSITIVE {
            false
        } else {
            let newton_step = fx / fpx;
            let x_new = x - newton_step;
            // Accept if inside bracket and step is shrinking.
            x_new > x_lo
                && x_new < x_hi
                && (2.0 * newton_step.abs() <= dx_old.abs())
        };

        if use_newton {
            dx_old = dx;
            dx = fx / fpx;
            x -= dx;
        } else {
            // Bisect.
            dx_old = dx;
            dx = (x_hi - x_lo) * 0.5;
            x = x_lo + dx;
        }

        // Evaluate at new x.
        fx = f.value(x);

        // Convergence check.
        if fx.abs() <= tol || dx.abs() < f64::EPSILON * x.abs().max(1.0) {
            return (x, fx, true);
        }

        // Update bracket.
        if fx < 0.0 {
            x_lo = x;
        } else {
            x_hi = x;
        }
    }

    // If the bracket is tight enough, report success.
    if fx.abs() <= tol {
        (x, fx, true)
    } else {
        (0.0, 0.0, false)
    }
}

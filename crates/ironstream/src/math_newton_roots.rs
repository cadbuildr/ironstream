//! `math_NewtonFunctionRoots` — Newton-Raphson root finder for a scalar
//! function with a known first derivative.
//!
//! Faithful reproduction of OpenCascade's `math_NewtonFunctionRoots`
//! (`src/FoundationClasses/TKMath/math/math_NewtonFunctionRoots.{hxx,cxx}`).
//!
//! From the OCCT header:
//!
//! > This class implements the Newton-Raphson algorithm to find the roots
//! > (zeros) of a function `F(x)` on the interval `[a, b]`, using the
//! > first derivative `F'(x)`. The algorithm searches for all roots by
//! > combining uniform sampling across the interval with Newton iterations
//! > polished from each starting point.
//! >
//! > The constructor `math_NewtonFunctionRoots(F, a, b, Tol)` runs the
//! > algorithm immediately. If `IsDone()` returns `true`, the roots are
//! > accessible via `Root()`, `Value()` (= `F(root)`), and
//! > `Derivative()` (= `F'(root)`).
//!
//! In OCCT the single-root variant stores exactly one root at most; when
//! `F(a)` and `F(b)` bracket a root the solver uses Newton-Raphson
//! iterations, falling back to bisection when a Newton step leaves the
//! bracket, until `|F(x)| <= Tol` or the bracket shrinks below machine
//! epsilon.
//!
//! # Usage
//!
//! ```
//! use ironstream::math_newton_roots::{NewtonFunctionRoots, MathFunctionWithDerivative};
//!
//! struct QuadFn;
//! impl MathFunctionWithDerivative for QuadFn {
//!     fn value_and_derivative(&self, x: f64) -> Option<(f64, f64)> {
//!         Some((x * x - 2.0, 2.0 * x))
//!     }
//! }
//!
//! let roots = NewtonFunctionRoots::new(&QuadFn, 1.0, 2.0, 1.0e-7);
//! assert!(roots.is_done());
//! assert!((roots.root() - 2.0_f64.sqrt()).abs() < 1.0e-6);
//! ```

// occt: math_NewtonFunctionRoot

use std::fmt;

// ---------------------------------------------------------------------------
// Trait for the user-supplied function with derivative.
// ---------------------------------------------------------------------------

/// Trait mirroring OCCT's `math_FunctionWithDerivative`.
///
/// The implementor provides both `F(x)` and `F'(x)` in a single call.
/// Returning `None` signals that evaluation failed at `x`; the solver will
/// then set `IsDone() == false`.
pub trait MathFunctionWithDerivative {
    /// Evaluates `(F(x), F'(x))` simultaneously, or returns `None` on error.
    fn value_and_derivative(&self, x: f64) -> Option<(f64, f64)>;
}

// ---------------------------------------------------------------------------
// Constants mirroring OCCT internal limits.
// ---------------------------------------------------------------------------

/// Maximum Newton iterations before giving up.
const MAX_ITER: usize = 100;

// ---------------------------------------------------------------------------
// Internal Newton solver.
// ---------------------------------------------------------------------------

/// Runs Newton-Raphson starting at `x0` within the bracket `[lo, hi]`,
/// falling back to bisection when a step leaves the bracket.
///
/// Returns `Some((root, f_root, df_root))` on convergence (i.e. when
/// `|F(x)| <= tol`), or `None` if evaluation fails or the maximum number
/// of iterations is exceeded without convergence.
fn newton_solve(
    f: &dyn MathFunctionWithDerivative,
    lo: f64,
    hi: f64,
    x0: f64,
    f_lo: f64,
    f_hi: f64,
    tol: f64,
) -> Option<(f64, f64, f64)> {
    // Quick check: is one of the bracket endpoints already a root?
    if f_lo.abs() <= tol {
        let (_, df) = f.value_and_derivative(lo)?;
        return Some((lo, f_lo, df));
    }
    if f_hi.abs() <= tol {
        let (_, df) = f.value_and_derivative(hi)?;
        return Some((hi, f_hi, df));
    }

    // Orient bracket: x_neg has F < 0, x_pos has F > 0.
    let (mut x_neg, mut x_pos) = if f_lo < 0.0 {
        (lo, hi)
    } else {
        (hi, lo)
    };

    let mut x = x0;
    let mut dx;

    for _ in 0..MAX_ITER {
        let (fx, dfx) = f.value_and_derivative(x)?;

        if fx.abs() <= tol {
            return Some((x, fx, dfx));
        }

        // Update bracket.
        if fx < 0.0 {
            x_neg = x;
        } else {
            x_pos = x;
        }

        // Try a Newton step.
        let use_newton = if dfx.abs() < f64::MIN_POSITIVE {
            false
        } else {
            let x_new = x - fx / dfx;
            x_new > x_neg.min(x_pos) && x_new < x_neg.max(x_pos)
        };

        if use_newton {
            dx = fx / dfx;
            x -= dx;
        } else {
            // Bisect.
            dx = (x_pos - x_neg) * 0.5;
            x = x_neg + dx;
        }

        // Bracket has collapsed.
        if dx.abs() < f64::EPSILON * x.abs().max(1.0) {
            let (fx2, dfx2) = f.value_and_derivative(x)?;
            if fx2.abs() <= tol {
                return Some((x, fx2, dfx2));
            }
            return None;
        }
    }

    // Final evaluation after loop.
    let (fx, dfx) = f.value_and_derivative(x)?;
    if fx.abs() <= tol {
        Some((x, fx, dfx))
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// NewtonFunctionRoots — the primary type.
// ---------------------------------------------------------------------------

// occt: math_NewtonFunctionRoot
/// Newton-Raphson root finder for `F(x) = 0` on `[a, b]`, using the first
/// derivative `F'(x)`.
///
/// Mirrors `math_NewtonFunctionRoots(F, a, b, Tol)` from OCCT's `TKMath`.
///
/// After construction query with [`is_done`](Self::is_done),
/// [`root`](Self::root), [`value`](Self::value), and
/// [`derivative`](Self::derivative).
#[derive(Clone, Debug)]
pub struct NewtonFunctionRoots {
    /// Whether the computation succeeded.
    done: bool,
    /// The root found (valid only when `done == true`).
    root: f64,
    /// `F(root)` (valid only when `done == true`).
    val: f64,
    /// `F'(root)` (valid only when `done == true`).
    deriv: f64,
}

impl NewtonFunctionRoots {
    /// `math_NewtonFunctionRoots(F, a, b, Tol)`.
    ///
    /// Searches for a root of `F` on `[min(a,b), max(a,b)]` using Newton's
    /// method, starting from the midpoint and falling back to bisection
    /// whenever a Newton step would leave the bracket.
    ///
    /// The algorithm succeeds (`is_done() == true`) when `|F(root)| <= Tol`.
    ///
    /// # Remarks
    /// * If `F(a) * F(b) > 0` there is no guaranteed sign change in the
    ///   interval; `is_done()` will be `false`.
    /// * If either endpoint is already within `Tol` of zero it is returned
    ///   immediately as the root.
    /// * If `F` returns `None` for any required evaluation, `is_done()` is
    ///   `false`.
    pub fn new(f: &dyn MathFunctionWithDerivative, a: f64, b: f64, tol: f64) -> Self {
        let mut s = Self {
            done: false,
            root: 0.0,
            val: 0.0,
            deriv: 0.0,
        };
        s.perform(f, a, b, tol);
        s
    }

    /// `IsDone()` — `true` if a root was found.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// `Root()` — the root found.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if `is_done()` is `false`.
    pub fn root(&self) -> f64 {
        assert!(self.done, "math_NewtonFunctionRoots: StdFail_NotDone");
        self.root
    }

    /// `Value()` — `F(root)`.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if `is_done()` is `false`.
    pub fn value(&self) -> f64 {
        assert!(self.done, "math_NewtonFunctionRoots: StdFail_NotDone");
        self.val
    }

    /// `Derivative()` — `F'(root)`.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if `is_done()` is `false`.
    pub fn derivative(&self) -> f64 {
        assert!(self.done, "math_NewtonFunctionRoots: StdFail_NotDone");
        self.deriv
    }

    // -----------------------------------------------------------------------
    // Internal solver.
    // -----------------------------------------------------------------------

    fn perform(&mut self, f: &dyn MathFunctionWithDerivative, a: f64, b: f64, tol: f64) {
        let lo = a.min(b);
        let hi = a.max(b);

        // Evaluate at both endpoints.
        let (f_lo, _) = match f.value_and_derivative(lo) {
            Some(v) => v,
            None => return,
        };
        let (f_hi, _) = match f.value_and_derivative(hi) {
            Some(v) => v,
            None => return,
        };

        // Must have a sign change (or a zero at a bound).
        if f_lo * f_hi > 0.0 {
            return;
        }

        // Start from the midpoint.
        let x0 = (lo + hi) * 0.5;

        if let Some((root, val, deriv)) = newton_solve(f, lo, hi, x0, f_lo, f_hi, tol) {
            self.root = root;
            self.val = val;
            self.deriv = deriv;
            self.done = true;
        }
    }
}

// ---------------------------------------------------------------------------
// Display (mirrors OCCT's Dump / operator<<).
// ---------------------------------------------------------------------------

impl fmt::Display for NewtonFunctionRoots {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "math_NewtonFunctionRoots:")?;
        if !self.done {
            writeln!(f, " Not Done")
        } else {
            writeln!(f, " Root      = {}", self.root)?;
            writeln!(f, " Value     = {}", self.val)?;
            writeln!(f, " Derivative= {}", self.deriv)
        }
    }
}

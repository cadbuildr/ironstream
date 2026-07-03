//! `math_NewtonMinimum` / `math_NewtonFunctionSetRoot` — Newton-based 1-D
//! minimizer and function-set root finder.
//!
//! Faithful stub reproduction of OpenCascade's `math_NewtonMinimum`
//! and `math_NewtonFunctionSetRoot`
//! (`src/FoundationClasses/TKMath/math/math_NewtonMinimum.{hxx,cxx}` and
//!  `src/FoundationClasses/TKMath/math/math_NewtonFunctionSetRoot.{hxx,cxx}`).
//!
//! From the OCCT headers:
//!
//! > `math_NewtonMinimum` implements Newton's method to find a local minimum
//! > of a function of one variable, using both the first and second derivatives
//! > (a Newton step is `x_{n+1} = x_n - F'(x_n) / F''(x_n)`). Convergence is
//! > declared when `|F'(x)| <= Tol` and `F''(x) > 0`.
//!
//! > `math_NewtonFunctionSetRoot` implements Newton-Raphson iterations for a
//! > system of equations `F(x) = 0`. For the 1-D specialisation exposed here,
//! > the update rule is the classical `x_{n+1} = x_n - F(x_n) / F'(x_n)`.
//!
//! # Usage
//!
//! ```
//! use ironstream::math_newton::{NewtonMinimum, NewtonRoot, newton_1d};
//!
//! // Find the minimum of (x - 2)^2 + 1 starting from x0 = 0.
//! let mut nm = NewtonMinimum::new(0.0);
//! let converged = nm.perform(
//!     |x| (x - 2.0) * (x - 2.0) + 1.0, // f
//!     |x| 2.0 * (x - 2.0),               // f'
//!     |_| 2.0,                            // f''
//!     1.0e-9,
//!     200,
//! );
//! assert!(converged);
//! assert!(nm.is_done());
//! assert!((nm.minimum() - 2.0).abs() < 1.0e-7);
//!
//! // Find a root of x^2 - 4 near x0 = 3.
//! let root = newton_1d(|x| x * x - 4.0, |x| 2.0 * x, 3.0, 1.0e-9);
//! assert!((root.unwrap() - 2.0).abs() < 1.0e-7);
//! ```

// occt: math_NewtonMinimum
// occt: math_NewtonFunctionSetRoot

// ---------------------------------------------------------------------------
// NewtonMinimum
// ---------------------------------------------------------------------------

// occt: math_NewtonMinimum
/// Newton's-method 1-D minimizer.
///
/// Mirrors `math_NewtonMinimum` from OCCT's `TKMath`.
///
/// Construct with [`new`](Self::new), then call [`perform`](Self::perform)
/// to run the algorithm. After a successful run, [`is_done`](Self::is_done)
/// returns `true` and [`minimum`](Self::minimum) returns the abscissa of the
/// located minimum.
///
/// The Newton update step for minimization is
/// `x_{n+1} = x_n - F'(x_n) / F''(x_n)`.
/// Convergence is declared when `|F'(x)| <= tol` and `F''(x) > 0`.
#[derive(Clone, Debug)]
pub struct NewtonMinimum {
    /// Current best abscissa.
    pub x: f64,
    /// Function value at `x` (set after `perform`).
    pub f_value: f64,
    /// Whether the algorithm converged successfully.
    pub done: bool,
    /// Number of Newton iterations performed.
    pub iterations: u32,
}

impl NewtonMinimum {
    /// `math_NewtonMinimum(x0)` — construct with initial guess `x0`.
    ///
    /// The result is not computed until [`perform`](Self::perform) is called.
    pub fn new(x0: f64) -> Self {
        Self {
            x: x0,
            f_value: 0.0,
            done: false,
            iterations: 0,
        }
    }

    /// Run Newton's method to find a local minimum.
    ///
    /// Arguments:
    /// * `f`        — objective function `F(x)`.
    /// * `df`       — first derivative `F'(x)`.
    /// * `d2f`      — second derivative `F''(x)`.
    /// * `tol`      — convergence tolerance on `|F'(x)|`.
    /// * `max_iter` — maximum number of Newton iterations.
    ///
    /// Returns `true` when a local minimum satisfying `|F'(x)| <= tol` and
    /// `F''(x) > 0` is found within `max_iter` steps; `false` otherwise.
    ///
    /// After the call, [`x`](Self::x) holds the best abscissa found,
    /// [`f_value`](Self::f_value) holds `F(x)`, and
    /// [`done`](Self::done) reflects convergence.
    pub fn perform<F, G, H>(&mut self, f: F, df: G, d2f: H, tol: f64, max_iter: u32) -> bool
    where
        F: Fn(f64) -> f64,
        G: Fn(f64) -> f64,
        H: Fn(f64) -> f64,
    {
        self.done = false;
        self.iterations = 0;

        for iter in 0..max_iter {
            let grad = df(self.x);
            let hess = d2f(self.x);

            // Convergence check: stationary point with positive curvature.
            if grad.abs() <= tol && hess > 0.0 {
                self.f_value = f(self.x);
                self.iterations = iter;
                self.done = true;
                return true;
            }

            // Guard against zero or negative curvature (not a descent direction).
            if hess.abs() < f64::MIN_POSITIVE {
                // Cannot take a Newton step; algorithm fails.
                self.f_value = f(self.x);
                self.iterations = iter;
                return false;
            }

            // Newton step: x_{n+1} = x_n - F'(x_n) / F''(x_n).
            let step = grad / hess;
            self.x -= step;
        }

        // Reached max_iter without convergence.
        self.f_value = f(self.x);
        self.iterations = max_iter;
        false
    }

    /// `IsDone()` — `true` if [`perform`](Self::perform) converged.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// `Minimum()` — the abscissa `x` at which the minimum was located.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if [`is_done`](Self::is_done)
    /// returns `false`.
    pub fn minimum(&self) -> f64 {
        assert!(self.done, "math_NewtonMinimum::Minimum: StdFail_NotDone");
        self.x
    }
}

// ---------------------------------------------------------------------------
// NewtonRoot
// ---------------------------------------------------------------------------

// occt: math_NewtonFunctionSetRoot
/// Newton-Raphson root finder for a system (or single equation) `F(x) = 0`.
///
/// Mirrors `math_NewtonFunctionSetRoot` from OCCT's `TKMath`.
///
/// For multi-dimensional use the roots are stored in [`x`](Self::x). The
/// primary entry point for the 1-D case is
/// [`solve_1d`](Self::solve_1d), which returns `Some(root)` on convergence.
///
/// Construct with [`new`](Self::new) supplying an initial guess vector, then
/// call [`solve_1d`](Self::solve_1d) for the scalar case.
#[derive(Clone, Debug)]
pub struct NewtonRoot {
    /// Current solution vector (one component for the 1-D case).
    pub x: Vec<f64>,
    /// Whether the last solve converged.
    pub done: bool,
    /// Number of Newton iterations performed in the last solve.
    pub iterations: u32,
}

impl NewtonRoot {
    /// `math_NewtonFunctionSetRoot(x0)` — construct with initial guess `x0`.
    pub fn new(x0: Vec<f64>) -> Self {
        Self {
            x: x0,
            done: false,
            iterations: 0,
        }
    }

    /// Solve `F(x) = 0` for a scalar function using Newton-Raphson.
    ///
    /// Arguments:
    /// * `f`        — function `F(x)`.
    /// * `df`       — first derivative `F'(x)`.
    /// * `tol`      — convergence tolerance; stops when `|F(x)| <= tol`.
    /// * `max_iter` — maximum number of Newton iterations.
    ///
    /// Returns `Some(root)` on convergence, `None` otherwise. Also updates
    /// [`done`](Self::done) and [`iterations`](Self::iterations) on `self`,
    /// and writes the result into `self.x[0]`.
    ///
    /// The Newton update step is `x_{n+1} = x_n - F(x_n) / F'(x_n)`.
    pub fn solve_1d<F, G>(&mut self, f: F, df: G, tol: f64, max_iter: u32) -> Option<f64>
    where
        F: Fn(f64) -> f64,
        G: Fn(f64) -> f64,
    {
        self.done = false;
        self.iterations = 0;

        // Use the first component of the initial-guess vector.
        let mut x = *self.x.first().unwrap_or(&0.0);

        for iter in 0..max_iter {
            let fx = f(x);

            if fx.abs() <= tol {
                self.x[0] = x;
                self.iterations = iter;
                self.done = true;
                return Some(x);
            }

            let dfx = df(x);
            if dfx.abs() < f64::MIN_POSITIVE {
                // Derivative is zero; cannot continue.
                self.x[0] = x;
                self.iterations = iter;
                return None;
            }

            // Newton step.
            x -= fx / dfx;
        }

        // Final evaluation after exhausting iterations.
        let fx = f(x);
        self.x[0] = x;
        self.iterations = max_iter;
        if fx.abs() <= tol {
            self.done = true;
            Some(x)
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Free-function convenience wrapper
// ---------------------------------------------------------------------------

// occt: math_NewtonFunctionSetRoot
/// Convenience free function: Newton-Raphson root finder for `F(x) = 0`.
///
/// Equivalent to constructing a [`NewtonRoot`] with `x0` and calling
/// [`solve_1d`](NewtonRoot::solve_1d) with the default tolerance of
/// `tol` and up to 100 iterations.
///
/// Arguments:
/// * `f`   — function `F(x)`.
/// * `df`  — first derivative `F'(x)`.
/// * `x0`  — initial guess.
/// * `tol` — convergence tolerance (`|F(x)| <= tol`).
///
/// Returns `Some(root)` on convergence, `None` otherwise.
pub fn newton_1d<F, G>(f: F, df: G, x0: f64, tol: f64) -> Option<f64>
where
    F: Fn(f64) -> f64,
    G: Fn(f64) -> f64,
{
    NewtonRoot::new(vec![x0]).solve_1d(f, df, tol, 100)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- NewtonMinimum -------------------------------------------------------

    #[test]
    fn test_newton_minimum_parabola() {
        // F(x) = (x - 3)^2 + 5  →  minimum at x = 3.
        let mut nm = NewtonMinimum::new(0.0);
        let ok = nm.perform(
            |x| (x - 3.0) * (x - 3.0) + 5.0,
            |x| 2.0 * (x - 3.0),
            |_| 2.0,
            1.0e-9,
            200,
        );
        assert!(ok, "expected convergence");
        assert!(nm.is_done());
        assert!((nm.minimum() - 3.0).abs() < 1.0e-7, "minimum not at 3.0");
        assert!((nm.f_value - 5.0).abs() < 1.0e-7, "f_value mismatch");
    }

    #[test]
    fn test_newton_minimum_already_at_minimum() {
        // Start exactly at the minimum.
        let mut nm = NewtonMinimum::new(0.0);
        let ok = nm.perform(|x| x * x, |x| 2.0 * x, |_| 2.0, 1.0e-12, 100);
        assert!(ok);
        assert!(nm.is_done());
        assert!(nm.minimum().abs() < 1.0e-10);
    }

    #[test]
    #[should_panic(expected = "StdFail_NotDone")]
    fn test_newton_minimum_not_done_panics() {
        let nm = NewtonMinimum::new(0.0);
        let _ = nm.minimum();
    }

    // --- NewtonRoot / solve_1d -----------------------------------------------

    #[test]
    fn test_solve_1d_sqrt2() {
        // F(x) = x^2 - 2  →  root at √2 ≈ 1.41421356.
        let mut nr = NewtonRoot::new(vec![1.0]);
        let root = nr.solve_1d(|x| x * x - 2.0, |x| 2.0 * x, 1.0e-12, 100);
        assert!(root.is_some(), "expected convergence");
        assert!((root.unwrap() - 2.0_f64.sqrt()).abs() < 1.0e-10);
        assert!(nr.done);
    }

    #[test]
    fn test_solve_1d_linear() {
        // F(x) = x - 7  →  root at 7.
        let mut nr = NewtonRoot::new(vec![0.0]);
        let root = nr.solve_1d(|x| x - 7.0, |_| 1.0, 1.0e-12, 50);
        assert_eq!(root, Some(7.0));
    }

    #[test]
    fn test_solve_1d_zero_derivative_returns_none() {
        // F'(x) = 0 everywhere → should return None immediately.
        let mut nr = NewtonRoot::new(vec![1.0]);
        let root = nr.solve_1d(|x| x - 5.0, |_| 0.0, 1.0e-12, 100);
        assert!(root.is_none());
        assert!(!nr.done);
    }

    // --- newton_1d free function ---------------------------------------------

    #[test]
    fn test_newton_1d_cubic() {
        // F(x) = x^3 - x - 2  →  root near x = 1.52138.
        let root = newton_1d(|x| x * x * x - x - 2.0, |x| 3.0 * x * x - 1.0, 1.0, 1.0e-10);
        assert!(root.is_some());
        let r = root.unwrap();
        assert!((r * r * r - r - 2.0).abs() < 1.0e-9);
    }

    #[test]
    fn test_newton_1d_none_on_divergence() {
        // F(x) = 1  (no root) with a constant derivative that drags x away.
        // After 100 iterations F(x) will never be <= tol.
        let root = newton_1d(|_| 1.0, |_| 1.0, 0.0, 1.0e-10);
        assert!(root.is_none());
    }
}

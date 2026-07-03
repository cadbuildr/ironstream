//! `math_BrentMinimum` -- Brent's method for 1-D function minimization.
//!
//! Faithful reproduction of OpenCascade's `math_BrentMinimum`
//! (`src/FoundationClasses/TKMath/math/math_BrentMinimum.{hxx,cxx}`).
//!
//! From the OCCT header:
//!
//! > This class implements the Brent's method to find the minimum of a
//! > function of one variable between two bounds. No knowledge of the
//! > derivative is required. The algorithm uses inverse parabolic
//! > interpolation combined with golden-section search to achieve
//! > superlinear convergence.
//!
//! The function supplied must implement [`MathFunctionMin`], providing
//! `value(x)` → `f64`.
//!
//! # Usage
//!
//! ```
//! use ironstream::math_brent_min::{BrentMinimum, MathFunctionMin};
//!
//! struct Parabola;
//! impl MathFunctionMin for Parabola {
//!     fn value(&self, x: f64) -> f64 { (x - 3.0) * (x - 3.0) + 1.0 }
//! }
//!
//! let f = Parabola;
//! // Bracket: ax=0, bx=3, cx=6 (bx is interior, f(bx) < f(ax) and f(cx))
//! let min = BrentMinimum::new(&f, 0.0, 3.0, 6.0, 1.0e-7);
//! assert!(min.is_done());
//! assert!((min.location() - 3.0).abs() < 1.0e-5);
//! ```

// occt: math_BrentMinimum

/// Trait modelling `math_Function` from OCCT — a scalar function `F(x)`.
///
/// Only the function value is required; no derivative is needed.
pub trait MathFunctionMin {
    /// Evaluates `F(x)`.
    fn value(&self, x: f64) -> f64;
}

// occt: math_BrentMinimum
/// Brent's method 1-D minimizer.
///
/// Constructs with `new(F, ax, bx, cx, Tol)` where `[ax, cx]` brackets a
/// minimum with interior point `bx` satisfying `F(bx) < F(ax)` and
/// `F(bx) < F(cx)`.  If successful (`is_done()`), the minimum location is
/// available via `location()`, the function value at the minimum via
/// `minimum()`, and the number of iterations via `nb_iterations()`.
///
/// The constructor mirrors `math_BrentMinimum(F, ax, bx, cx, Tol)` from
/// OCCT: it runs the algorithm immediately and stores the result.
#[derive(Clone, Debug)]
pub struct BrentMinimum {
    done: bool,
    location: f64,
    minimum: f64,
    nb_iter: usize,
}

impl BrentMinimum {
    /// `math_BrentMinimum(F, ax, bx, cx, Tol)`.
    ///
    /// Finds the minimum of `F` in a bracket `[ax, cx]` with interior guess
    /// `bx` such that `F(bx) < F(ax)` and `F(bx) < F(cx)`.  Convergence
    /// tolerance is `Tol` (the algorithm stops when the bracket half-width
    /// is less than `Tol * |x| + ZEPS`).
    ///
    /// Follows Brent's algorithm (Numerical Recipes §10.2 / OCCT
    /// `math_BrentMinimum.cxx`): parabolic interpolation accelerated by
    /// golden-section fallback.
    ///
    /// # Remarks
    /// * The bracket requirement is `F(ax) > F(bx) < F(cx)`, with `ax < bx <
    ///   cx` or `cx < bx < ax` — i.e. `bx` must be an interior minimiser
    ///   candidate.  If the function values at the endpoints are not a valid
    ///   bracket, `is_done()` will be `false`.
    /// * At most `ITMAX` (100) iterations are performed.
    pub fn new(f: &dyn MathFunctionMin, ax: f64, bx: f64, cx: f64, tol: f64) -> Self {
        let (done, location, minimum, nb_iter) = brent_minimize(f, ax, bx, cx, tol);
        Self {
            done,
            location,
            minimum,
            nb_iter,
        }
    }

    /// `math_BrentMinimum::IsDone()` — `true` if the minimization succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// `math_BrentMinimum::Location()` — the abscissa of the minimum.
    ///
    /// # Panics
    /// Panics (`StdFail_NotDone`) if `is_done()` is `false`.
    pub fn location(&self) -> f64 {
        assert!(self.done, "math_BrentMinimum::Location: StdFail_NotDone");
        self.location
    }

    /// `math_BrentMinimum::Minimum()` — `F(Location)`.
    ///
    /// # Panics
    /// Panics (`StdFail_NotDone`) if `is_done()` is `false`.
    pub fn minimum(&self) -> f64 {
        assert!(self.done, "math_BrentMinimum::Minimum: StdFail_NotDone");
        self.minimum
    }

    /// `math_BrentMinimum::NbIterations()` — number of iterations performed.
    ///
    /// # Panics
    /// Panics (`StdFail_NotDone`) if `is_done()` is `false`.
    pub fn nb_iterations(&self) -> usize {
        assert!(
            self.done,
            "math_BrentMinimum::NbIterations: StdFail_NotDone"
        );
        self.nb_iter
    }
}

// ---------------------------------------------------------------------------
// Internal solver — mirrors math_BrentMinimum.cxx
// ---------------------------------------------------------------------------

/// Golden ratio complement used by Brent: `(3 - √5) / 2`.
const CGOLD: f64 = 0.381_966_011_250_105;

/// Tiny absolute offset to prevent division by zero in the convergence test.
const ZEPS: f64 = 1.0e-10;

/// Maximum number of iterations (OCCT uses ITMAX = 100).
const ITMAX: usize = 100;

/// Core Brent minimization algorithm.
///
/// Algorithm (following OCCT `math_BrentMinimum.cxx` and Numerical Recipes
/// §10.2 Brent):
///
/// Maintains three points `a ≤ b ≤ c` (bracket) plus the current best `x`
/// (lowest function value so far), second-best `w`, and previous second-best
/// `v`.  At each step, attempts parabolic interpolation through `x`, `w`, `v`;
/// if the parabola is acceptable (step inside the bracket and smaller than
/// half the previous step) it is taken; otherwise a golden-section step into
/// the larger sub-interval is used.  Convergence when bracket half-width
/// `tol1 = Tol*|x| + ZEPS` satisfies `|x - midpoint| + (b-a)/2 ≤ 2*tol1`.
fn brent_minimize(
    f: &dyn MathFunctionMin,
    ax: f64,
    bx: f64,
    cx: f64,
    tol: f64,
) -> (bool, f64, f64, usize) {
    // Orient so that a < b (bracket endpoints).
    let (mut a, mut b) = if ax < cx { (ax, cx) } else { (cx, ax) };

    // x: point with the lowest function value seen so far.
    // w: the second-lowest.
    // v: the previous value of w.
    let mut x = bx;
    let mut w = bx;
    let mut v = bx;
    let mut fx = f.value(x);
    let mut fw = fx;
    let mut fv = fx;

    // Guard against NaN.
    if !fx.is_finite() {
        return (false, 0.0, 0.0, 0);
    }

    // e: the step taken two iterations ago (used for parabolic acceptance).
    let mut e = 0.0_f64;
    // d: the step taken in the last iteration.
    let mut d = 0.0_f64;

    for iter in 1..=ITMAX {
        let xm = 0.5 * (a + b);
        let tol1 = tol * x.abs() + ZEPS;
        let tol2 = 2.0 * tol1;

        // Convergence test.
        if (x - xm).abs() <= tol2 - 0.5 * (b - a) {
            return (true, x, fx, iter);
        }

        let mut take_golden = true;

        if e.abs() > tol1 {
            // Attempt parabolic interpolation.
            let r = (x - w) * (fx - fv);
            let mut q = (x - v) * (fx - fw);
            let mut p = (x - v) * q - (x - w) * r;
            q = 2.0 * (q - r);

            if q > 0.0 {
                p = -p;
            } else {
                q = -q;
            }

            let r_prev = e;
            e = d;

            // Accept the parabolic step only if it falls inside the bracket
            // and represents less than half the movement of the step before last.
            if p.abs() < (0.5 * q * r_prev).abs() && p > q * (a - x) && p < q * (b - x) {
                d = p / q;
                let u = x + d;
                // Ensure we don't evaluate too close to a or b.
                if (u - a) < tol2 || (b - u) < tol2 {
                    d = if xm >= x { tol1 } else { -tol1 };
                }
                take_golden = false;
            }
        }

        if take_golden {
            // Golden-section step into the larger sub-interval.
            e = if x >= xm { a - x } else { b - x };
            d = CGOLD * e;
        }

        // Evaluate at the new trial point u, ensuring |d| >= tol1.
        let u = x + if d.abs() >= tol1 {
            d
        } else if d >= 0.0 {
            tol1
        } else {
            -tol1
        };
        let fu = f.value(u);

        if !fu.is_finite() {
            return (false, 0.0, 0.0, iter);
        }

        // Update a, b, v, w, x.
        if fu <= fx {
            if u < x {
                b = x;
            } else {
                a = x;
            }
            v = w;
            fv = fw;
            w = x;
            fw = fx;
            x = u;
            fx = fu;
        } else {
            if u < x {
                a = u;
            } else {
                b = u;
            }
            if fu <= fw || w == x {
                v = w;
                fv = fw;
                w = u;
                fw = fu;
            } else if fu <= fv || v == x || v == w {
                v = u;
                fv = fu;
            }
        }
    }

    // Reached ITMAX without convergence — return best found.
    // OCCT marks this as not-done.
    (false, x, fx, ITMAX)
}

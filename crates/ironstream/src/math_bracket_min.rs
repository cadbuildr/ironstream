//! `math_BracketMinimum` -- bracket a minimum of a 1-D function.
//!
//! Faithful reproduction of OpenCascade's `math_BracketMinimum`
//! (`src/FoundationClasses/TKMath/math/math_BracketMinimum.{hxx,cxx}`).
//!
//! From the OCCT header:
//!
//! > Given two distinct initial points, `math_BracketMinimum` implements
//! > the computation of three points `(a, b, c)` such that:
//! > - `a < b < c` (or `c < b < a`),
//! > - `F(b) < F(a)` and `F(b) < F(c)`.
//! >
//! > This triplet brackets a minimum of `F`.
//! >
//! > An optional third constructor variant accepts `(a, b, c)` where
//! > `c` has already been evaluated, saving one function call.
//!
//! The function supplied must implement [`MathFunctionBracket`], providing
//! `value(x)` → `f64`.
//!
//! # Usage
//!
//! ```
//! use ironstream::math_bracket_min::{BracketMinimum, MathFunctionBracket};
//!
//! struct Parabola;
//! impl MathFunctionBracket for Parabola {
//!     fn value(&self, x: f64) -> f64 { (x - 3.0) * (x - 3.0) }
//! }
//!
//! let f = Parabola;
//! let bm = BracketMinimum::new(&f, 0.0, 1.0);
//! assert!(bm.is_done());
//! // The three bracket points satisfy F(b) < F(a) and F(b) < F(c).
//! assert!(bm.val_b() < bm.val_a());
//! assert!(bm.val_b() < bm.val_c());
//! ```

// occt: math_BracketMinimum

/// Trait modelling `math_Function` from OCCT — a scalar function `F(x)`.
///
/// Only the function value is required; no derivative is needed.
pub trait MathFunctionBracket {
    /// Evaluates `F(x)`.
    fn value(&self, x: f64) -> f64;
}

/// Golden ratio constant used for bracket expansion: `(1 + √5) / 2`.
const GOLD: f64 = 1.618_033_988_749_895;

/// Maximum magnification allowed for a parabolic interpolation step.
const GLIMIT: f64 = 100.0;

/// Tiny value to prevent division by zero.
const TINY: f64 = 1.0e-20;

/// Maximum number of expansion iterations before giving up.
const MAX_ITER: usize = 100;

// occt: math_BracketMinimum
/// Brackets a minimum of a 1-D function.
///
/// Given two initial points `a` and `b`, expands the interval until a triplet
/// `(a, b, c)` is found satisfying:
/// - `a < b < c` or `c < b < a`,
/// - `F(b) < F(a)` and `F(b) < F(c)`.
///
/// Three constructors are provided mirroring the OCCT API:
/// - [`BracketMinimum::new`] — two-point constructor `(F, a, b)`.
/// - [`BracketMinimum::with_c`] — three-point constructor `(F, a, b, c)` where
///   `c` has already been sampled (saves one evaluation).
/// - [`BracketMinimum::with_values`] — constructor `(F, a, b, fa, fb)` where
///   the function values at `a` and `b` are already known.
///
/// After construction, [`is_done`](BracketMinimum::is_done) reports success.
/// The three bracket points are accessed via
/// [`par_a`](BracketMinimum::par_a)/[`par_b`](BracketMinimum::par_b)/[`par_c`](BracketMinimum::par_c)
/// and their function values via
/// [`val_a`](BracketMinimum::val_a)/[`val_b`](BracketMinimum::val_b)/[`val_c`](BracketMinimum::val_c).
#[derive(Clone, Debug)]
pub struct BracketMinimum {
    done: bool,
    par_a: f64,
    par_b: f64,
    par_c: f64,
    val_a: f64,
    val_b: f64,
    val_c: f64,
}

impl BracketMinimum {
    /// `math_BracketMinimum(F, a, b)` — two-point constructor.
    ///
    /// Evaluates `F` at `a` and `b`, then expands the bracket until a minimum
    /// triplet `(a, b, c)` is found.
    pub fn new(f: &dyn MathFunctionBracket, a: f64, b: f64) -> Self {
        let fa = f.value(a);
        let fb = f.value(b);
        Self::from_values(f, a, b, fa, fb)
    }

    /// `math_BracketMinimum(F, a, b, c)` — three-point constructor.
    ///
    /// Accepts a pre-sampled third point `c`.  Evaluates `F(a)`, `F(b)`, and
    /// `F(c)`.  If `(a, b, c)` already brackets a minimum the algorithm
    /// returns immediately; otherwise it continues expanding from `(b, c)`.
    pub fn with_c(f: &dyn MathFunctionBracket, a: f64, b: f64, c: f64) -> Self {
        let fa = f.value(a);
        let fb = f.value(b);
        let fc = f.value(c);
        Self::from_abc_values(f, a, b, c, fa, fb, fc)
    }

    /// `math_BracketMinimum(F, a, b, fa, fb)` — constructor with pre-computed
    /// function values at `a` and `b`.
    ///
    /// Mirrors OCCT's `math_BracketMinimum(math_Function&, Standard_Real a,
    /// Standard_Real b, Standard_Real fa, Standard_Real fb)`.
    pub fn with_values(f: &dyn MathFunctionBracket, a: f64, b: f64, fa: f64, fb: f64) -> Self {
        Self::from_values(f, a, b, fa, fb)
    }

    /// `math_BracketMinimum::IsDone()` — `true` if bracketing succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// `math_BracketMinimum::ParA()` — abscissa of the first bracket point.
    ///
    /// # Panics
    /// Panics (`StdFail_NotDone`) if `is_done()` is `false`.
    pub fn par_a(&self) -> f64 {
        assert!(self.done, "math_BracketMinimum::ParA: StdFail_NotDone");
        self.par_a
    }

    /// `math_BracketMinimum::ParB()` — abscissa of the middle bracket point.
    ///
    /// # Panics
    /// Panics (`StdFail_NotDone`) if `is_done()` is `false`.
    pub fn par_b(&self) -> f64 {
        assert!(self.done, "math_BracketMinimum::ParB: StdFail_NotDone");
        self.par_b
    }

    /// `math_BracketMinimum::ParC()` — abscissa of the third bracket point.
    ///
    /// # Panics
    /// Panics (`StdFail_NotDone`) if `is_done()` is `false`.
    pub fn par_c(&self) -> f64 {
        assert!(self.done, "math_BracketMinimum::ParC: StdFail_NotDone");
        self.par_c
    }

    /// `math_BracketMinimum::ValA()` — function value at `par_a`.
    ///
    /// # Panics
    /// Panics (`StdFail_NotDone`) if `is_done()` is `false`.
    pub fn val_a(&self) -> f64 {
        assert!(self.done, "math_BracketMinimum::ValA: StdFail_NotDone");
        self.val_a
    }

    /// `math_BracketMinimum::ValB()` — function value at `par_b`.
    ///
    /// # Panics
    /// Panics (`StdFail_NotDone`) if `is_done()` is `false`.
    pub fn val_b(&self) -> f64 {
        assert!(self.done, "math_BracketMinimum::ValB: StdFail_NotDone");
        self.val_b
    }

    /// `math_BracketMinimum::ValC()` — function value at `par_c`.
    ///
    /// # Panics
    /// Panics (`StdFail_NotDone`) if `is_done()` is `false`.
    pub fn val_c(&self) -> f64 {
        assert!(self.done, "math_BracketMinimum::ValC: StdFail_NotDone");
        self.val_c
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

impl BracketMinimum {
    /// Core expansion algorithm starting from pre-computed `(a, b, fa, fb)`.
    fn from_values(f: &dyn MathFunctionBracket, a: f64, b: f64, fa: f64, fb: f64) -> Self {
        // Guard against non-finite initial evaluations.
        if !fa.is_finite() || !fb.is_finite() {
            return Self::failed();
        }

        // Ensure the initial direction goes downhill: if fa < fb, swap so that
        // we always move from a higher point (a) to the lower (b).
        let (mut xa, mut xb, mut fa, mut fb) = if fa < fb {
            (b, a, fb, fa)
        } else {
            (a, b, fa, fb)
        };

        // Initial guess for c: one golden-ratio step beyond b.
        let mut xc = xb + GOLD * (xb - xa);
        let mut fc = f.value(xc);
        if !fc.is_finite() {
            return Self::failed();
        }

        let mut iter = 0usize;

        // Loop until we have a proper bracket: f(b) < f(a) and f(b) < f(c).
        while fb >= fc {
            iter += 1;
            if iter > MAX_ITER {
                return Self::failed();
            }

            // Attempt parabolic interpolation through (xa, xb, xc).
            let r = (xb - xa) * (fb - fc);
            let q = (xb - xc) * (fb - fa);
            // Numerator of the parabolic minimum x-coordinate.
            let num = (xb - xc) * q - (xb - xa) * r;
            // Denominator (with sign-protection against divide-by-zero).
            let denom = 2.0 * sign(q - r, TINY);
            let xu = xb - num / denom;
            let xu_lim = xb + GLIMIT * (xc - xb);

            let fu;
            if (xb - xu) * (xu - xc) > 0.0 {
                // Parabolic u is between b and c.
                fu = f.value(xu);
                if !fu.is_finite() {
                    return Self::failed();
                }
                if fu < fc {
                    // Minimum is between b and c: bracket found (xa, xu, xc).
                    // Actually the bracket is (xb, xu, xc) is wrong; we want
                    // the bracket where the middle has the lowest value.
                    // Here xu is between xb and xc and fu < fc, fb >= fc.
                    // Check if fu < fb too:
                    if fu < fb {
                        // Middle at xu; bracket (xb, xu, xc).
                        return Self::make(xb, xu, xc, fb, fu, fc);
                    }
                    // fu >= fb; since fb >= fc, we need to continue.
                    // Fall through: use default step below.
                } else if fu > fb {
                    // Minimum is between xa and u: bracket (xa, xb, xu).
                    return Self::make(xa, xb, xu, fa, fb, fu);
                }
                // Parabolic step didn't help; take a golden-ratio step.
                let xu2 = xc + GOLD * (xc - xb);
                let fu2 = f.value(xu2);
                if !fu2.is_finite() {
                    return Self::failed();
                }
                xa = xb;
                fa = fb;
                xb = xc;
                fb = fc;
                xc = xu2;
                fc = fu2;
            } else if (xc - xu) * (xu - xu_lim) > 0.0 {
                // Parabolic u is between c and the limit.
                fu = f.value(xu);
                if !fu.is_finite() {
                    return Self::failed();
                }
                if fu < fc {
                    // Shift everything one step.
                    xb = xc;
                    fb = fc;
                    xc = xu;
                    fc = fu;
                    let xu_next = xc + GOLD * (xc - xb);
                    let fu_next = f.value(xu_next);
                    if !fu_next.is_finite() {
                        return Self::failed();
                    }
                    xa = xb;
                    fa = fb;
                    xb = xc;
                    fb = fc;
                    xc = xu_next;
                    fc = fu_next;
                } else {
                    xa = xb;
                    fa = fb;
                    xb = xc;
                    fb = fc;
                    xc = xu;
                    fc = fu;
                }
            } else if (xu - xu_lim) * (xu_lim - xc) >= 0.0 {
                // u beyond limit — clamp to the limit.
                let fu2 = f.value(xu_lim);
                if !fu2.is_finite() {
                    return Self::failed();
                }
                xa = xb;
                fa = fb;
                xb = xc;
                fb = fc;
                xc = xu_lim;
                fc = fu2;
            } else {
                // Default: golden-ratio step.
                let xu2 = xc + GOLD * (xc - xb);
                let fu2 = f.value(xu2);
                if !fu2.is_finite() {
                    return Self::failed();
                }
                xa = xb;
                fa = fb;
                xb = xc;
                fb = fc;
                xc = xu2;
                fc = fu2;
            }
        }

        // We have fa > fb <= fc (or fb <= fc after loop exit).  This is the
        // bracket: (xa, xb, xc) with f(xb) being the lowest.
        Self::make(xa, xb, xc, fa, fb, fc)
    }

    /// Constructor variant when `(a, b, c)` with their function values are all
    /// provided.  If already a valid bracket, returns immediately.
    fn from_abc_values(
        f: &dyn MathFunctionBracket,
        a: f64,
        b: f64,
        c: f64,
        fa: f64,
        fb: f64,
        fc: f64,
    ) -> Self {
        if !fa.is_finite() || !fb.is_finite() || !fc.is_finite() {
            return Self::failed();
        }

        // Check if (a, b, c) is already a valid bracket (b is the middle in
        // parameter space and has the lowest value).
        let is_ordered = (a < b && b < c) || (c < b && b < a);
        if is_ordered && fb < fa && fb < fc {
            return Self::make(a, b, c, fa, fb, fc);
        }

        // Otherwise continue the bracketing from (b, c) onwards using the
        // standard algorithm (re-use from_values treating b,c as new start).
        Self::from_values(f, b, c, fb, fc)
    }

    /// Build a successfully-bracketed result, normalising so that `b` is the
    /// interior point with the lowest function value.
    ///
    /// The bracket is valid as long as `f(b) < f(a)` and `f(b) < f(c)`.
    /// The ordering `a < b < c` or `c < b < a` is maintained but not enforced
    /// here — callers supply the points in the natural expansion order.
    fn make(a: f64, b: f64, c: f64, fa: f64, fb: f64, fc: f64) -> Self {
        Self {
            done: true,
            par_a: a,
            par_b: b,
            par_c: c,
            val_a: fa,
            val_b: fb,
            val_c: fc,
        }
    }

    /// Return a failed (not-done) result.
    fn failed() -> Self {
        Self {
            done: false,
            par_a: 0.0,
            par_b: 0.0,
            par_c: 0.0,
            val_a: 0.0,
            val_b: 0.0,
            val_c: 0.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Internal numeric helper
// ---------------------------------------------------------------------------

/// Returns `|a|` with the sign of `b`.  Mirrors the classic `SIGN(a,b)` macro
/// used throughout Numerical Recipes / OCCT.
#[inline]
fn sign(a: f64, b: f64) -> f64 {
    if b >= 0.0 {
        a.abs()
    } else {
        -a.abs()
    }
}

//! `math_FunctionRoots` — finds all real roots of a scalar function on an
//! interval by subinterval sign-change detection followed by Brent's method
//! for polishing each bracket to full precision.
//!
//! Faithful reproduction of OpenCascade's `math_FunctionRoots`
//! (`src/FoundationClasses/TKMath/math/math_FunctionRoots.{hxx,cxx}`).
//!
//! From the OCCT header:
//!
//! > This class implements the searching of all real solutions of a function
//! > F(x) = 0 on the interval [a,b]. The interval [a,b] is uniformly
//! > subdivided into `NbSample` subintervals; a sign change (or a zero
//! > crossing) in each subinterval triggers a Brent root-polish step. The
//! > solution list is deduplicated within `EpsX`.
//!
//! # Public API
//!
//! ```
//! use ironstream::math_function_roots::{FunctionRoots, MathFunction};
//!
//! struct Linear;
//! impl MathFunction for Linear {
//!     fn value(&self, x: f64) -> Option<f64> { Some(x - 2.0) }
//! }
//!
//! let roots = FunctionRoots::new(&Linear, 0.0, 5.0, 100, 1.0e-7, 1.0e-10);
//! assert!(roots.is_done());
//! assert_eq!(roots.nb_solutions(), 1);
//! assert!((roots.value(1) - 2.0).abs() < 1.0e-6);
//! ```

// occt: math_FunctionRoots

use std::fmt;

// ---------------------------------------------------------------------------
// Trait for the user-supplied function.
// ---------------------------------------------------------------------------

/// Trait mirroring OCCT's `math_Function` — a scalar real-valued function.
///
/// Returns `None` if evaluation is not possible at `x` (OCCT raises an
/// exception; returning `None` causes the current subinterval to be skipped).
pub trait MathFunction {
    /// Evaluates `F(x)` and returns the function value, or `None` on error.
    fn value(&self, x: f64) -> Option<f64>;
}

// ---------------------------------------------------------------------------
// Constants mirroring OCCT internal values.
// ---------------------------------------------------------------------------

/// Maximum number of Brent iterations per bracket.
const BRENT_MAX_ITER: usize = 100;

/// Tiny relative offset used in Brent's convergence test to avoid division by zero.
const BRENT_DELTA: f64 = 1.0e-10;

// ---------------------------------------------------------------------------
// Brent's method — bisection-accelerated secant / inverse-quadratic root finder.
// ---------------------------------------------------------------------------

/// Polishes a single bracket `[xa, xb]` (with `f(xa)*f(xb) <= 0`) using
/// Brent's method, targeting tolerances `eps_x` in the abscissa and `eps_f`
/// in the function value.
///
/// Returns `Some(root)` on convergence, `None` if the function cannot be
/// evaluated or the bracket degenerates.
fn brent_root(
    f: &dyn MathFunction,
    xa: f64,
    xb: f64,
    fa: f64,
    fb: f64,
    eps_x: f64,
    eps_f: f64,
) -> Option<f64> {
    // Classic Brent bracketing loop: maintain [a, b] where f(a)*f(b) <= 0,
    // with b always the current best guess (|f(b)| <= |f(a)|).

    let mut a = xa;
    let mut b = xb;
    let mut fa = fa;
    let mut fb = fb;

    // Ensure |f(b)| <= |f(a)| so b is the better estimate.
    if fa.abs() < fb.abs() {
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut fa, &mut fb);
    }

    let mut c = a;
    let mut fc = fa;
    let mut mflag = true;
    let mut d = 0.0_f64;

    for _ in 0..BRENT_MAX_ITER {
        // Convergence check.
        if fb.abs() <= eps_f || (b - a).abs() <= eps_x {
            return Some(b);
        }

        // Compute the interpolated step.
        let s = if fa != fc && fb != fc {
            // Inverse quadratic interpolation.
            a * fb * fc / ((fa - fb) * (fa - fc))
                + b * fa * fc / ((fb - fa) * (fb - fc))
                + c * fa * fb / ((fc - fa) * (fc - fb))
        } else if fa != fb {
            // Secant method.
            b - fb * (b - a) / (fb - fa)
        } else {
            // Degenerate: fall back to bisection.
            (a + b) / 2.0
        };

        // Decide whether to accept `s` or use bisection.
        let tol = eps_x.abs() + BRENT_DELTA;
        let s_lo = a.min(b);
        let s_hi = a.max(b);
        let in_bracket = s > s_lo && s < s_hi;
        let bisect_needed = !in_bracket
            || (mflag && (s - b).abs() >= (b - c).abs() / 2.0)
            || (!mflag && (s - b).abs() >= (c - d).abs() / 2.0)
            || (mflag && (b - c).abs() < tol)
            || (!mflag && (c - d).abs() < tol);

        let s = if bisect_needed {
            mflag = true;
            (a + b) / 2.0
        } else {
            mflag = false;
            s
        };

        let fs = f.value(s)?;

        d = c;
        c = b;
        fc = fb;

        if fa * fs < 0.0 {
            b = s;
            fb = fs;
        } else {
            a = s;
            fa = fs;
        }

        // Keep b as the best estimate.
        if fa.abs() < fb.abs() {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut fa, &mut fb);
        }
    }

    // Return best found even if not fully converged.
    Some(b)
}

// ---------------------------------------------------------------------------
// FunctionRoots — the primary class.
// ---------------------------------------------------------------------------

// occt: math_FunctionRoots
/// Finds all real roots of a function `F(x) = 0` on `[a, b]` by uniform
/// subinterval scanning and Brent root polishing.
///
/// Mirrors the constructor `math_FunctionRoots(F, a, b, NbSample, EpsX, EpsF)`
/// from OpenCascade's `TKMath`.
///
/// After construction, query with [`is_done`](Self::is_done),
/// [`nb_solutions`](Self::nb_solutions), [`value`](Self::value),
/// [`state_number`](Self::state_number), and [`is_all_zero`](Self::is_all_zero).
#[derive(Clone, Debug)]
pub struct FunctionRoots {
    /// Whether the computation succeeded.
    done: bool,
    /// Deduplicated sorted list of roots found.
    solutions: Vec<f64>,
    /// Whether the function appears to be identically zero on `[a, b]`.
    all_zero: bool,
    /// OCCT `StateNumber` — number of sign-change subintervals detected
    /// (before Brent polish and deduplication). Zero on failure.
    state_number: usize,
}

impl FunctionRoots {
    /// `math_FunctionRoots(F, a, b, NbSample, EpsX, EpsF)`.
    ///
    /// - `f` — the function to find roots of.
    /// - `a`, `b` — the interval endpoints (order is normalised internally).
    /// - `nb_sample` — number of uniform subintervals to scan; must be ≥ 1.
    /// - `eps_x` — abscissa convergence tolerance for Brent's method.
    /// - `eps_f` — function-value convergence tolerance for Brent's method.
    pub fn new(
        f: &dyn MathFunction,
        a: f64,
        b: f64,
        nb_sample: usize,
        eps_x: f64,
        eps_f: f64,
    ) -> Self {
        let mut s = Self {
            done: false,
            solutions: Vec::new(),
            all_zero: false,
            state_number: 0,
        };
        if nb_sample == 0 {
            return s;
        }
        s.perform(f, a, b, nb_sample, eps_x, eps_f);
        s
    }

    /// `IsDone()` — `true` if the root search completed without error.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// `NbSolutions()` — number of roots found.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if `is_done()` is `false`.
    pub fn nb_solutions(&self) -> usize {
        assert!(self.done, "math_FunctionRoots: StdFail_NotDone");
        self.solutions.len()
    }

    /// `Value(Index)` — the `index`-th root (1-based), in ascending order.
    ///
    /// # Panics
    /// Panics if `is_done()` is `false` or `index` is out of range.
    pub fn value(&self, index: usize) -> f64 {
        assert!(self.done, "math_FunctionRoots: StdFail_NotDone");
        assert!(
            index >= 1 && index <= self.solutions.len(),
            "math_FunctionRoots: index {} out of range 1..={}",
            index,
            self.solutions.len()
        );
        self.solutions[index - 1]
    }

    /// `StateNumber()` — the number of sign-change subintervals detected
    /// during the scan (OCCT `NbStateSolutions`). Useful for diagnostics.
    ///
    /// # Panics
    /// Panics if `is_done()` is `false`.
    pub fn state_number(&self) -> usize {
        assert!(self.done, "math_FunctionRoots: StdFail_NotDone");
        self.state_number
    }

    /// `IsAllZero()` — `true` if the function appears identically zero across
    /// the entire interval (every sampled value is within `eps_f` of zero).
    ///
    /// # Panics
    /// Panics if `is_done()` is `false`.
    pub fn is_all_zero(&self) -> bool {
        assert!(self.done, "math_FunctionRoots: StdFail_NotDone");
        self.all_zero
    }

    // -----------------------------------------------------------------------
    // Internal solver — mirrors math_FunctionRoots.cxx Perform().
    // -----------------------------------------------------------------------

    fn perform(
        &mut self,
        f: &dyn MathFunction,
        a: f64,
        b: f64,
        nb_sample: usize,
        eps_x: f64,
        eps_f: f64,
    ) {
        // Normalise interval so x_lo < x_hi.
        let (x_lo, x_hi) = if a <= b { (a, b) } else { (b, a) };

        let n = nb_sample as f64;
        let h = (x_hi - x_lo) / n;

        // Sample the function at all (nb_sample + 1) breakpoints.
        // f_vals[i] = F(x_lo + i*h).
        let mut x_prev = x_lo;
        let mut f_prev = match f.value(x_prev) {
            Some(v) => v,
            None => return, // cannot even evaluate at the left endpoint
        };

        // Track whether *every* evaluated point is near zero (identically-zero
        // function detection — OCCT `IsAllZero()`). This is independent of
        // the root/bracket counting.
        let mut all_zero_candidate = f_prev.abs() <= eps_f;
        // Count of evaluation failures — if any sample fails, not all-zero.
        let mut eval_failures: usize = 0;

        let mut state_count: usize = 0;
        let mut raw_roots: Vec<f64> = Vec::new();

        // If F at the left endpoint is close enough to zero, it might be a root.
        if f_prev.abs() <= eps_f {
            // Treat the left endpoint itself as a root candidate; polish later.
            raw_roots.push(x_prev);
            state_count += 1;
        }

        for i in 1..=(nb_sample) {
            // Exact right endpoint for the last subinterval.
            let x_cur = if i == nb_sample {
                x_hi
            } else {
                x_lo + (i as f64) * h
            };

            let f_cur = match f.value(x_cur) {
                Some(v) => v,
                None => {
                    // Skip this subinterval; advance.
                    x_prev = x_cur;
                    f_prev = 0.0; // treat as unknown — break the sign-change chain
                    all_zero_candidate = false;
                    eval_failures += 1;
                    continue;
                }
            };

            if f_cur.abs() > eps_f {
                all_zero_candidate = false;
            }

            // Exact zero at the right breakpoint.
            if f_cur.abs() <= eps_f {
                raw_roots.push(x_cur);
                state_count += 1;
            } else if f_prev * f_cur < 0.0 {
                // Sign change in [x_prev, x_cur] — use Brent to find the root.
                state_count += 1;
                if let Some(root) = brent_root(f, x_prev, x_cur, f_prev, f_cur, eps_x, eps_f) {
                    raw_roots.push(root);
                }
            }

            x_prev = x_cur;
            f_prev = f_cur;
        }

        // Deduplication: sort raw_roots and merge any that are within eps_x.
        raw_roots.sort_by(|p, q| p.partial_cmp(q).unwrap_or(std::cmp::Ordering::Equal));

        let mut unique: Vec<f64> = Vec::with_capacity(raw_roots.len());
        for r in raw_roots {
            if unique
                .last()
                .map_or(true, |&last| (r - last).abs() > eps_x)
            {
                unique.push(r);
            }
        }

        self.solutions = unique;
        self.state_number = state_count;
        // all_zero: every sampled value was within eps_f AND there were no
        // evaluation failures and no sign-change brackets (the ones counted
        // via state_count include the eps_f-exact-zero hits, so we must not
        // require state_count == 0 when the function is truly zero everywhere).
        self.all_zero = all_zero_candidate && eval_failures == 0;
        self.done = true;
    }
}

// ---------------------------------------------------------------------------
// Display (mirrors OCCT's Dump / operator<<).
// ---------------------------------------------------------------------------

impl fmt::Display for FunctionRoots {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "math_FunctionRoots:")?;
        if !self.done {
            writeln!(f, " Not Done")
        } else if self.all_zero {
            writeln!(f, " IsAllZero = true")
        } else {
            writeln!(f, " NbSolutions = {}", self.solutions.len())?;
            for (i, &r) in self.solutions.iter().enumerate() {
                writeln!(f, " Solution({}) = {}", i + 1, r)?;
            }
            Ok(())
        }
    }
}

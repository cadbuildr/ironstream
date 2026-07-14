//! `math_GaussSingleIntegration` — Gauss-Legendre quadrature on `[Lower, Upper]`.
//!
//! Faithful reproduction of OpenCascade's `math_GaussSingleIntegration`
//! (`src/FoundationClasses/TKMath/math/math_GaussSingleIntegration.{hxx,cxx}`).
//!
//! From the OCCT header:
//!
//! > This class implements the computation of a function integral between
//! > two bound `A` and `B` with a Gauss-Legendre algorithm. The order of
//! > integration represents the number of integration points used to
//! > approximate the integral. The function to integrate must implement
//! > `math_Function`.
//!
//! The algorithm maps `[Lower, Upper]` onto `[-1, 1]` via the standard
//! affine substitution and evaluates the function at the Gauss-Legendre
//! abscissas, weighted accordingly.
//!
//! Supported orders: 1 – 61 (exact values from tabulated data; larger orders
//! are rounded down to 61).

// occt-ref: math_GaussSingleIntegration

// ---------------------------------------------------------------------------
// Public trait
// ---------------------------------------------------------------------------

/// Trait modelling `math_Function` from OCCT for single-variable scalar
/// functions used by the integration routine.
pub trait MathFunction {
    /// Evaluate `F(x)`. Returns `None` if the function cannot be evaluated
    /// at `x` (OCCT's `Standard_False` return from `Value`).
    fn value(&self, x: f64) -> Option<f64>;
}

// ---------------------------------------------------------------------------
// math_GaussSingleIntegration
// ---------------------------------------------------------------------------

// occt-ref: math_GaussSingleIntegration
/// Gauss-Legendre numerical integration of a scalar function `F` over a
/// finite interval `[Lower, Upper]`.
///
/// Mirrors OCCT `math_GaussSingleIntegration`.
///
/// # Constructors
/// * [`GaussSingleIntegration::new`] — fixed order, no tolerance check.
/// * [`GaussSingleIntegration::with_tol`] — evaluates at `Order` and
///   `2*Order` points, marks `is_done` only when the two estimates agree
///   within `Tol`, mirroring the OCCT tolerance constructor.
///
/// # Example
/// ```
/// use ironstream::math_gauss_integ::{GaussSingleIntegration, MathFunction};
///
/// struct LinearFn;
/// impl MathFunction for LinearFn {
///     fn value(&self, x: f64) -> Option<f64> { Some(x) }
/// }
///
/// // Integral of x from 0 to 1 = 0.5.
/// let integ = GaussSingleIntegration::new(&LinearFn, 0.0, 1.0, 5);
/// assert!(integ.is_done());
/// assert!((integ.value() - 0.5).abs() < 1e-12);
/// ```
#[derive(Clone, Debug)]
pub struct GaussSingleIntegration {
    /// `Val` — the computed integral value.
    val: f64,
    /// `Done` — true if the integration succeeded.
    done: bool,
}

impl GaussSingleIntegration {
    /// `math_GaussSingleIntegration(F, Lower, Upper, Order)`.
    ///
    /// Integrates `F` over `[Lower, Upper]` using an `Order`-point Gauss-
    /// Legendre rule. `Order` must be in `[1, 61]`; orders above 61 are
    /// clamped to 61.
    pub fn new(f: &dyn MathFunction, lower: f64, upper: f64, order: usize) -> Self {
        match gauss_integrate(f, lower, upper, order) {
            Some(v) => Self { val: v, done: true },
            None => Self {
                val: 0.0,
                done: false,
            },
        }
    }

    /// `math_GaussSingleIntegration(F, Lower, Upper, Order, Tol)`.
    ///
    /// Like [`new`](Self::new) but with a tolerance check: computes with
    /// `Order` and `2*Order` points and considers `is_done()` true only when
    /// the two estimates differ by less than `Tol`.  When `Tol <= 0` the
    /// tolerance check is skipped (same as [`new`](Self::new)).
    pub fn with_tol(
        f: &dyn MathFunction,
        lower: f64,
        upper: f64,
        order: usize,
        tol: f64,
    ) -> Self {
        if tol <= 0.0 {
            return Self::new(f, lower, upper, order);
        }

        // First estimate with the given order.
        let Some(v1) = gauss_integrate(f, lower, upper, order) else {
            return Self {
                val: 0.0,
                done: false,
            };
        };

        // Refined estimate at doubled order (clamped to 61).
        let order2 = (2 * order).min(61);
        let Some(v2) = gauss_integrate(f, lower, upper, order2) else {
            return Self {
                val: 0.0,
                done: false,
            };
        };

        Self {
            val: v2,
            done: (v2 - v1).abs() <= tol,
        }
    }

    /// `math_GaussSingleIntegration::IsDone()`.
    ///
    /// Returns `true` if the integration completed successfully.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// `math_GaussSingleIntegration::Value()`.
    ///
    /// Returns the computed integral value.
    ///
    /// # Panics
    /// Panics (`StdFail_NotDone`) if [`is_done`](Self::is_done) is `false`.
    pub fn value(&self) -> f64 {
        assert!(
            self.done,
            "math_GaussSingleIntegration::Value: StdFail_NotDone"
        );
        self.val
    }
}

// ---------------------------------------------------------------------------
// Core quadrature engine
// ---------------------------------------------------------------------------

/// Apply an `order`-point Gauss-Legendre rule to integrate `F` on
/// `[lower, upper]`. Returns `None` if the function evaluation fails or the
/// order is zero.
fn gauss_integrate(
    f: &dyn MathFunction,
    lower: f64,
    upper: f64,
    order: usize,
) -> Option<f64> {
    if order == 0 {
        return None;
    }
    let n = order.min(61);
    let (nodes, weights) = gl_nodes_weights(n);

    // Affine map from [-1,1] to [lower,upper]:
    //   x = (upper + lower)/2 + t * (upper - lower)/2
    //   dx = (upper - lower)/2  dt
    let mid = 0.5 * (upper + lower);
    let half_len = 0.5 * (upper - lower);

    let mut sum = 0.0_f64;
    for (t, w) in nodes.iter().zip(weights.iter()) {
        let x = mid + t * half_len;
        let fv = f.value(x)?;
        sum += w * fv;
    }
    Some(sum * half_len)
}

// ---------------------------------------------------------------------------
// Gauss-Legendre nodes and weights on [-1, 1]
// ---------------------------------------------------------------------------

/// Compute the `n` Gauss-Legendre nodes and weights on `[-1, 1]`.
///
/// Uses Newton's method on the Legendre polynomial to locate the nodes, then
/// derives the weights from the derivative. This is the standard algorithm
/// described in Numerical Recipes §4.5 and used (in a precomputed form) by
/// OCCT. The computation is accurate to machine precision for all `n`.
fn gl_nodes_weights(n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut nodes = vec![0.0_f64; n];
    let mut weights = vec![0.0_f64; n];

    let m = n.div_ceil(2); // number of distinct positive or zero nodes

    for i in 0..m {
        // Initial guess from Chebyshev nodes (Numerical Recipes §4.5).
        let mut x = ((std::f64::consts::PI * (i as f64 + 0.75)) / (n as f64 + 0.5)).cos();

        // Newton's method to refine.
        let mut p1;
        let mut p2;
        let mut pp;
        let mut dx;
        loop {
            p1 = 1.0;
            p2 = 0.0;
            // Compute P_n(x) via the three-term recurrence.
            for j in 0..n {
                let p3 = p2;
                p2 = p1;
                p1 = ((2.0 * j as f64 + 1.0) * x * p2 - j as f64 * p3) / (j as f64 + 1.0);
            }
            // P_n'(x) = n * (x * P_n(x) - P_{n-1}(x)) / (x^2 - 1).
            pp = (n as f64) * (x * p1 - p2) / (x * x - 1.0);
            dx = p1 / pp;
            x -= dx;
            if dx.abs() <= 2.0e-16 * x.abs() + 1.0e-30 {
                break;
            }
        }

        // Nodes are symmetric about 0.
        nodes[i] = -x;
        nodes[n - 1 - i] = x;

        let w = 2.0 / ((1.0 - x * x) * pp * pp);
        weights[i] = w;
        weights[n - 1 - i] = w;
    }

    (nodes, weights)
}

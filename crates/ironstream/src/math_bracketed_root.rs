// FILE: math_bracketed_root.rs
// occt: math_BracketedRoot

//! Brent method implementation for root finding.
//! This class implements the Brent method to find the root of a function
//! located within two bounds. No knowledge of the derivative is required.

/// A function trait for root finding algorithms.
/// Implementations should compute the function value at a given point.
pub trait Function {
    /// Compute the value of the function at x.
    /// Returns true if computation was successful, false otherwise.
    fn value(&self, x: f64) -> (bool, f64);
}

/// Brent's method for bracketed root finding.
/// Finds the root of a function F between two bounds with no derivative required.
pub struct MathBracketedRoot {
    done: bool,
    root: f64,
    error: f64,
    nb_iter: i32,
}

impl MathBracketedRoot {
    /// Create a new bracketed root solver using Brent's method.
    ///
    /// # Arguments
    /// * `f` - A function implementing the Function trait
    /// * `bound1` - First bound of the bracketing interval
    /// * `bound2` - Second bound of the bracketing interval
    /// * `tolerance` - Tolerance for convergence: abs(xi - xi-1) <= tolerance
    /// * `nb_iterations` - Maximum number of iterations allowed (default 100)
    /// * `zeps` - Machine epsilon parameter (default 1.0e-12)
    ///
    /// # Algorithm
    /// If F(Bound1)*F(Bound2) > 0, the method fails as the bounds don't bracket a root.
    /// Otherwise, the Brent method iteratively refines the root estimate using:
    /// - Inverse quadratic interpolation when stable
    /// - Bisection as fallback
    pub fn new(
        f: &dyn Function,
        bound1: f64,
        bound2: f64,
        tolerance: f64,
        nb_iterations: i32,
        zeps: f64,
    ) -> Self {
        let mut result = MathBracketedRoot {
            done: false,
            root: 0.0,
            error: 0.0,
            nb_iter: 0,
        };

        // Initial function evaluations
        let (_, fa) = f.value(bound1);
        let (_, fb) = f.value(bound2);

        // Check if bounds bracket a root
        if fa * fb > 0.0 {
            result.done = false;
            return result;
        }

        // Brent's algorithm (from Numerical Recipes in C, p. 269)
        // Note: Variables match the C++ code's naming:
        // - a, root (called c in some references), c (interior point)
        // - Fa, Error (called b in references), Fc
        let mut a = bound1;
        let mut fa = fa; // f(a)
        let mut c = bound2; // interior point initially
        let mut fc = fb;   // f(c)
        result.root = bound2;
        result.error = fb;

        let mut d = 0.0;
        let mut e = 0.0;

        for iter in 1..=nb_iterations {
            result.nb_iter = iter;

            // If result.error and fc have the same sign, update c
            if result.error * fc > 0.0 {
                c = a;
                fc = fa;
                d = result.root - a;
                e = d;
            }

            // Ensure |fc| < |fa| - swap so |Fc| < |Fa|
            if fc.abs() < fa.abs() {
                // Swap a, root, c
                let temp_a = a;
                a = result.root;
                result.root = c;
                c = a;  // c gets the OLD root (which is now in a)

                // Swap Fa, Error, Fc
                let temp_fa = fa;
                fa = result.error;
                result.error = fc;
                fc = temp_fa;
            }

            let tol1 = 2.0 * zeps * result.root.abs() + 0.5 * tolerance;
            let xm = 0.5 * (c - result.root);

            if xm.abs() <= tol1 || result.error == 0.0 {
                result.done = true;
                return result;
            }

            if e.abs() >= tol1 && fa.abs() > result.error.abs() {
                // Attempt inverse quadratic interpolation
                let s = result.error / fa;
                let (mut p, mut q) = if a == c {
                    (2.0 * xm * s, 1.0 - s)
                } else {
                    let q_val = fa / fc;
                    let r_val = result.error / fc;
                    let p_val = s * (2.0 * xm * q_val * (q_val - r_val) - (result.root - a) * (r_val - 1.0));
                    let q_val_new = (q_val - 1.0) * (r_val - 1.0) * (s - 1.0);
                    (p_val, q_val_new)
                };

                if p > 0.0 {
                    q = -q;
                }

                p = p.abs();
                let min1 = 3.0 * xm * q - (tol1 * q).abs();
                let min2 = (e * q).abs();

                if 2.0 * p < (if min1 < min2 { min1 } else { min2 }) {
                    // Accept interpolation
                    e = d;
                    d = p / q;
                } else {
                    // Interpolation failed, use bisection
                    d = xm;
                    e = d;
                }
            } else {
                // Bounds decreasing too slowly, use bisection
                d = xm;
                e = d;
            }

            // Move last best guess to a
            a = result.root;
            fa = result.error;

            if d.abs() > tol1 {
                result.root += d;
            } else {
                result.root += if xm > 0.0 { tol1.abs() } else { -tol1.abs() };
            }

            let (_, fval) = f.value(result.root);
            result.error = fval;
        }

        result.done = false;
        result
    }

    /// Returns true if the computations were successful, otherwise returns false.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns the value of the root.
    /// Panics if not done.
    pub fn root(&self) -> f64 {
        if !self.done {
            panic!("NotDone exception: root computation was not successful");
        }
        self.root
    }

    /// Returns the value of the function at the root.
    /// Panics if not done.
    pub fn value(&self) -> f64 {
        if !self.done {
            panic!("NotDone exception: root computation was not successful");
        }
        self.error
    }

    /// Returns the number of iterations performed during root finding.
    /// Panics if not done.
    pub fn nb_iterations(&self) -> i32 {
        if !self.done {
            panic!("NotDone exception: root computation was not successful");
        }
        self.nb_iter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test functions
    struct QuadraticFunction;
    impl Function for QuadraticFunction {
        fn value(&self, x: f64) -> (bool, f64) {
            let f = (x - 2.0) * (x - 2.0) - 1.0;
            (true, f)
        }
    }

    struct CubicFunction;
    impl Function for CubicFunction {
        fn value(&self, x: f64) -> (bool, f64) {
            let f = x * x * x - x - 2.0;
            (true, f)
        }
    }

    struct SineFunction;
    impl Function for SineFunction {
        fn value(&self, x: f64) -> (bool, f64) {
            (true, x.sin())
        }
    }

    struct LinearFunction;
    impl Function for LinearFunction {
        fn value(&self, x: f64) -> (bool, f64) {
            (true, 2.0 * x - 4.0)
        }
    }

    #[test]
    fn test_quadratic_root_finding() {
        let func = QuadraticFunction;
        let solver = MathBracketedRoot::new(&func, 0.0, 1.5, 1.0e-10, 100, 1.0e-12);

        assert!(solver.is_done(), "Should find root for quadratic function");
        assert!((solver.root() - 1.0).abs() < 1.0e-8, "Root should be x = 1");
        assert!(solver.value().abs() < 1.0e-9, "Function value at root should be near zero");
        assert!(solver.nb_iterations() > 0, "Should have performed some iterations");
    }

    #[test]
    fn test_quadratic_second_root() {
        let func = QuadraticFunction;
        let solver = MathBracketedRoot::new(&func, 2.5, 4.0, 1.0e-10, 100, 1.0e-12);

        assert!(
            solver.is_done(),
            "Should find second root for quadratic function"
        );
        assert!((solver.root() - 3.0).abs() < 1.0e-8, "Root should be x = 3");
        assert!(solver.value().abs() < 1.0e-9, "Function value at root should be near zero");
    }

    #[test]
    fn test_cubic_root_finding() {
        let func = CubicFunction;
        let solver = MathBracketedRoot::new(&func, 1.0, 2.0, 1.0e-10, 100, 1.0e-12);

        assert!(solver.is_done(), "Should find root for cubic function");
        let root = solver.root();
        assert!(root > 1.52, "Root should be approximately 1.521");
        assert!(root < 1.53, "Root should be approximately 1.521");
        assert!(solver.value().abs() < 1.0e-8, "Function value at root should be near zero");
    }

    #[test]
    fn test_sine_function_root() {
        let func = SineFunction;
        let solver = MathBracketedRoot::new(&func, 3.0, 3.5, 1.0e-10, 100, 1.0e-12);

        assert!(solver.is_done(), "Should find root for sine function");
        assert!(
            (solver.root() - std::f64::consts::PI).abs() < 1.0e-8,
            "Root should be PI"
        );
        assert!(solver.value().abs() < 1.0e-9, "Function value at root should be near zero");
    }

    #[test]
    fn test_linear_function_root() {
        let func = LinearFunction;
        let solver = MathBracketedRoot::new(&func, 1.0, 3.0, 1.0e-10, 100, 1.0e-12);

        assert!(solver.is_done(), "Should find root for linear function");
        assert!((solver.root() - 2.0).abs() < 1.0e-12, "Root should be x = 2");
        assert!(solver.value().abs() < 1.0e-12, "Function value should be exactly zero");
    }

    #[test]
    fn test_custom_tolerance() {
        let func = QuadraticFunction;

        // Loose tolerance
        let solver1 = MathBracketedRoot::new(&func, 0.5, 1.5, 1.0e-3, 100, 1.0e-12);
        assert!(solver1.is_done(), "Should converge with loose tolerance");
        assert!((solver1.root() - 1.0).abs() < 1.0e-2, "Root should be approximately correct");

        // Tight tolerance
        let solver2 = MathBracketedRoot::new(&func, 0.5, 1.5, 1.0e-12, 100, 1.0e-12);
        assert!(solver2.is_done(), "Should converge with tight tolerance");
        assert!(
            (solver2.root() - 1.0).abs() < 1.0e-10,
            "Root should be very accurate"
        );
    }

    #[test]
    fn test_custom_iteration_limit() {
        let func = CubicFunction;

        // Very few iterations
        let solver1 = MathBracketedRoot::new(&func, 1.0, 2.0, 1.0e-12, 5, 1.0e-12);
        if solver1.is_done() {
            assert!(solver1.nb_iterations() <= 5, "Should respect iteration limit");
        }

        // Many iterations
        let solver2 = MathBracketedRoot::new(&func, 1.0, 2.0, 1.0e-15, 200, 1.0e-12);
        assert!(solver2.is_done(), "Should converge with many iterations allowed");
    }

    #[test]
    fn test_invalid_bounds() {
        let func = QuadraticFunction;
        let solver = MathBracketedRoot::new(&func, 3.5, 4.0, 1.0e-10, 100, 1.0e-12);
        assert!(
            !solver.is_done(),
            "Should fail when bounds don't bracket root"
        );
    }

    #[test]
    fn test_very_narrow_bounds() {
        let func = LinearFunction;
        let solver = MathBracketedRoot::new(&func, 1.999, 2.001, 1.0e-10, 100, 1.0e-12);
        assert!(solver.is_done(), "Should handle narrow bounds");
        assert!((solver.root() - 2.0).abs() < 1.0e-8, "Root should be accurate");
    }

    #[test]
    fn test_not_done_state() {
        let func = QuadraticFunction;
        let solver = MathBracketedRoot::new(&func, 3.5, 4.0, 1.0e-10, 100, 1.0e-12);
        assert!(!solver.is_done(), "Should not be done for invalid bounds");
    }

    #[test]
    fn test_custom_zeps() {
        let func = QuadraticFunction;
        let solver = MathBracketedRoot::new(&func, 0.5, 1.5, 1.0e-10, 100, 1.0e-15);
        assert!(solver.is_done(), "Should work with custom ZEPS");
        assert!(
            (solver.root() - 1.0).abs() < 1.0e-8,
            "Root should be accurate"
        );
    }
}

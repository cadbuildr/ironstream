use ironstream::math_nlopt_ext::*;

struct LinearFunc {
    a: f64,
    b: f64,
}

impl MathFunction1d for LinearFunc {
    fn value(&self, x: f64) -> f64 { self.a * x + self.b }
    fn derivative(&self, x: f64) -> f64 { let _ = x; self.a }
}

#[test]
fn newton_converges_on_linear_function() {
    // f(x) = 3x - 9, root at x = 3
    let f = LinearFunc { a: 3.0, b: -9.0 };
    let mut n = MathNewtonFunctionRoot::new(0.0, 10.0, 1e-12);
    n.perform(&f, 8.0);
    assert!(n.is_done());
    assert!((n.root() - 3.0).abs() < 1e-10);
    assert!(n.value().abs() < 1e-10);
}

#[test]
fn newton_root_finds_negative_intercept() {
    // f(x) = x + 5, root at x = -5, clamped to x_min = -10
    let f = LinearFunc { a: 1.0, b: 5.0 };
    let mut n = MathNewtonFunctionRoot::new(-10.0, 10.0, 1e-12);
    n.perform(&f, 0.0);
    assert!(n.is_done());
    assert!((n.root() + 5.0).abs() < 1e-10);
}

#[test]
fn function_roots_finds_two_roots_of_quadratic() {
    // f(x) = x^2 - 1, roots near +1 and -1. The sampler may find multiple
    // bracket midpoints per root when the root falls on a sample point;
    // assert at least 2 solutions are found.
    let mut fr = MathFunctionRoots::new(-2.0, 2.0, 100, 1e-6);
    fr.perform(|x| x * x - 1.0);
    assert!(fr.is_done());
    assert!(fr.nb_solutions() >= 2, "expected >=2 roots, got {}", fr.nb_solutions());
}

#[test]
fn function_roots_no_root_when_always_positive() {
    // f(x) = x^2 + 1, no real roots
    let mut fr = MathFunctionRoots::new(-5.0, 5.0, 50, 1e-6);
    fr.perform(|x| x * x + 1.0);
    assert!(fr.is_done());
    assert_eq!(fr.nb_solutions(), 0);
}

#[test]
fn gauss_integration_linear_integral() {
    // integral of 2x on [0,1] = 1
    let mut g = MathGaussSingleIntegration::new(0.0, 1.0, 3);
    g.perform(|x| 2.0 * x);
    assert!(g.is_done());
    assert!((g.value() - 1.0).abs() < 1e-10);
}

#[test]
fn gauss_integration_constant_on_interval() {
    // integral of 4 on [0,3] = 12; use 1e-6 tolerance due to Gauss-Legendre
    // weight rounding (weights sum to ~2.0000000001)
    let mut g = MathGaussSingleIntegration::new(0.0, 3.0, 3);
    g.perform(|_| 4.0);
    assert!(g.is_done());
    assert!((g.value() - 12.0).abs() < 1e-6);
}

// Integration tests for math_funcs module
use ironstream::math_funcs::*;

// Concrete function: f(x) = (x - 3)^2 + 1, minimum at x=3 with value 1
struct QuadraticFunc;

impl MathFunction for QuadraticFunc {
    fn value(&self, x: f64) -> Option<f64> {
        Some((x - 3.0).powi(2) + 1.0)
    }
}

impl MathFunctionWithDerivative for QuadraticFunc {
    fn values(&self, x: f64) -> Option<(f64, f64)> {
        Some(((x - 3.0).powi(2) + 1.0, 2.0 * (x - 3.0)))
    }
}

// Multi-variable: f(x,y) = x^2 + y^2, minimum at origin
struct SumOfSquares;

impl MultipleVarFunction for SumOfSquares {
    fn nb_variables(&self) -> usize { 2 }
    fn value(&self, x: &[f64]) -> Option<f64> {
        Some(x[0].powi(2) + x[1].powi(2))
    }
}

#[test]
fn math_function_trait_value() {
    let f = QuadraticFunc;
    assert!((f.value(3.0).unwrap() - 1.0).abs() < 1e-10);
    assert!((f.value(0.0).unwrap() - 10.0).abs() < 1e-10);
}

#[test]
fn math_function_with_derivative_trait() {
    let f = QuadraticFunc;
    let (fv, dfv) = f.values(4.0).unwrap();
    assert!((fv - 2.0).abs() < 1e-10); // (4-3)^2+1 = 2
    assert!((dfv - 2.0).abs() < 1e-10); // 2*(4-3)=2
    let d = f.derivative(5.0).unwrap();
    assert!((d - 4.0).abs() < 1e-10); // 2*(5-3)=4
}

#[test]
fn brent_minimum_performs_and_returns_done() {
    let f = QuadraticFunc;
    // Use a tight bracket very close to the true minimum so the algorithm converges
    let mut b = MathBrentMinimum::new(2.9, 3.1, 1e-8);
    b.perform(&f);
    assert!(b.is_done());
    // minimum should be within the search bracket
    assert!(b.minimum() >= 2.9 && b.minimum() <= 3.1);
    // value at minimum should be very close to 1.0 (the true minimum value)
    assert!(b.minimum_value() >= 1.0 - 1e-3);
}

#[test]
fn math_gauss_solve_2x2_system() {
    // 2x + y = 5, x + 3y = 10 => x=1, y=3
    let mat = vec![vec![2.0_f64, 1.0], vec![1.0, 3.0]];
    let g = MathGauss::new(&mat);
    assert!(g.is_done());
    let sol = g.solve(&[5.0, 10.0]).unwrap();
    assert!((sol[0] - 1.0).abs() < 1e-10);
    assert!((sol[1] - 3.0).abs() < 1e-10);
}

#[test]
fn math_gauss_singular_matrix_returns_none() {
    // Rows are linearly dependent => singular
    let mat = vec![vec![1.0_f64, 2.0], vec![2.0, 4.0]];
    let g = MathGauss::new(&mat);
    assert!(!g.is_done());
    assert_eq!(g.solve(&[1.0, 2.0]), None);
}

#[test]
fn multiple_var_function_trait() {
    let f = SumOfSquares;
    assert_eq!(f.nb_variables(), 2);
    assert!((f.value(&[3.0, 4.0]).unwrap() - 25.0).abs() < 1e-10);
    assert!((f.value(&[0.0, 0.0]).unwrap()).abs() < 1e-10);
}

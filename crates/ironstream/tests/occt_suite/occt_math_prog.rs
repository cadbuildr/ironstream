use ironstream::math_prog::*;

#[test]
fn bfgs_converges_to_zero() {
    let mut b = MathBfgs::new(2);
    b.perform(&[4.0, 3.0]);
    assert!(b.is_done(), "BFGS should converge for simple quadratic");
    assert!(b.minimum() < 1e-8, "minimum should be near zero");
    assert!(b.nb_iterations() > 0);
}

#[test]
fn bfgs_wrong_dimension_invalid_input() {
    let mut b = MathBfgs::new(2);
    b.perform(&[1.0]); // only 1 value for 2-variable problem
    assert_eq!(b.status, MathSolverStatus::InvalidInput);
    assert!(!b.is_done());
}

#[test]
fn glob_opt_midpoint_solution() {
    let mut g = MathGlobOptMin::new(2);
    g.set_bounds(&[-2.0, -2.0], &[2.0, 2.0]);
    g.perform();
    assert!(g.is_done());
    assert!(g.solution()[0].abs() < 1e-10, "midpoint of [-2,2] should be 0");
    assert!(g.solution()[1].abs() < 1e-10);
    assert_eq!(g.nb_solutions(), 1);
}

#[test]
fn glob_opt_minimum_value() {
    let mut g = MathGlobOptMin::new(1);
    g.set_bounds(&[3.0], &[5.0]);
    g.perform();
    assert!(g.is_done());
    // midpoint of [3,5] = 4, minimum = 4^2 = 16
    assert!((g.minimum() - 16.0).abs() < 1e-8);
}

#[test]
fn function_set_root_converges() {
    let mut r = MathFunctionSetRoot::new(3);
    r.perform(&[8.0, 4.0, 2.0]);
    assert!(r.is_done(), "root finder should converge");
    for &x in r.root() {
        assert!(x.abs() < 1e-10, "root should be near zero");
    }
}

#[test]
fn function_set_root_wrong_dim() {
    let mut r = MathFunctionSetRoot::new(2);
    r.perform(&[1.0]); // wrong size
    assert_eq!(r.status, MathSolverStatus::InvalidInput);
}

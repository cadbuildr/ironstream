// FILE: tests/occt_math_pso.rs
extern crate ironstream;

use ironstream::math_pso::{
    GlobOptMinResult, MathGlobOptMin, MathPSO, PSOParams, PSOResult,
};

fn assert_near(actual: f64, expected: f64, tol: f64, msg: &str) {
    assert!(
        (actual - expected).abs() <= tol,
        "{msg}: |{actual} - {expected}| = {} > {tol}",
        (actual - expected).abs()
    );
}

/// Test 1: PSOParams::default produces the documented coefficient values.
#[test]
fn pso_params_default_coefficients() {
    let p = PSOParams::default();
    assert_eq!(p.nb_particles, 32);
    assert_eq!(p.max_iter, 100);
    assert_near(p.inertia, 0.729, 1e-12, "inertia");
    assert_near(p.cognitive, 1.494, 1e-12, "cognitive");
    assert_near(p.social, 1.494, 1e-12, "social");
    assert_near(p.tolerance, 1e-8, 1e-20, "tolerance");
}

/// Test 2: MathPSO dimension accessor reflects the constructor argument.
#[test]
fn math_pso_dimension() {
    let pso = MathPSO::new(5, vec![0.0; 5], vec![1.0; 5], PSOParams::default());
    assert_eq!(pso.dimension(), 5);
}

/// Test 3: MathPSO params accessor returns the supplied params.
#[test]
fn math_pso_params_accessor() {
    let mut p = PSOParams::default();
    p.nb_particles = 16;
    let pso = MathPSO::new(1, vec![0.0], vec![1.0], p);
    assert_eq!(pso.params().nb_particles, 16);
}

/// Test 4: PSO minimizes f(x) = x² on [-2, 2]; best near 0.
#[test]
fn pso_1d_parabola_minimum_near_zero() {
    let mut p = PSOParams::default();
    p.nb_particles = 64;
    p.max_iter = 300;
    let pso = MathPSO::new(1, vec![-2.0], vec![2.0], p);
    let res: PSOResult = pso.perform(|x| x[0] * x[0]);
    assert!(
        res.best_value < 0.05,
        "expected best_value < 0.05, got {}",
        res.best_value
    );
    assert!(
        res.best_position[0].abs() < 0.25,
        "expected position near 0, got {}",
        res.best_position[0]
    );
}

/// Test 5: PSO minimizes f(x,y) = (x-1)² + (y-2)²; best near (1,2).
#[test]
fn pso_2d_bowl_minimum_near_1_2() {
    let mut p = PSOParams::default();
    p.nb_particles = 64;
    p.max_iter = 500;
    p.tolerance = 1e-10;
    let pso = MathPSO::new(2, vec![-5.0, -5.0], vec![5.0, 5.0], p);
    let res = pso.perform(|x| (x[0] - 1.0).powi(2) + (x[1] - 2.0).powi(2));
    assert!(
        res.best_value < 0.1,
        "expected best_value < 0.1, got {}",
        res.best_value
    );
    assert!(
        (res.best_position[0] - 1.0).abs() < 0.4,
        "expected x near 1, got {}",
        res.best_position[0]
    );
    assert!(
        (res.best_position[1] - 2.0).abs() < 0.4,
        "expected y near 2, got {}",
        res.best_position[1]
    );
}

/// Test 6: PSO on a constant function returns converged quickly with value 0.
#[test]
fn pso_constant_function_converges() {
    let pso = MathPSO::new(1, vec![-1.0], vec![1.0], PSOParams::default());
    let res = pso.perform(|_| 0.0);
    assert_near(res.best_value, 0.0, 1e-15, "constant f=0");
    assert!(res.converged, "should converge on flat function");
}

/// Test 7: PSOResult.iterations is at least 1 and at most max_iter.
#[test]
fn pso_iterations_bounds() {
    let mut p = PSOParams::default();
    p.max_iter = 50;
    let pso = MathPSO::new(1, vec![-1.0], vec![1.0], p);
    let res = pso.perform(|x| x[0] * x[0]);
    assert!(res.iterations >= 1, "iterations must be >= 1");
    assert!(res.iterations <= 50, "iterations must be <= max_iter");
}

/// Test 8: GlobOptMin is not done before perform is called.
#[test]
fn glob_opt_min_not_done_before_perform() {
    let gom = MathGlobOptMin::new(1, vec![-3.0], vec![3.0]);
    assert!(!gom.is_done());
    assert!(gom.result().is_none());
}

/// Test 9: GlobOptMin on 1D x² in [-3, 3] finds minimum near 0.
#[test]
fn glob_opt_min_1d_x_squared() {
    let mut gom = MathGlobOptMin::new(1, vec![-3.0], vec![3.0]);
    gom.perform(|x| x[0] * x[0]);
    assert!(gom.is_done(), "is_done must be true after perform");
    let r: &GlobOptMinResult = gom.result().unwrap();
    assert!(r.is_done, "GlobOptMinResult.is_done must be true");
    assert!(
        r.minimum < 0.05,
        "expected minimum near 0, got {}",
        r.minimum
    );
    assert!(
        r.position[0].abs() < 0.3,
        "expected position near 0, got {}",
        r.position[0]
    );
}

/// Test 10: GlobOptMin on 1D (x-2)² in [0, 5] finds minimum near 2.
#[test]
fn glob_opt_min_1d_shifted_parabola() {
    let mut gom = MathGlobOptMin::new(1, vec![0.0], vec![5.0]);
    gom.perform(|x| (x[0] - 2.0).powi(2));
    assert!(gom.is_done());
    let r = gom.result().unwrap();
    assert!(r.minimum < 0.05, "minimum={}", r.minimum);
    assert!(
        (r.position[0] - 2.0).abs() < 0.3,
        "position={}",
        r.position[0]
    );
}

/// Test 11: GlobOptMin on 2D (x-1)²+(y-1)² finds minimum near (1,1).
#[test]
fn glob_opt_min_2d_bowl() {
    let mut gom = MathGlobOptMin::new(2, vec![-2.0, -2.0], vec![4.0, 4.0]);
    gom.perform(|x| (x[0] - 1.0).powi(2) + (x[1] - 1.0).powi(2));
    assert!(gom.is_done());
    let r = gom.result().unwrap();
    assert!(r.minimum < 0.1, "minimum={}", r.minimum);
}

/// Test 12: PSO best_value is less than or equal to f evaluated at any seed point.
#[test]
fn pso_best_no_worse_than_uniform_seed() {
    let pso = MathPSO::new(1, vec![-1.0], vec![1.0], PSOParams::default());
    let f = |x: &[f64]| (x[0] - 0.3).powi(2);
    let res = pso.perform(f);
    let seed_val = f(&[0.0]);
    assert!(
        res.best_value <= seed_val + 1e-12,
        "PSO should not return worse than a seed point"
    );
}

/// Test 13: MathPSO works correctly on a 3D function.
#[test]
fn pso_3d_sum_of_squares() {
    let mut p = PSOParams::default();
    p.nb_particles = 64;
    p.max_iter = 300;
    let pso = MathPSO::new(
        3,
        vec![-3.0, -3.0, -3.0],
        vec![3.0, 3.0, 3.0],
        p,
    );
    let res = pso.perform(|x| x[0].powi(2) + x[1].powi(2) + x[2].powi(2));
    assert!(
        res.best_value < 0.2,
        "expected value near 0, got {}",
        res.best_value
    );
}

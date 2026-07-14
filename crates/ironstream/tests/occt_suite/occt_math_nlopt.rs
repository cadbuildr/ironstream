// FILE: tests/occt_math_nlopt.rs
extern crate ironstream;
use ironstream::math_nlopt::*;

#[test]
fn test_math_status_is_done() {
    assert!(MathStatus::Ok.is_done());
    assert!(!MathStatus::NotDone.is_done());
    assert!(!MathStatus::NoConvergence.is_done());
    assert!(!MathStatus::MaxIterReached.is_done());
}

#[test]
fn test_math_bfgs_perform() {
    let mut bfgs = MathBfgs::new(2, 1e-6, 100, 1.0);
    assert!(!bfgs.is_done());
    bfgs.perform(vec![1.0, 2.0]);
    assert!(bfgs.is_done());
    assert_eq!(bfgs.location().len(), 2);
    assert!((bfgs.minimum()).abs() < 1e-12);
}

#[test]
fn test_function_set_root_perform() {
    let mut fsr = FunctionSetRoot::new(2, 2, 1e-7).with_max_iter(50);
    fsr.perform(vec![0.5, 0.5]);
    assert!(fsr.is_done());
    assert_eq!(fsr.root().len(), 2);
    assert!((fsr.max_residual()).abs() < 1e-12);
}

#[test]
fn test_glob_opt_min_midpoint() {
    let mut gom = GlobOptMin::new(2, vec![0.0, 0.0], vec![2.0, 4.0])
        .with_sampling(200)
        .with_tolerance(1e-8);
    gom.perform();
    assert!(gom.is_done());
    assert_eq!(gom.nb_extrema(), 1);
    let pt = gom.point(0).unwrap();
    assert!((pt[0] - 1.0).abs() < 1e-12);
    assert!((pt[1] - 2.0).abs() < 1e-12);
    assert!((gom.minimum()).abs() < 1e-12);
}

#[test]
fn test_math_pso_perform() {
    let mut pso = MathPso::new(3, vec![0.0; 3], vec![2.0; 3]).with_particles(64);
    pso.perform();
    assert!(pso.is_done());
    let (pos, val) = pso.best();
    assert_eq!(pos.len(), 3);
    assert!((val).abs() < 1e-12);
    assert!((pos[0] - 1.0).abs() < 1e-12);
}

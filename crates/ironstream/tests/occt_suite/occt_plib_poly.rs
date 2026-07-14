// Integration tests for plib_poly
use ironstream::plib_poly::*;

#[test]
fn plib_eval_quadratic() {
    // p(t) = 1 + 0t + 3t^2 → p(2) = 1 + 12 = 13
    let v = PLib::eval(&[1.0, 0.0, 3.0], 2.0);
    assert!((v - 13.0).abs() < 1e-10);
}

#[test]
fn plib_bezier_basis_partition_of_unity() {
    for n in [1usize, 2, 3, 4] {
        for ti in 0..=10 {
            let t = ti as f64 / 10.0;
            let b = PLib::bezier_basis(n, t);
            let sum: f64 = b.iter().sum();
            assert!((sum - 1.0).abs() < 1e-10, "n={n}, t={t}");
        }
    }
}

#[test]
fn plib_bezier_eval_endpoints() {
    let poles = [[0.0, 0.0, 0.0], [1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
    let p0 = PLib::bezier_eval(&poles, 0.0);
    let p1 = PLib::bezier_eval(&poles, 1.0);
    assert!((p0[0] - 0.0).abs() < 1e-10);
    assert!((p1[0] - 4.0).abs() < 1e-10);
    assert!((p1[1] - 5.0).abs() < 1e-10);
    assert!((p1[2] - 6.0).abs() < 1e-10);
}

#[test]
fn jacobi_poly_p0_is_one() {
    let jp = PlibJacobiPolynomial::new(2, 0.5, 0.5);
    // P_0 = 1 always
    assert!((jp.value(0, 0.0) - 1.0).abs() < 1e-10);
    assert!((jp.value(0, 0.7) - 1.0).abs() < 1e-10);
}

#[test]
fn hermit_jacobi_built_when_degree_ge_order() {
    let hj_ok = PlibHermitJacobi::new(4, 2);
    assert!(hj_ok.is_built);
    let hj_bad = PlibHermitJacobi::new(1, 3);
    assert!(!hj_bad.is_built);
}

#[test]
fn poly_status_default_ok() {
    assert!(PolyStatus::Ok.is_ok());
    assert!(!PolyStatus::BadDegree.is_ok());
    assert_eq!(PolyStatus::default(), PolyStatus::Ok);
}

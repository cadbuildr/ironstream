// FILE: tests/occt_bisect_lib.rs
extern crate ironstream;

use ironstream::bisect_lib::{
    BisecType, BisecStatus, BisecResult, BisectorBisec, Mat2dCircuit, Mat2dConnexion,
};

#[test]
fn bisec_status_is_done() {
    assert!(BisecStatus::Done.is_done());
    assert!(!BisecStatus::NotDone.is_done());
    assert!(!BisecStatus::Error.is_done());
    assert!(!BisecStatus::NoSolution.is_done());
}

#[test]
fn bisec_perform_cc_success() {
    let mut b = BisectorBisec::new();
    b.perform_cc(1, 0.0, 2, 1.0, [0.5, 0.5], 1, 1e-7);
    assert!(b.is_done());
    assert_eq!(b.nb_bisectors(), 1);
    let id = b.curve_id().unwrap();
    // curve_id = geom1_id + geom2_id + 90000
    assert_eq!(id, 1 + 2 + 90000);
}

#[test]
fn bisec_perform_cc_error_on_zero_geom() {
    let mut b = BisectorBisec::new();
    b.perform_cc(0, 0.0, 5, 1.0, [0.0, 0.0], 1, 1e-7);
    assert_eq!(b.status, BisecStatus::Error);
    assert!(!b.is_done());
    assert_eq!(b.nb_bisectors(), 0);
}

#[test]
fn bisec_result_sense() {
    let mut b = BisectorBisec::new();
    b.perform_cc(3, 0.0, 4, 1.0, [1.0, 1.0], -1, 1e-7);
    assert!(b.is_done());
    assert_eq!(b.bisector(0).unwrap().sense, -1);
    assert!(b.bisector(1).is_none());
}

#[test]
fn mat2d_circuit_perform_and_access() {
    let mut c = Mat2dCircuit::new();
    c.add_geom(10);
    c.add_geom(20);
    c.add_geom(30);
    c.perform();
    assert_eq!(c.nb_geoms(), 3);
    assert_eq!(c.nb_connexions(), 3);
    assert!(c.is_closed());
    assert_eq!(c.geom_id(0), Some(10));
    assert_eq!(c.geom_id(2), Some(30));
    assert_eq!(c.geom_id(3), None);
}

#[test]
fn mat2d_connexion_fields() {
    let conn = Mat2dConnexion::new(2, 5, 3.14);
    assert!((conn.distance() - 3.14).abs() < 1e-10);
    assert_eq!(conn.prim1(), 2);
    assert_eq!(conn.prim2(), 5);
}

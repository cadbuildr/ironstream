// Integration tests for brep_fill_ext
use ironstream::brep_fill_ext::*;

#[test]
fn fill_filling_g0_error_bounded() {
    let mut f = BrepFillFilling::new();
    f.add_edge(10, FillOrder::G0);
    f.add_edge(11, FillOrder::G0);
    f.build();
    assert!(f.is_done());
    assert!(f.g0_error() > 0.0);
    assert!(f.g1_error() > 0.0);
}

#[test]
fn fill_filling_approx_params_applied() {
    let mut f = BrepFillFilling::new();
    f.set_approx_param(4, 20, 3);
    assert_eq!(f.degree, 4);
    assert_eq!(f.nb_pts_on_curve, 20);
    assert_eq!(f.nb_iter, 3);
}

#[test]
fn fill_filling_add_edge_with_face() {
    let mut f = BrepFillFilling::new();
    f.add_edge_with_face(5, 100, FillOrder::G1);
    f.build();
    assert!(f.is_done());
    assert_eq!(f.nb_edges(), 1);
}

#[test]
fn fill_generator_three_wires() {
    let mut g = BrepFillGenerator::new();
    g.add_wire(1);
    g.add_wire(2);
    g.add_wire(3);
    g.perform();
    assert!(g.is_done());
    assert_eq!(g.nb_wires(), 3);
    assert!(g.generated_face(1).is_some());
    assert!(g.generated_face(2).is_none()); // last wire index has no next
}

#[test]
fn curve_constraint_with_face_fields() {
    let mut c = BrepFillCurveConstraint::with_face(7, 42, FillOrder::G2);
    c.set_nb_points(25);
    c.set_tolerance(1e-6);
    c.set_inner(true);
    assert_eq!(c.order(), FillOrder::G2);
    assert!(c.is_inner());
    assert_eq!(c.nb_points, 25);
    assert_eq!(c.edge_id, 7);
    assert_eq!(c.face_id, 42);
}

#[test]
fn fill_status_done_is_done() {
    assert!(FillStatus::Done.is_done());
    assert!(!FillStatus::NotDone.is_done());
    assert!(!FillStatus::NoConstraints.is_done());
}

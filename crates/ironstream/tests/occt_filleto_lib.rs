// Integration tests for filleto_lib module
extern crate ironstream;

use ironstream::filleto_lib::*;

#[test]
fn fillet_status_default_and_predicates() {
    let s = FilletStatus::default();
    assert_eq!(s, FilletStatus::NotDone);
    assert!(!s.is_done());
    assert!(FilletStatus::Done.is_done());
}

#[test]
fn fillet_edge_constant_and_variable() {
    let e1 = FilletEdge::constant(10, 0.5);
    assert_eq!(e1.edge_id, 10);
    assert!((e1.radius - 0.5).abs() < 1e-9);
    assert!((e1.radius2 - 0.5).abs() < 1e-9);
    assert_eq!(e1.radius_type, FilletRadiusType::Constant);

    let e2 = FilletEdge::variable(11, 0.2, 0.8);
    assert!((e2.radius - 0.2).abs() < 1e-9);
    assert!((e2.radius2 - 0.8).abs() < 1e-9);
    assert_eq!(e2.radius_type, FilletRadiusType::Variable);
}

#[test]
fn make_fillet_basic_build() {
    let mut f = BrepFilletMakeFillet::new(1);
    assert!(f.add(10, 0.5));
    assert!(f.add(11, 0.5));
    assert!(!f.add(0, 0.5)); // zero edge_id rejected
    f.build();
    assert!(f.is_done());
    assert!(f.shape() > 0);
    assert_eq!(f.nb_faults(), 0);
    assert_eq!(f.nb_contours(), 2);
    assert_eq!(f.radius(10), Some(0.5));
    assert_eq!(f.radius(99), None);
}

#[test]
fn make_fillet_error_cases() {
    // no solid
    let mut f1 = BrepFilletMakeFillet::new(0);
    f1.add(10, 0.5);
    f1.build();
    assert_eq!(f1.status, FilletStatus::NoSolid);

    // no edges
    let mut f2 = BrepFilletMakeFillet::new(1);
    f2.build();
    assert_eq!(f2.status, FilletStatus::BadEdge);

    // non-positive radius
    let mut f3 = BrepFilletMakeFillet::new(1);
    f3.add_variable(5, -1.0, 1.0);
    f3.build();
    assert_eq!(f3.status, FilletStatus::BadEdge);
}

#[test]
fn make_fillet_remove_edge() {
    let mut f = BrepFilletMakeFillet::new(1);
    f.add(10, 0.5);
    f.add(11, 0.5);
    assert!(f.remove_edge(10));
    assert!(!f.remove_edge(99)); // not present
    f.build();
    assert!(f.is_done());
    assert_eq!(f.nb_edges_in_contour(0), 1);
    assert_eq!(f.radius(11), Some(0.5));
    assert_eq!(f.radius(10), None);
}

#[test]
fn make_chamfer_two_dist_and_dist_angle() {
    let mut c1 = BrepFilletMakeChamfer::new(2);
    c1.add_with_two_dist(10, 20, 0.3, 0.4);
    c1.build();
    assert!(c1.is_done());
    assert!(c1.shape() > 0);
    assert_eq!(c1.nb_edges(), 1);

    let mut c2 = BrepFilletMakeChamfer::new(3);
    c2.add_with_dist_angle(10, 20, 0.5, 45.0_f64.to_radians());
    c2.build();
    assert!(c2.is_done());
    assert_eq!(c2.nb_edges(), 1);

    // no solid
    let mut c3 = BrepFilletMakeChamfer::new(0);
    c3.add_with_two_dist(1, 2, 0.1, 0.1);
    c3.build();
    assert_eq!(c3.status, FilletStatus::NoSolid);
}

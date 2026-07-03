extern crate ironstream;
use ironstream::chfi3d_fillet::*;

#[test]
fn test_chfi3d_status_predicates() {
    assert!(Chfi3dStatus::Done.is_done());
    assert!(!Chfi3dStatus::NotDone.is_done());
    assert!(Chfi3dStatus::FailTopology.is_fail());
    assert!(Chfi3dStatus::FailSmallRadius.is_fail());
    assert!(!Chfi3dStatus::Done.is_fail());
    assert!(!Chfi3dStatus::NotDone.is_fail());
}

#[test]
fn test_fillet_edge_spec_constant_and_variable() {
    let s = FilletEdgeSpec::new(10, 0.5);
    assert_eq!(s.edge_id, 10);
    assert!((s.radius - 0.5).abs() < 1e-10);
    assert!(!s.is_variable);

    let sv = FilletEdgeSpec::variable(10, 0.2, 0.8);
    assert!(sv.is_variable);
    assert!((sv.radius - 0.2).abs() < 1e-10);
    assert!((sv.radius2 - 0.8).abs() < 1e-10);
}

#[test]
fn test_chfi3d_builder_add_remove_edge() {
    let mut b = Chfi3dBuilder::new(1);
    b.add_edge(10, 0.5);
    b.add_edge(11, 0.3);
    assert_eq!(b.nb_contours(), 2);
    b.remove_edge(10);
    assert_eq!(b.nb_contours(), 1);
    assert_eq!(b.edge_specs[0].edge_id, 11);
    // zero radius edges should not be added
    b.add_edge(12, 0.0);
    assert_eq!(b.nb_contours(), 1);
}

#[test]
fn test_chfi3d_builder_build() {
    let mut b = Chfi3dBuilder::new(5);
    b.add_edge(20, 1.0);
    b.build();
    assert!(b.is_done());
    assert!(b.shape() > 0);
    // contour accessor
    let spec = b.contour(0).unwrap();
    assert_eq!(spec.edge_id, 20);
    assert!(b.contour(99).is_none());
}

#[test]
fn test_chfi3d_builder_null_solid() {
    let mut b = Chfi3dBuilder::new(0);
    b.add_edge(10, 0.5);
    b.build();
    assert!(!b.is_done());
    assert_eq!(b.status(), Chfi3dStatus::FailTopology);
}

#[test]
fn test_chfi3d_chamfer_builder_and_fillet_algo() {
    // ChamferMode enum
    assert_eq!(ChamferMode::default(), ChamferMode::ClassicChamfer);

    // Chfi3dChBuilder
    let mut ch = Chfi3dChBuilder::new(3);
    ch.add_edge_equal(5, 0.3);
    ch.add_edge_equal(6, 0.3);
    ch.build();
    assert!(ch.is_done());
    assert_eq!(ch.nb_contours(), 2);

    // Chfi3dFilletAlgo with variable radius
    let mut fa = Chfi3dFilletAlgo::new(1);
    fa.set_radius_variable(10, 0.2, 0.5);
    fa.build();
    assert!(fa.is_done());
    let spec = fa.builder.contour(0).unwrap();
    assert!(spec.is_variable);
    assert!((spec.radius - 0.2).abs() < 1e-10);
    assert!((spec.radius2 - 0.5).abs() < 1e-10);
}

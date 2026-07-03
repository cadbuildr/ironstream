use ironstream::nlplate_ext::*;

#[test]
fn constraint_order_default_is_g0() {
    assert_eq!(ConstraintOrder::default(), ConstraintOrder::G0);
}

#[test]
fn hgpp_constraint_weight() {
    let mut c = HGPPConstraint::new([0.5, 0.5], [0.0, 0.0, 5.0]);
    c.set_weight(2.5);
    assert!((c.weight - 2.5).abs() < 1e-10);
    c.set_g1_free(true);
    assert!(c.is_g1_free);
    c.set_g2_free(true);
    assert!(c.is_g2_free);
}

#[test]
fn nlplate_with_mixed_constraints() {
    let mut plate = NLPlate::new();
    plate.load_constraint_g0(HPG0Constraint::new([0.0, 0.0], [0.0, 0.0, 0.0]));
    plate.load_constraint_g1(HPG1Constraint::new([1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]));
    let c = HGPPConstraint::new([0.5, 0.5], [0.5, 0.5, 0.0]).with_normal([0.0, 0.0, 1.0]);
    plate.load_constraint_gpp(c);
    plate.perform(2);
    assert!(plate.is_done());
    assert_eq!(plate.nb_constraints(), 3);
    assert_eq!(plate.nb_iter(), 2);
}

#[test]
fn nlplate_empty_error_is_zero() {
    let mut plate = NLPlate::new();
    plate.perform(1);
    // no constraints => max_error == 0.0
    assert!((plate.max_error() - 0.0).abs() < 1e-15);
}

#[test]
fn hg0_constraint_nb_uv_params() {
    let c = HPG0Constraint::new([0.1, 0.9], [5.0, 6.0, 7.0]);
    assert_eq!(c.nb_uv_params(), 1);
    assert!(!c.is_free());
}

#[test]
fn hg1_constraint_tangent_data() {
    let c = HPG1Constraint::new([0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]);
    assert_eq!(c.active_order(), ConstraintOrder::G1);
    assert_eq!(c.g0_target(), [0.0, 1.0, 0.0]);
    assert_eq!(c.g1_target(), [1.0, 0.0, 0.0]);
    assert_eq!(c.uv_params(), [0.0, 1.0]);
}

extern crate ironstream;
use ironstream::geom_nplate::*;

#[test]
fn test_geom_nplate_basic() {
    // ConstraintOrder variants
    assert_ne!(ConstraintOrder::G0, ConstraintOrder::G1);
    assert_ne!(ConstraintOrder::G1, ConstraintOrder::G2);

    // NlPtConstraint construction
    let c = NlPtConstraint::new([0.5, 0.5], [1.0, 2.0, 3.0]);
    assert_eq!(c.uv, [0.5, 0.5]);
    assert_eq!(c.point, [1.0, 2.0, 3.0]);
    assert_eq!(c.order, ConstraintOrder::G0);

    // with_order builder
    let c2 = NlPtConstraint::new([0.0, 0.0], [0.0, 0.0, 0.0]).with_order(ConstraintOrder::G1);
    assert_eq!(c2.order, ConstraintOrder::G1);

    // NlPlate - empty
    let mut plate = NlPlate::new();
    assert!(!plate.is_solved());

    // add constraints and solve
    plate.add_constraint(NlPtConstraint::new([0.0, 0.0], [0.0, 0.0, 0.0]));
    plate.add_constraint(NlPtConstraint::new([1.0, 0.0], [1.0, 0.0, 0.0]));
    plate.add_constraint(NlPtConstraint::new([0.5, 1.0], [0.5, 0.5, 1.0]));

    let solved = plate.solve(100, 1e-6);
    assert!(solved);
    assert!(plate.is_solved());

    // evaluate should return something
    let p = plate.evaluate(0.5, 0.5);
    assert_eq!(p.len(), 3);

    // build_nl_plate convenience function
    let constraints = vec![
        NlPtConstraint::new([0.0, 0.0], [0.0, 0.0, 0.0]),
        NlPtConstraint::new([1.0, 1.0], [1.0, 1.0, 0.0]),
    ];
    let plate2 = build_nl_plate(&constraints);
    assert!(plate2.is_solved());

    // Default
    let _d = NlPlate::default();
}

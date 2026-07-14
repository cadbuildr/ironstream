extern crate ironstream;
use ironstream::geom_surf_fill::*;

#[test]
fn bspline_curves_fill_build() {
    let mut f = BSplineCurvesFill::new(FillingStyle::Coons);
    f.add_curve(vec![[0.0,0.0,0.0],[1.0,0.0,0.0]]);
    f.add_curve(vec![[0.0,1.0,0.0],[1.0,1.0,0.0]]);
    f.build();
    assert!(f.is_done());
    assert_eq!(f.nb_curves(), 2);
}

#[test]
fn constrained_filling_two_boundaries() {
    let mut cf = ConstrainedFilling::new();
    cf.add_boundary(FillingBoundary::new(
        vec![[0.0,0.0,0.0],[1.0,0.0,0.0]],
        FillingConstraint::G0Constraint,
    ));
    cf.add_boundary(FillingBoundary::new(
        vec![[0.0,1.0,0.0],[1.0,1.0,0.0]],
        FillingConstraint::G1Constraint,
    ));
    cf.build();
    assert!(cf.is_done());
    assert_eq!(cf.nb_boundaries(), 2);
}

#[test]
fn simple_bound_degenerated() {
    let b_empty = SimpleBound::new(vec![], 1e-6, 1e-6);
    assert!(b_empty.is_degenerated());
    let b_ok = SimpleBound::new(vec![[0.0,0.0,0.0],[1.0,0.0,0.0]], 1e-6, 1e-6);
    assert!(!b_ok.is_degenerated());
}

#[test]
fn coons_patch_corners_and_center() {
    let c0 = vec![[0.0,0.0,0.0],[1.0,0.0,0.0]];
    let c1 = vec![[0.0,1.0,0.0],[1.0,1.0,0.0]];
    let c2 = vec![[0.0,0.0,0.0],[0.0,1.0,0.0]];
    let c3 = vec![[1.0,0.0,0.0],[1.0,1.0,0.0]];
    let patch = CoonsAlgPatch::new(c0, c1, c2, c3);
    let corner = patch.evaluate(0.0, 0.0);
    assert!((corner[0]).abs() < 1e-8 && (corner[1]).abs() < 1e-8);
}

#[test]
fn filling_style_equality() {
    assert_eq!(FillingStyle::Stretch, FillingStyle::Stretch);
    assert_ne!(FillingStyle::Curved, FillingStyle::Coons);
    assert_eq!(FillingConstraint::G2Constraint, FillingConstraint::G2Constraint);
}

extern crate ironstream;
use ironstream::shape_prim_api::*;

#[test]
fn make_box_surface_area_for_a_cube() {
    // Cube with side 3: surface area = 6 * 3^2 = 54
    let b = MakeBox::new(3.0, 3.0, 3.0);
    assert!(b.is_done());
    let sa = b.surface_area();
    assert!((sa - 54.0).abs() < 1e-10);
}

#[test]
fn make_cylinder_partial_is_partial_returns_true() {
    let half_turn = std::f64::consts::PI; // π radians = 180°
    let cyl = MakeCylinder::partial(1.0, 2.0, half_turn);
    assert!(cyl.is_done());
    assert!(cyl.is_partial());
    // Full cylinder should not be partial
    let full = MakeCylinder::new(1.0, 2.0);
    assert!(!full.is_partial());
}

#[test]
fn make_cylinder_lateral_area_formula_check() {
    // lateral_area = radius * angle * height
    let cyl = MakeCylinder::new(2.0, 5.0);
    assert!(cyl.is_done());
    let expected = 2.0 * 2.0 * std::f64::consts::PI * 5.0;
    assert!((cyl.lateral_area() - expected).abs() < 1e-8);
}

#[test]
fn make_sphere_surface_area() {
    // surface_area = 4 * PI * r^2
    let s = MakeSphere::new(3.0);
    assert!(s.is_done());
    let expected = 4.0 * std::f64::consts::PI * 9.0;
    assert!((s.surface_area() - expected).abs() < 1e-8);
}

#[test]
fn make_torus_with_minor_greater_or_equal_to_major_returns_not_done() {
    // minor >= major violates the constraint minor < major
    let t_equal = MakeTorus::new(2.0, 2.0);
    assert!(!t_equal.is_done());
    let t_larger = MakeTorus::new(1.0, 3.0);
    assert!(!t_larger.is_done());
    // Valid: minor strictly less than major
    let t_valid = MakeTorus::new(3.0, 1.0);
    assert!(t_valid.is_done());
}

#[test]
fn make_cone_with_both_radii_zero_returns_not_done() {
    let c = MakeCone::new(0.0, 0.0, 5.0);
    assert!(!c.is_done());
    assert_eq!(c.status, PrimStatus::NegativeDimension);
    // One nonzero radius is fine
    let c2 = MakeCone::new(0.0, 2.0, 5.0);
    assert!(c2.is_done());
}

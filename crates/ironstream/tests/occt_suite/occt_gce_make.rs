extern crate ironstream;
use ironstream::gce_make::*;

#[test]
fn gce_make_ax1_from_point_dir_non_unit_normalizes() {
    // direction (3, 4, 0) has magnitude 5; should be normalized to (0.6, 0.8, 0)
    let ax = GceMakeAx1::from_point_dir([1.0, 2.0, 3.0], [3.0, 4.0, 0.0]);
    assert!(ax.is_done());
    let d = ax.direction();
    assert!((d[0] - 0.6).abs() < 1e-10);
    assert!((d[1] - 0.8).abs() < 1e-10);
    assert!((d[2]).abs() < 1e-10);
    // magnitude should be exactly 1
    let mag = (d[0]*d[0] + d[1]*d[1] + d[2]*d[2]).sqrt();
    assert!((mag - 1.0).abs() < 1e-10);
}

#[test]
fn gce_make_circ_from_three_points_computes_circumradius() {
    // Equilateral triangle inscribed in circle of radius 1
    // For a right triangle with legs 3 and 4, hypotenuse = 5, circumradius = 5/2 = 2.5
    let p1 = [0.0_f64, 0.0, 0.0];
    let p2 = [5.0, 0.0, 0.0];
    let p3 = [0.0, 0.0, 4.0]; // right angle at p1 in XZ plane, but need 3 non-colinear in a plane
    // Use XY plane: right angle triangle (0,0),(3,0),(0,4)
    let a = [0.0_f64, 0.0, 0.0];
    let b = [3.0, 0.0, 0.0];
    let c = [0.0, 4.0, 0.0];
    let circ = GceMakeCirc::from_three_points(a, b, c);
    assert!(circ.is_done());
    // For right triangle with hypotenuse 5, circumradius = 5/2 = 2.5
    assert!((circ.radius - 2.5).abs() < 1e-8);
}

#[test]
fn gce_make_circ_circumference_for_known_radius() {
    let c = GceMakeCirc::from_center_normal_radius([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 2.0);
    assert!(c.is_done());
    let circ = c.circumference();
    let expected = 2.0 * std::f64::consts::PI * 2.0;
    assert!((circ - expected).abs() < 1e-10);
}

#[test]
fn gce_make_pln_from_equation_creates_plane() {
    // Plane z = 1: 0x + 0y + 1z - 1 = 0 => a=0, b=0, c=1, d=-1
    let pln = GceMakePln::from_equation(0.0, 0.0, 1.0, -1.0);
    assert!(pln.is_done());
    // Normal should be (0, 0, 1)
    assert!((pln.normal[2] - 1.0).abs() < 1e-10);
    // A point on the plane z=1 should be contained
    assert!(pln.contains_point([0.0, 0.0, 1.0], 1e-8));
    assert!(pln.contains_point([5.0, -3.0, 1.0], 1e-8));
}

#[test]
fn gce_make_pln_contains_point_rejects_far_off_point() {
    let pln = GceMakePln::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    assert!(pln.is_done());
    // point (0, 0, 5) is 5 units away from the XY plane
    assert!(!pln.contains_point([0.0, 0.0, 5.0], 1e-6));
    // point on the plane
    assert!(pln.contains_point([10.0, -7.0, 0.0], 1e-6));
}

#[test]
fn gce_make_lin_from_point_dir_with_unit_x_axis() {
    let l = GceMakeLin::from_point_dir([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    assert!(l.is_done());
    assert!((l.direction[0] - 1.0).abs() < 1e-10);
    assert!((l.direction[1]).abs() < 1e-10);
    assert!((l.direction[2]).abs() < 1e-10);
    // Point at t=3 should be (3, 0, 0)
    let pt = l.point_at(3.0);
    assert!((pt[0] - 3.0).abs() < 1e-10);
    assert!((pt[1]).abs() < 1e-10);
}

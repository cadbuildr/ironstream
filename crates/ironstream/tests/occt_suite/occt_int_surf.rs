// FILE: rust/ironstream/crates/ironstream/tests/occt_int_surf.rs
extern crate ironstream;
use ironstream::int_surf::*;

// --- IntSurfPntOn2S ---

#[test]
fn pnt_on_2s_new_and_point() {
    let p = IntSurfPntOn2S::new([1.0, 2.0, 3.0], 0.1, 0.2, 0.3, 0.4);
    assert_eq!(p.point(), [1.0, 2.0, 3.0]);
}

#[test]
fn pnt_on_2s_parameters() {
    let p = IntSurfPntOn2S::new([0.0, 0.0, 0.0], 0.1, 0.2, 0.3, 0.4);
    assert_eq!(p.parameters(), (0.1, 0.2, 0.3, 0.4));
}

#[test]
fn pnt_on_2s_set_value() {
    let mut p = IntSurfPntOn2S::new([0.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0);
    p.set_value([5.0, 6.0, 7.0], 0.5, 0.6, 0.7, 0.8);
    assert_eq!(p.point(), [5.0, 6.0, 7.0]);
    assert_eq!(p.parameters(), (0.5, 0.6, 0.7, 0.8));
}

#[test]
fn pnt_on_2s_fields_accessible() {
    let p = IntSurfPntOn2S::new([1.0, 2.0, 3.0], 0.1, 0.2, 0.3, 0.4);
    assert_eq!(p.pt, [1.0, 2.0, 3.0]);
    assert_eq!(p.u1, 0.1);
    assert_eq!(p.v1, 0.2);
    assert_eq!(p.u2, 0.3);
    assert_eq!(p.v2, 0.4);
}

#[test]
fn pnt_on_2s_distance_zero() {
    let a = IntSurfPntOn2S::new([1.0, 1.0, 1.0], 0.0, 0.0, 0.0, 0.0);
    let b = IntSurfPntOn2S::new([1.0, 1.0, 1.0], 0.0, 0.0, 0.0, 0.0);
    assert_eq!(a.distance(&b), 0.0);
}

#[test]
fn pnt_on_2s_distance_unit() {
    let a = IntSurfPntOn2S::new([0.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0);
    let b = IntSurfPntOn2S::new([1.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0);
    assert!((a.distance(&b) - 1.0).abs() < 1e-12);
}

#[test]
fn pnt_on_2s_distance_3d() {
    let a = IntSurfPntOn2S::new([0.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0);
    let b = IntSurfPntOn2S::new([1.0, 2.0, 2.0], 0.0, 0.0, 0.0, 0.0);
    assert!((a.distance(&b) - 3.0).abs() < 1e-12);
}

// --- IntSurfTransition ---

#[test]
fn transition_variants_exist() {
    let _ = IntSurfTransition::Unknown;
    let _ = IntSurfTransition::Touch;
    let _ = IntSurfTransition::Undecided;
    let _ = IntSurfTransition::In;
    let _ = IntSurfTransition::Out;
}

#[test]
fn transition_trans_s1_returns_self() {
    assert_eq!(IntSurfTransition::In.trans_s1(), IntSurfTransition::In);
    assert_eq!(IntSurfTransition::Out.trans_s1(), IntSurfTransition::Out);
    assert_eq!(IntSurfTransition::Touch.trans_s1(), IntSurfTransition::Touch);
    assert_eq!(IntSurfTransition::Unknown.trans_s1(), IntSurfTransition::Unknown);
    assert_eq!(IntSurfTransition::Undecided.trans_s1(), IntSurfTransition::Undecided);
}

#[test]
fn transition_trans_s2_inverts_in_out() {
    assert_eq!(IntSurfTransition::In.trans_s2(), IntSurfTransition::Out);
    assert_eq!(IntSurfTransition::Out.trans_s2(), IntSurfTransition::In);
}

#[test]
fn transition_trans_s2_passthrough_other() {
    assert_eq!(IntSurfTransition::Touch.trans_s2(), IntSurfTransition::Touch);
    assert_eq!(IntSurfTransition::Unknown.trans_s2(), IntSurfTransition::Unknown);
    assert_eq!(IntSurfTransition::Undecided.trans_s2(), IntSurfTransition::Undecided);
}

// --- IntSurfLineOn2S ---

#[test]
fn line_on_2s_empty() {
    let line = IntSurfLineOn2S::new();
    assert_eq!(line.nb_points(), 0);
    assert!(line.first().is_none());
    assert!(line.last().is_none());
}

#[test]
fn line_on_2s_add_and_nb_points() {
    let mut line = IntSurfLineOn2S::new();
    line.add(IntSurfPntOn2S::new([0.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0));
    line.add(IntSurfPntOn2S::new([1.0, 0.0, 0.0], 0.5, 0.5, 0.5, 0.5));
    assert_eq!(line.nb_points(), 2);
}

#[test]
fn line_on_2s_value_0based() {
    let mut line = IntSurfLineOn2S::new();
    line.add(IntSurfPntOn2S::new([0.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0));
    line.add(IntSurfPntOn2S::new([1.0, 0.0, 0.0], 0.5, 0.5, 0.5, 0.5));
    line.add(IntSurfPntOn2S::new([2.0, 0.0, 0.0], 1.0, 1.0, 1.0, 1.0));
    assert_eq!(line.value(0).pt, [0.0, 0.0, 0.0]);
    assert_eq!(line.value(1).pt, [1.0, 0.0, 0.0]);
    assert_eq!(line.value(2).pt, [2.0, 0.0, 0.0]);
}

#[test]
fn line_on_2s_value_returns_ref() {
    let mut line = IntSurfLineOn2S::new();
    line.add(IntSurfPntOn2S::new([3.0, 4.0, 5.0], 0.1, 0.2, 0.3, 0.4));
    let p: &IntSurfPntOn2S = line.value(0);
    assert_eq!(p.point(), [3.0, 4.0, 5.0]);
    assert_eq!(p.parameters(), (0.1, 0.2, 0.3, 0.4));
}

#[test]
fn line_on_2s_reverse() {
    let mut line = IntSurfLineOn2S::new();
    line.add(IntSurfPntOn2S::new([0.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0));
    line.add(IntSurfPntOn2S::new([1.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0));
    line.add(IntSurfPntOn2S::new([2.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0));
    line.reverse();
    assert_eq!(line.value(0).pt, [2.0, 0.0, 0.0]);
    assert_eq!(line.value(1).pt, [1.0, 0.0, 0.0]);
    assert_eq!(line.value(2).pt, [0.0, 0.0, 0.0]);
}

#[test]
fn line_on_2s_first_last() {
    let mut line = IntSurfLineOn2S::new();
    line.add(IntSurfPntOn2S::new([10.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0));
    line.add(IntSurfPntOn2S::new([20.0, 0.0, 0.0], 1.0, 1.0, 1.0, 1.0));
    assert_eq!(line.first().unwrap().pt, [10.0, 0.0, 0.0]);
    assert_eq!(line.last().unwrap().pt, [20.0, 0.0, 0.0]);
}

#[test]
fn line_on_2s_default() {
    let line = IntSurfLineOn2S::default();
    assert_eq!(line.nb_points(), 0);
}

// --- retained extras: IntSurfOrientation, IntSurfInteriorPoint, IntSurfQuadric ---

#[test]
fn orientation_variants() {
    assert_eq!(IntSurfOrientation::Inside, IntSurfOrientation::Inside);
    assert_ne!(IntSurfOrientation::Inside, IntSurfOrientation::Outside);
    let _ = IntSurfOrientation::Unknown;
}

#[test]
fn interior_point_fields() {
    let pnt = IntSurfPntOn2S::new([3.0, 4.0, 5.0], 0.1, 0.2, 0.3, 0.4);
    let tan = [0.0, 1.0, 0.0];
    let ip = IntSurfInteriorPoint::new(pnt, tan);
    assert_eq!(ip.pnt.pt, [3.0, 4.0, 5.0]);
    assert_eq!(ip.tangent, [0.0, 1.0, 0.0]);
}

#[test]
fn quadric_plane() {
    let q = IntSurfQuadric::plane();
    assert_eq!(q.kind, IntSurfQuadricKind::Plane);
    assert_eq!(q.radius, 0.0);
}

#[test]
fn quadric_cylinder() {
    let q = IntSurfQuadric::cylinder(5.0);
    assert_eq!(q.kind, IntSurfQuadricKind::Cylinder);
    assert_eq!(q.radius, 5.0);
}

#[test]
fn quadric_sphere() {
    let q = IntSurfQuadric::sphere(3.5);
    assert_eq!(q.kind, IntSurfQuadricKind::Sphere);
    assert_eq!(q.radius, 3.5);
}

#[test]
fn quadric_cone() {
    let q = IntSurfQuadric::cone(0.785, 2.0);
    assert_eq!(q.kind, IntSurfQuadricKind::Cone);
    assert!((q.half_angle - 0.785).abs() < 1e-12);
    assert_eq!(q.radius, 2.0);
}

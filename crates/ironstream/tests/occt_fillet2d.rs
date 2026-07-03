// FILE: tests/occt_fillet2d.rs
extern crate ironstream;
use ironstream::fillet2d::*;
use std::f64::consts::PI;

#[test]
fn fillet_params_roundtrip_radius() {
    let params = FilletParams2d {
        p1: [-1.0, 0.0],
        corner: [0.0, 0.0],
        p2: [0.0, 1.0],
        radius: 4.0,
    };
    let mut b = Fillet2dBuilder::new();
    b.set_params(params);
    let arc = b.perform().unwrap();
    assert!((arc.radius - 4.0).abs() < 1e-12);
}

#[test]
fn fillet_center_is_corner_in_stub() {
    let corner = [3.0, 7.0];
    let mut b = Fillet2dBuilder::new();
    b.set_params(FilletParams2d {
        p1: [0.0, 7.0],
        corner,
        p2: [3.0, 0.0],
        radius: 1.0,
    });
    let arc = b.perform().unwrap();
    assert!((arc.center[0] - corner[0]).abs() < 1e-12);
    assert!((arc.center[1] - corner[1]).abs() < 1e-12);
}

#[test]
fn fillet_arc_ccw_flag() {
    let mut b = Fillet2dBuilder::new();
    b.set_params(FilletParams2d {
        p1: [0.0, 0.0],
        corner: [1.0, 0.0],
        p2: [1.0, 1.0],
        radius: 0.5,
    });
    let arc = b.perform().unwrap();
    assert!(arc.ccw);
}

#[test]
fn fillet_arc_angles_span_pi() {
    let mut b = Fillet2dBuilder::new();
    b.set_params(FilletParams2d {
        p1: [0.0, 0.0],
        corner: [1.0, 0.0],
        p2: [1.0, 1.0],
        radius: 0.5,
    });
    let arc = b.perform().unwrap();
    assert!((arc.start_angle - 0.0).abs() < 1e-12);
    assert!((arc.end_angle - PI).abs() < 1e-12);
}

#[test]
fn arc_length_quarter_circle() {
    let arc = Fillet2dArc {
        center: [0.0, 0.0],
        radius: 1.0,
        start_angle: 0.0,
        end_angle: PI / 2.0,
        ccw: true,
    };
    assert!((arc.arc_length() - PI / 2.0).abs() < 1e-12);
}

#[test]
fn arc_length_full_circle() {
    let arc = Fillet2dArc {
        center: [0.0, 0.0],
        radius: 3.0,
        start_angle: 0.0,
        end_angle: 2.0 * PI,
        ccw: true,
    };
    assert!((arc.arc_length() - 6.0 * PI).abs() < 1e-12);
}

#[test]
fn fillet_builder_missing_params_is_error() {
    let b = Fillet2dBuilder::new();
    assert_eq!(b.perform().unwrap_err(), Fillet2dError::BadInput);
}

#[test]
fn fillet_builder_default_is_equivalent_to_new() {
    let b: Fillet2dBuilder = Default::default();
    assert_eq!(b.perform().unwrap_err(), Fillet2dError::BadInput);
}

#[test]
fn chamfer_missing_corner_is_error() {
    let b = Chamfer2dBuilder::new();
    assert_eq!(b.perform().unwrap_err(), Fillet2dError::BadInput);
}

#[test]
fn chamfer_points_lie_at_correct_distance() {
    let mut b = Chamfer2dBuilder::new();
    // Corner at origin; p1 along +x, p2 along +y.
    b.set_corner([1.0, 0.0], [0.0, 0.0], [0.0, 1.0]);
    b.set_distance(0.5);
    let (a, c) = b.perform().unwrap();
    // a should be 0.5 along [0,0]->[1,0] from [0,0]: (0.5, 0)
    assert!((a[0] - 0.5).abs() < 1e-12, "a.x={}", a[0]);
    assert!((a[1] - 0.0).abs() < 1e-12, "a.y={}", a[1]);
    // c should be 0.5 along [0,0]->[0,1] from [0,0]: (0, 0.5)
    assert!((c[0] - 0.0).abs() < 1e-12, "c.x={}", c[0]);
    assert!((c[1] - 0.5).abs() < 1e-12, "c.y={}", c[1]);
}

#[test]
fn chamfer_distance_zero_returns_corner() {
    let mut b = Chamfer2dBuilder::new();
    b.set_corner([2.0, 0.0], [0.0, 0.0], [0.0, 2.0]);
    b.set_distance(0.0);
    let (a, c) = b.perform().unwrap();
    assert!((a[0] - 0.0).abs() < 1e-12);
    assert!((a[1] - 0.0).abs() < 1e-12);
    assert!((c[0] - 0.0).abs() < 1e-12);
    assert!((c[1] - 0.0).abs() < 1e-12);
}

#[test]
fn chamfer_default_is_bad_input() {
    let b: Chamfer2dBuilder = Default::default();
    assert_eq!(b.perform().unwrap_err(), Fillet2dError::BadInput);
}

#[test]
fn error_variants_are_copy() {
    let e = Fillet2dError::DistanceTooLarge;
    let e2 = e;
    assert_eq!(e, e2);
}

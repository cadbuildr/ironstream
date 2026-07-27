// FILE: tests/occt_ais_axis.rs
extern crate ironstream;
use ironstream::ais_axis::*;

#[test]
fn axis_end_point() {
    // Default length is 100; direction z → end z = 100
    let a = AisAxis::new(1, [0.0; 3], [0.0, 0.0, 1.0]);
    let ep = a.end_point();
    assert!((ep[0]).abs() < 1e-10);
    assert!((ep[1]).abs() < 1e-10);
    assert!((ep[2] - 100.0).abs() < 1e-10);
}

#[test]
fn axis_normalizes_direction() {
    // Supply direction [0,0,5] → normalised to [0,0,1]
    let a = AisAxis::new(1, [0.0; 3], [0.0, 0.0, 5.0]);
    assert!((a.direction[0]).abs() < 1e-10);
    assert!((a.direction[1]).abs() < 1e-10);
    assert!((a.direction[2] - 1.0).abs() < 1e-10);
}

#[test]
fn point_marker() {
    let mut p = AisPoint::new(2, [1.0, 2.0, 3.0]);
    p.set_marker(AisMarkerType::Star);
    assert_eq!(p.marker_type, AisMarkerType::Star);
    assert!(p.is_visible());
}

#[test]
fn circle_arc_length() {
    use std::f64::consts::PI;
    let mut c = AisCircle::new(3, [0.0; 3], [0.0, 0.0, 1.0], 5.0);
    c.set_range(0.0, PI);
    assert!(c.is_arc());
    // arc_length = r * |end - start| = 5 * PI
    assert!((c.arc_length() - 5.0 * PI).abs() < 1e-9);
}

#[test]
fn circle_full_not_arc() {
    // Default param_end = TAU → not an arc
    let c = AisCircle::new(3, [0.0; 3], [0.0, 0.0, 1.0], 1.0);
    assert!(!c.is_arc());
}

#[test]
fn connected_interactive_translation() {
    let mut ci = AisConnectedInteractive::new(4, 10);
    assert!(ci.is_connected());
    ci.set_translation(1.0, 2.0, 3.0);
    let t = ci.translation();
    assert!((t[0] - 1.0).abs() < 1e-10);
    assert!((t[1] - 2.0).abs() < 1e-10);
    assert!((t[2] - 3.0).abs() < 1e-10);
}

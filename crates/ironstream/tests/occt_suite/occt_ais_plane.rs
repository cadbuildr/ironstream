// FILE: tests/occt_ais_plane.rs
extern crate ironstream;
use ironstream::ais_plane::*;

#[test]
fn ais_plane_four_corners_in_plane() {
    let p = AisPlane::new([0.0; 3], [0.0, 0.0, 1.0], 4.0);
    let corners = p.corners();
    assert_eq!(corners.len(), 4);
    for c in &corners {
        assert!(c[2].abs() < 1e-10, "corner not in z=0 plane");
    }
}

#[test]
fn ais_plane_project_onto_xy() {
    let p = AisPlane::new([0.0; 3], [0.0, 0.0, 1.0], 4.0);
    let proj = p.project_point([3.0, -2.0, 7.0]);
    assert!((proj[0] - 3.0).abs() < 1e-10);
    assert!((proj[1] + 2.0).abs() < 1e-10);
    assert!(proj[2].abs() < 1e-10);
}

#[test]
fn ais_plane_contains_point() {
    let p = AisPlane::new([0.0; 3], [0.0, 0.0, 1.0], 10.0);
    assert!(p.contains_point([5.0, 5.0, 0.0], 1e-9));
    assert!(!p.contains_point([0.0, 0.0, 1.0], 1e-9));
}

#[test]
fn ais_circle_perimeter_and_area() {
    let c = AisCircle::new([0.0; 3], [0.0, 0.0, 1.0], 5.0);
    assert!(c.is_full_circle());
    let expected_perim = 2.0 * std::f64::consts::PI * 5.0;
    assert!((c.perimeter() - expected_perim).abs() < 1e-8);
}

#[test]
fn ais_circle_arc_not_full() {
    let c = AisCircle::arc([0.0; 3], [0.0, 0.0, 1.0], 3.0, 0.0, std::f64::consts::PI);
    assert!(!c.is_full_circle());
    let expected_len = 3.0 * std::f64::consts::PI;
    assert!((c.perimeter() - expected_len).abs() < 1e-8);
}

#[test]
fn ais_line_segment_length() {
    let l = AisLine::segment([0.0, 0.0, 0.0], [3.0, 4.0, 0.0]);
    assert!((l.length() - 5.0).abs() < 1e-10);
    assert!(!l.is_infinite);
}

#[test]
fn ais_point_builder() {
    let pt = AisPoint::new([1.0, 2.0, 3.0])
        .with_marker(PointMarker::Star)
        .with_size(8.0);
    assert_eq!(pt.marker_type, PointMarker::Star);
    assert!((pt.marker_size - 8.0).abs() < 1e-6);
}

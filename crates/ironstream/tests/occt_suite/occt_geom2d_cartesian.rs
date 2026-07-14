// FILE: tests/occt_geom2d_cartesian.rs
extern crate ironstream;
use ironstream::geom2d_cartesian::*;
use ironstream::geom2d_circle::Dir2d;
use ironstream::gp2d::{Pnt2d, Vec2d, Ax2d};
use core::f64::consts::{PI, FRAC_PI_2, FRAC_PI_4};

// ----------------------------------------------------------------
// Integration tests: Geom2dCartesianPoint
// ----------------------------------------------------------------

#[test]
fn test_integration_point_construction_and_coords() {
    let p = Geom2dCartesianPoint::new(-1.5, 3.75);
    assert!(
        (p.x() + 1.5).abs() < 1e-12,
        "x should be -1.5, got {}",
        p.x()
    );
    assert!(
        (p.y() - 3.75).abs() < 1e-12,
        "y should be 3.75, got {}",
        p.y()
    );
}

#[test]
fn test_integration_point_from_pnt2d() {
    let pnt = Pnt2d::new(7.0, -2.0);
    let p = Geom2dCartesianPoint::new_from_pnt(pnt);
    assert!((p.x() - 7.0).abs() < 1e-12);
    assert!((p.y() + 2.0).abs() < 1e-12);
}

#[test]
fn test_integration_point_distance_known_triangle() {
    // 3-4-5 right triangle
    let p1 = Geom2dCartesianPoint::new(0.0, 0.0);
    let p2 = Geom2dCartesianPoint::new(3.0, 4.0);
    assert!((p1.distance(&p2) - 5.0).abs() < 1e-12);
    assert!((p2.distance(&p1) - 5.0).abs() < 1e-12);
}

#[test]
fn test_integration_point_distance_same_point() {
    let p = Geom2dCartesianPoint::new(99.0, -99.0);
    assert!(p.distance(&p.clone()) < 1e-14);
}

#[test]
fn test_integration_point_set_coords() {
    let mut p = Geom2dCartesianPoint::new(0.0, 0.0);
    p.set_x(100.0);
    p.set_y(-200.0);
    assert!((p.x() - 100.0).abs() < 1e-12);
    assert!((p.y() + 200.0).abs() < 1e-12);
}

#[test]
fn test_integration_point_translate_vec() {
    let mut p = Geom2dCartesianPoint::new(10.0, 20.0);
    let v = Vec2d::new(-5.0, 3.0);
    p.translate_vec(&v);
    assert!((p.x() - 5.0).abs() < 1e-12);
    assert!((p.y() - 23.0).abs() < 1e-12);
}

#[test]
fn test_integration_point_translate_pnt() {
    let mut p = Geom2dCartesianPoint::new(0.0, 0.0);
    let from = Pnt2d::new(-1.0, -1.0);
    let to = Pnt2d::new(1.0, 1.0);
    p.translate_pnt(&from, &to);
    assert!((p.x() - 2.0).abs() < 1e-12);
    assert!((p.y() - 2.0).abs() < 1e-12);
}

#[test]
fn test_integration_point_mirror_through_origin() {
    let mut p = Geom2dCartesianPoint::new(5.0, -3.0);
    let center = Pnt2d::new(0.0, 0.0);
    p.mirror_pt(&center);
    assert!((p.x() + 5.0).abs() < 1e-12);
    assert!((p.y() - 3.0).abs() < 1e-12);
}

#[test]
fn test_integration_point_mirror_through_point() {
    let mut p = Geom2dCartesianPoint::new(1.0, 1.0);
    let center = Pnt2d::new(3.0, 3.0);
    p.mirror_pt(&center);
    assert!((p.x() - 5.0).abs() < 1e-12);
    assert!((p.y() - 5.0).abs() < 1e-12);
}

#[test]
fn test_integration_point_mirror_ax_x_axis() {
    // Mirror (2, 5) through the X axis
    let mut p = Geom2dCartesianPoint::new(2.0, 5.0);
    let ax = Ax2d::new(Pnt2d::new(0.0, 0.0), Dir2d::new(1.0, 0.0));
    p.mirror_ax(&ax);
    assert!((p.x() - 2.0).abs() < 1e-12, "x should remain 2, got {}", p.x());
    assert!((p.y() + 5.0).abs() < 1e-12, "y should be -5, got {}", p.y());
}

#[test]
fn test_integration_point_mirror_ax_shifted_axis() {
    // Mirror (4, 0) through a horizontal axis at y=1
    // Axis: origin (0,1), direction (1,0)
    let mut p = Geom2dCartesianPoint::new(4.0, 0.0);
    let ax = Ax2d::new(Pnt2d::new(0.0, 1.0), Dir2d::new(1.0, 0.0));
    p.mirror_ax(&ax);
    // The reflection over a horizontal line at y=1:
    // translate to axis: (4, -1), reflect in x-axis: (4, 1), translate back: (4, 2)
    assert!((p.x() - 4.0).abs() < 1e-12, "x should be 4, got {}", p.x());
    assert!((p.y() - 2.0).abs() < 1e-12, "y should be 2, got {}", p.y());
}

#[test]
fn test_integration_point_rotate_quarter_turn() {
    let mut p = Geom2dCartesianPoint::new(1.0, 0.0);
    let center = Pnt2d::new(0.0, 0.0);
    p.rotate(&center, FRAC_PI_2);
    assert!((p.x()).abs() < 1e-12, "x should be 0, got {}", p.x());
    assert!((p.y() - 1.0).abs() < 1e-12, "y should be 1, got {}", p.y());
}

#[test]
fn test_integration_point_rotate_around_non_origin_center() {
    // Rotate (2, 0) around (1, 0) by 90 degrees
    let mut p = Geom2dCartesianPoint::new(2.0, 0.0);
    let center = Pnt2d::new(1.0, 0.0);
    p.rotate(&center, FRAC_PI_2);
    // (1,0) + rot90(1,0) = (1,0) + (0,1) = (1,1)
    assert!((p.x() - 1.0).abs() < 1e-12, "x should be 1, got {}", p.x());
    assert!((p.y() - 1.0).abs() < 1e-12, "y should be 1, got {}", p.y());
}

#[test]
fn test_integration_point_rotate_preserves_distance() {
    let center = Pnt2d::new(0.0, 0.0);
    let original = Geom2dCartesianPoint::new(3.0, 4.0);
    let mut rotated = original.clone();
    rotated.rotate(&center, 1.234);
    let center_pt = Geom2dCartesianPoint::new(0.0, 0.0);
    let orig_dist = original.distance(&center_pt);
    let rot_dist = rotated.distance(&center_pt);
    assert!((orig_dist - rot_dist).abs() < 1e-12);
}

#[test]
fn test_integration_point_scale_up() {
    let mut p = Geom2dCartesianPoint::new(1.0, 2.0);
    let center = Pnt2d::new(0.0, 0.0);
    p.scale(&center, 3.0);
    assert!((p.x() - 3.0).abs() < 1e-12);
    assert!((p.y() - 6.0).abs() < 1e-12);
}

#[test]
fn test_integration_point_scale_toward_center() {
    let mut p = Geom2dCartesianPoint::new(4.0, 4.0);
    let center = Pnt2d::new(2.0, 2.0);
    p.scale(&center, 0.5);
    assert!((p.x() - 3.0).abs() < 1e-12);
    assert!((p.y() - 3.0).abs() < 1e-12);
}

#[test]
fn test_integration_point_chain_transformations() {
    // Start at (1, 0), rotate 90 degrees, then translate by (0, 1)
    let mut p = Geom2dCartesianPoint::new(1.0, 0.0);
    let center = Pnt2d::new(0.0, 0.0);
    p.rotate(&center, FRAC_PI_2);
    // Now at approximately (0, 1)
    let v = Vec2d::new(0.0, 1.0);
    p.translate_vec(&v);
    // Now at approximately (0, 2)
    assert!((p.x()).abs() < 1e-12);
    assert!((p.y() - 2.0).abs() < 1e-12);
}

// ----------------------------------------------------------------
// Integration tests: Geom2dAxisPlacement
// ----------------------------------------------------------------

#[test]
fn test_integration_axis_construction() {
    let loc = Pnt2d::new(3.0, -1.0);
    let dir = Dir2d::new(0.0, 1.0);
    let ax = Geom2dAxisPlacement::new(loc, dir);
    assert!((ax.location().x - 3.0).abs() < 1e-12);
    assert!((ax.location().y + 1.0).abs() < 1e-12);
    assert!((ax.direction().x).abs() < 1e-12);
    assert!((ax.direction().y - 1.0).abs() < 1e-12);
}

#[test]
fn test_integration_axis_angle_zero() {
    let loc = Pnt2d::new(0.0, 0.0);
    let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
    let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
    assert!(ax1.angle(&ax2) < 1e-12);
}

#[test]
fn test_integration_axis_angle_90() {
    let loc = Pnt2d::new(0.0, 0.0);
    let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
    let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(0.0, 1.0));
    let angle = ax1.angle(&ax2);
    assert!((angle - FRAC_PI_2).abs() < 1e-12);
}

#[test]
fn test_integration_axis_angle_180() {
    let loc = Pnt2d::new(0.0, 0.0);
    let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
    let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(-1.0, 0.0));
    let angle = ax1.angle(&ax2);
    assert!((angle - PI).abs() < 1e-12);
}

#[test]
fn test_integration_axis_angle_symmetric() {
    let sq2 = 2.0_f64.sqrt();
    let loc = Pnt2d::new(0.0, 0.0);
    let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
    let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0 / sq2, 1.0 / sq2));
    let angle_12 = ax1.angle(&ax2);
    let angle_21 = ax2.angle(&ax1);
    assert!((angle_12 - angle_21).abs() < 1e-12);
}

#[test]
fn test_integration_axis_is_normal() {
    let loc = Pnt2d::new(5.0, 5.0);
    let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
    let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(0.0, 1.0));
    assert!(ax1.is_normal(&ax2, 1e-10));
    assert!(!ax1.is_normal(&ax1, 1e-10));
}

#[test]
fn test_integration_axis_is_opposite() {
    let loc = Pnt2d::new(0.0, 0.0);
    let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
    let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(-1.0, 0.0));
    assert!(ax1.is_opposite(&ax2, 1e-10));
    assert!(!ax1.is_opposite(&ax1, 1e-10));
}

#[test]
fn test_integration_axis_is_parallel_same_direction() {
    let ax1 = Geom2dAxisPlacement::new(Pnt2d::new(0.0, 0.0), Dir2d::new(1.0, 0.0));
    let ax2 = Geom2dAxisPlacement::new(Pnt2d::new(0.0, 5.0), Dir2d::new(1.0, 0.0));
    assert!(ax1.is_parallel(&ax2, 1e-10));
}

#[test]
fn test_integration_axis_is_parallel_opposite_direction() {
    let ax1 = Geom2dAxisPlacement::new(Pnt2d::new(0.0, 0.0), Dir2d::new(0.0, 1.0));
    let ax2 = Geom2dAxisPlacement::new(Pnt2d::new(3.0, 0.0), Dir2d::new(0.0, -1.0));
    assert!(ax1.is_parallel(&ax2, 1e-10));
}

#[test]
fn test_integration_axis_is_coaxial_same_axis() {
    let loc = Pnt2d::new(1.0, 1.0);
    let ax1 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
    let ax2 = Geom2dAxisPlacement::new(loc.clone(), Dir2d::new(1.0, 0.0));
    assert!(ax1.is_coaxial(&ax2, 1e-7, 1e-10));
}

#[test]
fn test_integration_axis_is_coaxial_different_origin() {
    let ax1 = Geom2dAxisPlacement::new(Pnt2d::new(0.0, 0.0), Dir2d::new(1.0, 0.0));
    let ax2 = Geom2dAxisPlacement::new(Pnt2d::new(0.0, 1.0), Dir2d::new(1.0, 0.0));
    assert!(!ax1.is_coaxial(&ax2, 1e-7, 1e-10));
}

#[test]
fn test_integration_axis_reverse_in_place() {
    let mut ax = Geom2dAxisPlacement::new(Pnt2d::new(0.0, 0.0), Dir2d::new(1.0, 0.0));
    ax.reverse();
    assert!((ax.direction().x + 1.0).abs() < 1e-12);
    assert!((ax.direction().y).abs() < 1e-12);
    // Location preserved
    assert!((ax.location().x).abs() < 1e-12);
    assert!((ax.location().y).abs() < 1e-12);
}

#[test]
fn test_integration_axis_reversed_result_immutable() {
    let ax = Geom2dAxisPlacement::new(Pnt2d::new(1.0, 2.0), Dir2d::new(0.0, 1.0));
    let rev = ax.reversed_result();
    // ax unchanged
    assert!((ax.direction().y - 1.0).abs() < 1e-12);
    // rev is opposite
    assert!((rev.direction().y + 1.0).abs() < 1e-12);
    // location preserved
    assert!((rev.location().x - 1.0).abs() < 1e-12);
    assert!((rev.location().y - 2.0).abs() < 1e-12);
}

#[test]
fn test_integration_axis_reversed_is_opposite_of_original() {
    let ax = Geom2dAxisPlacement::new(Pnt2d::new(0.0, 0.0), Dir2d::new(1.0, 0.0));
    let rev = ax.reversed_result();
    assert!(ax.is_opposite(&rev, 1e-10));
}

#[test]
fn test_integration_axis_double_reverse_identity() {
    let ax = Geom2dAxisPlacement::new(Pnt2d::new(3.0, -4.0), Dir2d::new(0.0, 1.0));
    let rev = ax.reversed_result().reversed_result();
    assert!((rev.direction().x - ax.direction().x).abs() < 1e-12);
    assert!((rev.direction().y - ax.direction().y).abs() < 1e-12);
}

#[test]
fn test_integration_axis_angle_range_is_0_to_pi() {
    // For any two directions, angle should be in [0, pi]
    let directions = [
        Dir2d::new(1.0, 0.0),
        Dir2d::new(0.0, 1.0),
        Dir2d::new(-1.0, 0.0),
        Dir2d::new(0.0, -1.0),
    ];
    let loc = Pnt2d::new(0.0, 0.0);
    for d1 in &directions {
        for d2 in &directions {
            let ax1 = Geom2dAxisPlacement::new(loc.clone(), d1.clone());
            let ax2 = Geom2dAxisPlacement::new(loc.clone(), d2.clone());
            let angle = ax1.angle(&ax2);
            assert!(angle >= 0.0 - 1e-12 && angle <= PI + 1e-12,
                "angle {} out of [0, pi]", angle);
        }
    }
}

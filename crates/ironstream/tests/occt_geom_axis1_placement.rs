// FILE: tests/occt_geom_axis1_placement.rs
extern crate ironstream;
use ironstream::geom_axis1_placement::{GeomAxis1Placement, GeomAxis2Placement};

use std::f64::consts::{PI, FRAC_PI_2, FRAC_PI_4};

const EPS: f64 = 1e-10;

fn approx_eq(a: [f64; 3], b: [f64; 3]) -> bool {
    (a[0] - b[0]).abs() < EPS
        && (a[1] - b[1]).abs() < EPS
        && (a[2] - b[2]).abs() < EPS
}

fn magnitude(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

// ── GeomAxis1Placement integration tests ─────────────────────────────────────

#[test]
fn integration_axis1_construction_and_getters() {
    let loc = [3.0, -1.0, 2.0];
    let dir = [4.0, 0.0, 3.0]; // magnitude 5
    let ax = GeomAxis1Placement::new(loc, dir);
    assert!(approx_eq(ax.location(), loc));
    // Direction must be normalised to magnitude 1.
    let d = ax.direction();
    assert!((magnitude(d) - 1.0).abs() < EPS);
    assert!((d[0] - 0.8).abs() < EPS);
    assert!((d[1]).abs() < EPS);
    assert!((d[2] - 0.6).abs() < EPS);
}

#[test]
fn integration_axis1_ax1_value() {
    let ax = GeomAxis1Placement::new([1.0, 2.0, 3.0], [0.0, 1.0, 0.0]);
    let a = ax.ax1();
    assert!(approx_eq(a.location, [1.0, 2.0, 3.0]));
    assert!(approx_eq(a.direction, [0.0, 1.0, 0.0]));
}

#[test]
fn integration_axis1_set_location_and_direction() {
    let mut ax = GeomAxis1Placement::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    ax.set_location([5.0, 6.0, 7.0]);
    ax.set_direction([0.0, 0.0, 3.0]);
    assert!(approx_eq(ax.location(), [5.0, 6.0, 7.0]));
    assert!(approx_eq(ax.direction(), [0.0, 0.0, 1.0]));
}

#[test]
fn integration_axis1_reverse_twice_is_identity() {
    let ax = GeomAxis1Placement::new([1.0, 2.0, 3.0], [1.0, 1.0, 1.0]);
    let twice = ax.reversed().reversed();
    assert!(approx_eq(twice.direction(), ax.direction()));
    assert!(approx_eq(twice.location(), ax.location()));
}

#[test]
fn integration_axis1_coaxial_within_tolerance() {
    // Two axes on the same line but slightly offset within lin_tol.
    let ax1 = GeomAxis1Placement::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let ax2 = GeomAxis1Placement::new([0.0, 1e-8, 0.0], [1.0, 0.0, 0.0]);
    assert!(ax1.is_coaxial(&ax2, 1e-7, 1e-7));
    // But not outside tolerance.
    let ax3 = GeomAxis1Placement::new([0.0, 1.0, 0.0], [1.0, 0.0, 0.0]);
    assert!(!ax1.is_coaxial(&ax3, 1e-7, 1e-7));
}

#[test]
fn integration_axis1_is_normal_near_boundary() {
    // Slightly less than 90 deg.
    let angle = FRAC_PI_2 - 1e-6;
    let ax1 = GeomAxis1Placement::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let ax2 = GeomAxis1Placement::new([0.0, 0.0, 0.0], [angle.cos(), angle.sin(), 0.0]);
    // With ang_tol = 1e-5, should be within tolerance.
    assert!(ax1.is_normal(&ax2, 1e-5));
    // With ang_tol = 1e-7, should NOT be within tolerance.
    assert!(!ax1.is_normal(&ax2, 1e-7));
}

#[test]
fn integration_axis1_is_opposite_vs_parallel() {
    let ax_pos = GeomAxis1Placement::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let ax_neg = GeomAxis1Placement::new([0.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
    assert!(ax_pos.is_parallel(&ax_neg, 1e-7));
    assert!(ax_pos.is_opposite(&ax_neg, 1e-7));
    assert!(!ax_pos.is_opposite(&ax_pos, 1e-7));
}

#[test]
fn integration_axis1_translate_then_mirror_through_point() {
    let ax = GeomAxis1Placement::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let moved = ax.translate([3.0, 0.0, 0.0]);
    let mirrored = moved.mirror_through_point([0.0, 0.0, 0.0]);
    assert!(approx_eq(mirrored.location(), [-3.0, 0.0, 0.0]));
    // Direction negated.
    assert!(approx_eq(mirrored.direction(), [-1.0, 0.0, 0.0]));
}

#[test]
fn integration_axis1_rotate_45_degrees() {
    let ax = GeomAxis1Placement::new([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let rotated = ax.rotate([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], FRAC_PI_4);
    let expected_loc = [FRAC_PI_4.cos(), FRAC_PI_4.sin(), 0.0];
    assert!(approx_eq(rotated.location(), expected_loc));
    let expected_dir = [FRAC_PI_4.cos(), FRAC_PI_4.sin(), 0.0];
    assert!(approx_eq(rotated.direction(), expected_dir));
}

#[test]
fn integration_axis1_scale_from_nonzero_centre() {
    let ax = GeomAxis1Placement::new([5.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    // Scale by 2 from centre [3,0,0]: new loc = 3 + 2*(5-3) = [7,0,0].
    let scaled = ax.scale([3.0, 0.0, 0.0], 2.0);
    assert!(approx_eq(scaled.location(), [7.0, 0.0, 0.0]));
    assert!(approx_eq(scaled.direction(), [0.0, 1.0, 0.0]));
}

#[test]
fn integration_axis1_mirror_through_plane_yz() {
    // Mirror through YZ plane (normal = X).
    let ax = GeomAxis1Placement::new([3.0, 1.0, 0.0], [1.0, 0.0, 0.0]);
    let m = ax.mirror_through_plane([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    assert!(approx_eq(m.location(), [-3.0, 1.0, 0.0]));
    // X direction [1,0,0] flips to [-1,0,0] through YZ plane.
    assert!(approx_eq(m.direction(), [-1.0, 0.0, 0.0]));
}

#[test]
fn integration_axis1_mirror_through_axis_identity_on_axis() {
    // An axis on the mirror axis should have its location unchanged.
    let ax = GeomAxis1Placement::new([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
    let m = ax.mirror_through_axis([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    assert!(approx_eq(m.location(), [0.0, 0.0, 5.0]));
    assert!(approx_eq(m.direction(), [0.0, 0.0, 1.0]));
}

// ── GeomAxis2Placement integration tests ─────────────────────────────────────

#[test]
fn integration_axis2_construction_right_handed() {
    let ax2 = GeomAxis2Placement::new(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
    );
    // Should form a right-handed triad.
    let cross = [
        ax2.direction()[1] * ax2.x_direction()[2] - ax2.direction()[2] * ax2.x_direction()[1],
        ax2.direction()[2] * ax2.x_direction()[0] - ax2.direction()[0] * ax2.x_direction()[2],
        ax2.direction()[0] * ax2.x_direction()[1] - ax2.direction()[1] * ax2.x_direction()[0],
    ];
    // y should equal direction cross x_direction.
    assert!(approx_eq(cross, ax2.y_direction()));
}

#[test]
fn integration_axis2_all_axes_orthonormal() {
    let ax2 = GeomAxis2Placement::new(
        [1.0, -2.0, 3.0],
        [1.0, 2.0, 2.0], // magnitude 3
        [0.0, -1.0, 1.0],
    );
    let d = ax2.direction();
    let x = ax2.x_direction();
    let y = ax2.y_direction();
    // Unit.
    assert!((magnitude(d) - 1.0).abs() < EPS);
    assert!((magnitude(x) - 1.0).abs() < EPS);
    assert!((magnitude(y) - 1.0).abs() < EPS);
    // Pairwise orthogonal.
    assert!(dot(d, x).abs() < EPS);
    assert!(dot(d, y).abs() < EPS);
    assert!(dot(x, y).abs() < EPS);
}

#[test]
fn integration_axis2_rotate_180_degrees() {
    let ax2 = GeomAxis2Placement::new(
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
    );
    let rotated = ax2.rotate_around_ax1([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], PI);
    // Location [1,0,0] -> [-1,0,0].
    assert!((rotated.location()[0] + 1.0).abs() < EPS);
    assert!((rotated.location()[1]).abs() < EPS);
    // Direction Z unchanged.
    assert!(approx_eq(rotated.direction(), [0.0, 0.0, 1.0]));
    // X direction [1,0,0] -> [-1,0,0].
    assert!((rotated.x_direction()[0] + 1.0).abs() < EPS);
    assert!((rotated.x_direction()[1]).abs() < EPS);
}

#[test]
fn integration_axis2_scale_then_translate() {
    let ax2 = GeomAxis2Placement::new(
        [2.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
    );
    let scaled = ax2.scale([0.0, 0.0, 0.0], 3.0);
    let moved = scaled.translate([0.0, 1.0, 0.0]);
    assert!(approx_eq(moved.location(), [6.0, 1.0, 0.0]));
    assert!(approx_eq(moved.direction(), [0.0, 0.0, 1.0]));
}

#[test]
fn integration_axis2_mirror_through_origin() {
    let ax2 = GeomAxis2Placement::new(
        [1.0, 2.0, 3.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
    );
    let m = ax2.mirror_through_point([0.0, 0.0, 0.0]);
    assert!(approx_eq(m.location(), [-1.0, -2.0, -3.0]));
    assert!(approx_eq(m.direction(), [0.0, 0.0, -1.0]));
    assert!(approx_eq(m.x_direction(), [-1.0, 0.0, 0.0]));
}

#[test]
fn integration_axis2_mirror_plane_preserves_orthonormality() {
    let ax2 = GeomAxis2Placement::new(
        [1.0, 2.0, 3.0],
        [1.0, 1.0, 1.0],
        [1.0, -1.0, 0.0],
    );
    let m = ax2.mirror_through_plane([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let d = m.direction();
    let x = m.x_direction();
    let y = m.y_direction();
    assert!((magnitude(d) - 1.0).abs() < EPS);
    assert!((magnitude(x) - 1.0).abs() < EPS);
    assert!((magnitude(y) - 1.0).abs() < EPS);
    assert!(dot(d, x).abs() < EPS);
    assert!(dot(d, y).abs() < EPS);
    assert!(dot(x, y).abs() < EPS);
}

#[test]
fn integration_axis2_mirror_axis_preserves_orthonormality() {
    let ax2 = GeomAxis2Placement::new(
        [0.0, 3.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 0.0, 0.0],
    );
    let m = ax2.mirror_through_axis([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let d = m.direction();
    let x = m.x_direction();
    let y = m.y_direction();
    assert!((magnitude(d) - 1.0).abs() < EPS);
    assert!((magnitude(x) - 1.0).abs() < EPS);
    assert!((magnitude(y) - 1.0).abs() < EPS);
    assert!(dot(d, x).abs() < EPS);
}

#[test]
fn integration_axis2_set_x_direction_removes_z_component() {
    let mut ax2 = GeomAxis2Placement::new(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
    );
    // New X has a Z component which should be stripped.
    ax2.set_x_direction([1.0, 0.0, 1.0]);
    // After projection onto plane perp to Z, should be [1,0,0] normalised.
    assert!((ax2.x_direction()[2]).abs() < EPS);
    assert!((magnitude(ax2.x_direction()) - 1.0).abs() < EPS);
    // Y must be orthogonal to both.
    assert!(dot(ax2.y_direction(), ax2.direction()).abs() < EPS);
    assert!(dot(ax2.y_direction(), ax2.x_direction()).abs() < EPS);
}

#[test]
fn integration_axis2_ax2_value_consistent() {
    let ax2 = GeomAxis2Placement::new(
        [7.0, 8.0, 9.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    );
    let a = ax2.ax2();
    assert!(approx_eq(a.location, ax2.location()));
    assert!(approx_eq(a.direction, ax2.direction()));
    assert!(approx_eq(a.x_direction, ax2.x_direction()));
    assert!(approx_eq(a.y_direction, ax2.y_direction()));
}

#[test]
fn integration_axis2_rotate_preserves_handedness() {
    // After any rotation, the triad must remain right-handed with unit axes.
    let ax2 = GeomAxis2Placement::new(
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
    );
    for i in 0..8 {
        let angle = (i as f64) * PI / 4.0;
        let rotated = ax2.rotate_around_ax1([0.0, 0.0, 0.0], [1.0, 1.0, 0.0], angle);
        let d = rotated.direction();
        let x = rotated.x_direction();
        let y = rotated.y_direction();
        assert!((magnitude(d) - 1.0).abs() < EPS, "d not unit at angle {}", angle);
        assert!((magnitude(x) - 1.0).abs() < EPS, "x not unit at angle {}", angle);
        assert!((magnitude(y) - 1.0).abs() < EPS, "y not unit at angle {}", angle);
        assert!(dot(d, x).abs() < EPS, "d,x not ortho at angle {}", angle);
        assert!(dot(d, y).abs() < EPS, "d,y not ortho at angle {}", angle);
        assert!(dot(x, y).abs() < EPS, "x,y not ortho at angle {}", angle);
    }
}

// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_axis_placement.rs
//! Integration tests for `GeomAxis1Pl` and `GeomAxis2Pl`.
//!
//! Faithful port of OCCT behaviour for `Geom_Axis1Placement` and
//! `Geom_Axis2Placement`.

extern crate ironstream;
use ironstream::geom_axis_placement::*;

use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, PI};

const EPS: f64 = 1e-12;

fn near3(a: [f64; 3], b: [f64; 3]) {
    for i in 0..3 {
        assert!(
            (a[i] - b[i]).abs() <= EPS,
            "component {i}: expected {} ≈ {} (tol {EPS})",
            a[i],
            b[i]
        );
    }
}

fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
}

fn magnitude(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

// ── GeomAxis1Pl ──────────────────────────────────────────────────────────────

// Construction normalizes direction
#[test]
fn axis1_new_normalizes_direction() {
    let ax = GeomAxis1Pl::new([0.0, 0.0, 0.0], [3.0, 0.0, 0.0]);
    near3(ax.direction(), [1.0, 0.0, 0.0]);
}

// Location is stored as-is
#[test]
fn axis1_new_stores_location() {
    let ax = GeomAxis1Pl::new([1.0, 2.0, 3.0], [0.0, 1.0, 0.0]);
    near3(ax.location(), [1.0, 2.0, 3.0]);
}

// Diagonal direction normalised correctly
#[test]
fn axis1_new_diagonal_direction() {
    let sq2 = 2.0_f64.sqrt();
    let ax = GeomAxis1Pl::new([0.0, 0.0, 0.0], [1.0, 1.0, 0.0]);
    near3(ax.direction(), [1.0 / sq2, 1.0 / sq2, 0.0]);
}

// set_location changes origin
#[test]
fn axis1_set_location() {
    let mut ax = GeomAxis1Pl::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    ax.set_location([5.0, 6.0, 7.0]);
    near3(ax.location(), [5.0, 6.0, 7.0]);
    near3(ax.direction(), [0.0, 0.0, 1.0]);
}

// set_direction re-normalizes
#[test]
fn axis1_set_direction_renormalizes() {
    let mut ax = GeomAxis1Pl::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    ax.set_direction([0.0, 4.0, 0.0]);
    near3(ax.direction(), [0.0, 1.0, 0.0]);
}

// reversed() returns opposite direction, original unchanged
#[test]
fn axis1_reversed_immutable() {
    let ax = GeomAxis1Pl::new([1.0, 2.0, 3.0], [1.0, 0.0, 0.0]);
    let rev = ax.reversed();
    near3(ax.direction(), [1.0, 0.0, 0.0]);
    near3(rev.direction(), [-1.0, 0.0, 0.0]);
    near3(rev.location(), [1.0, 2.0, 3.0]);
}

// reversed() twice yields original
#[test]
fn axis1_reversed_twice_is_identity() {
    let ax = GeomAxis1Pl::new([0.0, 0.0, 0.0], [1.0, 2.0, 3.0]);
    let rev2 = ax.reversed().reversed();
    near3(rev2.direction(), ax.direction());
}

// is_parallel: same direction
#[test]
fn axis1_is_parallel_same_direction() {
    let ax1 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let ax2 = GeomAxis1Pl::new([5.0, 1.0, 0.0], [2.0, 0.0, 0.0]);
    assert!(ax1.is_parallel(&ax2, 1e-7));
}

// is_parallel: anti-parallel (opposite direction)
#[test]
fn axis1_is_parallel_anti_parallel() {
    let ax1 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let ax2 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
    assert!(ax1.is_parallel(&ax2, 1e-7));
}

// is_parallel: perpendicular directions are not parallel
#[test]
fn axis1_is_parallel_perpendicular_is_false() {
    let ax1 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let ax2 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    assert!(!ax1.is_parallel(&ax2, 1e-7));
}

// angle between parallel axes is 0
#[test]
fn axis1_angle_parallel_is_zero() {
    let ax1 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let ax2 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    near(ax1.angle(&ax2), 0.0, EPS);
}

// angle between perpendicular axes is π/2
#[test]
fn axis1_angle_perpendicular_is_pi_half() {
    let ax1 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let ax2 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    near(ax1.angle(&ax2), FRAC_PI_2, EPS);
}

// angle between anti-parallel axes is π
#[test]
fn axis1_angle_anti_parallel_is_pi() {
    let ax1 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let ax2 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
    near(ax1.angle(&ax2), PI, EPS);
}

// angle between 45-degree axes
#[test]
fn axis1_angle_45_degrees() {
    let sq2 = 2.0_f64.sqrt();
    let ax1 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let ax2 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [1.0 / sq2, 1.0 / sq2, 0.0]);
    near(ax1.angle(&ax2), FRAC_PI_4, EPS);
}

// angle between 30-degree axes
#[test]
fn axis1_angle_30_degrees() {
    let ax1 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let ax2 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [FRAC_PI_6.cos(), FRAC_PI_6.sin(), 0.0]);
    near(ax1.angle(&ax2), FRAC_PI_6, EPS);
}

// angle is symmetric
#[test]
fn axis1_angle_symmetric() {
    let ax1 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let ax2 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    near(ax1.angle(&ax2), ax2.angle(&ax1), EPS);
}

// is_parallel with tight tolerance rejects near-parallel
#[test]
fn axis1_is_parallel_tight_tolerance_rejects() {
    let ax1 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    // 0.01 rad off from X axis
    let ax2 = GeomAxis1Pl::new([0.0, 0.0, 0.0], [0.01_f64.cos(), 0.01_f64.sin(), 0.0]);
    // With tolerance 1e-7 (much smaller than 0.01 rad), should not be parallel.
    assert!(!ax1.is_parallel(&ax2, 1e-7));
}

// direction is unit length after construction with arbitrary input
#[test]
fn axis1_direction_is_unit() {
    let ax = GeomAxis1Pl::new([0.0, 0.0, 0.0], [3.0, 4.0, 0.0]);
    near(magnitude(ax.direction()), 1.0, EPS);
}

// Copy semantics (PartialEq)
#[test]
fn axis1_copy_equals_original() {
    let ax = GeomAxis1Pl::new([1.0, 2.0, 3.0], [0.0, 0.0, 1.0]);
    let copy = ax;
    assert_eq!(ax, copy);
}

// ── GeomAxis2Pl ──────────────────────────────────────────────────────────────

// Basic construction: standard XYZ frame
#[test]
fn axis2_new_standard_xyz_frame() {
    let ax = GeomAxis2Pl::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    near3(ax.z_dir(), [0.0, 0.0, 1.0]);
    near3(ax.x_dir(), [1.0, 0.0, 0.0]);
    near3(ax.y_dir(), [0.0, 1.0, 0.0]);
    near3(ax.location(), [0.0, 0.0, 0.0]);
}

// Location is stored as-is
#[test]
fn axis2_new_stores_location() {
    let ax = GeomAxis2Pl::new([3.0, -1.0, 5.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    near3(ax.location(), [3.0, -1.0, 5.0]);
}

// x_dir is orthogonalized against z_dir via Gram–Schmidt
#[test]
fn axis2_new_orthogonalizes_xdir() {
    // x_dir has a component along z_dir
    let ax = GeomAxis2Pl::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 1.0]);
    // After orthogonalization the Z component is removed -> [1,0,0] after norm
    near3(ax.x_dir(), [1.0, 0.0, 0.0]);
    near3(ax.y_dir(), [0.0, 1.0, 0.0]);
}

// y_dir = z cross x
#[test]
fn axis2_y_dir_is_z_cross_x() {
    let ax = GeomAxis2Pl::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0]);
    // z = [0,0,1], x = [0,1,0]  =>  y = z×x = [0*0-1*1, 1*0-0*0, 0*1-0*0] = [-1,0,0]
    near3(ax.y_dir(), [-1.0, 0.0, 0.0]);
}

// x_dir and y_dir are unit length
#[test]
fn axis2_dirs_are_unit_length() {
    let ax = GeomAxis2Pl::new([0.0, 0.0, 0.0], [1.0, 2.0, 3.0], [1.0, 0.0, 0.0]);
    near(magnitude(ax.x_dir()), 1.0, EPS);
    near(magnitude(ax.y_dir()), 1.0, EPS);
    near(magnitude(ax.z_dir()), 1.0, EPS);
}

// x, y, z are mutually orthogonal
#[test]
fn axis2_dirs_mutually_orthogonal() {
    let ax = GeomAxis2Pl::new([0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
    near(dot(ax.x_dir(), ax.y_dir()).abs(), 0.0, EPS);
    near(dot(ax.x_dir(), ax.z_dir()).abs(), 0.0, EPS);
    near(dot(ax.y_dir(), ax.z_dir()).abs(), 0.0, EPS);
}

// set_location changes origin, directions unchanged
#[test]
fn axis2_set_location() {
    let mut ax = GeomAxis2Pl::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    ax.set_location([9.0, 8.0, 7.0]);
    near3(ax.location(), [9.0, 8.0, 7.0]);
    near3(ax.z_dir(), [0.0, 0.0, 1.0]);
    near3(ax.x_dir(), [1.0, 0.0, 0.0]);
}

// set_z_dir recomputes y = z cross x using stored x
#[test]
fn axis2_set_z_dir_recomputes_y() {
    let mut ax = GeomAxis2Pl::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    // Rotate z_dir to Y axis; x_dir stays [1,0,0], so y = [0,1,0] cross [1,0,0] = [0,0,-1]
    ax.set_z_dir([0.0, 1.0, 0.0]);
    near3(ax.z_dir(), [0.0, 1.0, 0.0]);
    near3(ax.x_dir(), [1.0, 0.0, 0.0]);
    near3(ax.y_dir(), [0.0, 0.0, -1.0]);
}

// set_z_dir orthogonalizes stored x_dir against new z_dir
#[test]
fn axis2_set_z_dir_orthogonalizes_x() {
    // Start with z=[0,0,1], x=[1,0,0], y=[0,1,0]
    let mut ax = GeomAxis2Pl::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    // New z = [1,0,0]; stored x_dir=[1,0,0] is parallel to new z!
    // After Gram-Schmidt the projection removes everything; this would panic.
    // Instead set a new z that is non-parallel to x_dir.
    ax.set_z_dir([0.0, 1.0, 0.0]); // z = Y
    // Stored x = [1,0,0]; component along Y = 0 => x stays [1,0,0]
    near3(ax.x_dir(), [1.0, 0.0, 0.0]);
    near(dot(ax.x_dir(), ax.z_dir()).abs(), 0.0, EPS);
}

// is_direct always returns true
#[test]
fn axis2_is_direct_always_true() {
    let ax = GeomAxis2Pl::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    assert!(ax.is_direct());
}

// is_direct true for arbitrary orientation
#[test]
fn axis2_is_direct_arbitrary() {
    let ax = GeomAxis2Pl::new([1.0, 2.0, 3.0], [1.0, 1.0, 1.0], [0.0, 0.0, 1.0]);
    assert!(ax.is_direct());
}

// y_dir always orthogonal to both x and z after construction
#[test]
fn axis2_y_orthogonal_after_arbitrary_construction() {
    let ax = GeomAxis2Pl::new([0.0, 0.0, 0.0], [2.0, 3.0, 1.0], [1.0, 0.0, 0.0]);
    near(dot(ax.y_dir(), ax.x_dir()).abs(), 0.0, EPS);
    near(dot(ax.y_dir(), ax.z_dir()).abs(), 0.0, EPS);
}

// y_dir unit length after set_z_dir
#[test]
fn axis2_y_unit_after_set_z_dir() {
    let mut ax = GeomAxis2Pl::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    ax.set_z_dir([1.0, 1.0, 0.0]);
    near(magnitude(ax.y_dir()), 1.0, EPS);
    near(magnitude(ax.x_dir()), 1.0, EPS);
    near(magnitude(ax.z_dir()), 1.0, EPS);
}

// z_dir is normalised even if input is not unit
#[test]
fn axis2_z_dir_normalized() {
    let ax = GeomAxis2Pl::new([0.0, 0.0, 0.0], [0.0, 0.0, 5.0], [1.0, 0.0, 0.0]);
    near3(ax.z_dir(), [0.0, 0.0, 1.0]);
}

// Copy semantics (PartialEq)
#[test]
fn axis2_copy_equals_original() {
    let ax = GeomAxis2Pl::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    let copy = ax;
    assert_eq!(ax, copy);
}

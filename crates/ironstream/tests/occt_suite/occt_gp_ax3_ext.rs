// FILE: rust/ironstream/crates/ironstream/tests/occt_gp_ax3_ext.rs

//! Ported OpenCascade unit tests — `gp_Dir`, `gp_Ax1`, `gp_Ax3` (ext port).
//!
//! These tests exercise `GpDir`, `GpAx1Ext`, and `GpAx3Ext` from the
//! zero-dependency `gp_ax3_ext` module.
//!
//! Tolerances:
//! * linear:  1e-7
//! * angular: 1e-12

extern crate ironstream;
use ironstream::gp_ax3_ext::*;

use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

const LIN_TOL: f64 = 1e-7;
const ANG_TOL: f64 = 1e-12;

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ~= {b} within {tol} (diff {})",
        (a - b).abs()
    );
}

// ── GpDir ────────────────────────────────────────────────────────────────────

// 1. Constructor normalizes the input vector.
#[test]
fn gpdir_new_normalizes() {
    let d = GpDir::new(3.0, 0.0, 4.0); // magnitude 5
    near(d.x(), 0.6, LIN_TOL);
    near(d.y(), 0.0, LIN_TOL);
    near(d.z(), 0.8, LIN_TOL);
    let mag = (d.x() * d.x() + d.y() * d.y() + d.z() * d.z()).sqrt();
    near(mag, 1.0, LIN_TOL);
}

// 2. Unit axis X is stored unchanged.
#[test]
fn gpdir_unit_x() {
    let d = GpDir::new(1.0, 0.0, 0.0);
    near(d.x(), 1.0, ANG_TOL);
    near(d.y(), 0.0, ANG_TOL);
    near(d.z(), 0.0, ANG_TOL);
}

// 3. reversed() negates all components and leaves the original unchanged.
#[test]
fn gpdir_reversed_non_mutating() {
    let d = GpDir::new(1.0, 0.0, 0.0);
    let r = d.reversed();
    near(r.x(), -1.0, LIN_TOL);
    near(r.y(), 0.0, LIN_TOL);
    near(r.z(), 0.0, LIN_TOL);
    // original unchanged
    near(d.x(), 1.0, LIN_TOL);
}

// 4. dot() of same direction is 1.
#[test]
fn gpdir_dot_same_is_one() {
    let d = GpDir::new(1.0, 0.0, 0.0);
    near(d.dot(&d), 1.0, LIN_TOL);
}

// 5. dot() of perpendicular directions is 0.
#[test]
fn gpdir_dot_perpendicular_is_zero() {
    let dx = GpDir::new(1.0, 0.0, 0.0);
    let dy = GpDir::new(0.0, 1.0, 0.0);
    near(dx.dot(&dy), 0.0, LIN_TOL);
}

// 6. dot() of opposite directions is -1.
#[test]
fn gpdir_dot_opposite_is_minus_one() {
    let d = GpDir::new(1.0, 0.0, 0.0);
    let neg = d.reversed();
    near(d.dot(&neg), -1.0, LIN_TOL);
}

// 7. cross(X, Y) = Z.
#[test]
fn gpdir_cross_x_y_gives_z() {
    let dx = GpDir::new(1.0, 0.0, 0.0);
    let dy = GpDir::new(0.0, 1.0, 0.0);
    let c = dx.cross(&dy);
    near(c.x(), 0.0, LIN_TOL);
    near(c.y(), 0.0, LIN_TOL);
    near(c.z(), 1.0, LIN_TOL);
}

// 8. cross(Y, X) = -Z (antisymmetric).
#[test]
fn gpdir_cross_antisymmetric() {
    let dx = GpDir::new(1.0, 0.0, 0.0);
    let dy = GpDir::new(0.0, 1.0, 0.0);
    let ab = dx.cross(&dy);
    let ba = dy.cross(&dx);
    near(ab.x(), -ba.x(), ANG_TOL);
    near(ab.y(), -ba.y(), ANG_TOL);
    near(ab.z(), -ba.z(), ANG_TOL);
}

// 9. angle() between X and Y is π/2.
#[test]
fn gpdir_angle_90_degrees() {
    let dx = GpDir::new(1.0, 0.0, 0.0);
    let dy = GpDir::new(0.0, 1.0, 0.0);
    near(dx.angle(&dy), FRAC_PI_2, ANG_TOL);
}

// 10. angle() between X and X is 0.
#[test]
fn gpdir_angle_to_self_is_zero() {
    let d = GpDir::new(1.0, 0.0, 0.0);
    near(d.angle(&d), 0.0, ANG_TOL);
}

// 11. angle() between opposite directions is π.
#[test]
fn gpdir_angle_antiparallel_is_pi() {
    let d = GpDir::new(1.0, 0.0, 0.0);
    let neg = d.reversed();
    near(d.angle(&neg), PI, LIN_TOL);
}

// 12. angle() at 45°.
#[test]
fn gpdir_angle_45_degrees() {
    let dx = GpDir::new(1.0, 0.0, 0.0);
    let d45 = GpDir::new(1.0, 1.0, 0.0);
    near(dx.angle(&d45), FRAC_PI_4, LIN_TOL);
}

// 13. angle() is symmetric.
#[test]
fn gpdir_angle_symmetric() {
    let a = GpDir::new(1.0, 2.0, 3.0);
    let b = GpDir::new(3.0, 1.0, 2.0);
    near(a.angle(&b), b.angle(&a), ANG_TOL);
}

// 14. is_parallel(): same direction → true.
#[test]
fn gpdir_is_parallel_same() {
    let d = GpDir::new(1.0, 0.0, 0.0);
    assert!(d.is_parallel(&d, 1e-9));
}

// 15. is_parallel(): anti-parallel → true.
#[test]
fn gpdir_is_parallel_antiparallel() {
    let d = GpDir::new(1.0, 0.0, 0.0);
    let neg = d.reversed();
    assert!(d.is_parallel(&neg, 1e-9));
}

// 16. is_parallel(): perpendicular → false.
#[test]
fn gpdir_is_parallel_perp_is_false() {
    let dx = GpDir::new(1.0, 0.0, 0.0);
    let dy = GpDir::new(0.0, 1.0, 0.0);
    assert!(!dx.is_parallel(&dy, 1e-9));
}

// 17. is_normal(): perpendicular → true.
#[test]
fn gpdir_is_normal_perpendicular() {
    let dx = GpDir::new(1.0, 0.0, 0.0);
    let dy = GpDir::new(0.0, 1.0, 0.0);
    assert!(dx.is_normal(&dy, 1e-9));
}

// 18. is_normal(): parallel → false.
#[test]
fn gpdir_is_normal_parallel_is_false() {
    let d = GpDir::new(1.0, 0.0, 0.0);
    assert!(!d.is_normal(&d, 1e-9));
}

// 19. Double reverse is identity.
#[test]
fn gpdir_double_reverse_identity() {
    let original = GpDir::new(3.0, 0.0, 4.0);
    let r = original.reversed().reversed();
    near(r.x(), original.x(), ANG_TOL);
    near(r.y(), original.y(), ANG_TOL);
    near(r.z(), original.z(), ANG_TOL);
}

// 20. Diagonal direction is unit.
#[test]
fn gpdir_diagonal_is_unit() {
    let d = GpDir::new(1.0, 1.0, 1.0);
    let expected = 1.0_f64 / 3.0_f64.sqrt();
    near(d.x(), expected, LIN_TOL);
    near(d.y(), expected, LIN_TOL);
    near(d.z(), expected, LIN_TOL);
    let mag = (d.x() * d.x() + d.y() * d.y() + d.z() * d.z()).sqrt();
    near(mag, 1.0, LIN_TOL);
}

// ── GpAx1Ext ─────────────────────────────────────────────────────────────────

// 21. location() and direction() round-trip.
#[test]
fn gpax1ext_location_direction_roundtrip() {
    let ax = GpAx1Ext::new([1.0, 2.0, 3.0], [0.0, 0.0, 1.0]);
    let loc = ax.location();
    near(loc[0], 1.0, LIN_TOL);
    near(loc[1], 2.0, LIN_TOL);
    near(loc[2], 3.0, LIN_TOL);
    near(ax.direction().z(), 1.0, LIN_TOL);
}

// 22. Direction is normalized on construction.
#[test]
fn gpax1ext_direction_normalized() {
    let ax = GpAx1Ext::new([0.0, 0.0, 0.0], [3.0, 0.0, 4.0]);
    near(ax.direction().x(), 0.6, LIN_TOL);
    near(ax.direction().y(), 0.0, LIN_TOL);
    near(ax.direction().z(), 0.8, LIN_TOL);
}

// 23. set_location() changes the origin.
#[test]
fn gpax1ext_set_location() {
    let mut ax = GpAx1Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    ax.set_location([5.0, 6.0, 7.0]);
    let loc = ax.location();
    near(loc[0], 5.0, LIN_TOL);
    near(loc[1], 6.0, LIN_TOL);
    near(loc[2], 7.0, LIN_TOL);
}

// 24. set_direction() updates and normalizes.
#[test]
fn gpax1ext_set_direction() {
    let mut ax = GpAx1Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    ax.set_direction([1.0, 0.0, 0.0]);
    near(ax.direction().x(), 1.0, LIN_TOL);
    near(ax.direction().y(), 0.0, LIN_TOL);
    near(ax.direction().z(), 0.0, LIN_TOL);
}

// 25. reversed() negates direction, location unchanged.
#[test]
fn gpax1ext_reversed() {
    let ax = GpAx1Ext::new([1.0, 2.0, 3.0], [0.0, 0.0, 1.0]);
    let r = ax.reversed();
    near(r.direction().z(), -1.0, LIN_TOL);
    let loc = r.location();
    near(loc[0], 1.0, LIN_TOL);
    near(loc[1], 2.0, LIN_TOL);
    near(loc[2], 3.0, LIN_TOL);
}

// 26. angle() between two parallel axes is 0.
#[test]
fn gpax1ext_angle_parallel_is_zero() {
    let a = GpAx1Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let b = GpAx1Ext::new([5.0, 5.0, 0.0], [0.0, 0.0, 1.0]);
    near(a.angle(&b), 0.0, ANG_TOL);
}

// 27. angle() between perpendicular axes is π/2.
#[test]
fn gpax1ext_angle_perpendicular() {
    let a = GpAx1Ext::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let b = GpAx1Ext::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    near(a.angle(&b), FRAC_PI_2, ANG_TOL);
}

// 28. angle() between antiparallel axes is π.
#[test]
fn gpax1ext_angle_antiparallel() {
    let a = GpAx1Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let b = GpAx1Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]);
    near(a.angle(&b), PI, LIN_TOL);
}

// 29. is_parallel(): parallel axes.
#[test]
fn gpax1ext_is_parallel_same_direction() {
    let a = GpAx1Ext::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let b = GpAx1Ext::new([0.0, 5.0, 0.0], [2.0, 0.0, 0.0]);
    assert!(a.is_parallel(&b, 1e-9));
}

// 30. is_parallel(): antiparallel axes are parallel.
#[test]
fn gpax1ext_is_parallel_antiparallel() {
    let a = GpAx1Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let b = GpAx1Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]);
    assert!(a.is_parallel(&b, 1e-9));
}

// 31. is_parallel(): perpendicular axes are not parallel.
#[test]
fn gpax1ext_is_parallel_perp_false() {
    let a = GpAx1Ext::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let b = GpAx1Ext::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    assert!(!a.is_parallel(&b, 1e-9));
}

// 32. is_coaxial(): same axis → coaxial.
#[test]
fn gpax1ext_is_coaxial_same_axis() {
    let a = GpAx1Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let b = GpAx1Ext::new([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
    assert!(a.is_coaxial(&b, 1e-9, 1e-7));
}

// 33. is_coaxial(): parallel but offset → not coaxial.
#[test]
fn gpax1ext_is_coaxial_parallel_offset_false() {
    let a = GpAx1Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let b = GpAx1Ext::new([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    assert!(!a.is_coaxial(&b, 1e-9, 1e-7));
}

// 34. is_coaxial(): perpendicular axes → not coaxial.
#[test]
fn gpax1ext_is_coaxial_perpendicular_false() {
    let a = GpAx1Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let b = GpAx1Ext::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    assert!(!a.is_coaxial(&b, 1e-9, 1e-7));
}

// ── GpAx3Ext ─────────────────────────────────────────────────────────────────

// 35. Standard frame: Z=+Z, X=+X → Y=+Y.
#[test]
fn gpax3ext_standard_frame() {
    let ax = GpAx3Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    near(ax.x_direction().x(), 1.0, LIN_TOL);
    near(ax.x_direction().y(), 0.0, LIN_TOL);
    near(ax.x_direction().z(), 0.0, LIN_TOL);
    near(ax.y_direction().x(), 0.0, LIN_TOL);
    near(ax.y_direction().y(), 1.0, LIN_TOL);
    near(ax.y_direction().z(), 0.0, LIN_TOL);
    near(ax.z_direction().x(), 0.0, LIN_TOL);
    near(ax.z_direction().y(), 0.0, LIN_TOL);
    near(ax.z_direction().z(), 1.0, LIN_TOL);
}

// 36. origin() round-trips correctly.
#[test]
fn gpax3ext_origin_roundtrip() {
    let ax = GpAx3Ext::new([3.0, 4.0, 5.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    let o = ax.origin();
    near(o[0], 3.0, LIN_TOL);
    near(o[1], 4.0, LIN_TOL);
    near(o[2], 5.0, LIN_TOL);
}

// 37. is_direct() is always true.
#[test]
fn gpax3ext_is_direct_always_true() {
    let ax = GpAx3Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    assert!(ax.is_direct());
}

// 38. Y direction is orthogonal to X and Z.
#[test]
fn gpax3ext_y_orthogonal_to_x_and_z() {
    let ax = GpAx3Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    let dot_xy = ax.x_direction().dot(ax.y_direction());
    let dot_zy = ax.z_direction().dot(ax.y_direction());
    near(dot_xy, 0.0, LIN_TOL);
    near(dot_zy, 0.0, LIN_TOL);
}

// 39. angle() between identical frames is 0.
#[test]
fn gpax3ext_angle_same_frame_is_zero() {
    let a = GpAx3Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    let b = GpAx3Ext::new([5.0, 5.0, 5.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    near(a.angle(&b), 0.0, ANG_TOL);
}

// 40. angle() between Z=+Z and Z=+X frames is π/2.
#[test]
fn gpax3ext_angle_z_vs_x_is_pi_over_2() {
    let a = GpAx3Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    // Frame with Z = +X, X = +Y
    let b = GpAx3Ext::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    near(a.angle(&b), FRAC_PI_2, ANG_TOL);
}

// 41. angle() between Z=+Z and Z=-Z frames is π.
#[test]
fn gpax3ext_angle_opposite_z_is_pi() {
    let a = GpAx3Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    let b = GpAx3Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0]);
    near(a.angle(&b), PI, LIN_TOL);
}

// 42. angle() is symmetric.
#[test]
fn gpax3ext_angle_symmetric() {
    let a = GpAx3Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    let b = GpAx3Ext::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    near(a.angle(&b), b.angle(&a), ANG_TOL);
}

// 43. x_dir not perpendicular to z_dir is orthogonalized correctly.
#[test]
fn gpax3ext_x_orthogonalized_against_z() {
    // Supply x_dir = (1,0,1), z_dir = (0,0,1).
    // Orthogonal component of x in the XY plane = (1,0,0).
    let ax = GpAx3Ext::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 1.0]);
    near(ax.x_direction().x(), 1.0, LIN_TOL);
    near(ax.x_direction().y(), 0.0, LIN_TOL);
    near(ax.x_direction().z(), 0.0, LIN_TOL);
}

// 44. Non-standard frame: Z=+Y, X=+Z → Y = +Y × +Z = +X.
#[test]
fn gpax3ext_nonstandard_frame_y_up() {
    // z_dir = +Y, x_dir = +Z → y = z × x = +Y × +Z = +X
    let ax = GpAx3Ext::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
    near(ax.z_direction().y(), 1.0, LIN_TOL);
    near(ax.x_direction().z(), 1.0, LIN_TOL);
    near(ax.y_direction().x(), 1.0, LIN_TOL);
    // Verify orthogonality
    near(ax.x_direction().dot(ax.y_direction()), 0.0, LIN_TOL);
    near(ax.x_direction().dot(ax.z_direction()), 0.0, LIN_TOL);
    near(ax.y_direction().dot(ax.z_direction()), 0.0, LIN_TOL);
    // Always direct
    assert!(ax.is_direct());
}

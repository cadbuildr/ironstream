// FILE: rust/ironstream/crates/ironstream/tests/occt_brep_pattern.rs
//! Integration tests for `brep_pattern` — BRepFeat pattern types.
//!
//! Covers [`PatternType`], [`LinearPatternParams`], [`CircularPatternParams`],
//! and [`PatternResult`], including transform correctness for both linear and
//! circular patterns.

extern crate ironstream;
use ironstream::brep_pattern::*;
use std::f64::consts::PI;

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
}

/// Apply a 4×4 row-major homogeneous matrix to a point `[x, y, z, 1]` and
/// return the resulting `[x', y', z']`.
fn apply_mat(m: [[f64; 4]; 4], p: [f64; 3]) -> [f64; 3] {
    let [x, y, z] = p;
    let w = 1.0_f64;
    [
        m[0][0] * x + m[0][1] * y + m[0][2] * z + m[0][3] * w,
        m[1][0] * x + m[1][1] * y + m[1][2] * z + m[1][3] * w,
        m[2][0] * x + m[2][1] * y + m[2][2] * z + m[2][3] * w,
    ]
}

fn vec3_near(a: [f64; 3], b: [f64; 3], tol: f64) {
    for i in 0..3 {
        near(a[i], b[i], tol);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PatternType
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn pattern_type_debug_and_eq() {
    assert_eq!(PatternType::Linear, PatternType::Linear);
    assert_ne!(PatternType::Linear, PatternType::Circular);
    assert_ne!(PatternType::Mirror, PatternType::Scale);
}

#[test]
fn pattern_type_copy() {
    let t = PatternType::Circular;
    let t2 = t; // Copy trait
    assert_eq!(t, t2);
}

#[test]
fn pattern_type_all_variants_exist() {
    let variants = [
        PatternType::Linear,
        PatternType::Circular,
        PatternType::Mirror,
        PatternType::Scale,
    ];
    // Each variant is distinct.
    for i in 0..variants.len() {
        for j in 0..variants.len() {
            if i == j {
                assert_eq!(variants[i], variants[j]);
            } else {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LinearPatternParams
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn linear_params_new_stores_fields() {
    let p = LinearPatternParams::new([1.0, 0.0, 0.0], 4, 5.0);
    assert_eq!(p.direction(), [1.0, 0.0, 0.0]);
    assert_eq!(p.count(), 4);
    near(p.pitch(), 5.0, 1e-12);
}

#[test]
fn linear_params_total_length_multi() {
    // 5 instances at pitch 3.0 → span = (5-1)*3.0 = 12.0
    let p = LinearPatternParams::new([0.0, 1.0, 0.0], 5, 3.0);
    near(p.total_length(), 12.0, 1e-12);
}

#[test]
fn linear_params_total_length_single_instance() {
    let p = LinearPatternParams::new([1.0, 0.0, 0.0], 1, 10.0);
    near(p.total_length(), 0.0, 1e-12);
}

#[test]
fn linear_params_total_length_zero_count() {
    let p = LinearPatternParams::new([1.0, 0.0, 0.0], 0, 10.0);
    near(p.total_length(), 0.0, 1e-12);
}

#[test]
fn linear_params_negative_pitch() {
    // Negative pitch is valid; total_length mirrors the sign.
    let p = LinearPatternParams::new([1.0, 0.0, 0.0], 3, -4.0);
    near(p.total_length(), -8.0, 1e-12); // (3-1)*(-4) = -8
}

// ─────────────────────────────────────────────────────────────────────────────
// CircularPatternParams
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn circular_params_new_stores_fields() {
    let p = CircularPatternParams::new([0.0, 0.0, 1.0], 6, PI);
    assert_eq!(p.axis(), [0.0, 0.0, 1.0]);
    assert_eq!(p.count(), 6);
    near(p.angle(), PI, 1e-12);
}

#[test]
fn circular_params_partial_angle_not_full() {
    let p = CircularPatternParams::new([0.0, 0.0, 1.0], 4, PI / 2.0);
    assert!(!p.is_full_rotation());
}

#[test]
fn circular_params_two_pi_is_full() {
    let p = CircularPatternParams::new([0.0, 0.0, 1.0], 8, 2.0 * PI);
    assert!(p.is_full_rotation());
}

#[test]
fn circular_params_above_two_pi_is_full() {
    let p = CircularPatternParams::new([0.0, 0.0, 1.0], 4, 7.0); // > 2π ≈ 6.28
    assert!(p.is_full_rotation());
}

#[test]
fn circular_params_set_full_rotation_override() {
    let mut p = CircularPatternParams::new([0.0, 0.0, 1.0], 4, PI / 2.0);
    assert!(!p.is_full_rotation());
    p.set_full_rotation(true);
    assert!(p.is_full_rotation());
    p.set_full_rotation(false);
    assert!(!p.is_full_rotation());
}

#[test]
fn circular_params_copy() {
    let p = CircularPatternParams::new([1.0, 0.0, 0.0], 3, PI);
    let p2 = p; // Copy
    assert_eq!(p.count(), p2.count());
}

// ─────────────────────────────────────────────────────────────────────────────
// PatternResult — construction & state
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn result_new_not_done() {
    let r = PatternResult::new(PatternType::Linear);
    assert!(!r.is_done());
    assert_eq!(r.nb_instances(), 0);
    assert_eq!(r.pattern_type(), PatternType::Linear);
}

#[test]
fn result_transform_out_of_range_returns_identity() {
    let r = PatternResult::new(PatternType::Circular);
    let id = r.transform(99);
    // Diagonal must be 1, off-diagonal 0 (identity).
    for row in 0..4 {
        for col in 0..4 {
            let expected = if row == col { 1.0 } else { 0.0 };
            near(id[row][col], expected, 1e-12);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PatternResult::perform_linear
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn linear_result_is_done_after_perform() {
    let params = LinearPatternParams::new([1.0, 0.0, 0.0], 3, 2.0);
    let mut r = PatternResult::new(PatternType::Linear);
    r.perform_linear(&params);
    assert!(r.is_done());
    assert_eq!(r.nb_instances(), 3);
}

#[test]
fn linear_result_first_transform_is_identity() {
    let params = LinearPatternParams::new([0.0, 1.0, 0.0], 4, 1.5);
    let mut r = PatternResult::new(PatternType::Linear);
    r.perform_linear(&params);
    let t0 = r.transform(0);
    for row in 0..4 {
        for col in 0..4 {
            let expected = if row == col { 1.0 } else { 0.0 };
            near(t0[row][col], expected, 1e-12);
        }
    }
}

#[test]
fn linear_result_translates_along_x() {
    // 4 instances, pitch 3.0, direction +X.
    let params = LinearPatternParams::new([1.0, 0.0, 0.0], 4, 3.0);
    let mut r = PatternResult::new(PatternType::Linear);
    r.perform_linear(&params);

    let origin = [0.0, 0.0, 0.0];
    vec3_near(apply_mat(r.transform(0), origin), [0.0, 0.0, 0.0], 1e-10);
    vec3_near(apply_mat(r.transform(1), origin), [3.0, 0.0, 0.0], 1e-10);
    vec3_near(apply_mat(r.transform(2), origin), [6.0, 0.0, 0.0], 1e-10);
    vec3_near(apply_mat(r.transform(3), origin), [9.0, 0.0, 0.0], 1e-10);
}

#[test]
fn linear_result_translates_along_y() {
    let params = LinearPatternParams::new([0.0, 1.0, 0.0], 3, 5.0);
    let mut r = PatternResult::new(PatternType::Linear);
    r.perform_linear(&params);

    let pt = [1.0, 1.0, 1.0];
    vec3_near(apply_mat(r.transform(0), pt), [1.0, 1.0, 1.0], 1e-10);
    vec3_near(apply_mat(r.transform(1), pt), [1.0, 6.0, 1.0], 1e-10);
    vec3_near(apply_mat(r.transform(2), pt), [1.0, 11.0, 1.0], 1e-10);
}

#[test]
fn linear_result_normalises_direction() {
    // Direction [2, 0, 0] should behave identically to [1, 0, 0] after
    // normalisation; only the pitch governs the spacing.
    let params = LinearPatternParams::new([2.0, 0.0, 0.0], 3, 4.0);
    let mut r = PatternResult::new(PatternType::Linear);
    r.perform_linear(&params);

    let origin = [0.0, 0.0, 0.0];
    vec3_near(apply_mat(r.transform(1), origin), [4.0, 0.0, 0.0], 1e-10);
    vec3_near(apply_mat(r.transform(2), origin), [8.0, 0.0, 0.0], 1e-10);
}

#[test]
fn linear_result_diagonal_direction() {
    // Direction [1,1,0], pitch sqrt(2) → each step moves 1 unit in X and Y.
    let pitch = 2.0_f64.sqrt();
    let params = LinearPatternParams::new([1.0, 1.0, 0.0], 3, pitch);
    let mut r = PatternResult::new(PatternType::Linear);
    r.perform_linear(&params);

    let origin = [0.0, 0.0, 0.0];
    // instance 1: translate by pitch along unit([1,1,0]) = (1,1,0)
    let p1 = apply_mat(r.transform(1), origin);
    near(p1[0], 1.0, 1e-10);
    near(p1[1], 1.0, 1e-10);
    near(p1[2], 0.0, 1e-10);

    let p2 = apply_mat(r.transform(2), origin);
    near(p2[0], 2.0, 1e-10);
    near(p2[1], 2.0, 1e-10);
}

#[test]
fn linear_result_zero_count_not_done() {
    let params = LinearPatternParams::new([1.0, 0.0, 0.0], 0, 1.0);
    let mut r = PatternResult::new(PatternType::Linear);
    r.perform_linear(&params);
    assert!(!r.is_done());
    assert_eq!(r.nb_instances(), 0);
}

#[test]
fn linear_result_single_instance_is_identity_only() {
    let params = LinearPatternParams::new([0.0, 0.0, 1.0], 1, 10.0);
    let mut r = PatternResult::new(PatternType::Linear);
    r.perform_linear(&params);
    assert!(r.is_done());
    assert_eq!(r.nb_instances(), 1);
    // The sole transform is identity.
    let t = r.transform(0);
    for row in 0..4 {
        for col in 0..4 {
            near(t[row][col], if row == col { 1.0 } else { 0.0 }, 1e-12);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PatternResult::perform_circular
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn circular_result_is_done_after_perform() {
    let params = CircularPatternParams::new([0.0, 0.0, 1.0], 4, PI / 2.0);
    let mut r = PatternResult::new(PatternType::Circular);
    r.perform_circular(&params);
    assert!(r.is_done());
    assert_eq!(r.nb_instances(), 4);
}

#[test]
fn circular_result_first_transform_is_identity() {
    let params = CircularPatternParams::new([0.0, 0.0, 1.0], 6, PI);
    let mut r = PatternResult::new(PatternType::Circular);
    r.perform_circular(&params);
    let t0 = r.transform(0);
    for row in 0..4 {
        for col in 0..4 {
            near(t0[row][col], if row == col { 1.0 } else { 0.0 }, 1e-12);
        }
    }
}

#[test]
fn circular_result_quarter_turns_about_z() {
    // 4 instances at 90° steps about Z; maps (1,0,0) → (0,1,0) → (-1,0,0) → (0,-1,0).
    let mut params = CircularPatternParams::new([0.0, 0.0, 1.0], 4, PI / 2.0);
    params.set_full_rotation(false); // step = (π/2)/3 if not full; we want 90° steps
    // For a non-full rotation with 4 instances: step = angle/(4-1) = π/6 — not what we want.
    // Use full rotation to get step = 2π/4 = π/2.
    params.set_full_rotation(true);
    let mut r = PatternResult::new(PatternType::Circular);
    r.perform_circular(&params);

    let p = [1.0, 0.0, 0.0];
    vec3_near(apply_mat(r.transform(0), p), [1.0, 0.0, 0.0], 1e-10);
    vec3_near(apply_mat(r.transform(1), p), [0.0, 1.0, 0.0], 1e-10);
    vec3_near(apply_mat(r.transform(2), p), [-1.0, 0.0, 0.0], 1e-10);
    vec3_near(apply_mat(r.transform(3), p), [0.0, -1.0, 0.0], 1e-10);
}

#[test]
fn circular_result_180_degree_two_instances_partial() {
    // 2 instances spanning π: instance 0 = identity, instance 1 = 180° rotation.
    // With partial (not full): step = π/(2-1) = π.
    let params = CircularPatternParams::new([0.0, 0.0, 1.0], 2, PI);
    let mut r = PatternResult::new(PatternType::Circular);
    r.perform_circular(&params);
    assert_eq!(r.nb_instances(), 2);

    let p = [1.0, 0.0, 0.0];
    // Instance 0: identity.
    vec3_near(apply_mat(r.transform(0), p), [1.0, 0.0, 0.0], 1e-10);
    // Instance 1: 180° about Z maps (1,0,0) → (-1,0,0).
    let p1 = apply_mat(r.transform(1), p);
    near(p1[0], -1.0, 1e-10);
    near(p1[1], 0.0, 1e-10);
    near(p1[2], 0.0, 1e-10);
}

#[test]
fn circular_result_rotation_about_x_axis() {
    // 4 instances, full rotation about X: steps at 0°, 90°, 180°, 270°.
    // (0,1,0) → (0,0,1) → (0,-1,0) → (0,0,-1).
    let params = CircularPatternParams::new([1.0, 0.0, 0.0], 4, 2.0 * PI);
    let mut r = PatternResult::new(PatternType::Circular);
    r.perform_circular(&params);

    let p = [0.0, 1.0, 0.0];
    vec3_near(apply_mat(r.transform(0), p), [0.0, 1.0, 0.0], 1e-10);
    // 90° about X: (0,1,0) → (0,0,1)
    let p1 = apply_mat(r.transform(1), p);
    near(p1[0], 0.0, 1e-10);
    near(p1[1], 0.0, 1e-10);
    near(p1[2], 1.0, 1e-10);
    // 180° about X: (0,1,0) → (0,-1,0)
    let p2 = apply_mat(r.transform(2), p);
    near(p2[1], -1.0, 1e-10);
    near(p2[2], 0.0, 1e-10);
}

#[test]
fn circular_result_rotation_preserves_z_component() {
    // Rotation about Z should not change the Z coordinate of any point.
    let params = CircularPatternParams::new([0.0, 0.0, 1.0], 8, 2.0 * PI);
    let mut r = PatternResult::new(PatternType::Circular);
    r.perform_circular(&params);

    let p = [3.0, 4.0, 7.0];
    for i in 0..8 {
        let q = apply_mat(r.transform(i), p);
        near(q[2], 7.0, 1e-10); // Z unchanged by rotation about Z-axis
    }
}

#[test]
fn circular_result_rotation_preserves_radius() {
    // Each rotated point must lie at the same XY radius as the original.
    let params = CircularPatternParams::new([0.0, 0.0, 1.0], 5, 2.0 * PI);
    let mut r = PatternResult::new(PatternType::Circular);
    r.perform_circular(&params);

    let p = [3.0, 4.0, 0.0]; // radius = 5
    for i in 0..5 {
        let q = apply_mat(r.transform(i), p);
        let rq = (q[0] * q[0] + q[1] * q[1]).sqrt();
        near(rq, 5.0, 1e-10);
    }
}

#[test]
fn circular_result_zero_count_not_done() {
    let params = CircularPatternParams::new([0.0, 0.0, 1.0], 0, PI);
    let mut r = PatternResult::new(PatternType::Circular);
    r.perform_circular(&params);
    assert!(!r.is_done());
    assert_eq!(r.nb_instances(), 0);
}

#[test]
fn circular_result_single_instance_is_identity_only() {
    let params = CircularPatternParams::new([0.0, 0.0, 1.0], 1, PI);
    let mut r = PatternResult::new(PatternType::Circular);
    r.perform_circular(&params);
    assert!(r.is_done());
    assert_eq!(r.nb_instances(), 1);
    let t = r.transform(0);
    for row in 0..4 {
        for col in 0..4 {
            near(t[row][col], if row == col { 1.0 } else { 0.0 }, 1e-12);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PatternResult — re-perform resets state
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn result_perform_linear_twice_replaces_previous() {
    let p1 = LinearPatternParams::new([1.0, 0.0, 0.0], 3, 2.0);
    let p2 = LinearPatternParams::new([0.0, 1.0, 0.0], 5, 1.0);

    let mut r = PatternResult::new(PatternType::Linear);
    r.perform_linear(&p1);
    assert_eq!(r.nb_instances(), 3);

    r.perform_linear(&p2);
    assert_eq!(r.nb_instances(), 5);
    // Second perform must overwrite; translation should now be in Y.
    let q = apply_mat(r.transform(1), [0.0, 0.0, 0.0]);
    near(q[0], 0.0, 1e-10);
    near(q[1], 1.0, 1e-10);
}

#[test]
fn result_perform_circular_twice_replaces_previous() {
    let p1 = CircularPatternParams::new([0.0, 0.0, 1.0], 4, 2.0 * PI);
    let p2 = CircularPatternParams::new([0.0, 0.0, 1.0], 6, 2.0 * PI);

    let mut r = PatternResult::new(PatternType::Circular);
    r.perform_circular(&p1);
    assert_eq!(r.nb_instances(), 4);
    r.perform_circular(&p2);
    assert_eq!(r.nb_instances(), 6);
}

// ─────────────────────────────────────────────────────────────────────────────
// PatternResult — pattern_type accessor
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn result_stores_pattern_type() {
    let r_lin = PatternResult::new(PatternType::Linear);
    assert_eq!(r_lin.pattern_type(), PatternType::Linear);

    let r_circ = PatternResult::new(PatternType::Circular);
    assert_eq!(r_circ.pattern_type(), PatternType::Circular);

    let r_mir = PatternResult::new(PatternType::Mirror);
    assert_eq!(r_mir.pattern_type(), PatternType::Mirror);

    let r_scl = PatternResult::new(PatternType::Scale);
    assert_eq!(r_scl.pattern_type(), PatternType::Scale);
}

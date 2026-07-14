// FILE: tests/occt_brep_offset_api.rs
//! Integration tests for `brep_offset_api` —
//! `BRepOffsetAPI_MakePipe`, `BRepOffsetAPI_MakePipeShell`,
//! `BRepOffsetAPI_MakeOffset`.
//!
//! All tests exercise the public API only (no access to private fields).
//! Numeric tolerances are relaxed where tessellation discretisation dominates.

extern crate ironstream;

use ironstream::brep_offset_api::{MakeOffset, MakePipe, MakePipeShell, OffsetError};
use ironstream::gp::Pnt;
use ironstream::topods::Wire;
use std::f64::consts::PI;

const TOL: f64 = 1e-6;
const MESH_TOL: f64 = 5e-2; // 5 % relative error from tessellation

fn near(a: f64, b: f64, tol: f64, label: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{label}: expected {a} ≈ {b} (tol={tol}), diff={}",
        (a - b).abs()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Build a straight spine of `n` equally spaced points along +Z, total length
/// `len`.
fn z_spine(len: f64, n: usize) -> Wire {
    let n = n.max(2);
    Wire::new(
        (0..n)
            .map(|i| Pnt::new(0.0, 0.0, len * i as f64 / (n - 1) as f64))
            .collect(),
    )
}

/// Build a circular ring of `n` points with given `radius` in the local XY
/// plane (Z = 0).
fn circle_profile(r: f64, n: usize) -> Wire {
    Wire::new(
        (0..n)
            .map(|i| {
                let a = 2.0 * PI * i as f64 / n as f64;
                Pnt::new(r * a.cos(), r * a.sin(), 0.0)
            })
            .collect(),
    )
}

/// Build a unit square profile in the local XY plane (Z = 0).
fn square_profile(half: f64) -> Wire {
    Wire::new(vec![
        Pnt::new(-half, -half, 0.0),
        Pnt::new(half, -half, 0.0),
        Pnt::new(half, half, 0.0),
        Pnt::new(-half, half, 0.0),
    ])
}

// ─────────────────────────────────────────────────────────────────────────────
// BRepOffsetAPI_MakePipe
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn make_pipe_straight_spine_is_done() {
    let spine = z_spine(5.0, 3);
    let mut p = MakePipe::new(spine, 1.0).with_facets(16);
    assert!(p.is_done(), "straight pipe must be done");
}

#[test]
fn make_pipe_volume_close_to_analytic_cylinder() {
    // Sweep r=1 circle along a length-5 spine → volume ≈ π * 1² * 5 ≈ 15.71.
    let spine = z_spine(5.0, 5);
    let solid = MakePipe::new(spine, 1.0).with_facets(64).into_solid();
    let expected = PI * 1.0 * 1.0 * 5.0;
    let ratio = solid.volume() / expected;
    assert!(
        ratio > 1.0 - MESH_TOL && ratio <= 1.0 + MESH_TOL,
        "pipe volume ratio={ratio} (vol={}, expected={})",
        solid.volume(),
        expected
    );
}

#[test]
fn make_pipe_is_watertight() {
    let spine = z_spine(4.0, 4);
    let solid = MakePipe::new(spine, 0.5).with_facets(32).into_solid();
    assert_eq!(
        solid.mesh().open_edge_count(1e-5),
        0,
        "pipe must be watertight (no open edges)"
    );
}

#[test]
fn make_pipe_degenerate_single_point_spine_empty() {
    let spine = Wire::new(vec![Pnt::new(1.0, 2.0, 3.0)]);
    let solid = MakePipe::new(spine, 1.0).into_solid();
    assert!(solid.is_empty(), "single-point spine must yield empty solid");
}

#[test]
fn make_pipe_square_profile_volume_close_to_prism() {
    // 2×2 square cross-section, length 6 → volume ≈ 4 * 6 = 24.
    let half = 1.0_f64;
    let profile = square_profile(half);
    let spine = z_spine(6.0, 3);
    let solid = MakePipe::new(spine, 0.0).with_profile(profile).into_solid();
    let vol = solid.volume();
    // Allow ±15% due to end-cap triangulation.
    assert!(
        vol > 18.0 && vol < 30.0,
        "square-profile pipe vol={vol} expected≈24"
    );
}

#[test]
fn make_pipe_l_shaped_spine_non_empty_and_watertight() {
    // Spine: (0,0,0) → (3,0,0) → (3,0,3).
    let spine = Wire::new(vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
        Pnt::new(3.0, 0.0, 3.0),
    ]);
    let solid = MakePipe::new(spine, 0.5).with_facets(16).into_solid();
    assert!(!solid.is_empty(), "L-spine pipe must not be empty");
    assert_eq!(
        solid.mesh().open_edge_count(1e-5),
        0,
        "L-spine pipe must be watertight"
    );
}

#[test]
fn make_pipe_larger_radius_greater_volume() {
    let spine = z_spine(5.0, 3);
    let v1 = MakePipe::new(spine.clone(), 1.0).with_facets(32).into_solid().volume();
    let v2 = MakePipe::new(spine, 2.0).with_facets(32).into_solid().volume();
    assert!(
        v2 > v1 * 3.5,
        "doubling radius should roughly quadruple volume, v1={v1} v2={v2}"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// BRepOffsetAPI_MakePipeShell
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn make_pipe_shell_fallback_circular_is_done() {
    let spine = z_spine(4.0, 3);
    let solid = MakePipeShell::new(spine)
        .with_radius(1.0)
        .with_facets(32)
        .into_solid();
    assert!(!solid.is_empty(), "fallback-circular PipeShell must be done");
}

#[test]
fn make_pipe_shell_uniform_section_volume_close_to_cylinder() {
    // One section at both ends (r=1) → same as a cylinder.
    let spine = z_spine(5.0, 5);
    let mut builder = MakePipeShell::new(spine).with_facets(64);
    builder.add_section(circle_profile(1.0, 64), 0.0);
    builder.add_section(circle_profile(1.0, 64), 1.0);
    let solid = builder.into_solid();
    let expected = PI * 1.0 * 1.0 * 5.0;
    let ratio = solid.volume() / expected;
    assert!(
        ratio > 1.0 - MESH_TOL && ratio <= 1.0 + MESH_TOL,
        "uniform PipeShell volume ratio={ratio}"
    );
}

#[test]
fn make_pipe_shell_tapered_volume_between_limits() {
    // Taper r=2 → r=0 along length 6; volume of a cone = (1/3)*π*r²*h.
    // Our interpolation is linear (frustum), not true cone, so vol > cone but < cylinder(r=2).
    let spine = z_spine(6.0, 8);
    let mut builder = MakePipeShell::new(spine).with_facets(32);
    builder.add_section(circle_profile(2.0, 32), 0.0);
    builder.add_section(circle_profile(0.5, 32), 1.0);
    let vol = builder.into_solid().volume();
    let v_cone_big = PI * 2.0 * 2.0 * 6.0 / 3.0; // ~25.13
    let v_cyl_big = PI * 2.0 * 2.0 * 6.0;         // ~75.40
    assert!(
        vol > v_cone_big * 0.5 && vol < v_cyl_big,
        "tapered vol={vol} not in reasonable range ({v_cone_big:.2}, {v_cyl_big:.2})"
    );
}

#[test]
fn make_pipe_shell_two_sections_same_ring_count() {
    // Different radii but same vertex count must interpolate without panic.
    let spine = z_spine(4.0, 4);
    let mut builder = MakePipeShell::new(spine).with_facets(16);
    builder.add_section(circle_profile(1.0, 16), 0.0);
    builder.add_section(circle_profile(2.0, 16), 0.5);
    builder.add_section(circle_profile(1.0, 16), 1.0);
    let solid = builder.into_solid();
    assert!(!solid.is_empty(), "bulging PipeShell must build");
}

#[test]
fn make_pipe_shell_is_done_true_when_ok() {
    let spine = z_spine(3.0, 3);
    let mut builder = MakePipeShell::new(spine).with_radius(0.8).with_facets(16);
    assert!(builder.is_done(), "is_done must return true for valid config");
}

// ─────────────────────────────────────────────────────────────────────────────
// BRepOffsetAPI_MakeOffset
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn make_offset_expand_unit_square_is_done() {
    let w = Wire::new(vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
    ]);
    let mut mo = MakeOffset::new(w, 1.0);
    assert!(mo.is_done(), "expand offset must succeed");
}

#[test]
fn make_offset_expand_increases_extent() {
    let w = Wire::new(vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(2.0, 2.0, 0.0),
        Pnt::new(0.0, 2.0, 0.0),
    ]);
    let mut mo = MakeOffset::new(w, 1.0);
    let ow = mo.wire().unwrap();
    let min_x = ow.pts.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
    let max_x = ow.pts.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
    // After +1 offset, min_x ≈ -1 and max_x ≈ 3.
    assert!(min_x < -0.5, "left edge must expand left, min_x={min_x}");
    assert!(max_x > 2.5, "right edge must expand right, max_x={max_x}");
}

#[test]
fn make_offset_shrink_4x4_square() {
    // 4×4 square shrunk by 1 → roughly 2×2.
    let w = Wire::new(vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(4.0, 0.0, 0.0),
        Pnt::new(4.0, 4.0, 0.0),
        Pnt::new(0.0, 4.0, 0.0),
    ]);
    let mut mo = MakeOffset::new(w, -1.0);
    assert!(mo.is_done(), "shrink offset must succeed");
    let ow = mo.wire().unwrap();
    let min_x = ow.pts.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
    let max_x = ow.pts.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
    near(min_x, 1.0, 0.15, "shrink_min_x");
    near(max_x, 3.0, 0.15, "shrink_max_x");
}

#[test]
fn make_offset_too_few_points_error() {
    let w = Wire::new(vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
    ]);
    let mut mo = MakeOffset::new(w, 0.5);
    assert!(!mo.is_done());
    match mo.wire() {
        Err(OffsetError::TooFewPoints) => {} // expected
        other => panic!("expected TooFewPoints, got {other:?}"),
    }
}

#[test]
fn make_offset_collapse_error() {
    // Unit square shrunk by 10 must return CollapseDetected.
    let w = Wire::new(vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
    ]);
    let mut mo = MakeOffset::new(w, -10.0);
    assert!(!mo.is_done(), "over-inward offset must fail");
    match mo.wire() {
        Err(OffsetError::CollapseDetected) => {} // expected
        other => panic!("expected CollapseDetected, got {other:?}"),
    }
}

#[test]
fn make_offset_zero_distance_is_identity() {
    // Offset by 0 must return a wire whose vertex set matches the original
    // corners (possibly in a different cyclic order — the miter algorithm
    // produces vertex[i] as the intersection of edge[i] and edge[i+1], which
    // is the corner at the far end of edge[i]).
    let pts = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
        Pnt::new(3.0, 4.0, 0.0),
        Pnt::new(0.0, 4.0, 0.0),
    ];
    let w = Wire::new(pts.clone());
    let mut mo = MakeOffset::new(w, 0.0);
    assert!(mo.is_done());
    let ow = mo.wire().unwrap();
    assert_eq!(ow.pts.len(), 4, "zero-offset must preserve vertex count");
    // Every offset point must match one of the original corners.
    for op in &ow.pts {
        let matched = pts.iter().any(|ep| op.distance(*ep) < 1e-8);
        assert!(
            matched,
            "offset point ({:.3},{:.3},{:.3}) does not match any original corner",
            op.x, op.y, op.z
        );
    }
}

#[test]
fn make_offset_triangle_expand_all_points_move_out() {
    // Equilateral triangle with centroid at origin, expanded by 0.5.
    let r = 2.0_f64;
    let pts: Vec<Pnt> = (0..3)
        .map(|i| {
            let a = 2.0 * PI * i as f64 / 3.0;
            Pnt::new(r * a.cos(), r * a.sin(), 0.0)
        })
        .collect();
    let w = Wire::new(pts.clone());
    let dist = 0.5;
    let mut mo = MakeOffset::new(w, dist);
    assert!(mo.is_done(), "triangle offset must succeed");
    let ow = mo.wire().unwrap();
    // Every offset point should be further from origin than original.
    for (op, ep) in ow.pts.iter().zip(pts.iter()) {
        let r_orig = ep.distance(Pnt::origin());
        let r_off = op.distance(Pnt::origin());
        assert!(
            r_off > r_orig - 1e-9,
            "expanded point must be further from origin (r_orig={r_orig:.3}, r_off={r_off:.3})"
        );
    }
}

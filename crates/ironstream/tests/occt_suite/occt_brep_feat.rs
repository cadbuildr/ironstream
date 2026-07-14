// FILE: tests/occt_brep_feat.rs
//! Integration tests for `brep_feat` — BRepFeat feature operations.
//!
//! Exercises the public API of `BRepFeat_MakePrism`, `BRepFeat_MakeRevol`, and
//! `BRepFeat_MakePipe` using only exported types. Numeric tolerances reflect
//! the tessellation-based kernel (errors of order 1% are normal).

extern crate ironstream;

use ironstream::brep_feat::{BRepFeat_MakePipe, BRepFeat_MakePrism, BRepFeat_MakeRevol, FeatMode};
use ironstream::brep_prim_api::{make_box, make_cylinder, MeshParams};
use ironstream::gp::{Ax1, Pnt};
use ironstream::topods::{Face, Solid, Wire};
use std::f64::consts::PI;

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

fn near(a: f64, b: f64, tol: f64, label: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{label}: got {a}, expected ~{b} (tol={tol}), diff={}",
        (a - b).abs()
    );
}

/// 4×4×4 box at the origin (volume = 64).
fn base_box() -> Solid {
    make_box(Pnt::origin(), 4.0, 4.0, 4.0)
}

/// Closed rectangular wire in a plane.
fn rect_wire(x0: f64, y0: f64, z: f64, x1: f64, y1: f64) -> Wire {
    Wire::new(vec![
        Pnt::new(x0, y0, z),
        Pnt::new(x1, y0, z),
        Pnt::new(x1, y1, z),
        Pnt::new(x0, y1, z),
    ])
}

// ─────────────────────────────────────────────────────────────────────────────
// FeatMode
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn feat_mode_variants_are_distinct() {
    assert_ne!(FeatMode::Add, FeatMode::Remove);
}

#[test]
fn feat_mode_default_is_add() {
    assert_eq!(FeatMode::default(), FeatMode::Add);
}

// ─────────────────────────────────────────────────────────────────────────────
// BRepFeat_MakePrism — additive (pad / boss)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn prism_boss_on_top_of_box_increases_volume() {
    let base = base_box();
    let base_vol = base.volume();
    // 1×1 boss extruded +2 in Z from the top face (z=4).
    let profile = Face::new(rect_wire(1.5, 1.5, 4.0, 2.5, 2.5));
    let result = BRepFeat_MakePrism::new(base, profile, Pnt::new(0.0, 0.0, 2.0), FeatMode::Add)
        .into_solid();
    // Boss volume = 1 * 1 * 2 = 2.
    let expected_min = base_vol + 1.5; // at least 75% of theoretical boss
    assert!(
        result.volume() > expected_min,
        "volume={} should be > {expected_min}",
        result.volume()
    );
}

#[test]
fn prism_boss_volume_close_to_analytic() {
    let base = make_box(Pnt::origin(), 10.0, 10.0, 1.0);
    // 2×2 boss, height=3, on top (z=1).
    let profile = Face::new(rect_wire(4.0, 4.0, 1.0, 6.0, 6.0));
    let result = BRepFeat_MakePrism::new(base, profile, Pnt::new(0.0, 0.0, 3.0), FeatMode::Add)
        .into_solid();
    let exact = 10.0 * 10.0 * 1.0 + 2.0 * 2.0 * 3.0;
    near(result.volume(), exact, exact * 0.02, "prism_boss_volume");
}

// ─────────────────────────────────────────────────────────────────────────────
// BRepFeat_MakePrism — subtractive (pocket)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn prism_pocket_decreases_volume() {
    let base = base_box();
    let base_vol = base.volume();
    // Pocket: 2×2 cross-section, depth=2, cut from top.
    let profile = Face::new(rect_wire(1.0, 1.0, 4.0, 3.0, 3.0));
    let result =
        BRepFeat_MakePrism::new(base, profile, Pnt::new(0.0, 0.0, -2.0), FeatMode::Remove)
            .into_solid();
    assert!(
        result.volume() < base_vol,
        "pocket should decrease volume; vol={}",
        result.volume()
    );
}

#[test]
fn prism_pocket_volume_less_than_base() {
    let base = make_box(Pnt::origin(), 10.0, 10.0, 10.0);
    let base_vol = base.volume();
    // 2×2×5 pocket from the top face.
    let profile = Face::new(rect_wire(4.0, 4.0, 10.0, 6.0, 6.0));
    let result =
        BRepFeat_MakePrism::new(base, profile, Pnt::new(0.0, 0.0, -5.0), FeatMode::Remove)
            .into_solid();
    assert!(
        result.volume() < base_vol,
        "pocket must reduce volume; vol={} base={}",
        result.volume(), base_vol
    );
    // Should remove at least 10 cubic units (lower bound).
    assert!(
        result.volume() < base_vol - 10.0,
        "pocket must remove a significant volume; vol={}",
        result.volume()
    );
}

#[test]
fn prism_accessors_are_correct() {
    let dir = Pnt::new(1.0, 2.0, 3.0);
    let op = BRepFeat_MakePrism::new(
        base_box(),
        Face::new(rect_wire(0.0, 0.0, 0.0, 1.0, 1.0)),
        dir,
        FeatMode::Remove,
    );
    assert!((op.direction().x - 1.0).abs() < 1e-12);
    assert!((op.direction().y - 2.0).abs() < 1e-12);
    assert!((op.direction().z - 3.0).abs() < 1e-12);
    assert_eq!(op.mode(), FeatMode::Remove);
}

// ─────────────────────────────────────────────────────────────────────────────
// BRepFeat_MakeRevol — additive
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn revol_full_turn_annular_boss_increases_volume() {
    // Thin slab 20×20×1.
    let base = make_box(Pnt::new(-10.0, -10.0, 0.0), 20.0, 20.0, 1.0);
    let base_vol = base.volume();

    // Rectangle in the XZ half-plane from (3,0,1) to (5,0,1) to (5,0,3) to (3,0,3).
    // Revolved 2π about Z => annular tube.
    let profile = Wire::new(vec![
        Pnt::new(3.0, 0.0, 1.0),
        Pnt::new(5.0, 0.0, 1.0),
        Pnt::new(5.0, 0.0, 3.0),
        Pnt::new(3.0, 0.0, 3.0),
    ]);
    let mut op = BRepFeat_MakeRevol::new(base, profile, Ax1::oz(), 2.0 * PI, FeatMode::Add);
    let result = op.build();
    assert!(
        result.volume() > base_vol,
        "annular boss should increase volume; vol={}",
        result.volume()
    );
    assert!(op.is_done(), "build must succeed");
}

#[test]
fn revol_axis_and_angle_accessors() {
    let axis = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let op = BRepFeat_MakeRevol::new(
        base_box(),
        Wire::new(vec![
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
            Pnt::new(2.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 1.0),
        ]),
        axis,
        PI / 2.0,
        FeatMode::Add,
    );
    near(op.angle(), PI / 2.0, 1e-12, "angle");
    assert!((op.axis().direction.z - 1.0).abs() < 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────
// BRepFeat_MakeRevol — subtractive
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn revol_cut_decreases_volume() {
    let base = make_box(Pnt::new(-10.0, -10.0, 0.0), 20.0, 20.0, 10.0);
    let base_vol = base.volume();

    // Cut a slot: thin rectangle near the Z axis.
    let profile = Wire::new(vec![
        Pnt::new(2.0, 0.0, 2.0),
        Pnt::new(3.0, 0.0, 2.0),
        Pnt::new(3.0, 0.0, 8.0),
        Pnt::new(2.0, 0.0, 8.0),
    ]);
    let result = BRepFeat_MakeRevol::new(
        base,
        profile,
        Ax1::oz(),
        2.0 * PI,
        FeatMode::Remove,
    )
    .into_solid();
    assert!(
        result.volume() < base_vol,
        "annular groove should decrease volume; vol={}",
        result.volume()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// BRepFeat_MakePipe — additive
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn pipe_boss_along_straight_spine_increases_volume() {
    let base = make_box(Pnt::origin(), 10.0, 10.0, 1.0);
    let base_vol = base.volume();

    // Straight spine running in X above the slab.
    let spine = Wire::new(vec![
        Pnt::new(1.0, 5.0, 1.0),
        Pnt::new(9.0, 5.0, 1.0),
    ]);
    let mut op = BRepFeat_MakePipe::new(base, spine, 0.5, FeatMode::Add).with_facets(16);
    let result = op.build();
    assert!(
        result.volume() > base_vol,
        "pipe boss should increase volume; vol={}",
        result.volume()
    );
    assert!(op.is_done());
}

#[test]
fn pipe_channel_decreases_volume() {
    let base = make_box(Pnt::origin(), 10.0, 10.0, 10.0);
    let base_vol = base.volume();

    // Spine runs through the middle of the box along Y.
    let spine = Wire::new(vec![
        Pnt::new(5.0, -1.0, 5.0),
        Pnt::new(5.0, 11.0, 5.0),
    ]);
    let result = BRepFeat_MakePipe::new(base, spine, 1.0, FeatMode::Remove)
        .with_facets(32)
        .into_solid();
    assert!(
        result.volume() < base_vol,
        "pipe channel should decrease volume; vol={}",
        result.volume()
    );
}

#[test]
fn pipe_radius_and_spine_accessors() {
    let spine = Wire::new(vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(5.0, 0.0, 0.0)]);
    let op = BRepFeat_MakePipe::new(base_box(), spine, 1.5, FeatMode::Add);
    near(op.radius(), 1.5, 1e-12, "radius");
    assert_eq!(op.spine().pts.len(), 2);
    assert!((op.spine().pts[1].x - 5.0).abs() < 1e-12);
}

// FILE: tests/occt_top_loc.rs
extern crate ironstream;

use ironstream::gp::{Ax1, Pnt};
use ironstream::top_loc::{Datum3D, Location, LocationBuilder};

const EPS: f64 = 1.0e-9;

// ─────────────────────────────────────────────────────────────────────────────
// Datum3D integration tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn datum_identity_apply_is_noop() {
    let d = Datum3D::identity();
    let p = Pnt::new(3.14, -2.71, 42.0);
    let q = d.apply(p);
    assert!((q.x - p.x).abs() < EPS);
    assert!((q.y - p.y).abs() < EPS);
    assert!((q.z - p.z).abs() < EPS);
}

#[test]
fn datum_identity_is_identity_flag() {
    assert!(Datum3D::identity().is_identity());
}

#[test]
fn datum_translation_moves_point() {
    let d = Datum3D::translation(Pnt::new(5.0, 0.0, 0.0));
    let p = d.apply(Pnt::new(1.0, 1.0, 1.0));
    assert!((p.x - 6.0).abs() < EPS);
    assert!((p.y - 1.0).abs() < EPS);
    assert!((p.z - 1.0).abs() < EPS);
}

#[test]
fn datum_translation_not_identity() {
    let d = Datum3D::translation(Pnt::new(0.0, 0.1, 0.0));
    assert!(!d.is_identity());
}

#[test]
fn datum_rotation_90_around_z() {
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let d = Datum3D::rotation(axis, std::f64::consts::FRAC_PI_2);
    let p = d.apply(Pnt::new(1.0, 0.0, 0.0));
    assert!((p.x - 0.0).abs() < EPS, "expected x≈0, got {}", p.x);
    assert!((p.y - 1.0).abs() < EPS, "expected y≈1, got {}", p.y);
    assert!((p.z - 0.0).abs() < EPS, "expected z≈0, got {}", p.z);
}

#[test]
fn datum_rotation_180_around_y() {
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 1.0, 0.0));
    let d = Datum3D::rotation(axis, std::f64::consts::PI);
    // (1,0,0) under 180° around Y → (-1,0,0)
    let p = d.apply(Pnt::new(1.0, 0.0, 0.0));
    assert!((p.x - (-1.0)).abs() < EPS, "expected x≈-1, got {}", p.x);
    assert!((p.y - 0.0).abs() < EPS, "expected y≈0, got {}", p.y);
    assert!((p.z - 0.0).abs() < EPS, "expected z≈0, got {}", p.z);
}

#[test]
fn datum_inverted_round_trips_point() {
    let axis = Ax1::new(Pnt::new(1.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let d = Datum3D::rotation(axis, 1.2345);
    let inv = d.inverted();
    let original = Pnt::new(-3.0, 7.0, 2.0);
    let moved = d.apply(original);
    let back = inv.apply(moved);
    assert!((back.x - original.x).abs() < EPS);
    assert!((back.y - original.y).abs() < EPS);
    assert!((back.z - original.z).abs() < EPS);
}

#[test]
fn datum_from_trsf_round_trips() {
    let axis = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let trsf = ironstream::gp::Trsf::rotation(axis, 0.7);
    let d = Datum3D::from_trsf(trsf);
    let p = Pnt::new(0.0, 1.0, 0.0);
    let expected = trsf.apply_point(p);
    let got = d.apply(p);
    assert!((got.x - expected.x).abs() < EPS);
    assert!((got.y - expected.y).abs() < EPS);
    assert!((got.z - expected.z).abs() < EPS);
}

// ─────────────────────────────────────────────────────────────────────────────
// Location integration tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn location_identity_does_not_move_point() {
    let loc = Location::identity();
    let p = Pnt::new(1.0, 2.0, 3.0);
    let q = loc.apply(p);
    assert!((q.x - p.x).abs() < EPS);
    assert!((q.y - p.y).abs() < EPS);
    assert!((q.z - p.z).abs() < EPS);
}

#[test]
fn location_identity_is_identity_flag() {
    assert!(Location::identity().is_identity());
    assert_eq!(Location::identity().depth(), 0);
}

#[test]
fn location_from_datum_applies_datum() {
    let d = Datum3D::translation(Pnt::new(0.0, 0.0, 10.0));
    let loc = Location::from_datum(d);
    let p = loc.apply(Pnt::origin());
    assert!((p.z - 10.0).abs() < EPS);
}

#[test]
fn location_translation_convenience() {
    let loc = Location::translation(Pnt::new(-5.0, 0.0, 0.0));
    let p = loc.apply(Pnt::new(5.0, 0.0, 0.0));
    assert!((p.x - 0.0).abs() < EPS);
}

#[test]
fn location_rotation_convenience() {
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let loc = Location::rotation(axis, std::f64::consts::FRAC_PI_2);
    let p = loc.apply(Pnt::new(1.0, 0.0, 0.0));
    assert!((p.x - 0.0).abs() < EPS);
    assert!((p.y - 1.0).abs() < EPS);
}

#[test]
fn location_multiplied_combines_translations() {
    let a = Location::translation(Pnt::new(2.0, 0.0, 0.0));
    let b = Location::translation(Pnt::new(0.0, 3.0, 0.0));
    let ab = a.multiplied(&b);
    let p = ab.apply(Pnt::origin());
    assert!((p.x - 2.0).abs() < EPS, "expected x=2, got {}", p.x);
    assert!((p.y - 3.0).abs() < EPS, "expected y=3, got {}", p.y);
    assert!((p.z - 0.0).abs() < EPS);
}

#[test]
fn location_multiplied_rotation_then_translation() {
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    // 90° rotation: (1,0,0) → (0,1,0)
    let rot = Location::rotation(axis, std::f64::consts::FRAC_PI_2);
    // then translate by (0,0,5)
    let trans = Location::translation(Pnt::new(0.0, 0.0, 5.0));
    let combined = rot.multiplied(&trans);
    let p = combined.apply(Pnt::new(1.0, 0.0, 0.0));
    assert!((p.x - 0.0).abs() < EPS, "x: expected 0, got {}", p.x);
    assert!((p.y - 1.0).abs() < EPS, "y: expected 1, got {}", p.y);
    assert!((p.z - 5.0).abs() < EPS, "z: expected 5, got {}", p.z);
}

#[test]
fn location_inverted_cancels_translation() {
    let loc = Location::translation(Pnt::new(100.0, -50.0, 25.0));
    let inv = loc.inverted();
    let p = Pnt::new(1.0, 2.0, 3.0);
    let round_trip = inv.apply(loc.apply(p));
    assert!((round_trip.x - p.x).abs() < EPS);
    assert!((round_trip.y - p.y).abs() < EPS);
    assert!((round_trip.z - p.z).abs() < EPS);
}

#[test]
fn location_inverted_cancels_rotation() {
    let axis = Ax1::new(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
    let loc = Location::rotation(axis, 2.1);
    let inv = loc.inverted();
    let p = Pnt::new(3.0, -1.0, 7.0);
    let round_trip = inv.apply(loc.apply(p));
    assert!((round_trip.x - p.x).abs() < EPS, "x round-trip failed");
    assert!((round_trip.y - p.y).abs() < EPS, "y round-trip failed");
    assert!((round_trip.z - p.z).abs() < EPS, "z round-trip failed");
}

#[test]
fn location_powered_zero_is_identity() {
    let loc = Location::translation(Pnt::new(5.0, 5.0, 5.0));
    let loc0 = loc.powered(0);
    assert!(loc0.is_identity());
    let p = loc0.apply(Pnt::new(1.0, 1.0, 1.0));
    assert!((p.x - 1.0).abs() < EPS);
}

#[test]
fn location_powered_minus_one_is_inverse() {
    let loc = Location::translation(Pnt::new(1.0, 2.0, 3.0));
    let inv = loc.powered(-1);
    let p = Pnt::new(0.0, 0.0, 0.0);
    let q = inv.apply(loc.apply(p));
    assert!((q.x - p.x).abs() < EPS);
    assert!((q.y - p.y).abs() < EPS);
    assert!((q.z - p.z).abs() < EPS);
}

#[test]
fn location_powered_two_is_double_translation() {
    let loc = Location::translation(Pnt::new(3.0, 0.0, 0.0));
    let loc2 = loc.powered(2);
    let p = loc2.apply(Pnt::origin());
    assert!((p.x - 6.0).abs() < EPS, "expected x=6, got {}", p.x);
}

#[test]
fn location_powered_three_is_triple_translation() {
    let loc = Location::translation(Pnt::new(0.0, 2.0, 0.0));
    let loc3 = loc.powered(3);
    let p = loc3.apply(Pnt::origin());
    assert!((p.y - 6.0).abs() < EPS, "expected y=6, got {}", p.y);
}

#[test]
fn location_depth_and_next_location() {
    let a = Location::translation(Pnt::new(1.0, 0.0, 0.0));
    let b = Location::translation(Pnt::new(0.0, 1.0, 0.0));
    let c = Location::translation(Pnt::new(0.0, 0.0, 1.0));
    let abc = a.multiplied(&b).multiplied(&c);
    assert_eq!(abc.depth(), 3);

    let next = abc.next_location();
    assert_eq!(next.depth(), 2);

    let next2 = next.next_location();
    assert_eq!(next2.depth(), 1);

    let next3 = next2.next_location();
    assert!(next3.is_identity());
}

#[test]
fn location_first_datum_is_none_for_identity() {
    let loc = Location::identity();
    assert!(loc.first_datum().is_none());
    assert_eq!(loc.first_power(), 0);
}

#[test]
fn location_is_equal_same_transform() {
    let a = Location::translation(Pnt::new(1.0, 2.0, 3.0));
    let b = Location::translation(Pnt::new(1.0, 2.0, 3.0));
    assert!(a.is_equal(&b, 1.0e-9));
}

#[test]
fn location_is_equal_different_transforms() {
    let a = Location::translation(Pnt::new(1.0, 0.0, 0.0));
    let b = Location::translation(Pnt::new(2.0, 0.0, 0.0));
    assert!(!a.is_equal(&b, 1.0e-9));
}

#[test]
fn location_hash_same_for_same_translation() {
    let a = Location::translation(Pnt::new(1.0, 2.0, 3.0));
    let b = Location::translation(Pnt::new(1.0, 2.0, 3.0));
    assert_eq!(a.hash_code(), b.hash_code());
}

#[test]
fn location_hash_differs_for_different_translations() {
    let a = Location::translation(Pnt::new(1.0, 0.0, 0.0));
    let b = Location::translation(Pnt::new(2.0, 0.0, 0.0));
    // Very likely to differ (not guaranteed by hash contract but true for these values)
    assert_ne!(a.hash_code(), b.hash_code());
}

// ─────────────────────────────────────────────────────────────────────────────
// LocationBuilder integration tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn builder_empty_produces_identity() {
    let loc = LocationBuilder::new().build();
    assert!(loc.is_identity());
}

#[test]
fn builder_single_datum() {
    let d = Datum3D::translation(Pnt::new(0.0, 0.0, 3.0));
    let loc = LocationBuilder::new().push(d, 1).build();
    let p = loc.apply(Pnt::origin());
    assert!((p.z - 3.0).abs() < EPS);
}

#[test]
fn builder_chained_datums() {
    let d1 = Datum3D::translation(Pnt::new(1.0, 0.0, 0.0));
    let d2 = Datum3D::translation(Pnt::new(0.0, 2.0, 0.0));
    let loc = LocationBuilder::new().push(d1, 1).push(d2, 1).build();
    let p = loc.apply(Pnt::origin());
    assert!((p.x - 1.0).abs() < EPS);
    assert!((p.y - 2.0).abs() < EPS);
}

#[test]
fn builder_ignores_identity_datum() {
    let d = Datum3D::identity();
    let loc = LocationBuilder::new().push(d, 1).build();
    assert!(loc.is_identity());
}

#[test]
fn builder_ignores_zero_power() {
    let d = Datum3D::translation(Pnt::new(5.0, 0.0, 0.0));
    let loc = LocationBuilder::new().push(d, 0).build();
    assert!(loc.is_identity());
}

// ─────────────────────────────────────────────────────────────────────────────
// Realistic use-case: nesting a location inside a compound location
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn nested_location_apply_order() {
    // The spine is applied as: result = items[0] o items[1] o ...
    // meaning items[0] is outermost (applied last), items[1] is innermost (applied first).
    // So `a.multiplied(&b)` gives a spine where `b` is applied first, then `a`.
    //
    // Here: rotate.multiplied(&place) → spine [rotate, place]
    // apply order: place first (innermost), then rotate (outermost).
    // Pnt::origin() → place → (10,0,0) → rotate 90° around Z → (0,10,0).
    let place = Location::translation(Pnt::new(10.0, 0.0, 0.0));
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let rotate = Location::rotation(axis, std::f64::consts::FRAC_PI_2);
    // rotate is outer (applied last), place is inner (applied first)
    let loc = rotate.multiplied(&place);
    // Pnt::origin() → translate +10X → (10,0,0) → rotate 90° Z → (0,10,0)
    let p = loc.apply(Pnt::origin());
    assert!((p.x - 0.0).abs() < EPS, "x: expected 0, got {}", p.x);
    assert!((p.y - 10.0).abs() < EPS, "y: expected 10, got {}", p.y);
    assert!((p.z - 0.0).abs() < EPS, "z: expected 0, got {}", p.z);
}

#[test]
fn location_trsf_identity_for_identity_location() {
    let loc = Location::identity();
    let t = loc.trsf();
    // Matrix should be identity
    assert!((t.m[0][0] - 1.0).abs() < EPS);
    assert!((t.m[1][1] - 1.0).abs() < EPS);
    assert!((t.m[2][2] - 1.0).abs() < EPS);
    // Translation should be zero
    assert!(t.t[0].abs() < EPS);
    assert!(t.t[1].abs() < EPS);
    assert!(t.t[2].abs() < EPS);
}

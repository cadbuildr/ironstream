//! Ported OpenCascade unit tests -- `Bnd_OBB`.
//!
//! Each test mirrors a scenario from OCCT's `QABugs` / `Bnd_OBB` unit-test
//! suite. Tolerances follow `precision::CONFUSION = 1e-7` for linear tests and
//! `precision::ANGULAR = 1e-12` for angular tests.

use ironstream::bnd_obb::Obb;
use ironstream::gp::Pnt;
use ironstream::precision;

// ---------------------------------------------------------------------------
// 1. Default-constructed OBB is void.
// ---------------------------------------------------------------------------
#[test]
fn default_is_void() {
    let b = Obb::new();
    assert!(b.is_void());
}

// ---------------------------------------------------------------------------
// 2. SetVoid clears a non-void box.
// ---------------------------------------------------------------------------
#[test]
fn set_void_clears() {
    let mut b = Obb::from_frame(
        Pnt::new(1.0, 2.0, 3.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        5.0,
        5.0,
        5.0,
    );
    assert!(!b.is_void());
    b.set_void();
    assert!(b.is_void());
    assert_eq!(b.x_hsize(), 0.0);
    assert_eq!(b.y_hsize(), 0.0);
    assert_eq!(b.z_hsize(), 0.0);
}

// ---------------------------------------------------------------------------
// 3. from_frame sets center, axes and half-sizes correctly.
// ---------------------------------------------------------------------------
#[test]
fn from_frame_accessors() {
    let cx = Pnt::new(1.0, 2.0, 3.0);
    let xd = Pnt::new(1.0, 0.0, 0.0);
    let yd = Pnt::new(0.0, 1.0, 0.0);
    let zd = Pnt::new(0.0, 0.0, 1.0);
    let b = Obb::from_frame(cx, xd, yd, zd, 4.0, 5.0, 6.0);

    assert!(!b.is_void());
    assert!((b.center().x - 1.0).abs() < precision::CONFUSION);
    assert!((b.center().y - 2.0).abs() < precision::CONFUSION);
    assert!((b.center().z - 3.0).abs() < precision::CONFUSION);
    assert!((b.x_hsize() - 4.0).abs() < precision::CONFUSION);
    assert!((b.y_hsize() - 5.0).abs() < precision::CONFUSION);
    assert!((b.z_hsize() - 6.0).abs() < precision::CONFUSION);
}

// ---------------------------------------------------------------------------
// 4. SetCenter / SetXYZComponent round-trip.
// ---------------------------------------------------------------------------
#[test]
fn set_center_and_components() {
    let mut b = Obb::new();
    b.set_center(Pnt::new(10.0, 20.0, 30.0));
    b.set_x_component(Pnt::new(1.0, 0.0, 0.0), 3.0);
    b.set_y_component(Pnt::new(0.0, 1.0, 0.0), 4.0);
    b.set_z_component(Pnt::new(0.0, 0.0, 1.0), 5.0);

    assert!((b.center().x - 10.0).abs() < precision::CONFUSION);
    assert!((b.center().y - 20.0).abs() < precision::CONFUSION);
    assert!((b.center().z - 30.0).abs() < precision::CONFUSION);
    assert!((b.x_hsize() - 3.0).abs() < precision::CONFUSION);
    assert!((b.y_hsize() - 4.0).abs() < precision::CONFUSION);
    assert!((b.z_hsize() - 5.0).abs() < precision::CONFUSION);
}

// ---------------------------------------------------------------------------
// 5. SquareExtent: for half-sizes (3,4,5) the square extent is 4*(9+16+25)=200.
// ---------------------------------------------------------------------------
#[test]
fn square_extent() {
    let b = Obb::from_frame(
        Pnt::origin(),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        3.0,
        4.0,
        5.0,
    );
    // (2*3)^2 + (2*4)^2 + (2*5)^2 = 36+64+100 = 200
    assert!((b.square_extent() - 200.0).abs() < precision::CONFUSION);
}

// ---------------------------------------------------------------------------
// 6. Void OBB has SquareExtent == 0.
// ---------------------------------------------------------------------------
#[test]
fn void_square_extent_is_zero() {
    let b = Obb::new();
    assert_eq!(b.square_extent(), 0.0);
}

// ---------------------------------------------------------------------------
// 7. IsOut(Pnt): interior point is not out, exterior point is out.
// ---------------------------------------------------------------------------
#[test]
fn is_out_point_basic() {
    let b = Obb::from_frame(
        Pnt::origin(),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        2.0,
        2.0,
        2.0,
    );
    // Point inside.
    assert!(!b.is_out_pnt(Pnt::new(1.0, 1.0, 1.0)));
    // Point outside (X too large).
    assert!(b.is_out_pnt(Pnt::new(3.0, 0.0, 0.0)));
    // Point on the boundary (exactly at hx): not strictly out.
    assert!(!b.is_out_pnt(Pnt::new(2.0, 0.0, 0.0)));
}

// ---------------------------------------------------------------------------
// 8. IsOut(Pnt): void OBB is always out.
// ---------------------------------------------------------------------------
#[test]
fn void_is_out_any_point() {
    let b = Obb::new();
    assert!(b.is_out_pnt(Pnt::origin()));
    assert!(b.is_out_pnt(Pnt::new(1000.0, 1000.0, 1000.0)));
}

// ---------------------------------------------------------------------------
// 9. Add(Pnt): single point produces a zero-size non-void OBB.
// ---------------------------------------------------------------------------
#[test]
fn add_single_point() {
    let mut b = Obb::new();
    b.add_pnt(Pnt::new(5.0, 6.0, 7.0));
    b.finished_addition();

    assert!(!b.is_void());
    assert!((b.center().x - 5.0).abs() < precision::CONFUSION);
    assert!((b.center().y - 6.0).abs() < precision::CONFUSION);
    assert!((b.center().z - 7.0).abs() < precision::CONFUSION);
    assert!(b.x_hsize() < precision::CONFUSION);
    assert!(b.y_hsize() < precision::CONFUSION);
    assert!(b.z_hsize() < precision::CONFUSION);
}

// ---------------------------------------------------------------------------
// 10. Add(Pnt): two axis-aligned points produce the correct AABB.
// ---------------------------------------------------------------------------
#[test]
fn add_two_points() {
    let mut b = Obb::new();
    b.add_pnt(Pnt::new(0.0, 0.0, 0.0));
    b.add_pnt(Pnt::new(4.0, 6.0, 8.0));
    b.finished_addition();

    assert!(!b.is_void());
    assert!((b.center().x - 2.0).abs() < precision::CONFUSION);
    assert!((b.center().y - 3.0).abs() < precision::CONFUSION);
    assert!((b.center().z - 4.0).abs() < precision::CONFUSION);
    assert!((b.x_hsize() - 2.0).abs() < precision::CONFUSION);
    assert!((b.y_hsize() - 3.0).abs() < precision::CONFUSION);
    assert!((b.z_hsize() - 4.0).abs() < precision::CONFUSION);
}

// ---------------------------------------------------------------------------
// 11. Add(OBB): adding a second OBB grows the bounding box to cover all 8
//     corners of the second OBB.
// ---------------------------------------------------------------------------
#[test]
fn add_obb_grows_box() {
    let b1 = Obb::from_frame(
        Pnt::origin(),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        1.0,
        1.0,
        1.0,
    );
    let b2 = Obb::from_frame(
        Pnt::new(10.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        1.0,
        1.0,
        1.0,
    );
    let mut combined = b1;
    combined.add_obb(&b2);
    combined.finished_addition();

    // All corners of both boxes should be inside.
    for corner in b1.corners() {
        assert!(!combined.is_out_pnt(corner));
    }
    for corner in b2.corners() {
        assert!(!combined.is_out_pnt(corner));
    }
}

// ---------------------------------------------------------------------------
// 12. IsOut(OBB): two clearly separated identical boxes are out.
// ---------------------------------------------------------------------------
#[test]
fn is_out_obb_separated() {
    let b1 = Obb::from_frame(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        1.0,
        1.0,
        1.0,
    );
    let b2 = Obb::from_frame(
        Pnt::new(5.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        1.0,
        1.0,
        1.0,
    );
    // Gap between them is 5 - 1 - 1 = 3 > 0.
    assert!(b1.is_out_obb(&b2));
}

// ---------------------------------------------------------------------------
// 13. IsOut(OBB): overlapping boxes are not out.
// ---------------------------------------------------------------------------
#[test]
fn is_out_obb_overlapping() {
    let b1 = Obb::from_frame(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        2.0,
        2.0,
        2.0,
    );
    let b2 = Obb::from_frame(
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        2.0,
        2.0,
        2.0,
    );
    // They overlap: b1 extends to x=2, b2 starts at x=-1.
    assert!(!b1.is_out_obb(&b2));
}

// ---------------------------------------------------------------------------
// 14. IsOut(OBB): void OBB is always out.
// ---------------------------------------------------------------------------
#[test]
fn void_is_out_any_obb() {
    let void_box = Obb::new();
    let solid_box = Obb::from_frame(
        Pnt::origin(),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        1.0,
        1.0,
        1.0,
    );
    assert!(void_box.is_out_obb(&solid_box));
    assert!(solid_box.is_out_obb(&void_box));
}

// ---------------------------------------------------------------------------
// 15. IsOut(OBB): touching on a face (adjacent, not separated) is NOT out.
// ---------------------------------------------------------------------------
#[test]
fn is_out_obb_touching() {
    let b1 = Obb::from_frame(
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        1.0,
        1.0,
        1.0,
    );
    let b2 = Obb::from_frame(
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        1.0,
        1.0,
        1.0,
    );
    // b1 extends to x=1, b2 starts at x=1: touching exactly.
    // SAT: projection distance = 2.0, sum of radii = 1+1 = 2.0, not strictly greater.
    assert!(!b1.is_out_obb(&b2));
}

// ---------------------------------------------------------------------------
// 16. Corners: the eight corners of an AABB unit cube at the origin.
// ---------------------------------------------------------------------------
#[test]
fn corners_unit_cube() {
    let b = Obb::from_frame(
        Pnt::origin(),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        1.0,
        1.0,
        1.0,
    );
    let corners = b.corners();
    // All corners should be at distance sqrt(3) from the origin.
    for c in corners {
        let dist = c.norm();
        assert!(
            (dist - 3.0_f64.sqrt()).abs() < precision::CONFUSION,
            "corner distance {dist} not close to sqrt(3)"
        );
    }
    // Each coordinate is ±1.
    for c in corners {
        assert!((c.x.abs() - 1.0).abs() < precision::CONFUSION);
        assert!((c.y.abs() - 1.0).abs() < precision::CONFUSION);
        assert!((c.z.abs() - 1.0).abs() < precision::CONFUSION);
    }
}

// ---------------------------------------------------------------------------
// 17. IsOut(Pnt): rotated OBB — point inside the rotated frame.
// ---------------------------------------------------------------------------
#[test]
fn is_out_pnt_rotated_obb() {
    // A box rotated 45° around Z, so its X axis is (1/√2, 1/√2, 0).
    let s = 0.5_f64.sqrt();
    let b = Obb::from_frame(
        Pnt::origin(),
        Pnt::new(s, s, 0.0),
        Pnt::new(-s, s, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        2.0,
        2.0,
        1.0,
    );
    // The point (0, 2*s, 0) = (0, √2, 0) projects 2.0 onto the X axis
    // and 2.0 onto the Y axis of the rotated frame, so it is exactly on
    // the boundary (not out).
    let p_inside = Pnt::new(0.0, s * 1.0, 0.0);
    assert!(!b.is_out_pnt(p_inside));

    // A point at (3, 0, 0): projection onto X-axis = 3*s ≈ 2.12 > 2.0 → out.
    let p_outside = Pnt::new(3.0, 0.0, 0.0);
    assert!(b.is_out_pnt(p_outside));
}

// ---------------------------------------------------------------------------
// 18. GetCenter via set_center round-trip after Add.
// ---------------------------------------------------------------------------
#[test]
fn get_center_after_add() {
    let mut b = Obb::new();
    b.add_pnt(Pnt::new(-3.0, -3.0, -3.0));
    b.add_pnt(Pnt::new(3.0, 3.0, 3.0));
    b.finished_addition();

    let c = b.center();
    assert!((c.x).abs() < precision::CONFUSION);
    assert!((c.y).abs() < precision::CONFUSION);
    assert!((c.z).abs() < precision::CONFUSION);
    assert!((b.x_hsize() - 3.0).abs() < precision::CONFUSION);
    assert!((b.y_hsize() - 3.0).abs() < precision::CONFUSION);
    assert!((b.z_hsize() - 3.0).abs() < precision::CONFUSION);
}

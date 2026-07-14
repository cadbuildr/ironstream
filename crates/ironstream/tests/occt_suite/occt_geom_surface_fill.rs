// FILE: tests/occt_geom_surface_fill.rs
//! Integration tests for `ironstream::geom_surface_fill` — the GeomFill
//! surface filling types port.
//!
//! Tests exercise the public API only, covering:
//! - [`GeomFillBoundaryType`] (variant distinctness, Copy/Clone)
//! - [`GeomFillBoundary`]     (field storage, accessors, Clone)
//! - [`GeomFillSurface`]      (boundary accumulation, build lifecycle)

extern crate ironstream;
use ironstream::geom_surface_fill::*;

// ---------------------------------------------------------------------------
// GeomFillBoundaryType
// ---------------------------------------------------------------------------

#[test]
fn boundary_type_all_variants_distinct() {
    assert_ne!(GeomFillBoundaryType::CornerOrder, GeomFillBoundaryType::CurvedOrder);
    assert_ne!(GeomFillBoundaryType::CurvedOrder, GeomFillBoundaryType::OtherCurve);
    assert_ne!(GeomFillBoundaryType::CornerOrder, GeomFillBoundaryType::OtherCurve);
}

#[test]
fn boundary_type_is_copy() {
    let t = GeomFillBoundaryType::CornerOrder;
    let t2 = t; // copy — original still usable
    assert_eq!(t, t2);
}

#[test]
fn boundary_type_is_clone() {
    let t = GeomFillBoundaryType::CurvedOrder;
    let t2 = t.clone();
    assert_eq!(t, t2);
}

#[test]
fn boundary_type_debug_contains_name() {
    let s = format!("{:?}", GeomFillBoundaryType::OtherCurve);
    assert!(s.contains("OtherCurve"), "debug repr: {s}");
}

// ---------------------------------------------------------------------------
// GeomFillBoundary — construction and accessors
// ---------------------------------------------------------------------------

#[test]
fn boundary_new_corner_order() {
    let b = GeomFillBoundary::new(0, GeomFillBoundaryType::CornerOrder, 4, 1.0e-3);
    assert_eq!(b.id(), 0);
    assert_eq!(b.boundary_type(), GeomFillBoundaryType::CornerOrder);
    assert_eq!(b.nb_control_points(), 4);
    assert!((b.tolerance() - 1.0e-3).abs() < 1.0e-12);
}

#[test]
fn boundary_new_curved_order() {
    let b = GeomFillBoundary::new(42, GeomFillBoundaryType::CurvedOrder, 16, 1.0e-7);
    assert_eq!(b.id(), 42);
    assert_eq!(b.boundary_type(), GeomFillBoundaryType::CurvedOrder);
    assert_eq!(b.nb_control_points(), 16);
    assert!((b.tolerance() - 1.0e-7).abs() < 1.0e-15);
}

#[test]
fn boundary_new_other_curve() {
    let b = GeomFillBoundary::new(99, GeomFillBoundaryType::OtherCurve, 8, 0.05);
    assert_eq!(b.id(), 99);
    assert_eq!(b.boundary_type(), GeomFillBoundaryType::OtherCurve);
    assert_eq!(b.nb_control_points(), 8);
    assert!((b.tolerance() - 0.05).abs() < 1.0e-15);
}

#[test]
fn boundary_clone_produces_equal_values() {
    let b = GeomFillBoundary::new(5, GeomFillBoundaryType::CurvedOrder, 10, 2.5e-4);
    let b2 = b.clone();
    assert_eq!(b.id(), b2.id());
    assert_eq!(b.boundary_type(), b2.boundary_type());
    assert_eq!(b.nb_control_points(), b2.nb_control_points());
    assert!((b.tolerance() - b2.tolerance()).abs() < 1.0e-15);
}

#[test]
fn boundary_id_zero_is_valid() {
    let b = GeomFillBoundary::new(0, GeomFillBoundaryType::CornerOrder, 2, 1.0e-4);
    assert_eq!(b.id(), 0);
}

#[test]
fn boundary_single_control_point() {
    let b = GeomFillBoundary::new(1, GeomFillBoundaryType::OtherCurve, 1, 1.0e-4);
    assert_eq!(b.nb_control_points(), 1);
}

// ---------------------------------------------------------------------------
// GeomFillSurface — initial state
// ---------------------------------------------------------------------------

#[test]
fn surface_new_has_no_boundaries() {
    let s = GeomFillSurface::new();
    assert_eq!(s.nb_boundaries(), 0);
}

#[test]
fn surface_new_is_not_done() {
    let s = GeomFillSurface::new();
    assert!(!s.is_done());
}

#[test]
fn surface_new_poles_are_zero() {
    let s = GeomFillSurface::new();
    assert_eq!(s.nb_u_poles(), 0);
    assert_eq!(s.nb_v_poles(), 0);
}

#[test]
fn surface_default_equals_new() {
    let s1 = GeomFillSurface::new();
    let s2 = GeomFillSurface::default();
    assert_eq!(s1.nb_boundaries(), s2.nb_boundaries());
    assert_eq!(s1.is_done(), s2.is_done());
    assert_eq!(s1.nb_u_poles(), s2.nb_u_poles());
    assert_eq!(s1.nb_v_poles(), s2.nb_v_poles());
}

// ---------------------------------------------------------------------------
// GeomFillSurface — add_boundary
// ---------------------------------------------------------------------------

#[test]
fn surface_add_one_boundary() {
    let mut s = GeomFillSurface::new();
    s.add_boundary(GeomFillBoundary::new(0, GeomFillBoundaryType::CornerOrder, 4, 1.0e-4));
    assert_eq!(s.nb_boundaries(), 1);
}

#[test]
fn surface_add_four_boundaries() {
    let mut s = GeomFillSurface::new();
    for i in 0..4 {
        s.add_boundary(GeomFillBoundary::new(i, GeomFillBoundaryType::CornerOrder, 4, 1.0e-4));
    }
    assert_eq!(s.nb_boundaries(), 4);
}

#[test]
fn surface_add_boundary_resets_done() {
    let mut s = GeomFillSurface::new();
    s.add_boundary(GeomFillBoundary::new(0, GeomFillBoundaryType::CornerOrder, 4, 1.0e-4));
    s.build();
    assert!(s.is_done());
    // Adding another boundary must invalidate the result.
    s.add_boundary(GeomFillBoundary::new(1, GeomFillBoundaryType::CurvedOrder, 4, 1.0e-4));
    assert!(!s.is_done());
}

#[test]
fn surface_add_boundary_resets_u_poles() {
    let mut s = GeomFillSurface::new();
    s.add_boundary(GeomFillBoundary::new(0, GeomFillBoundaryType::CornerOrder, 4, 1.0e-4));
    s.build();
    assert_eq!(s.nb_u_poles(), 4);
    s.add_boundary(GeomFillBoundary::new(1, GeomFillBoundaryType::CurvedOrder, 4, 1.0e-4));
    assert_eq!(s.nb_u_poles(), 0);
}

#[test]
fn surface_add_boundary_resets_v_poles() {
    let mut s = GeomFillSurface::new();
    s.add_boundary(GeomFillBoundary::new(0, GeomFillBoundaryType::CornerOrder, 4, 1.0e-4));
    s.build();
    assert_eq!(s.nb_v_poles(), 4);
    s.add_boundary(GeomFillBoundary::new(1, GeomFillBoundaryType::CurvedOrder, 4, 1.0e-4));
    assert_eq!(s.nb_v_poles(), 0);
}

// ---------------------------------------------------------------------------
// GeomFillSurface — build lifecycle
// ---------------------------------------------------------------------------

#[test]
fn surface_build_sets_is_done() {
    let mut s = GeomFillSurface::new();
    s.add_boundary(GeomFillBoundary::new(0, GeomFillBoundaryType::CornerOrder, 4, 1.0e-4));
    s.build();
    assert!(s.is_done());
}

#[test]
fn surface_build_sets_nb_u_poles_to_four() {
    let mut s = GeomFillSurface::new();
    s.add_boundary(GeomFillBoundary::new(0, GeomFillBoundaryType::CurvedOrder, 6, 1.0e-5));
    s.build();
    assert_eq!(s.nb_u_poles(), 4);
}

#[test]
fn surface_build_sets_nb_v_poles_to_four() {
    let mut s = GeomFillSurface::new();
    s.add_boundary(GeomFillBoundary::new(0, GeomFillBoundaryType::CurvedOrder, 6, 1.0e-5));
    s.build();
    assert_eq!(s.nb_v_poles(), 4);
}

#[test]
fn surface_build_four_boundaries() {
    let mut s = GeomFillSurface::new();
    for i in 0..4 {
        let bt = match i % 3 {
            0 => GeomFillBoundaryType::CornerOrder,
            1 => GeomFillBoundaryType::CurvedOrder,
            _ => GeomFillBoundaryType::OtherCurve,
        };
        s.add_boundary(GeomFillBoundary::new(i, bt, 4, 1.0e-4));
    }
    s.build();
    assert!(s.is_done());
    assert_eq!(s.nb_u_poles(), 4);
    assert_eq!(s.nb_v_poles(), 4);
    assert_eq!(s.nb_boundaries(), 4);
}

#[test]
fn surface_build_idempotent() {
    let mut s = GeomFillSurface::new();
    s.add_boundary(GeomFillBoundary::new(0, GeomFillBoundaryType::CornerOrder, 4, 1.0e-4));
    s.build();
    s.build(); // second call must keep the result consistent
    assert!(s.is_done());
    assert_eq!(s.nb_u_poles(), 4);
    assert_eq!(s.nb_v_poles(), 4);
}

#[test]
fn surface_rebuild_after_invalidation() {
    let mut s = GeomFillSurface::new();
    s.add_boundary(GeomFillBoundary::new(0, GeomFillBoundaryType::CornerOrder, 4, 1.0e-4));
    s.build();
    assert!(s.is_done());
    s.add_boundary(GeomFillBoundary::new(1, GeomFillBoundaryType::CurvedOrder, 4, 1.0e-4));
    assert!(!s.is_done());
    s.build();
    assert!(s.is_done());
    assert_eq!(s.nb_boundaries(), 2);
}

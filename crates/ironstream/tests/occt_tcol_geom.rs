// FILE: tests/occt_tcol_geom.rs
extern crate ironstream;

use ironstream::tcol_geom::{
    ColGeom1dCurve, ColGeom1dSurface, ColGeomHSeqBezierCurve, ColGeomSeqCurve, ColGeomSeqSurface,
};

// ── ColGeom1dCurve ────────────────────────────────────────────────────────────

#[test]
fn array1_curve_bounds_and_size() {
    let arr = ColGeom1dCurve::new(1, 5);
    assert_eq!(arr.lower(), 1);
    assert_eq!(arr.upper(), 5);
    assert_eq!(arr.size(), 5);
    assert!(!arr.is_empty());
}

#[test]
fn array1_curve_set_get_roundtrip() {
    let mut arr = ColGeom1dCurve::new(1, 3);
    arr.set(1, "Geom_Line".to_string());
    arr.set(2, "Geom_Circle".to_string());
    arr.set(3, "Geom_BSplineCurve".to_string());
    assert_eq!(arr.get(1), Some("Geom_Line"));
    assert_eq!(arr.get(2), Some("Geom_Circle"));
    assert_eq!(arr.get(3), Some("Geom_BSplineCurve"));
}

#[test]
fn array1_curve_unset_slot_is_none() {
    let arr = ColGeom1dCurve::new(1, 4);
    for i in 1..=4 {
        assert_eq!(arr.get(i), None);
    }
}

#[test]
fn array1_curve_nonunit_lower_bound() {
    let mut arr = ColGeom1dCurve::new(10, 12);
    assert_eq!(arr.lower(), 10);
    assert_eq!(arr.upper(), 12);
    assert_eq!(arr.size(), 3);
    arr.set(11, "Geom_Ellipse".to_string());
    assert_eq!(arr.get(10), None);
    assert_eq!(arr.get(11), Some("Geom_Ellipse"));
    assert_eq!(arr.get(12), None);
}

#[test]
fn array1_curve_overwrite_slot() {
    let mut arr = ColGeom1dCurve::new(1, 2);
    arr.set(1, "Line".to_string());
    arr.set(1, "Arc".to_string());
    assert_eq!(arr.get(1), Some("Arc"));
}

#[test]
#[should_panic]
fn array1_curve_set_below_lower_panics() {
    let mut arr = ColGeom1dCurve::new(2, 5);
    arr.set(1, "Line".to_string());
}

#[test]
#[should_panic]
fn array1_curve_get_above_upper_panics() {
    let arr = ColGeom1dCurve::new(1, 3);
    let _ = arr.get(4);
}

// ── ColGeom1dSurface ──────────────────────────────────────────────────────────

#[test]
fn array1_surface_set_get_and_none() {
    let mut arr = ColGeom1dSurface::new(1, 4);
    arr.set(2, "Geom_Plane".to_string());
    arr.set(4, "Geom_ToroidalSurface".to_string());
    assert_eq!(arr.get(1), None);
    assert_eq!(arr.get(2), Some("Geom_Plane"));
    assert_eq!(arr.get(3), None);
    assert_eq!(arr.get(4), Some("Geom_ToroidalSurface"));
    assert_eq!(arr.size(), 4);
}

#[test]
fn array1_surface_zero_lower_bound() {
    let mut arr = ColGeom1dSurface::new(0, 2);
    arr.set(0, "Geom_CylindricalSurface".to_string());
    assert_eq!(arr.lower(), 0);
    assert_eq!(arr.upper(), 2);
    assert_eq!(arr.get(0), Some("Geom_CylindricalSurface"));
}

// ── ColGeomSeqCurve ───────────────────────────────────────────────────────────

#[test]
fn seq_curve_append_length_get() {
    let mut seq = ColGeomSeqCurve::new();
    assert!(seq.is_empty());
    seq.append("Geom_Line".to_string());
    seq.append("Geom_Circle".to_string());
    seq.append("Geom_BezierCurve".to_string());
    assert_eq!(seq.length(), 3);
    assert_eq!(seq.get(0), Some("Geom_Line"));
    assert_eq!(seq.get(1), Some("Geom_Circle"));
    assert_eq!(seq.get(2), Some("Geom_BezierCurve"));
    assert_eq!(seq.get(3), None);
}

#[test]
fn seq_curve_prepend_reorders() {
    let mut seq = ColGeomSeqCurve::new();
    seq.append("Second".to_string());
    seq.prepend("First".to_string());
    assert_eq!(seq.first(), Some("First"));
    assert_eq!(seq.last(), Some("Second"));
    assert_eq!(seq.length(), 2);
}

#[test]
fn seq_curve_remove_and_reindex() {
    let mut seq = ColGeomSeqCurve::new();
    seq.append("A".to_string());
    seq.append("B".to_string());
    seq.append("C".to_string());
    seq.remove(1);
    assert_eq!(seq.length(), 2);
    assert_eq!(seq.get(0), Some("A"));
    assert_eq!(seq.get(1), Some("C"));
}

#[test]
fn seq_curve_clear_empties_sequence() {
    let mut seq = ColGeomSeqCurve::new();
    seq.append("Geom_Line".to_string());
    seq.append("Geom_Arc".to_string());
    seq.clear();
    assert!(seq.is_empty());
    assert_eq!(seq.length(), 0);
    assert_eq!(seq.first(), None);
    assert_eq!(seq.last(), None);
}

// ── ColGeomSeqSurface ─────────────────────────────────────────────────────────

#[test]
fn seq_surface_append_prepend_first_last() {
    let mut seq = ColGeomSeqSurface::new();
    seq.append("Geom_Plane".to_string());
    seq.append("Geom_SphericalSurface".to_string());
    seq.prepend("Geom_ConicalSurface".to_string());
    assert_eq!(seq.length(), 3);
    assert_eq!(seq.first(), Some("Geom_ConicalSurface"));
    assert_eq!(seq.last(), Some("Geom_SphericalSurface"));
    assert_eq!(seq.get(1), Some("Geom_Plane"));
}

#[test]
fn seq_surface_remove_first() {
    let mut seq = ColGeomSeqSurface::new();
    seq.append("X".to_string());
    seq.append("Y".to_string());
    seq.remove(0);
    assert_eq!(seq.length(), 1);
    assert_eq!(seq.first(), Some("Y"));
}

// ── ColGeomHSeqBezierCurve ────────────────────────────────────────────────────

#[test]
fn hseq_bezier_append_length_get() {
    let mut seq = ColGeomHSeqBezierCurve::new();
    assert!(seq.is_empty());
    seq.append("Geom_BezierCurve".to_string());
    seq.append("Geom_BSplineCurve".to_string());
    assert_eq!(seq.length(), 2);
    assert_eq!(seq.get(0), Some("Geom_BezierCurve"));
    assert_eq!(seq.get(1), Some("Geom_BSplineCurve"));
    assert_eq!(seq.get(2), None);
}

#[test]
fn hseq_bezier_clear_resets() {
    let mut seq = ColGeomHSeqBezierCurve::new();
    seq.append("Geom_BezierCurve".to_string());
    seq.append("Geom_TrimmedCurve".to_string());
    seq.clear();
    assert!(seq.is_empty());
    assert_eq!(seq.length(), 0);
}

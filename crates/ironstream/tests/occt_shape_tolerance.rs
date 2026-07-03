// FILE: rust/ironstream/crates/ironstream/tests/occt_shape_tolerance.rs
//! Integration tests for `shape_tolerance` — mirrors OCCT's
//! `ShapeAnalysis_ShapeTolerance` and `ShapeFix_ShapeTolerance` behaviour.
extern crate ironstream;
use ironstream::shape_tolerance::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn vertex_entry(label: &str, v: f64) -> ToleranceEntry {
    ToleranceEntry::new(label, ToleranceType::Vertex, v)
}

fn edge_entry(label: &str, v: f64) -> ToleranceEntry {
    ToleranceEntry::new(label, ToleranceType::Edge, v)
}

fn face_entry(label: &str, v: f64) -> ToleranceEntry {
    ToleranceEntry::new(label, ToleranceType::Face, v)
}

fn overall_entry(label: &str, v: f64) -> ToleranceEntry {
    ToleranceEntry::new(label, ToleranceType::Overall, v)
}

// ---------------------------------------------------------------------------
// ToleranceEntry
// ---------------------------------------------------------------------------

#[test]
fn test_entry_label() {
    let e = vertex_entry("Vertex_42", 0.001);
    assert_eq!(e.label(), "Vertex_42");
}

#[test]
fn test_entry_tolerance_type_vertex() {
    let e = vertex_entry("v0", 1e-5);
    assert_eq!(e.tolerance_type(), ToleranceType::Vertex);
}

#[test]
fn test_entry_tolerance_type_edge() {
    let e = edge_entry("e0", 1e-4);
    assert_eq!(e.tolerance_type(), ToleranceType::Edge);
}

#[test]
fn test_entry_tolerance_type_face() {
    let e = face_entry("f0", 1e-3);
    assert_eq!(e.tolerance_type(), ToleranceType::Face);
}

#[test]
fn test_entry_tolerance_type_overall() {
    let e = overall_entry("shape", 5e-4);
    assert_eq!(e.tolerance_type(), ToleranceType::Overall);
}

#[test]
fn test_entry_value() {
    let e = edge_entry("e1", 0.007);
    assert!((e.value() - 0.007).abs() < 1e-15);
}

#[test]
fn test_entry_clone_is_independent() {
    let e1 = vertex_entry("v0", 1.0);
    let e2 = e1.clone();
    assert_eq!(e1.label(), e2.label());
    assert_eq!(e1.value(), e2.value());
}

// ---------------------------------------------------------------------------
// ToleranceType Copy / PartialEq
// ---------------------------------------------------------------------------

#[test]
fn test_tolerance_type_eq() {
    assert_eq!(ToleranceType::Vertex, ToleranceType::Vertex);
    assert_ne!(ToleranceType::Vertex, ToleranceType::Edge);
    assert_ne!(ToleranceType::Face, ToleranceType::Overall);
}

#[test]
fn test_tolerance_type_copy() {
    let t = ToleranceType::Face;
    let t2 = t; // copy
    assert_eq!(t, t2);
}

// ---------------------------------------------------------------------------
// ShapeToleranceAnalysis — construction and nb_entries
// ---------------------------------------------------------------------------

#[test]
fn test_analysis_starts_empty() {
    let a = ShapeToleranceAnalysis::new();
    assert_eq!(a.nb_entries(), 0);
}

#[test]
fn test_analysis_add_single() {
    let mut a = ShapeToleranceAnalysis::new();
    a.add(vertex_entry("v0", 0.001));
    assert_eq!(a.nb_entries(), 1);
}

#[test]
fn test_analysis_add_multiple() {
    let mut a = ShapeToleranceAnalysis::new();
    a.add(vertex_entry("v0", 0.001));
    a.add(edge_entry("e0", 0.002));
    a.add(face_entry("f0", 0.003));
    assert_eq!(a.nb_entries(), 3);
}

// ---------------------------------------------------------------------------
// ShapeToleranceAnalysis — entry()
// ---------------------------------------------------------------------------

#[test]
fn test_entry_by_index_zero() {
    let mut a = ShapeToleranceAnalysis::new();
    a.add(vertex_entry("first", 0.1));
    a.add(edge_entry("second", 0.2));
    assert_eq!(a.entry(0).label(), "first");
}

#[test]
fn test_entry_by_index_last() {
    let mut a = ShapeToleranceAnalysis::new();
    a.add(vertex_entry("v0", 1.0));
    a.add(edge_entry("e0", 2.0));
    a.add(face_entry("f0", 3.0));
    assert_eq!(a.entry(2).label(), "f0");
    assert!((a.entry(2).value() - 3.0).abs() < 1e-15);
}

#[test]
fn test_entry_type_preserved() {
    let mut a = ShapeToleranceAnalysis::new();
    a.add(overall_entry("shape", 0.05));
    assert_eq!(a.entry(0).tolerance_type(), ToleranceType::Overall);
}

// ---------------------------------------------------------------------------
// ShapeToleranceAnalysis — min_tolerance
// ---------------------------------------------------------------------------

#[test]
fn test_min_tolerance_empty_is_infinity() {
    let a = ShapeToleranceAnalysis::new();
    assert_eq!(a.min_tolerance(), f64::INFINITY);
}

#[test]
fn test_min_tolerance_single() {
    let mut a = ShapeToleranceAnalysis::new();
    a.add(vertex_entry("v0", 0.042));
    assert!((a.min_tolerance() - 0.042).abs() < 1e-15);
}

#[test]
fn test_min_tolerance_picks_smallest() {
    let mut a = ShapeToleranceAnalysis::new();
    a.add(edge_entry("e0", 0.5));
    a.add(edge_entry("e1", 0.1));
    a.add(face_entry("f0", 0.9));
    assert!((a.min_tolerance() - 0.1).abs() < 1e-15);
}

// ---------------------------------------------------------------------------
// ShapeToleranceAnalysis — max_tolerance
// ---------------------------------------------------------------------------

#[test]
fn test_max_tolerance_empty_is_neg_infinity() {
    let a = ShapeToleranceAnalysis::new();
    assert_eq!(a.max_tolerance(), f64::NEG_INFINITY);
}

#[test]
fn test_max_tolerance_single() {
    let mut a = ShapeToleranceAnalysis::new();
    a.add(face_entry("f0", 0.099));
    assert!((a.max_tolerance() - 0.099).abs() < 1e-15);
}

#[test]
fn test_max_tolerance_picks_largest() {
    let mut a = ShapeToleranceAnalysis::new();
    a.add(vertex_entry("v0", 0.001));
    a.add(vertex_entry("v1", 0.003));
    a.add(edge_entry("e0", 0.002));
    assert!((a.max_tolerance() - 0.003).abs() < 1e-15);
}

// ---------------------------------------------------------------------------
// ShapeToleranceAnalysis — mean_tolerance
// ---------------------------------------------------------------------------

#[test]
fn test_mean_tolerance_empty_is_zero() {
    let a = ShapeToleranceAnalysis::new();
    assert_eq!(a.mean_tolerance(), 0.0);
}

#[test]
fn test_mean_tolerance_single() {
    let mut a = ShapeToleranceAnalysis::new();
    a.add(edge_entry("e0", 0.8));
    assert!((a.mean_tolerance() - 0.8).abs() < 1e-15);
}

#[test]
fn test_mean_tolerance_three_entries() {
    let mut a = ShapeToleranceAnalysis::new();
    a.add(vertex_entry("v0", 0.1));
    a.add(edge_entry("e0", 0.2));
    a.add(face_entry("f0", 0.3));
    // mean = (0.1 + 0.2 + 0.3) / 3 = 0.2
    assert!((a.mean_tolerance() - 0.2).abs() < 1e-14);
}

#[test]
fn test_mean_tolerance_equal_values() {
    let mut a = ShapeToleranceAnalysis::new();
    for _ in 0..5 {
        a.add(face_entry("f", 0.007));
    }
    assert!((a.mean_tolerance() - 0.007).abs() < 1e-15);
}

// ---------------------------------------------------------------------------
// ShapeToleranceAnalysis — tolerance_for_type
// ---------------------------------------------------------------------------

#[test]
fn test_tolerance_for_type_none_match() {
    let mut a = ShapeToleranceAnalysis::new();
    a.add(edge_entry("e0", 0.01));
    let vs = a.tolerance_for_type(ToleranceType::Vertex);
    assert!(vs.is_empty());
}

#[test]
fn test_tolerance_for_type_all_match() {
    let mut a = ShapeToleranceAnalysis::new();
    a.add(face_entry("f0", 0.1));
    a.add(face_entry("f1", 0.2));
    a.add(face_entry("f2", 0.3));
    let fs = a.tolerance_for_type(ToleranceType::Face);
    assert_eq!(fs.len(), 3);
    assert!((fs[0] - 0.1).abs() < 1e-15);
    assert!((fs[1] - 0.2).abs() < 1e-15);
    assert!((fs[2] - 0.3).abs() < 1e-15);
}

#[test]
fn test_tolerance_for_type_mixed() {
    let mut a = ShapeToleranceAnalysis::new();
    a.add(vertex_entry("v0", 0.001));
    a.add(edge_entry("e0", 0.01));
    a.add(vertex_entry("v1", 0.002));
    a.add(face_entry("f0", 0.1));
    a.add(overall_entry("shape", 0.5));
    let vs = a.tolerance_for_type(ToleranceType::Vertex);
    assert_eq!(vs.len(), 2);
    let es = a.tolerance_for_type(ToleranceType::Edge);
    assert_eq!(es.len(), 1);
    assert!((es[0] - 0.01).abs() < 1e-15);
    let os = a.tolerance_for_type(ToleranceType::Overall);
    assert_eq!(os.len(), 1);
    assert!((os[0] - 0.5).abs() < 1e-15);
}

#[test]
fn test_tolerance_for_type_overall_only() {
    let mut a = ShapeToleranceAnalysis::new();
    a.add(overall_entry("s", 0.05));
    let os = a.tolerance_for_type(ToleranceType::Overall);
    assert_eq!(os.len(), 1);
    assert!((os[0] - 0.05).abs() < 1e-15);
}

// ---------------------------------------------------------------------------
// update_tolerances
// ---------------------------------------------------------------------------

#[test]
fn test_update_tolerances_empty_vec() {
    let mut entries: Vec<ToleranceEntry> = vec![];
    update_tolerances(&mut entries, 10.0); // must not panic
    assert!(entries.is_empty());
}

#[test]
fn test_update_tolerances_scale_by_two() {
    let mut entries = vec![
        vertex_entry("v0", 1.0),
        edge_entry("e0", 2.0),
        face_entry("f0", 3.0),
    ];
    update_tolerances(&mut entries, 2.0);
    assert!((entries[0].value() - 2.0).abs() < 1e-15);
    assert!((entries[1].value() - 4.0).abs() < 1e-15);
    assert!((entries[2].value() - 6.0).abs() < 1e-15);
}

#[test]
fn test_update_tolerances_unit_scale_no_change() {
    let mut entries = vec![edge_entry("e0", 0.123)];
    update_tolerances(&mut entries, 1.0);
    assert!((entries[0].value() - 0.123).abs() < 1e-15);
}

#[test]
fn test_update_tolerances_zero_scale() {
    let mut entries = vec![face_entry("f0", 42.0), vertex_entry("v0", 7.0)];
    update_tolerances(&mut entries, 0.0);
    assert_eq!(entries[0].value(), 0.0);
    assert_eq!(entries[1].value(), 0.0);
}

#[test]
fn test_update_tolerances_mm_to_inches() {
    // 1 inch = 25.4 mm; scale by 1/25.4 to convert mm → inches.
    let mut entries = vec![edge_entry("e0", 25.4)];
    update_tolerances(&mut entries, 1.0 / 25.4);
    assert!((entries[0].value() - 1.0).abs() < 1e-12);
}

#[test]
fn test_update_tolerances_preserves_labels_and_types() {
    let mut entries = vec![
        overall_entry("shape", 10.0),
        vertex_entry("v0", 5.0),
    ];
    update_tolerances(&mut entries, 0.1);
    assert_eq!(entries[0].label(), "shape");
    assert_eq!(entries[0].tolerance_type(), ToleranceType::Overall);
    assert_eq!(entries[1].label(), "v0");
    assert_eq!(entries[1].tolerance_type(), ToleranceType::Vertex);
}

#[test]
fn test_update_tolerances_twice_is_multiplicative() {
    let mut entries = vec![edge_entry("e0", 1.0)];
    update_tolerances(&mut entries, 3.0);
    update_tolerances(&mut entries, 3.0);
    // 1.0 * 3 * 3 = 9.0
    assert!((entries[0].value() - 9.0).abs() < 1e-14);
}

// ---------------------------------------------------------------------------
// Round-trip: add entries, scale, then query statistics
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_add_scale_query() {
    let mut a = ShapeToleranceAnalysis::new();
    a.add(vertex_entry("v0", 0.1));
    a.add(edge_entry("e0", 0.2));
    a.add(face_entry("f0", 0.3));
    // Scale all by 10
    update_tolerances(&mut a.entries, 10.0);
    assert!((a.min_tolerance() - 1.0).abs() < 1e-14);
    assert!((a.max_tolerance() - 3.0).abs() < 1e-14);
    assert!((a.mean_tolerance() - 2.0).abs() < 1e-14);
    let vs = a.tolerance_for_type(ToleranceType::Vertex);
    assert!((vs[0] - 1.0).abs() < 1e-14);
}

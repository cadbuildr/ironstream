// FILE: tests/occt_tcolgp.rs
//! Integration tests for `ironstream::tcolgp`, mirroring the OpenCascade
//! `TColgp` collection types (`NCollection_Array1`, `NCollection_Array2`,
//! `NCollection_Sequence` specialised for `gp_Pnt`, `gp_Pnt2d`, `gp_Vec`).
//!
//! These tests exercise the public API only: construction, element access,
//! mutation, bounds queries, and higher-level convenience methods.

extern crate ironstream;

use ironstream::gp::Pnt;
use ironstream::gp2d::Pnt2d;
use ironstream::tcolgp::{
    Array1OfPnt, Array1OfPnt2d, Array1OfVec, Array2OfPnt, SequenceOfPnt,
};

const TOL: f64 = 1e-12;

fn near(a: f64, b: f64, label: &str) {
    assert!(
        (a - b).abs() <= TOL,
        "{label}: expected {a} ≈ {b} (diff {})",
        (a - b).abs()
    );
}

fn pnt_near(a: Pnt, b: Pnt, label: &str) {
    near(a.x, b.x, &format!("{label}.x"));
    near(a.y, b.y, &format!("{label}.y"));
    near(a.z, b.z, &format!("{label}.z"));
}

// ── Array1OfPnt ───────────────────────────────────────────────────────────────

#[test]
fn array1_pnt_construction_and_bounds() {
    let arr = Array1OfPnt::new(5);
    assert_eq!(arr.lower(), 1);
    assert_eq!(arr.upper(), 5);
    assert_eq!(arr.length(), 5);
    assert!(!arr.is_empty());
}

#[test]
fn array1_pnt_set_and_get() {
    let mut arr = Array1OfPnt::new(4);
    let pts = [
        Pnt::new(1.0, 2.0, 3.0),
        Pnt::new(4.0, 5.0, 6.0),
        Pnt::new(7.0, 8.0, 9.0),
        Pnt::new(10.0, 11.0, 12.0),
    ];
    for (i, &p) in pts.iter().enumerate() {
        arr.set_value(i + 1, p);
    }
    for (i, &expected) in pts.iter().enumerate() {
        pnt_near(arr.value(i + 1), expected, &format!("element {}", i + 1));
    }
}

#[test]
fn array1_pnt_custom_lower_bound() {
    let mut arr = Array1OfPnt::with_bounds(3, 7);
    assert_eq!(arr.lower(), 3);
    assert_eq!(arr.upper(), 7);
    assert_eq!(arr.length(), 5);

    arr.set_value(5, Pnt::new(42.0, 0.0, 0.0));
    near(arr.value(5).x, 42.0, "custom lower bound element");
    // Element at lower and upper remain at origin
    pnt_near(arr.value(3), Pnt::origin(), "lower bound default");
    pnt_near(arr.value(7), Pnt::origin(), "upper bound default");
}

#[test]
fn array1_pnt_fill() {
    let fill_pt = Pnt::new(5.0, 5.0, 5.0);
    let mut arr = Array1OfPnt::new(6);
    arr.fill(fill_pt);
    for i in 1..=6 {
        pnt_near(arr.value(i), fill_pt, &format!("fill element {i}"));
    }
}

#[test]
fn array1_pnt_centroid_unit_square() {
    // Four corners of a unit square in XY — centroid at (0.5, 0.5, 0.0)
    let mut arr = Array1OfPnt::new(4);
    arr.set_value(1, Pnt::new(0.0, 0.0, 0.0));
    arr.set_value(2, Pnt::new(1.0, 0.0, 0.0));
    arr.set_value(3, Pnt::new(1.0, 1.0, 0.0));
    arr.set_value(4, Pnt::new(0.0, 1.0, 0.0));
    let c = arr.centroid();
    pnt_near(c, Pnt::new(0.5, 0.5, 0.0), "square centroid");
}

#[test]
fn array1_pnt_bounding_box() {
    let mut arr = Array1OfPnt::new(5);
    arr.set_value(1, Pnt::new(-3.0, 0.0, 1.0));
    arr.set_value(2, Pnt::new(5.0, -2.0, 4.0));
    arr.set_value(3, Pnt::new(0.0, 7.0, 0.0));
    arr.set_value(4, Pnt::new(1.0, 1.0, 1.0));
    arr.set_value(5, Pnt::new(-1.0, -1.0, -1.0));
    let (lo, hi) = arr.bounding_box();
    near(lo.x, -3.0, "bbox lo.x");
    near(lo.y, -2.0, "bbox lo.y");
    near(lo.z, -1.0, "bbox lo.z");
    near(hi.x, 5.0, "bbox hi.x");
    near(hi.y, 7.0, "bbox hi.y");
    near(hi.z, 4.0, "bbox hi.z");
}

#[test]
fn array1_pnt_iter_indexed() {
    let mut arr = Array1OfPnt::new(3);
    arr.set_value(1, Pnt::new(10.0, 0.0, 0.0));
    arr.set_value(2, Pnt::new(20.0, 0.0, 0.0));
    arr.set_value(3, Pnt::new(30.0, 0.0, 0.0));
    let pairs: Vec<(usize, Pnt)> = arr.iter_indexed().collect();
    assert_eq!(pairs[0].0, 1);
    near(pairs[0].1.x, 10.0, "iter_indexed[0].x");
    assert_eq!(pairs[2].0, 3);
    near(pairs[2].1.x, 30.0, "iter_indexed[2].x");
}

#[test]
#[should_panic]
fn array1_pnt_out_of_bounds_panics() {
    let arr = Array1OfPnt::new(3);
    let _ = arr.value(0); // 0 is below lower bound of 1
}

// ── Array2OfPnt ───────────────────────────────────────────────────────────────

#[test]
fn array2_pnt_construction_and_size() {
    let arr = Array2OfPnt::new(4, 3);
    assert_eq!(arr.n_rows(), 4);
    assert_eq!(arr.n_cols(), 3);
    assert_eq!(arr.size(), 12);
    assert_eq!(arr.row_lower(), 1);
    assert_eq!(arr.row_upper(), 4);
    assert_eq!(arr.col_lower(), 1);
    assert_eq!(arr.col_upper(), 3);
}

#[test]
fn array2_pnt_set_get_all_cells() {
    let mut arr = Array2OfPnt::new(2, 2);
    arr.set_value(1, 1, Pnt::new(1.0, 1.0, 0.0));
    arr.set_value(1, 2, Pnt::new(2.0, 1.0, 0.0));
    arr.set_value(2, 1, Pnt::new(1.0, 2.0, 0.0));
    arr.set_value(2, 2, Pnt::new(2.0, 2.0, 0.0));
    pnt_near(arr.value(1, 1), Pnt::new(1.0, 1.0, 0.0), "2x2 [1,1]");
    pnt_near(arr.value(2, 2), Pnt::new(2.0, 2.0, 0.0), "2x2 [2,2]");
}

#[test]
fn array2_pnt_explicit_bounds() {
    let mut arr = Array2OfPnt::with_bounds(0, 2, 0, 3);
    assert_eq!(arr.n_rows(), 3);
    assert_eq!(arr.n_cols(), 4);
    arr.set_value(1, 2, Pnt::new(99.0, 0.0, 0.0));
    near(arr.value(1, 2).x, 99.0, "explicit bounds element");
}

#[test]
fn array2_pnt_row_and_col_extraction() {
    let mut arr = Array2OfPnt::new(3, 3);
    // Fill diagonal
    arr.set_value(1, 1, Pnt::new(1.0, 0.0, 0.0));
    arr.set_value(2, 2, Pnt::new(2.0, 0.0, 0.0));
    arr.set_value(3, 3, Pnt::new(3.0, 0.0, 0.0));
    // Row 2: only (2,2) is non-zero
    let r = arr.row(2);
    assert_eq!(r.length(), 3);
    near(r.value(1).x, 0.0, "row2 col1");
    near(r.value(2).x, 2.0, "row2 col2");
    near(r.value(3).x, 0.0, "row2 col3");
    // Col 3: only (3,3) is non-zero
    let c = arr.col(3);
    assert_eq!(c.length(), 3);
    near(c.value(1).x, 0.0, "col3 row1");
    near(c.value(2).x, 0.0, "col3 row2");
    near(c.value(3).x, 3.0, "col3 row3");
}

// ── SequenceOfPnt ─────────────────────────────────────────────────────────────

#[test]
fn sequence_pnt_basic_append() {
    let mut seq = SequenceOfPnt::new();
    assert!(seq.is_empty());
    seq.append(Pnt::new(1.0, 0.0, 0.0));
    seq.append(Pnt::new(2.0, 0.0, 0.0));
    seq.append(Pnt::new(3.0, 0.0, 0.0));
    assert_eq!(seq.length(), 3);
    near(seq.value(1).x, 1.0, "seq[1]");
    near(seq.value(3).x, 3.0, "seq[3]");
    near(seq.first().x, 1.0, "first");
    near(seq.last().x, 3.0, "last");
}

#[test]
fn sequence_pnt_prepend() {
    let mut seq = SequenceOfPnt::new();
    seq.append(Pnt::new(2.0, 0.0, 0.0));
    seq.prepend(Pnt::new(1.0, 0.0, 0.0));
    near(seq.first().x, 1.0, "prepend first");
    near(seq.last().x, 2.0, "prepend last");
}

#[test]
fn sequence_pnt_insert_and_remove() {
    let mut seq = SequenceOfPnt::from_slice(&[
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
        Pnt::new(5.0, 0.0, 0.0),
    ]);
    // Insert 2 before position 2
    seq.insert_before(2, Pnt::new(2.0, 0.0, 0.0));
    assert_eq!(seq.length(), 4);
    near(seq.value(2).x, 2.0, "after insert [2]");
    near(seq.value(3).x, 3.0, "after insert [3]");

    // Remove the newly inserted element
    seq.remove(2);
    assert_eq!(seq.length(), 3);
    near(seq.value(2).x, 3.0, "after remove [2]");
}

#[test]
fn sequence_pnt_arc_length_collinear() {
    // Points along X at unit spacing — arc length must equal 4
    let seq = SequenceOfPnt::from_slice(&[
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
        Pnt::new(4.0, 0.0, 0.0),
    ]);
    near(seq.arc_length(), 4.0, "arc_length collinear");
}

#[test]
fn sequence_pnt_centroid_triangle() {
    // Centroid of an equilateral-ish triangle at (0,0,0),(6,0,0),(3,6,0)
    // = (3, 2, 0)
    let seq = SequenceOfPnt::from_slice(&[
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(6.0, 0.0, 0.0),
        Pnt::new(3.0, 6.0, 0.0),
    ]);
    let c = seq.centroid();
    pnt_near(c, Pnt::new(3.0, 2.0, 0.0), "triangle centroid");
}

#[test]
fn sequence_pnt_reverse() {
    let mut seq = SequenceOfPnt::from_slice(&[
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
    ]);
    seq.reverse();
    near(seq.value(1).x, 3.0, "reversed [1]");
    near(seq.value(3).x, 1.0, "reversed [3]");
}

#[test]
fn sequence_pnt_set_value() {
    let mut seq = SequenceOfPnt::from_slice(&[
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ]);
    seq.set_value(2, Pnt::new(99.0, 0.0, 0.0));
    near(seq.value(2).x, 99.0, "set_value");
}

#[test]
fn sequence_pnt_clear() {
    let mut seq = SequenceOfPnt::from_slice(&[
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ]);
    seq.clear();
    assert!(seq.is_empty());
    assert_eq!(seq.length(), 0);
}

// ── Array1OfPnt2d ─────────────────────────────────────────────────────────────

#[test]
fn array1_pnt2d_construction() {
    let arr = Array1OfPnt2d::new(4);
    assert_eq!(arr.lower(), 1);
    assert_eq!(arr.upper(), 4);
    assert_eq!(arr.length(), 4);
}

#[test]
fn array1_pnt2d_set_and_get() {
    let mut arr = Array1OfPnt2d::new(3);
    arr.set_value(1, Pnt2d::new(1.0, 2.0));
    arr.set_value(2, Pnt2d::new(3.0, 4.0));
    arr.set_value(3, Pnt2d::new(5.0, 6.0));
    near(arr.value(1).x, 1.0, "2d[1].x");
    near(arr.value(1).y, 2.0, "2d[1].y");
    near(arr.value(3).x, 5.0, "2d[3].x");
}

#[test]
fn array1_pnt2d_centroid_square() {
    // Unit square corners: centroid at (0.5, 0.5)
    let mut arr = Array1OfPnt2d::new(4);
    arr.set_value(1, Pnt2d::new(0.0, 0.0));
    arr.set_value(2, Pnt2d::new(1.0, 0.0));
    arr.set_value(3, Pnt2d::new(1.0, 1.0));
    arr.set_value(4, Pnt2d::new(0.0, 1.0));
    let c = arr.centroid();
    near(c.x, 0.5, "2d centroid x");
    near(c.y, 0.5, "2d centroid y");
}

#[test]
fn array1_pnt2d_bounding_box() {
    let mut arr = Array1OfPnt2d::new(3);
    arr.set_value(1, Pnt2d::new(-2.0, 3.0));
    arr.set_value(2, Pnt2d::new(5.0, -1.0));
    arr.set_value(3, Pnt2d::new(0.0, 7.0));
    let (lo, hi) = arr.bounding_box();
    near(lo.x, -2.0, "2d bbox lo.x");
    near(lo.y, -1.0, "2d bbox lo.y");
    near(hi.x, 5.0, "2d bbox hi.x");
    near(hi.y, 7.0, "2d bbox hi.y");
}

// ── Array1OfVec ───────────────────────────────────────────────────────────────

#[test]
fn array1_vec_construction_and_access() {
    let mut arr = Array1OfVec::new(3);
    arr.set_value(1, Pnt::new(1.0, 0.0, 0.0));
    arr.set_value(2, Pnt::new(0.0, 1.0, 0.0));
    arr.set_value(3, Pnt::new(0.0, 0.0, 1.0));
    pnt_near(arr.value(1), Pnt::new(1.0, 0.0, 0.0), "vec[1]");
    pnt_near(arr.value(3), Pnt::new(0.0, 0.0, 1.0), "vec[3]");
}

#[test]
fn array1_vec_sum() {
    let mut arr = Array1OfVec::new(3);
    arr.set_value(1, Pnt::new(1.0, 2.0, 3.0));
    arr.set_value(2, Pnt::new(4.0, 5.0, 6.0));
    arr.set_value(3, Pnt::new(7.0, 8.0, 9.0));
    let s = arr.sum();
    pnt_near(s, Pnt::new(12.0, 15.0, 18.0), "vec sum");
}

#[test]
fn array1_vec_magnitudes() {
    let mut arr = Array1OfVec::new(2);
    // 3-4-5 triple and unit Z
    arr.set_value(1, Pnt::new(3.0, 4.0, 0.0));
    arr.set_value(2, Pnt::new(0.0, 0.0, 1.0));
    let mags = arr.magnitudes();
    near(mags[0], 5.0, "mag[0]");
    near(mags[1], 1.0, "mag[1]");
}

#[test]
fn array1_vec_normalize_all() {
    let mut arr = Array1OfVec::new(3);
    arr.set_value(1, Pnt::new(2.0, 0.0, 0.0));
    arr.set_value(2, Pnt::new(0.0, 5.0, 0.0));
    arr.set_value(3, Pnt::new(0.0, 0.0, 3.0));
    arr.normalize_all();
    let mags = arr.magnitudes();
    for (i, &m) in mags.iter().enumerate() {
        near(m, 1.0, &format!("normalized mag[{i}]"));
    }
    // Directions should be preserved
    pnt_near(arr.value(1), Pnt::new(1.0, 0.0, 0.0), "normalised dir x");
    pnt_near(arr.value(2), Pnt::new(0.0, 1.0, 0.0), "normalised dir y");
    pnt_near(arr.value(3), Pnt::new(0.0, 0.0, 1.0), "normalised dir z");
}

#[test]
fn array1_vec_with_bounds() {
    let mut arr = Array1OfVec::with_bounds(5, 8);
    assert_eq!(arr.lower(), 5);
    assert_eq!(arr.upper(), 8);
    assert_eq!(arr.length(), 4);
    arr.set_value(6, Pnt::new(1.0, 2.0, 3.0));
    near(arr.value(6).y, 2.0, "custom bounds vec[6].y");
}

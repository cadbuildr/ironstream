// FILE: tests/occt_tcolstd.rs
//! Integration tests for `ironstream::tcolstd`, mirroring the OpenCascade
//! `TColStd` collection types (`NCollection_Array1`, `NCollection_Array2`,
//! `NCollection_Sequence` specialised for `Standard_Integer` and
//! `Standard_Real`).
//!
//! These tests exercise the public API only: construction, element access,
//! mutation, bounds queries, and higher-level convenience methods.

extern crate ironstream;

use ironstream::tcolstd::{
    Array1OfInteger, Array1OfReal, Array2OfReal, SequenceOfInteger, SequenceOfReal,
};

const TOL: f64 = 1e-12;

fn near(a: f64, b: f64, label: &str) {
    assert!(
        (a - b).abs() <= TOL,
        "{label}: expected {a} ≈ {b} (diff {})",
        (a - b).abs()
    );
}

// ── Array1OfInteger ───────────────────────────────────────────────────────────

#[test]
fn array1_int_construction_and_bounds() {
    let arr = Array1OfInteger::new(6);
    assert_eq!(arr.lower(), 1);
    assert_eq!(arr.upper(), 6);
    assert_eq!(arr.length(), 6);
    assert!(!arr.is_empty());
}

#[test]
fn array1_int_with_bounds_and_set_get() {
    let mut arr = Array1OfInteger::with_bounds(2, 5);
    assert_eq!(arr.lower(), 2);
    assert_eq!(arr.upper(), 5);
    assert_eq!(arr.length(), 4);
    arr.set_value(3, 100);
    assert_eq!(arr.value(3), 100);
    // Bounds stay at 0 unless set
    assert_eq!(arr.value(2), 0);
    assert_eq!(arr.value(5), 0);
}

#[test]
fn array1_int_init_fills_all() {
    let mut arr = Array1OfInteger::new(5);
    arr.init(7);
    for i in 1..=5 {
        assert_eq!(arr.value(i), 7, "element {i} should be 7");
    }
}

#[test]
fn array1_int_min_max_sum() {
    let mut arr = Array1OfInteger::new(5);
    arr.set_value(1, -5);
    arr.set_value(2, 10);
    arr.set_value(3, 3);
    arr.set_value(4, -1);
    arr.set_value(5, 7);
    assert_eq!(arr.min_value(), -5);
    assert_eq!(arr.max_value(), 10);
    assert_eq!(arr.sum(), 14);
}

#[test]
fn array1_int_sort_ascending() {
    let mut arr = Array1OfInteger::new(5);
    arr.set_value(1, 9);
    arr.set_value(2, 2);
    arr.set_value(3, 6);
    arr.set_value(4, 1);
    arr.set_value(5, 4);
    arr.sort();
    assert_eq!(arr.value(1), 1);
    assert_eq!(arr.value(2), 2);
    assert_eq!(arr.value(3), 4);
    assert_eq!(arr.value(4), 6);
    assert_eq!(arr.value(5), 9);
}

#[test]
fn array1_int_contains() {
    let mut arr = Array1OfInteger::new(4);
    arr.set_value(1, 10);
    arr.set_value(2, 20);
    arr.set_value(3, 30);
    arr.set_value(4, 40);
    assert!(arr.contains(30));
    assert!(!arr.contains(25));
}

#[test]
fn array1_int_iter_indexed() {
    let mut arr = Array1OfInteger::with_bounds(5, 8);
    arr.set_value(5, 50);
    arr.set_value(6, 60);
    arr.set_value(7, 70);
    arr.set_value(8, 80);
    let pairs: Vec<(usize, i32)> = arr.iter_indexed().collect();
    assert_eq!(pairs[0].0, 5);
    assert_eq!(pairs[0].1, 50);
    assert_eq!(pairs[3].0, 8);
    assert_eq!(pairs[3].1, 80);
}

#[test]
#[should_panic]
fn array1_int_out_of_bounds_panics() {
    let arr = Array1OfInteger::new(3);
    let _ = arr.value(4);
}

#[test]
#[should_panic]
fn array1_int_zero_size_panics() {
    let _ = Array1OfInteger::new(0);
}

// ── Array1OfReal ──────────────────────────────────────────────────────────────

#[test]
fn array1_real_construction_and_from_slice() {
    let arr = Array1OfReal::from_slice(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(arr.lower(), 1);
    assert_eq!(arr.upper(), 5);
    assert_eq!(arr.length(), 5);
    near(arr.value(3), 3.0, "from_slice[3]");
}

#[test]
fn array1_real_with_bounds_set_get() {
    let mut arr = Array1OfReal::with_bounds(0, 3);
    assert_eq!(arr.lower(), 0);
    assert_eq!(arr.upper(), 3);
    assert_eq!(arr.length(), 4);
    arr.set_value(2, 3.14);
    near(arr.value(2), 3.14, "real with_bounds[2]");
}

#[test]
fn array1_real_init() {
    let mut arr = Array1OfReal::new(4);
    arr.init(2.71828);
    for i in 1..=4 {
        near(arr.value(i), 2.71828, &format!("init[{i}]"));
    }
}

#[test]
fn array1_real_min_max_mean_sum() {
    let arr = Array1OfReal::from_slice(&[1.0, 3.0, 5.0, 7.0, 9.0]);
    near(arr.min_value(), 1.0, "min");
    near(arr.max_value(), 9.0, "max");
    near(arr.sum(), 25.0, "sum");
    near(arr.mean(), 5.0, "mean");
}

#[test]
fn array1_real_normalize_unit_range() {
    // [0, 5, 10] should become [0.0, 0.5, 1.0]
    let mut arr = Array1OfReal::from_slice(&[0.0, 5.0, 10.0]);
    arr.normalize();
    near(arr.value(1), 0.0, "norm[1]");
    near(arr.value(2), 0.5, "norm[2]");
    near(arr.value(3), 1.0, "norm[3]");
}

#[test]
fn array1_real_normalize_flat_array() {
    // All equal values: normalize to 0.0 (range == 0)
    let mut arr = Array1OfReal::from_slice(&[5.0, 5.0, 5.0]);
    arr.normalize();
    for i in 1..=3 {
        near(arr.value(i), 0.0, &format!("flat norm[{i}]"));
    }
}

#[test]
fn array1_real_knot_vector_count_near() {
    // Clamped cubic B-spline knot vector: {0,0,0,0, 1,1,1,1}
    let arr = Array1OfReal::from_slice(&[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0]);
    assert_eq!(arr.count_near(0.0, 1e-9), 4, "knot 0 has multiplicity 4");
    assert_eq!(arr.count_near(1.0, 1e-9), 4, "knot 1 has multiplicity 4");
    assert_eq!(arr.count_near(0.5, 1e-9), 0, "knot 0.5 not present");
}

#[test]
fn array1_real_sort() {
    let mut arr = Array1OfReal::from_slice(&[5.0, 2.0, 8.0, 1.0, 4.0]);
    arr.sort();
    for i in 1..arr.length() {
        assert!(
            arr.value(i) <= arr.value(i + 1),
            "sort failed at index {i}"
        );
    }
}

#[test]
#[should_panic]
fn array1_real_out_of_bounds_panics() {
    let arr = Array1OfReal::new(3);
    let _ = arr.value(0); // below lower bound of 1
}

// ── Array2OfReal ──────────────────────────────────────────────────────────────

#[test]
fn array2_real_construction_and_size() {
    let arr = Array2OfReal::new(4, 5);
    assert_eq!(arr.n_rows(), 4);
    assert_eq!(arr.n_cols(), 5);
    assert_eq!(arr.size(), 20);
    assert_eq!(arr.row_lower(), 1);
    assert_eq!(arr.row_upper(), 4);
    assert_eq!(arr.col_lower(), 1);
    assert_eq!(arr.col_upper(), 5);
}

#[test]
fn array2_real_set_get_all_cells() {
    let mut arr = Array2OfReal::new(2, 3);
    arr.set_value(1, 1, 1.1);
    arr.set_value(1, 2, 1.2);
    arr.set_value(1, 3, 1.3);
    arr.set_value(2, 1, 2.1);
    arr.set_value(2, 2, 2.2);
    arr.set_value(2, 3, 2.3);

    near(arr.value(1, 3), 1.3, "arr[1,3]");
    near(arr.value(2, 2), 2.2, "arr[2,2]");
    near(arr.value(2, 3), 2.3, "arr[2,3]");
}

#[test]
fn array2_real_explicit_bounds() {
    let mut arr = Array2OfReal::with_bounds(0, 2, 0, 2);
    assert_eq!(arr.n_rows(), 3);
    assert_eq!(arr.n_cols(), 3);
    arr.set_value(1, 2, 42.0);
    near(arr.value(1, 2), 42.0, "explicit bounds arr[1,2]");
    near(arr.value(0, 0), 0.0, "default zero at [0,0]");
}

#[test]
fn array2_real_init() {
    let mut arr = Array2OfReal::new(3, 3);
    arr.init(9.9);
    for r in 1..=3 {
        for c in 1..=3 {
            near(arr.value(r, c), 9.9, &format!("init[{r},{c}]"));
        }
    }
}

#[test]
fn array2_real_row_extraction() {
    let mut arr = Array2OfReal::new(3, 4);
    // Set row 2 with known values
    for c in 1..=4 {
        arr.set_value(2, c, c as f64 * 10.0);
    }
    let row = arr.row(2);
    assert_eq!(row.length(), 4);
    near(row.value(1), 10.0, "row2[1]");
    near(row.value(2), 20.0, "row2[2]");
    near(row.value(3), 30.0, "row2[3]");
    near(row.value(4), 40.0, "row2[4]");
}

#[test]
fn array2_real_col_extraction() {
    let mut arr = Array2OfReal::new(4, 3);
    // Set col 3 with known values
    for r in 1..=4 {
        arr.set_value(r, 3, r as f64 * 100.0);
    }
    let col = arr.col(3);
    assert_eq!(col.length(), 4);
    near(col.value(1), 100.0, "col3[1]");
    near(col.value(4), 400.0, "col3[4]");
}

#[test]
fn array2_real_transpose() {
    let mut arr = Array2OfReal::new(2, 3);
    arr.set_value(1, 1, 1.0); arr.set_value(1, 2, 2.0); arr.set_value(1, 3, 3.0);
    arr.set_value(2, 1, 4.0); arr.set_value(2, 2, 5.0); arr.set_value(2, 3, 6.0);

    let t = arr.transposed();
    assert_eq!(t.n_rows(), 3, "transposed rows");
    assert_eq!(t.n_cols(), 2, "transposed cols");
    // t[i,j] == arr[j,i]
    near(t.value(1, 1), 1.0, "t[1,1]");
    near(t.value(2, 1), 2.0, "t[2,1]");  // was arr[1,2]
    near(t.value(3, 2), 6.0, "t[3,2]");  // was arr[2,3]
}

#[test]
fn array2_real_min_max() {
    let mut arr = Array2OfReal::new(2, 3);
    arr.set_value(1, 1, -7.0);
    arr.set_value(1, 2, 0.0);
    arr.set_value(1, 3, 3.0);
    arr.set_value(2, 1, 5.0);
    arr.set_value(2, 2, 2.0);
    arr.set_value(2, 3, 11.0);
    near(arr.min_value(), -7.0, "min");
    near(arr.max_value(), 11.0, "max");
}

#[test]
#[should_panic]
fn array2_real_out_of_bounds_row() {
    let arr = Array2OfReal::new(2, 3);
    let _ = arr.value(3, 1); // row 3 out of range
}

// ── SequenceOfInteger ─────────────────────────────────────────────────────────

#[test]
fn seq_int_basic_append_access() {
    let mut seq = SequenceOfInteger::new();
    assert!(seq.is_empty());
    seq.append(10);
    seq.append(20);
    seq.append(30);
    assert_eq!(seq.length(), 3);
    assert_eq!(seq.value(1), 10);
    assert_eq!(seq.value(2), 20);
    assert_eq!(seq.value(3), 30);
    assert_eq!(seq.first(), 10);
    assert_eq!(seq.last(), 30);
}

#[test]
fn seq_int_prepend_and_reverse() {
    let mut seq = SequenceOfInteger::from_slice(&[2, 3, 4]);
    seq.prepend(1);
    assert_eq!(seq.first(), 1);
    assert_eq!(seq.length(), 4);

    seq.reverse();
    assert_eq!(seq.value(1), 4);
    assert_eq!(seq.last(), 1);
}

#[test]
fn seq_int_insert_and_remove() {
    let mut seq = SequenceOfInteger::from_slice(&[1, 2, 4, 5]);
    seq.insert_before(3, 3); // insert 3 at position 3
    assert_eq!(seq.length(), 5);
    assert_eq!(seq.value(3), 3);
    assert_eq!(seq.value(4), 4);

    seq.remove(3);
    assert_eq!(seq.length(), 4);
    assert_eq!(seq.value(3), 4);
}

#[test]
fn seq_int_clear() {
    let mut seq = SequenceOfInteger::from_slice(&[1, 2, 3]);
    seq.clear();
    assert!(seq.is_empty());
    assert_eq!(seq.length(), 0);
}

#[test]
fn seq_int_min_max_sum_contains() {
    let seq = SequenceOfInteger::from_slice(&[5, 1, 8, 3, 2, 7]);
    assert_eq!(seq.min_value(), 1);
    assert_eq!(seq.max_value(), 8);
    assert_eq!(seq.sum(), 26);
    assert!(seq.contains(3));
    assert!(!seq.contains(99));
}

#[test]
fn seq_int_sort_and_unique() {
    let mut seq = SequenceOfInteger::from_slice(&[3, 1, 4, 1, 5, 9, 2, 6, 5]);
    // Remove duplicates first; unique keeps first occurrence preserving order
    seq.unique();
    // After unique: [3, 1, 4, 5, 9, 2, 6] (1 and 5 appear once each)
    assert_eq!(seq.length(), 7);
    assert!(seq.contains(3));
    assert!(seq.contains(9));

    seq.sort();
    assert_eq!(seq.value(1), 1);
    assert_eq!(seq.value(7), 9);
}

#[test]
fn seq_int_set_value() {
    let mut seq = SequenceOfInteger::from_slice(&[10, 20, 30]);
    seq.set_value(2, 99);
    assert_eq!(seq.value(2), 99);
    assert_eq!(seq.value(1), 10);
    assert_eq!(seq.value(3), 30);
}

#[test]
#[should_panic]
fn seq_int_out_of_bounds_panics() {
    let seq = SequenceOfInteger::from_slice(&[1, 2, 3]);
    let _ = seq.value(4);
}

// ── SequenceOfReal ────────────────────────────────────────────────────────────

#[test]
fn seq_real_basic_append_and_access() {
    let mut seq = SequenceOfReal::new();
    assert!(seq.is_empty());
    seq.append(1.0);
    seq.append(2.0);
    seq.append(3.0);
    assert_eq!(seq.length(), 3);
    near(seq.first(), 1.0, "first");
    near(seq.last(), 3.0, "last");
    near(seq.value(2), 2.0, "value(2)");
}

#[test]
fn seq_real_prepend_and_reverse() {
    let mut seq = SequenceOfReal::from_slice(&[2.0, 3.0]);
    seq.prepend(1.0);
    near(seq.first(), 1.0, "prepend first");

    seq.reverse();
    near(seq.value(1), 3.0, "reversed[1]");
    near(seq.value(3), 1.0, "reversed[3]");
}

#[test]
fn seq_real_insert_and_remove() {
    let mut seq = SequenceOfReal::from_slice(&[0.0, 0.5, 1.0]);
    // Insert a midpoint between first two
    seq.insert_before(2, 0.25);
    assert_eq!(seq.length(), 4);
    near(seq.value(2), 0.25, "inserted[2]");
    near(seq.value(3), 0.5, "shifted[3]");

    seq.remove(2);
    assert_eq!(seq.length(), 3);
    near(seq.value(2), 0.5, "after remove[2]");
}

#[test]
fn seq_real_clear() {
    let mut seq = SequenceOfReal::from_slice(&[1.0, 2.0, 3.0]);
    seq.clear();
    assert!(seq.is_empty());
}

#[test]
fn seq_real_sum_mean_min_max() {
    let seq = SequenceOfReal::from_slice(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    near(seq.sum(), 15.0, "sum");
    near(seq.mean(), 3.0, "mean");
    near(seq.min_value(), 1.0, "min");
    near(seq.max_value(), 5.0, "max");
}

#[test]
fn seq_real_differences_knot_spans() {
    // B-spline interior knot spans: {0.0, 0.25, 0.5, 0.75, 1.0}
    let seq = SequenceOfReal::from_slice(&[0.0, 0.25, 0.5, 0.75, 1.0]);
    let diff = seq.differences();
    assert_eq!(diff.length(), 4);
    for i in 1..=4 {
        near(diff.value(i), 0.25, &format!("span[{i}]"));
    }
}

#[test]
fn seq_real_norm_and_dot() {
    // 3-4-5 right triangle magnitudes
    let a = SequenceOfReal::from_slice(&[3.0, 4.0]);
    near(a.norm(), 5.0, "norm 3-4-5");

    let b = SequenceOfReal::from_slice(&[1.0, 0.0]);
    near(a.dot(&b), 3.0, "dot with unit x");
}

#[test]
fn seq_real_sort_ascending() {
    let mut seq = SequenceOfReal::from_slice(&[3.14, 1.41, 2.72, 0.58]);
    seq.sort();
    for i in 1..seq.length() {
        assert!(
            seq.value(i) <= seq.value(i + 1),
            "sort failed at index {i}: {} > {}",
            seq.value(i),
            seq.value(i + 1)
        );
    }
    near(seq.first(), 0.58, "sorted first");
}

#[test]
fn seq_real_set_value() {
    let mut seq = SequenceOfReal::from_slice(&[1.0, 2.0, 3.0]);
    seq.set_value(2, 99.9);
    near(seq.value(2), 99.9, "set_value");
}

#[test]
#[should_panic]
fn seq_real_out_of_bounds_panics() {
    let seq = SequenceOfReal::from_slice(&[1.0, 2.0]);
    let _ = seq.value(3);
}

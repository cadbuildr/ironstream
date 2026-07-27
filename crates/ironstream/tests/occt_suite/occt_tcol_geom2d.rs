// FILE: tests/occt_tcol_geom2d.rs
extern crate ironstream;

use ironstream::tcol_geom2d::{
    ColGeom2d1dBoundedCurve, ColGeom2d1dCurve, ColGeom2dSeqCurve, ColGeom2dSeqGeom,
};

#[test]
fn array1_curve_construction_and_bounds() {
    let arr = ColGeom2d1dCurve::new(1, 6);
    assert_eq!(arr.lower(), 1);
    assert_eq!(arr.upper(), 6);
    assert_eq!(arr.size(), 6);
    assert!(!arr.is_empty());
}

#[test]
fn array1_curve_all_slots_none_initially() {
    let arr = ColGeom2d1dCurve::new(1, 4);
    for idx in 1..=4 {
        assert!(arr.get(idx).is_none(), "slot {idx} should be None initially");
    }
}

#[test]
fn array1_curve_set_and_get_values() {
    let mut arr = ColGeom2d1dCurve::new(1, 4);
    arr.set(1, "line2d".to_string());
    arr.set(3, "circle2d".to_string());
    assert_eq!(arr.get(1).map(|s| s.as_str()), Some("line2d"));
    assert!(arr.get(2).is_none());
    assert_eq!(arr.get(3).map(|s| s.as_str()), Some("circle2d"));
    assert!(arr.get(4).is_none());
}

#[test]
fn array1_curve_non_one_lower_bound() {
    let mut arr = ColGeom2d1dCurve::new(3, 6);
    assert_eq!(arr.lower(), 3);
    assert_eq!(arr.upper(), 6);
    assert_eq!(arr.size(), 4);
    arr.set(4, "ellipse2d".to_string());
    assert_eq!(arr.get(4).unwrap(), "ellipse2d");
    assert!(arr.get(3).is_none());
}

#[test]
fn array1_curve_overwrite_value() {
    let mut arr = ColGeom2d1dCurve::new(1, 3);
    arr.set(2, "first".to_string());
    arr.set(2, "second".to_string());
    assert_eq!(arr.get(2).unwrap(), "second");
}

#[test]
#[should_panic]
fn array1_curve_get_below_lower_panics() {
    let arr = ColGeom2d1dCurve::new(2, 5);
    let _ = arr.get(1);
}

#[test]
#[should_panic]
fn array1_curve_get_above_upper_panics() {
    let arr = ColGeom2d1dCurve::new(1, 3);
    let _ = arr.get(4);
}

#[test]
fn seq_curve_new_is_empty() {
    let seq = ColGeom2dSeqCurve::new();
    assert!(seq.is_empty());
    assert_eq!(seq.length(), 0);
}

#[test]
fn seq_curve_append_and_access() {
    let mut seq = ColGeom2dSeqCurve::new();
    seq.append("line2d_a".to_string());
    seq.append("circle2d_b".to_string());
    seq.append("spline2d_c".to_string());
    assert_eq!(seq.length(), 3);
    assert!(!seq.is_empty());
    assert_eq!(seq.get(1), "line2d_a");
    assert_eq!(seq.get(2), "circle2d_b");
    assert_eq!(seq.get(3), "spline2d_c");
}

#[test]
fn seq_curve_first_and_last() {
    let mut seq = ColGeom2dSeqCurve::new();
    seq.append("alpha".to_string());
    seq.append("beta".to_string());
    seq.append("gamma".to_string());
    assert_eq!(seq.first(), "alpha");
    assert_eq!(seq.last(), "gamma");
}

#[test]
fn seq_curve_prepend_puts_at_front() {
    let mut seq = ColGeom2dSeqCurve::new();
    seq.append("b".to_string());
    seq.append("c".to_string());
    seq.prepend("a".to_string());
    assert_eq!(seq.length(), 3);
    assert_eq!(seq.first(), "a");
    assert_eq!(seq.get(2), "b");
    assert_eq!(seq.last(), "c");
}

#[test]
fn seq_curve_remove_middle() {
    let mut seq = ColGeom2dSeqCurve::new();
    seq.append("one".to_string());
    seq.append("two".to_string());
    seq.append("three".to_string());
    seq.remove(2);
    assert_eq!(seq.length(), 2);
    assert_eq!(seq.get(1), "one");
    assert_eq!(seq.get(2), "three");
}

#[test]
fn seq_curve_clear() {
    let mut seq = ColGeom2dSeqCurve::new();
    seq.append("x".to_string());
    seq.append("y".to_string());
    seq.clear();
    assert!(seq.is_empty());
    assert_eq!(seq.length(), 0);
}

#[test]
fn seq_geom_append_prepend_length() {
    let mut seq = ColGeom2dSeqGeom::new();
    seq.append("curve_a".to_string());
    seq.append("curve_b".to_string());
    seq.prepend("curve_z".to_string());
    assert_eq!(seq.length(), 3);
    assert_eq!(seq.first(), "curve_z");
    assert_eq!(seq.get(2), "curve_a");
    assert_eq!(seq.last(), "curve_b");
}

#[test]
fn seq_geom_remove_first_element() {
    let mut seq = ColGeom2dSeqGeom::new();
    seq.append("p".to_string());
    seq.append("q".to_string());
    seq.append("r".to_string());
    seq.remove(1);
    assert_eq!(seq.length(), 2);
    assert_eq!(seq.first(), "q");
}

#[test]
fn seq_geom_clear_then_repopulate() {
    let mut seq = ColGeom2dSeqGeom::new();
    seq.append("old".to_string());
    seq.clear();
    assert!(seq.is_empty());
    seq.append("new".to_string());
    assert_eq!(seq.length(), 1);
    assert_eq!(seq.get(1), "new");
}

#[test]
fn array1_bounded_curve_construction() {
    let arr = ColGeom2d1dBoundedCurve::new(1, 5);
    assert_eq!(arr.lower(), 1);
    assert_eq!(arr.upper(), 5);
    assert_eq!(arr.size(), 5);
    assert!(!arr.is_empty());
}

#[test]
fn array1_bounded_curve_set_get_and_none_slots() {
    let mut arr = ColGeom2d1dBoundedCurve::new(1, 4);
    arr.set(1, "arc2d".to_string());
    arr.set(3, "bezier2d".to_string());
    assert_eq!(arr.get(1).unwrap(), "arc2d");
    assert!(arr.get(2).is_none());
    assert_eq!(arr.get(3).unwrap(), "bezier2d");
    assert!(arr.get(4).is_none());
}

#[test]
fn array1_bounded_curve_negative_lower_bound() {
    let mut arr = ColGeom2d1dBoundedCurve::new(-2, 2);
    assert_eq!(arr.lower(), -2);
    assert_eq!(arr.upper(), 2);
    assert_eq!(arr.size(), 5);
    arr.set(-1, "trimmed2d".to_string());
    arr.set(2, "offset2d".to_string());
    assert_eq!(arr.get(-1).unwrap(), "trimmed2d");
    assert_eq!(arr.get(2).unwrap(), "offset2d");
    assert!(arr.get(0).is_none());
}

#[test]
#[should_panic]
fn array1_bounded_curve_out_of_bounds_panics() {
    let arr = ColGeom2d1dBoundedCurve::new(1, 3);
    let _ = arr.get(4);
}

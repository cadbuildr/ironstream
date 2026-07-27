//! Ported OpenCascade unit tests -- `math_DoubleTab`.
//!
//! Each test exercises a specific aspect of the `DoubleTab` API as it would
//! be exercised in OpenCascade's own test suite.  Tolerances follow
//! `precision::CONFUSION` (1e-7) and `precision::ANGULAR` (1e-12).

use ironstream::math_double_tab::DoubleTab;
use ironstream::precision;

// ---------------------------------------------------------------------------
// 1. Basic construction: lower == 1 (the usual OCCT default)
// ---------------------------------------------------------------------------
#[test]
fn test_new_default_lower() {
    let tab = DoubleTab::new(1, 5);
    assert_eq!(tab.lower(), 1);
    assert_eq!(tab.upper(), 5);
    assert_eq!(tab.length(), 5);
    // All elements must be initialised to 0.0.
    for i in 1..=5 {
        assert!(
            tab.value(i).abs() < precision::CONFUSION,
            "element {i} should be 0.0"
        );
    }
}

// ---------------------------------------------------------------------------
// 2. Construction with a negative lower bound (OCCT supports arbitrary bases)
// ---------------------------------------------------------------------------
#[test]
fn test_new_negative_lower() {
    let tab = DoubleTab::new(-3, 3);
    assert_eq!(tab.lower(), -3);
    assert_eq!(tab.upper(), 3);
    assert_eq!(tab.length(), 7);
    for i in -3..=3 {
        assert!(
            tab.value(i).abs() < precision::CONFUSION,
            "element {i} should be 0.0"
        );
    }
}

// ---------------------------------------------------------------------------
// 3. with_value constructor fills every element
// ---------------------------------------------------------------------------
#[test]
fn test_with_value_constructor() {
    let fill = std::f64::consts::PI;
    let tab = DoubleTab::with_value(0, 9, fill);
    assert_eq!(tab.lower(), 0);
    assert_eq!(tab.upper(), 9);
    assert_eq!(tab.length(), 10);
    for i in 0..=9 {
        assert!(
            (tab.value(i) - fill).abs() < precision::ANGULAR,
            "element {i}: expected {fill}, got {}",
            tab.value(i)
        );
    }
}

// ---------------------------------------------------------------------------
// 4. Init overwrites all elements
// ---------------------------------------------------------------------------
#[test]
fn test_init() {
    let mut tab = DoubleTab::new(1, 4);
    // Write distinct values first.
    for i in 1..=4 {
        tab.set(i, i as f64 * 10.0);
    }
    tab.init(42.0);
    for i in 1..=4 {
        assert!(
            (tab.value(i) - 42.0).abs() < precision::ANGULAR,
            "after Init element {i} should be 42.0"
        );
    }
}

// ---------------------------------------------------------------------------
// 5. set / value round-trip with arbitrary values
// ---------------------------------------------------------------------------
#[test]
fn test_set_value_round_trip() {
    let mut tab = DoubleTab::new(2, 6);
    let values = [1.1, 2.2, 3.3, 4.4, 5.5];
    for (k, &v) in values.iter().enumerate() {
        tab.set(2 + k as i32, v);
    }
    for (k, &expected) in values.iter().enumerate() {
        let got = tab.value(2 + k as i32);
        assert!(
            (got - expected).abs() < precision::ANGULAR,
            "index {}: expected {expected}, got {got}",
            2 + k
        );
    }
}

// ---------------------------------------------------------------------------
// 6. change_value returns a mutable reference that can be assigned through
// ---------------------------------------------------------------------------
#[test]
fn test_change_value_mutation() {
    let mut tab = DoubleTab::new(10, 13);
    *tab.change_value(11) = 99.5;
    assert!(
        (tab.value(11) - 99.5).abs() < precision::ANGULAR,
        "change_value assignment should persist"
    );
    // Neighbours are untouched.
    assert!(tab.value(10).abs() < precision::CONFUSION);
    assert!(tab.value(12).abs() < precision::CONFUSION);
    assert!(tab.value(13).abs() < precision::CONFUSION);
}

// ---------------------------------------------------------------------------
// 7. Clone produces an independent copy
// ---------------------------------------------------------------------------
#[test]
fn test_clone_independence() {
    let mut original = DoubleTab::with_value(1, 3, 7.0);
    let mut cloned = original.clone();
    // Mutate the clone — original must not change.
    cloned.set(1, 100.0);
    assert!(
        (original.value(1) - 7.0).abs() < precision::ANGULAR,
        "original should not be affected by clone mutation"
    );
    // Mutate the original — clone must not change.
    original.set(2, 200.0);
    assert!(
        (cloned.value(2) - 7.0).abs() < precision::ANGULAR,
        "clone should not be affected by original mutation"
    );
}

// ---------------------------------------------------------------------------
// 8. Single-element array (lower == upper)
// ---------------------------------------------------------------------------
#[test]
fn test_single_element() {
    let mut tab = DoubleTab::new(7, 7);
    assert_eq!(tab.lower(), 7);
    assert_eq!(tab.upper(), 7);
    assert_eq!(tab.length(), 1);
    tab.set(7, -3.14);
    assert!((tab.value(7) - (-3.14)).abs() < precision::ANGULAR);
}

// ---------------------------------------------------------------------------
// 9. Out-of-range access panics (Standard_RangeError equivalent)
// ---------------------------------------------------------------------------
#[test]
#[should_panic]
fn test_value_out_of_range_panics() {
    let tab = DoubleTab::new(1, 5);
    let _ = tab.value(6); // one past the upper bound — must panic
}

#[test]
#[should_panic]
fn test_value_below_lower_panics() {
    let tab = DoubleTab::new(3, 8);
    let _ = tab.value(2); // one below the lower bound — must panic
}

// ---------------------------------------------------------------------------
// 10. Large index base — stress test with offset 1_000_000
// ---------------------------------------------------------------------------
#[test]
fn test_large_index_base() {
    const BASE: i32 = 1_000_000;
    let mut tab = DoubleTab::new(BASE, BASE + 4);
    assert_eq!(tab.length(), 5);
    for i in BASE..=(BASE + 4) {
        tab.set(i, (i - BASE) as f64 * 0.5);
    }
    for i in BASE..=(BASE + 4) {
        let expected = (i - BASE) as f64 * 0.5;
        assert!(
            (tab.value(i) - expected).abs() < precision::ANGULAR,
            "index {i}: expected {expected}, got {}",
            tab.value(i)
        );
    }
}

// ---------------------------------------------------------------------------
// 11. Init after with_value resets to a different fill
// ---------------------------------------------------------------------------
#[test]
fn test_init_resets_with_value() {
    let mut tab = DoubleTab::with_value(-2, 2, 1.0);
    tab.init(0.0);
    for i in -2..=2 {
        assert!(
            tab.value(i).abs() < precision::CONFUSION,
            "after Init(0.0) element {i} should be 0.0"
        );
    }
}

// ---------------------------------------------------------------------------
// 12. PartialEq: two tabs with the same bounds and values compare equal
// ---------------------------------------------------------------------------
#[test]
fn test_partial_eq() {
    let mut a = DoubleTab::new(1, 3);
    let mut b = DoubleTab::new(1, 3);
    a.set(1, 1.0);
    a.set(2, 2.0);
    a.set(3, 3.0);
    b.set(1, 1.0);
    b.set(2, 2.0);
    b.set(3, 3.0);
    assert_eq!(a, b);

    b.set(3, 99.0);
    assert_ne!(a, b);
}

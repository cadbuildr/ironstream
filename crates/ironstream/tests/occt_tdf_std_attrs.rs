// FILE: tests/occt_tdf_std_attrs.rs
extern crate ironstream;

use ironstream::tdf_std_attrs::{TDataStdExtStringArray, TDataStdInteger, TDataStdName, TDataStdRealArray};

#[test]
fn tdatastd_name_value_set() {
    let mut n = TDataStdName::new("Part-1");
    assert_eq!(n.value(), "Part-1");
    n.set("Assembly-1");
    assert_eq!(n.value(), "Assembly-1");
    assert_eq!(n.length(), 10);
    assert!(!n.is_empty());

    let empty_n = TDataStdName::new("");
    assert!(empty_n.is_empty());
}

#[test]
fn tdatastd_integer_increment_decrement() {
    let mut i = TDataStdInteger::new(10);
    assert_eq!(i.value(), 10);
    i.increment();
    assert_eq!(i.value(), 11);
    i.decrement();
    i.decrement();
    assert_eq!(i.value(), 9);
    i.set(0);
    assert_eq!(i.value(), 0);
}

#[test]
fn tdatastd_real_array_mean_min_max() {
    let mut arr = TDataStdRealArray::new(0, 3);
    arr.set_value(0, 2.0);
    arr.set_value(1, 4.0);
    arr.set_value(2, 6.0);
    arr.set_value(3, 8.0);
    assert!((arr.mean() - 5.0).abs() < 1e-10);
    assert!((arr.min() - 2.0).abs() < 1e-10);
    assert!((arr.max() - 8.0).abs() < 1e-10);
    assert_eq!(arr.length(), 4);
}

#[test]
fn tdatastd_ext_string_array_value() {
    let mut arr = TDataStdExtStringArray::new(1, 3);
    arr.set_value(1, "first");
    arr.set_value(2, "second");
    arr.set_value(3, "third");
    assert_eq!(arr.value(1), Some("first"));
    assert_eq!(arr.value(2), Some("second"));
    assert_eq!(arr.value(3), Some("third"));
    assert!(arr.value(0).is_none(), "index 0 is below lower bound");
    assert!(arr.value(4).is_none(), "index 4 is above upper bound");
    assert_eq!(arr.length(), 3);
}

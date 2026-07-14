// FILE: rust/ironstream/crates/ironstream/tests/occt_tdf_attribute.rs

//! Ported OpenCascade unit tests — `TDF_Attribute` / `TDataStd` package.
//!
//! These tests verify the zero-dependency Rust port of the OCAF attribute
//! types: `TdfStringAttr`, `TdfIntegerAttr`, `TdfRealAttr`, and the
//! `TdfAttribute` tagged union. They mirror the contract exercised by
//! OCCT's `TDataStd` unit tests and integration harnesses.

extern crate ironstream;
use ironstream::tdf_attribute::*;

// ---------------------------------------------------------------------------
// TdfAttrType
// ---------------------------------------------------------------------------

#[test]
fn attr_type_variants_are_distinct() {
    assert_ne!(TdfAttrType::String, TdfAttrType::Integer);
    assert_ne!(TdfAttrType::Integer, TdfAttrType::Real);
    assert_ne!(TdfAttrType::Real, TdfAttrType::BooleanArray);
    assert_ne!(TdfAttrType::BooleanArray, TdfAttrType::ExtString);
    assert_ne!(TdfAttrType::ExtString, TdfAttrType::Guid);
}

// ---------------------------------------------------------------------------
// TdfStringAttr
// ---------------------------------------------------------------------------

#[test]
fn string_attr_new_and_value() {
    let a = TdfStringAttr::new("hello");
    assert_eq!(a.value(), "hello");
}

#[test]
fn string_attr_empty_string() {
    let a = TdfStringAttr::new("");
    assert_eq!(a.value(), "");
}

#[test]
fn string_attr_set_value() {
    let mut a = TdfStringAttr::new("initial");
    a.set_value("updated");
    assert_eq!(a.value(), "updated");
}

#[test]
fn string_attr_set_value_to_empty() {
    let mut a = TdfStringAttr::new("non-empty");
    a.set_value("");
    assert_eq!(a.value(), "");
}

#[test]
fn string_attr_attr_type() {
    let a = TdfStringAttr::new("test");
    assert_eq!(a.attr_type(), TdfAttrType::String);
}

#[test]
fn string_attr_unicode_value() {
    let a = TdfStringAttr::new("Ångström µ");
    assert_eq!(a.value(), "Ångström µ");
}

#[test]
fn string_attr_clone_is_independent() {
    let a = TdfStringAttr::new("original");
    let mut b = a.clone();
    b.set_value("clone");
    assert_eq!(a.value(), "original");
    assert_eq!(b.value(), "clone");
}

// ---------------------------------------------------------------------------
// TdfIntegerAttr
// ---------------------------------------------------------------------------

#[test]
fn integer_attr_new_and_value() {
    let a = TdfIntegerAttr::new(42);
    assert_eq!(a.value(), 42);
}

#[test]
fn integer_attr_zero() {
    let a = TdfIntegerAttr::new(0);
    assert_eq!(a.value(), 0);
}

#[test]
fn integer_attr_negative() {
    let a = TdfIntegerAttr::new(-7);
    assert_eq!(a.value(), -7);
}

#[test]
fn integer_attr_min_max() {
    let mn = TdfIntegerAttr::new(i64::MIN);
    let mx = TdfIntegerAttr::new(i64::MAX);
    assert_eq!(mn.value(), i64::MIN);
    assert_eq!(mx.value(), i64::MAX);
}

#[test]
fn integer_attr_set_value() {
    let mut a = TdfIntegerAttr::new(1);
    a.set_value(99);
    assert_eq!(a.value(), 99);
}

#[test]
fn integer_attr_attr_type() {
    let a = TdfIntegerAttr::new(0);
    assert_eq!(a.attr_type(), TdfAttrType::Integer);
}

// ---------------------------------------------------------------------------
// TdfRealAttr
// ---------------------------------------------------------------------------

#[test]
fn real_attr_new_and_value() {
    let a = TdfRealAttr::new(3.14);
    assert!((a.value() - 3.14).abs() < 1e-15);
}

#[test]
fn real_attr_zero() {
    let a = TdfRealAttr::new(0.0);
    assert_eq!(a.value(), 0.0);
}

#[test]
fn real_attr_negative() {
    let a = TdfRealAttr::new(-1.5);
    assert!((a.value() - (-1.5)).abs() < 1e-15);
}

#[test]
fn real_attr_set_value() {
    let mut a = TdfRealAttr::new(1.0);
    a.set_value(2.718_281_828);
    assert!((a.value() - 2.718_281_828).abs() < 1e-15);
}

#[test]
fn real_attr_attr_type() {
    let a = TdfRealAttr::new(0.0);
    assert_eq!(a.attr_type(), TdfAttrType::Real);
}

#[test]
fn real_attr_infinity() {
    let a = TdfRealAttr::new(f64::INFINITY);
    assert!(a.value().is_infinite());
}

#[test]
fn real_attr_nan_round_trip() {
    let a = TdfRealAttr::new(f64::NAN);
    assert!(a.value().is_nan());
}

// ---------------------------------------------------------------------------
// TdfAttribute — tagged union
// ---------------------------------------------------------------------------

#[test]
fn tdf_attribute_string_variant_type() {
    let attr = TdfAttribute::StringAttr(TdfStringAttr::new("name"));
    assert_eq!(attr.attr_type(), TdfAttrType::String);
}

#[test]
fn tdf_attribute_integer_variant_type() {
    let attr = TdfAttribute::IntegerAttr(TdfIntegerAttr::new(7));
    assert_eq!(attr.attr_type(), TdfAttrType::Integer);
}

#[test]
fn tdf_attribute_real_variant_type() {
    let attr = TdfAttribute::RealAttr(TdfRealAttr::new(1.0));
    assert_eq!(attr.attr_type(), TdfAttrType::Real);
}

#[test]
fn tdf_attribute_as_string_some() {
    let attr = TdfAttribute::StringAttr(TdfStringAttr::new("label"));
    let s = attr.as_string().expect("expected Some");
    assert_eq!(s.value(), "label");
}

#[test]
fn tdf_attribute_as_string_none_for_integer() {
    let attr = TdfAttribute::IntegerAttr(TdfIntegerAttr::new(0));
    assert!(attr.as_string().is_none());
}

#[test]
fn tdf_attribute_as_string_none_for_real() {
    let attr = TdfAttribute::RealAttr(TdfRealAttr::new(0.0));
    assert!(attr.as_string().is_none());
}

#[test]
fn tdf_attribute_as_integer_some() {
    let attr = TdfAttribute::IntegerAttr(TdfIntegerAttr::new(42));
    let i = attr.as_integer().expect("expected Some");
    assert_eq!(i.value(), 42);
}

#[test]
fn tdf_attribute_as_integer_none_for_string() {
    let attr = TdfAttribute::StringAttr(TdfStringAttr::new("x"));
    assert!(attr.as_integer().is_none());
}

#[test]
fn tdf_attribute_as_integer_none_for_real() {
    let attr = TdfAttribute::RealAttr(TdfRealAttr::new(1.0));
    assert!(attr.as_integer().is_none());
}

#[test]
fn tdf_attribute_as_real_some() {
    let attr = TdfAttribute::RealAttr(TdfRealAttr::new(2.5));
    let r = attr.as_real().expect("expected Some");
    assert!((r.value() - 2.5).abs() < 1e-15);
}

#[test]
fn tdf_attribute_as_real_none_for_string() {
    let attr = TdfAttribute::StringAttr(TdfStringAttr::new("y"));
    assert!(attr.as_real().is_none());
}

#[test]
fn tdf_attribute_as_real_none_for_integer() {
    let attr = TdfAttribute::IntegerAttr(TdfIntegerAttr::new(1));
    assert!(attr.as_real().is_none());
}

#[test]
fn tdf_attribute_clone_string() {
    let attr = TdfAttribute::StringAttr(TdfStringAttr::new("clone-me"));
    let attr2 = attr.clone();
    assert_eq!(attr, attr2);
}

#[test]
fn tdf_attribute_equality_string() {
    let a = TdfAttribute::StringAttr(TdfStringAttr::new("same"));
    let b = TdfAttribute::StringAttr(TdfStringAttr::new("same"));
    assert_eq!(a, b);
}

#[test]
fn tdf_attribute_inequality_different_types() {
    let a = TdfAttribute::IntegerAttr(TdfIntegerAttr::new(1));
    let b = TdfAttribute::RealAttr(TdfRealAttr::new(1.0));
    assert_ne!(a, b);
}

#[test]
fn tdf_attribute_vec_of_mixed_attrs() {
    let attrs: Vec<TdfAttribute> = vec![
        TdfAttribute::StringAttr(TdfStringAttr::new("part_name")),
        TdfAttribute::IntegerAttr(TdfIntegerAttr::new(3)),
        TdfAttribute::RealAttr(TdfRealAttr::new(12.5)),
    ];
    assert_eq!(attrs[0].attr_type(), TdfAttrType::String);
    assert_eq!(attrs[1].attr_type(), TdfAttrType::Integer);
    assert_eq!(attrs[2].attr_type(), TdfAttrType::Real);
    assert_eq!(attrs[0].as_string().unwrap().value(), "part_name");
    assert_eq!(attrs[1].as_integer().unwrap().value(), 3);
    assert!((attrs[2].as_real().unwrap().value() - 12.5).abs() < 1e-15);
}

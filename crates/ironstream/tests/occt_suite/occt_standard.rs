// FILE: tests/occt_standard.rs
//! Integration tests for `ironstream::standard` — the `Standard_*` base
//! infrastructure ported from OpenCascade's `Standard` package.
//!
//! Each test exercises the public API from an external crate perspective,
//! mirroring the patterns used in OCCT's own `Standard` GTests.

extern crate ironstream;

use ironstream::standard::{
    check_not_null, check_range, StandardBoolean, StandardCString, StandardFailure,
    StandardInteger, StandardNullObject, StandardOutOfRange, StandardReal, StandardTypeDef,
};

// ---------------------------------------------------------------------------
// Type-alias sanity checks
// ---------------------------------------------------------------------------

#[test]
fn standard_real_is_f64() {
    let v: StandardReal = std::f64::consts::PI;
    // Verify the alias resolves to the full f64 precision.
    assert!((v - 3.141_592_653_589_793_f64).abs() < 1.0e-15);
}

#[test]
fn standard_integer_is_i32() {
    let v: StandardInteger = -42_i32;
    assert_eq!(v.abs(), 42);
}

#[test]
fn standard_boolean_is_bool() {
    let t: StandardBoolean = true;
    let f: StandardBoolean = false;
    assert!(t);
    assert!(!f);
}

#[test]
fn standard_cstring_is_static_str() {
    let s: StandardCString = "IronStream";
    assert_eq!(s.len(), 10);
    assert!(s.starts_with('I'));
}

// ---------------------------------------------------------------------------
// StandardTypeDef — numerical limits
// ---------------------------------------------------------------------------

#[test]
fn typedef_real_epsilon_is_machine_eps() {
    // Machine epsilon is roughly 2.2e-16.
    assert!(StandardTypeDef::REAL_EPSILON > 0.0);
    assert!(StandardTypeDef::REAL_EPSILON < 1.0e-10);
    // Classic ULP property: 1.0 + eps != 1.0, but 1.0 + eps/2 == 1.0.
    let one_plus_eps: f64 = 1.0 + StandardTypeDef::REAL_EPSILON;
    assert_ne!(one_plus_eps, 1.0_f64);
}

#[test]
fn typedef_integer_size_is_four_bytes() {
    assert_eq!(StandardTypeDef::INTEGER_SIZE, 4);
}

#[test]
fn typedef_real_size_is_eight_bytes() {
    assert_eq!(StandardTypeDef::REAL_SIZE, 8);
}

#[test]
fn typedef_real_equal_identical_values() {
    assert!(StandardTypeDef::real_equal(0.0, 0.0));
    assert!(StandardTypeDef::real_equal(1.0, 1.0));
    assert!(StandardTypeDef::real_equal(-5.5, -5.5));
}

#[test]
fn typedef_real_equal_unequal_values() {
    // Values separated by more than machine epsilon must not be equal.
    assert!(!StandardTypeDef::real_equal(1.0, 1.0 + 1.0e-10));
    assert!(!StandardTypeDef::real_equal(0.0, 1.0e-15));
}

#[test]
fn typedef_is_integer_valid_boundary() {
    assert!(StandardTypeDef::is_integer_valid(i64::from(i32::MAX)));
    assert!(StandardTypeDef::is_integer_valid(i64::from(i32::MIN)));
    assert!(!StandardTypeDef::is_integer_valid(i64::from(i32::MAX) + 1));
    assert!(!StandardTypeDef::is_integer_valid(i64::from(i32::MIN) - 1));
}

// ---------------------------------------------------------------------------
// StandardFailure
// ---------------------------------------------------------------------------

#[test]
fn failure_stores_message() {
    let f = StandardFailure::new("topology mismatch");
    assert_eq!(f.message(), "topology mismatch");
}

#[test]
fn failure_display_includes_class_name() {
    let f = StandardFailure::new("boom");
    let s = f.to_string();
    assert!(s.contains("Standard_Failure"));
    assert!(s.contains("boom"));
}

#[test]
fn failure_raise_unwinds() {
    let result = std::panic::catch_unwind(|| {
        StandardFailure::raise("fatal error");
    });
    assert!(result.is_err(), "StandardFailure::raise must panic");
}

// ---------------------------------------------------------------------------
// StandardOutOfRange
// ---------------------------------------------------------------------------

#[test]
fn out_of_range_raise_if_panics_on_true() {
    let r = std::panic::catch_unwind(|| {
        StandardOutOfRange::raise_if(true, "index past end");
    });
    assert!(r.is_err());
}

#[test]
fn out_of_range_raise_if_silent_on_false() {
    // Must NOT panic.
    StandardOutOfRange::raise_if(false, "should never fire");
}

#[test]
fn out_of_range_check_err_on_true() {
    let r = StandardOutOfRange::check(true, "out of bounds");
    assert!(r.is_err());
    let e = r.unwrap_err();
    assert_eq!(e.message(), "out of bounds");
    assert!(e.to_string().contains("Standard_OutOfRange"));
}

#[test]
fn out_of_range_check_ok_on_false() {
    let r = StandardOutOfRange::check(false, "fine");
    assert!(r.is_ok());
}

// ---------------------------------------------------------------------------
// StandardNullObject
// ---------------------------------------------------------------------------

#[test]
fn null_object_raise_unwinds() {
    let r = std::panic::catch_unwind(|| {
        StandardNullObject::raise("null handle dereferenced");
    });
    assert!(r.is_err());
}

#[test]
fn null_object_check_none_is_err() {
    let opt: Option<f64> = None;
    let r = StandardNullObject::check(opt.is_none(), "shape handle is null");
    assert!(r.is_err());
    assert!(r.unwrap_err().to_string().contains("Standard_NullObject"));
}

#[test]
fn null_object_check_some_is_ok() {
    let opt: Option<f64> = Some(1.0);
    let r = StandardNullObject::check(opt.is_none(), "should not trigger");
    assert!(r.is_ok());
}

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

#[test]
fn check_range_passes_in_bounds() {
    // Must not panic for valid index.
    check_range(1, 1, 3);
    check_range(2, 1, 3);
    check_range(3, 1, 3);
}

#[test]
fn check_range_panics_out_of_bounds() {
    let lo = std::panic::catch_unwind(|| check_range(0, 1, 3));
    let hi = std::panic::catch_unwind(|| check_range(4, 1, 3));
    assert!(lo.is_err(), "index below lower bound must panic");
    assert!(hi.is_err(), "index above upper bound must panic");
}

#[test]
fn check_not_null_passes_some() {
    let opt: Option<i32> = Some(7);
    check_not_null(&opt, "should not panic");
}

#[test]
fn check_not_null_panics_none() {
    let opt: Option<i32> = None;
    let r = std::panic::catch_unwind(|| check_not_null(&opt, "null object"));
    assert!(r.is_err(), "None option must trigger StandardNullObject panic");
}

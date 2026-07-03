// FILE: standard.rs
//! `Standard` — base-infrastructure type definitions and exception hierarchy,
//! mirroring OpenCascade's `Standard` package
//! (`src/FoundationClasses/TKernel/Standard/`).
//!
//! OCCT uses C++ typedef aliases and a class-based exception hierarchy. In
//! pure Rust we model the typedef aliases as type aliases and `const` bounds,
//! and the exceptions as `struct` / `enum` types that carry a message string.
//! Panicking helpers mirror OCCT's `throw` semantics for callers that rely on
//! the `catch_unwind` pattern used elsewhere in IronStream tests.

// ---------------------------------------------------------------------------
// Typedef aliases
// ---------------------------------------------------------------------------

/// occt: Standard_Real
///
/// 64-bit IEEE 754 double — OCCT's `Standard_Real`.
pub type StandardReal = f64;

/// occt: Standard_Integer
///
/// 32-bit signed integer — OCCT's `Standard_Integer`.
pub type StandardInteger = i32;

/// occt: Standard_Boolean
///
/// Boolean flag — OCCT's `Standard_Boolean`.
pub type StandardBoolean = bool;

/// occt: Standard_CString
///
/// Immutable UTF-8 string slice — models OCCT's `Standard_CString`
/// (`const char*`). The lifetime is `'static` for constants; callers that need
/// heap-allocated strings use `String` and call `.as_str()` when an interface
/// expects `StandardCString`.
pub type StandardCString = &'static str;

// ---------------------------------------------------------------------------
// Standard_TypeDef — OCCT numerical-limits constants
// ---------------------------------------------------------------------------

/// occt: Standard_TypeDef
///
/// Compile-time numerical limits and sentinel values that correspond to the
/// macros OCCT exposes via `Standard_TypeDef.hxx`.
pub struct StandardTypeDef;

impl StandardTypeDef {
    /// Maximum value of `Standard_Real`.
    pub const REAL_MAX: StandardReal = f64::MAX;

    /// Minimum positive value of `Standard_Real`.
    pub const REAL_MIN: StandardReal = f64::MIN_POSITIVE;

    /// OCCT `RealEpsilon` — machine epsilon for `Standard_Real`.
    pub const REAL_EPSILON: StandardReal = f64::EPSILON;

    /// Maximum value of `Standard_Integer`.
    pub const INTEGER_MAX: StandardInteger = i32::MAX;

    /// Minimum value of `Standard_Integer`.
    pub const INTEGER_MIN: StandardInteger = i32::MIN;

    /// OCCT `IntegerSize` — byte width of `Standard_Integer`.
    pub const INTEGER_SIZE: usize = core::mem::size_of::<StandardInteger>();

    /// OCCT `RealSize` — byte width of `Standard_Real`.
    pub const REAL_SIZE: usize = core::mem::size_of::<StandardReal>();

    /// Returns `true` when two reals are equal within machine epsilon scaled
    /// by the larger magnitude (`ULP`-style relative comparison).
    #[inline]
    pub fn real_equal(a: StandardReal, b: StandardReal) -> StandardBoolean {
        (a - b).abs() <= Self::REAL_EPSILON * a.abs().max(b.abs()).max(1.0)
    }

    /// Returns `true` when the integer value fits in the valid range (always
    /// true for `i32`, but mirrors OCCT's `IsIntegerValid` predicate for
    /// completeness when the source value comes from a wider type).
    #[inline]
    pub fn is_integer_valid(v: i64) -> StandardBoolean {
        v >= i64::from(i32::MIN) && v <= i64::from(i32::MAX)
    }
}

// ---------------------------------------------------------------------------
// Standard_Failure — base exception
// ---------------------------------------------------------------------------

/// occt: Standard_Failure
///
/// Root of the OCCT exception hierarchy. Carries a message string, mirroring
/// `Standard_Failure::Raise("msg")`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandardFailure {
    /// Human-readable description of the failure.
    pub message: String,
}

impl StandardFailure {
    /// Construct a new `StandardFailure` with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }

    /// Raise (panic with) this failure — mirrors `Standard_Failure::Raise`.
    pub fn raise(message: impl Into<String> + std::fmt::Display) -> ! {
        panic!("Standard_Failure: {}", message)
    }

    /// Return the stored message — mirrors `GetMessageString()`.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for StandardFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Standard_Failure: {}", self.message)
    }
}

impl std::error::Error for StandardFailure {}

// ---------------------------------------------------------------------------
// Standard_OutOfRange
// ---------------------------------------------------------------------------

/// occt: Standard_OutOfRange
///
/// Raised when an index or parameter is outside the valid range.
/// Mirrors `Standard_OutOfRange::Raise("msg")`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandardOutOfRange {
    /// Human-readable description of the out-of-range condition.
    pub message: String,
}

impl StandardOutOfRange {
    /// Construct with a message.
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }

    /// Raise (panic) unconditionally — mirrors `Standard_OutOfRange::Raise`.
    pub fn raise(message: impl Into<String> + std::fmt::Display) -> ! {
        panic!("Standard_OutOfRange: {}", message)
    }

    /// Raise only when `condition` is true — mirrors the OCCT guard pattern
    /// `Standard_OutOfRange_Raise_if(cond, "msg")`.
    #[inline]
    pub fn raise_if(condition: bool, message: &'static str) {
        if condition {
            panic!("Standard_OutOfRange: {}", message);
        }
    }

    /// Non-panicking variant: returns `Err` when `condition` is true.
    #[inline]
    pub fn check(condition: bool, message: impl Into<String>) -> Result<(), Self> {
        if condition {
            Err(Self::new(message))
        } else {
            Ok(())
        }
    }

    /// Return the stored message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for StandardOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Standard_OutOfRange: {}", self.message)
    }
}

impl std::error::Error for StandardOutOfRange {}

// ---------------------------------------------------------------------------
// Standard_NullObject
// ---------------------------------------------------------------------------

/// occt: Standard_NullObject
///
/// Raised when a null handle / null pointer is dereferenced.
/// Mirrors `Standard_NullObject::Raise("msg")`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandardNullObject {
    /// Human-readable description.
    pub message: String,
}

impl StandardNullObject {
    /// Construct with a message.
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }

    /// Raise (panic) unconditionally.
    pub fn raise(message: impl Into<String> + std::fmt::Display) -> ! {
        panic!("Standard_NullObject: {}", message)
    }

    /// Raise only when `condition` is true — mirrors
    /// `Standard_NullObject_Raise_if(cond, "msg")`.
    #[inline]
    pub fn raise_if(condition: bool, message: &'static str) {
        if condition {
            panic!("Standard_NullObject: {}", message);
        }
    }

    /// Non-panicking variant: returns `Err` when `condition` is true.
    #[inline]
    pub fn check(condition: bool, message: impl Into<String>) -> Result<(), Self> {
        if condition {
            Err(Self::new(message))
        } else {
            Ok(())
        }
    }

    /// Return the stored message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for StandardNullObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Standard_NullObject: {}", self.message)
    }
}

impl std::error::Error for StandardNullObject {}

// ---------------------------------------------------------------------------
// Convenience constructors used by the rest of the kernel
// ---------------------------------------------------------------------------

/// Raise a `Standard_OutOfRange` panic when `index` is outside `[low, high]`.
///
/// Mirrors the macro pattern used pervasively in OCCT collection classes.
#[inline]
pub fn check_range(index: StandardInteger, low: StandardInteger, high: StandardInteger) {
    StandardOutOfRange::raise_if(
        index < low || index > high,
        "index out of range",
    );
}

/// Raise a `Standard_NullObject` panic when `ptr` is `None`.
#[inline]
pub fn check_not_null<T>(value: &Option<T>, context: &'static str) {
    StandardNullObject::raise_if(value.is_none(), context);
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- StandardTypeDef ---

    #[test]
    fn typedef_real_limits() {
        assert!(StandardTypeDef::REAL_MAX > 1.0e308);
        assert!(StandardTypeDef::REAL_MIN > 0.0);
        assert!(StandardTypeDef::REAL_EPSILON > 0.0);
        assert!(StandardTypeDef::REAL_EPSILON < 1.0e-10);
    }

    #[test]
    fn typedef_integer_limits() {
        assert_eq!(StandardTypeDef::INTEGER_MAX, i32::MAX);
        assert_eq!(StandardTypeDef::INTEGER_MIN, i32::MIN);
        assert_eq!(StandardTypeDef::INTEGER_SIZE, 4);
        assert_eq!(StandardTypeDef::REAL_SIZE, 8);
    }

    #[test]
    fn typedef_real_equal() {
        assert!(StandardTypeDef::real_equal(1.0, 1.0));
        assert!(StandardTypeDef::real_equal(0.0, 0.0));
        // Differs by exactly 2 * epsilon — outside the band.
        let a = 1.0_f64;
        let b = a + 3.0 * f64::EPSILON;
        assert!(!StandardTypeDef::real_equal(a, b));
    }

    #[test]
    fn typedef_is_integer_valid() {
        assert!(StandardTypeDef::is_integer_valid(0));
        assert!(StandardTypeDef::is_integer_valid(i32::MAX as i64));
        assert!(StandardTypeDef::is_integer_valid(i32::MIN as i64));
        assert!(!StandardTypeDef::is_integer_valid(i64::from(i32::MAX) + 1));
        assert!(!StandardTypeDef::is_integer_valid(i64::from(i32::MIN) - 1));
    }

    // --- StandardFailure ---

    #[test]
    fn failure_message_round_trip() {
        let f = StandardFailure::new("bad geometry");
        assert_eq!(f.message(), "bad geometry");
        assert!(f.to_string().contains("bad geometry"));
    }

    #[test]
    fn failure_raise_panics() {
        let result = std::panic::catch_unwind(|| {
            StandardFailure::raise("something went wrong");
        });
        assert!(result.is_err());
    }

    // --- StandardOutOfRange ---

    #[test]
    fn out_of_range_raise_if_triggers() {
        let result = std::panic::catch_unwind(|| {
            StandardOutOfRange::raise_if(true, "index 42 out of bounds");
        });
        assert!(result.is_err());
    }

    #[test]
    fn out_of_range_raise_if_no_trigger() {
        // Must not panic when condition is false.
        StandardOutOfRange::raise_if(false, "should not fire");
    }

    #[test]
    fn out_of_range_check_returns_err() {
        let r = StandardOutOfRange::check(true, "bad index");
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().message(), "bad index");
    }

    #[test]
    fn out_of_range_check_returns_ok() {
        let r = StandardOutOfRange::check(false, "fine");
        assert!(r.is_ok());
    }

    // --- StandardNullObject ---

    #[test]
    fn null_object_raise_if_triggers() {
        let result = std::panic::catch_unwind(|| {
            StandardNullObject::raise_if(true, "null handle");
        });
        assert!(result.is_err());
    }

    #[test]
    fn null_object_check_none() {
        let opt: Option<i32> = None;
        let r = StandardNullObject::check(opt.is_none(), "null ptr");
        assert!(r.is_err());
    }

    // --- check_range helper ---

    #[test]
    fn check_range_valid() {
        check_range(3, 1, 5); // should not panic
    }

    #[test]
    fn check_range_panics_below() {
        let result = std::panic::catch_unwind(|| check_range(0, 1, 5));
        assert!(result.is_err());
    }

    #[test]
    fn check_range_panics_above() {
        let result = std::panic::catch_unwind(|| check_range(6, 1, 5));
        assert!(result.is_err());
    }
}

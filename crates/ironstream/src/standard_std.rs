// FILE: standard_std.rs
// occt: Standard_Std

//! Type traits and template utilities for standard library compatibility.

/// Trait yielding true if class T1 is base of T2 but not the same.
pub trait IsBaseButNotSame {
    fn is_base_not_same() -> bool;
}

/// The type trait that checks if the passed type is integer.
/// Must be integral and not boolean.
pub trait IsInteger {
    fn is_integer() -> bool;
}

/// Implement IsInteger for i8
impl IsInteger for i8 {
    fn is_integer() -> bool {
        true
    }
}

/// Implement IsInteger for i16
impl IsInteger for i16 {
    fn is_integer() -> bool {
        true
    }
}

/// Implement IsInteger for i32
impl IsInteger for i32 {
    fn is_integer() -> bool {
        true
    }
}

/// Implement IsInteger for i64
impl IsInteger for i64 {
    fn is_integer() -> bool {
        true
    }
}

/// Implement IsInteger for u8
impl IsInteger for u8 {
    fn is_integer() -> bool {
        true
    }
}

/// Implement IsInteger for u16
impl IsInteger for u16 {
    fn is_integer() -> bool {
        true
    }
}

/// Implement IsInteger for u32
impl IsInteger for u32 {
    fn is_integer() -> bool {
        true
    }
}

/// Implement IsInteger for u64
impl IsInteger for u64 {
    fn is_integer() -> bool {
        true
    }
}

/// Implement IsInteger for bool (should return false)
impl IsInteger for bool {
    fn is_integer() -> bool {
        false
    }
}

/// The auxiliary template used for template argument deduction in function templates.
/// A function argument which type is a template type parameter and doesn't need deduction
/// must be declared using this class template based on the type of some other template parameter.
pub struct DisableDeduction<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> DisableDeduction<T> {
    pub fn new() -> Self {
        DisableDeduction {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> Default for DisableDeduction<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_integer_i32() {
        assert!(i32::is_integer());
    }

    #[test]
    fn test_is_integer_u64() {
        assert!(u64::is_integer());
    }

    #[test]
    fn test_is_integer_bool() {
        assert!(!bool::is_integer());
    }

    #[test]
    fn test_disable_deduction_creation() {
        let _dd: DisableDeduction<i32> = DisableDeduction::new();
    }

    #[test]
    fn test_disable_deduction_default() {
        let _dd: DisableDeduction<String> = DisableDeduction::default();
    }

    #[test]
    fn test_is_integer_all_types() {
        assert!(i8::is_integer());
        assert!(i16::is_integer());
        assert!(i32::is_integer());
        assert!(i64::is_integer());
        assert!(u8::is_integer());
        assert!(u16::is_integer());
        assert!(u32::is_integer());
        assert!(u64::is_integer());
    }
}

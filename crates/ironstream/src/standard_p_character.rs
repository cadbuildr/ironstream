// FILE: standard_p_character.rs
// occt: Standard_PCharacter

//! Pointer to character (char) type definition.

/// Standard pointer to character type.
/// Rust equivalent of Standard_PCharacter (char*)
pub type StandardPCharacter = *mut u8;

/// Helper function to create a StandardPCharacter from a mutable string.
pub fn pchar_from_str(s: &mut str) -> StandardPCharacter {
    s.as_mut_ptr()
}

/// Helper function to create a StandardPCharacter from a Vec of bytes.
pub fn pchar_from_vec(vec: &mut Vec<u8>) -> StandardPCharacter {
    vec.as_mut_ptr()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pchar_from_vec() {
        let mut data = vec![b'a', b'b', b'c'];
        let ptr = pchar_from_vec(&mut data);
        assert!(!ptr.is_null());
        unsafe {
            assert_eq!(*ptr, b'a');
        }
    }

    #[test]
    fn test_pchar_null() {
        let ptr: StandardPCharacter = std::ptr::null_mut();
        assert!(ptr.is_null());
    }

    #[test]
    fn test_pchar_is_pointer() {
        let mut data = vec![b'x', b'y'];
        let ptr = pchar_from_vec(&mut data);
        unsafe {
            assert_eq!(*ptr, b'x');
            assert_eq!(*ptr.add(1), b'y');
        }
    }
}

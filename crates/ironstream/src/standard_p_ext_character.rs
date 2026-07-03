// FILE: standard_p_ext_character.rs
// occt: Standard_PExtCharacter

//! Pointer to extended character (char16_t) type definition.

/// Standard pointer to extended character type (UTF-16).
/// Rust equivalent of Standard_PExtCharacter (char16_t*)
pub type StandardPExtCharacter = *mut u16;

/// Helper function to create a StandardPExtCharacter from a mutable slice.
pub fn pext_char_from_slice(slice: &mut [u16]) -> StandardPExtCharacter {
    slice.as_mut_ptr()
}

/// Helper function to create a StandardPExtCharacter from a Vec.
pub fn pext_char_from_vec(vec: &mut Vec<u16>) -> StandardPExtCharacter {
    vec.as_mut_ptr()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pext_char_from_slice() {
        let mut data = [0x0041u16, 0x0042u16, 0x0043u16]; // A, B, C in UTF-16
        let ptr = pext_char_from_slice(&mut data);
        assert!(!ptr.is_null());
        unsafe {
            assert_eq!(*ptr, 0x0041);
        }
    }

    #[test]
    fn test_pext_char_from_vec() {
        let mut vec = vec![0x0078u16, 0x0079u16]; // x, y in UTF-16
        let ptr = pext_char_from_vec(&mut vec);
        assert!(!ptr.is_null());
        unsafe {
            assert_eq!(*ptr, 0x0078);
        }
    }

    #[test]
    fn test_pext_char_null() {
        let ptr: StandardPExtCharacter = std::ptr::null_mut();
        assert!(ptr.is_null());
    }

    #[test]
    fn test_pext_char_pointer_arithmetic() {
        let mut data = vec![0x0041u16, 0x0042u16, 0x0043u16];
        let ptr = pext_char_from_vec(&mut data);
        unsafe {
            assert_eq!(*ptr, 0x0041);
            assert_eq!(*ptr.add(1), 0x0042);
            assert_eq!(*ptr.add(2), 0x0043);
        }
    }
}

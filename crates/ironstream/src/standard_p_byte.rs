// FILE: standard_p_byte.rs
// occt: Standard_PByte

//! Pointer to byte (uint8_t) type definition.

/// Standard pointer to unsigned byte type.
/// Rust equivalent of Standard_PByte (uint8_t*)
pub type StandardPByte = *mut u8;

/// Helper function to create a StandardPByte from a mutable u8 slice.
pub fn pbyte_from_slice(slice: &mut [u8]) -> StandardPByte {
    slice.as_mut_ptr()
}

/// Helper function to create a StandardPByte from a Vec.
pub fn pbyte_from_vec(vec: &mut Vec<u8>) -> StandardPByte {
    vec.as_mut_ptr()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pbyte_from_slice() {
        let mut data = [1u8, 2u8, 3u8];
        let ptr = pbyte_from_slice(&mut data);
        assert!(!ptr.is_null());
        unsafe {
            assert_eq!(*ptr, 1);
        }
    }

    #[test]
    fn test_pbyte_from_vec() {
        let mut vec = vec![10u8, 20u8, 30u8];
        let ptr = pbyte_from_vec(&mut vec);
        assert!(!ptr.is_null());
        unsafe {
            assert_eq!(*ptr, 10);
        }
    }

    #[test]
    fn test_pbyte_null() {
        let ptr: StandardPByte = std::ptr::null_mut();
        assert!(ptr.is_null());
    }
}

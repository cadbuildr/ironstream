// FILE: bin_obj_mgt_p_ext_char.rs
// occt: BinObjMgt_PExtChar

/// Pointer to an extended character (char16_t / u16) for binary object management serialization.
/// This is a type alias representing a raw pointer to 16-bit char data.
pub type BinObjMgtPExtChar = *mut u16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_ext_char_is_pointer_type() {
        // Verify that BinObjMgtPExtChar is a pointer type
        let _ptr: BinObjMgtPExtChar = std::ptr::null_mut();
        assert!(true, "BinObjMgtPExtChar compiles as a pointer type");
    }
}

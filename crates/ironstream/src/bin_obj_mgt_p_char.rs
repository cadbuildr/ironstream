// FILE: bin_obj_mgt_p_char.rs
// occt: BinObjMgt_PChar

/// Pointer to a character (char) for binary object management serialization.
/// This is a type alias representing a raw pointer to char data.
pub type BinObjMgtPChar = *mut i8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_char_is_pointer_type() {
        // Verify that BinObjMgtPChar is a pointer type
        let _ptr: BinObjMgtPChar = std::ptr::null_mut();
        assert!(true, "BinObjMgtPChar compiles as a pointer type");
    }
}

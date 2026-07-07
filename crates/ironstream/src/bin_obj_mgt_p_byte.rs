// FILE: bin_obj_mgt_p_byte.rs
// occt: BinObjMgt_PByte

/// Pointer to a byte (uint8_t) for binary object management serialization.
/// This is a type alias representing a raw pointer to unsigned 8-bit data.
pub type BinObjMgtPByte = *mut u8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_byte_is_pointer_type() {
        // Verify that BinObjMgtPByte is a pointer type
        let _ptr: BinObjMgtPByte = std::ptr::null_mut();
        assert!(true, "BinObjMgtPByte compiles as a pointer type");
    }
}

// FILE: bin_obj_mgt_p_short_real.rs
// occt: BinObjMgt_PShortReal

/// Pointer to a short real number (float) for binary object management serialization.
/// This is a type alias representing a raw pointer to f32 data.
pub type BinObjMgtPShortReal = *mut f32;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_short_real_is_pointer_type() {
        // Verify that BinObjMgtPShortReal is a pointer type
        let _ptr: BinObjMgtPShortReal = std::ptr::null_mut();
        assert!(true, "BinObjMgtPShortReal compiles as a pointer type");
    }
}

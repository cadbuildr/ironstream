// FILE: bin_obj_mgt_p_integer.rs
// occt: BinObjMgt_PInteger

/// Pointer to an integer (int) for binary object management serialization.
/// This is a type alias representing a raw pointer to i32 data.
pub type BinObjMgtPInteger = *mut i32;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_integer_is_pointer_type() {
        // Verify that BinObjMgtPInteger is a pointer type
        let _ptr: BinObjMgtPInteger = std::ptr::null_mut();
        assert!(true, "BinObjMgtPInteger compiles as a pointer type");
    }
}

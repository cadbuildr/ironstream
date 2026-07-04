// FILE: bin_obj_mgt_p_real.rs
// occt: BinObjMgt_PReal

/// Pointer to a real number (double) for binary object management serialization.
/// This is a type alias representing a raw pointer to f64 data.
pub type BinObjMgtPReal = *mut f64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_real_is_pointer_type() {
        // Verify that BinObjMgtPReal is a pointer type
        let _ptr: BinObjMgtPReal = std::ptr::null_mut();
        assert!(true, "BinObjMgtPReal compiles as a pointer type");
    }
}

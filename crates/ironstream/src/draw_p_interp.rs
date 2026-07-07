// FILE: draw_p_interp.rs
// occt: Draw_PInterp

//! Opaque pointer type for TCL interpreter.
//! This is a typedef over a C TCL_Interp pointer.

/// Opaque pointer to a Draw interpreter (TCL Interp)
pub type DrawPInterp = *mut std::ffi::c_void;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_p_interp_type() {
        // Verify the type is a mutable void pointer
        let _p: DrawPInterp = std::ptr::null_mut();
        // Type check passes - DrawPInterp is correctly defined as a mutable void pointer
    }
}

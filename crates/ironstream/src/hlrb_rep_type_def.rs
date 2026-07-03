// FILE: hlrb_rep_type_def.rs
// occt: HLRBRep_TypeDef

/// Type alias for curve pointer used in HLRBRep algorithms.
pub type HLRBRepCurvePtr = *mut std::ffi::c_void;

/// Type alias for surface pointer used in HLRBRep algorithms.
pub type HLRBRepSurfacePtr = *mut std::ffi::c_void;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_ptr_size() {
        // Type aliases should be pointer-sized
        assert_eq!(std::mem::size_of::<HLRBRepCurvePtr>(), std::mem::size_of::<*mut ()>());
    }

    #[test]
    fn test_surface_ptr_size() {
        assert_eq!(std::mem::size_of::<HLRBRepSurfacePtr>(), std::mem::size_of::<*mut ()>());
    }

    #[test]
    fn test_null_pointers() {
        let curve_ptr: HLRBRepCurvePtr = std::ptr::null_mut();
        let surf_ptr: HLRBRepSurfacePtr = std::ptr::null_mut();
        assert!(curve_ptr.is_null());
        assert!(surf_ptr.is_null());
    }
}

// FILE: aspect_display.rs
// occt: Aspect_Display

/// Display connection handle (opaque).
pub type AspectDisplay = *mut std::ffi::c_void;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aspect_display() {
        let _disp: AspectDisplay = std::ptr::null_mut();
    }
}

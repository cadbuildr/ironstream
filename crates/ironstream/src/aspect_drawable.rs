// FILE: aspect_drawable.rs
// occt: Aspect_Drawable

/// Drawable handle (platform-dependent).
#[cfg(target_os = "windows")]
pub type AspectDrawable = *mut std::ffi::c_void; // HDC under Windows

#[cfg(not(target_os = "windows"))]
pub type AspectDrawable = usize; // Window/Pixmap under UNIX

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aspect_drawable() {
        #[cfg(target_os = "windows")]
        {
            let _draw: AspectDrawable = std::ptr::null_mut();
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _draw: AspectDrawable = 0;
        }
    }
}

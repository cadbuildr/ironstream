// FILE: aspect_fb_config.rs
// occt: Aspect_FBConfig

/// Framebuffer configuration (opaque handle).
pub type AspectFBConfig = *mut std::ffi::c_void;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fb_config() {
        let _config: AspectFBConfig = std::ptr::null_mut();
    }
}

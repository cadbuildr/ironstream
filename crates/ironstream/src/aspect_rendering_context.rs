// FILE: aspect_rendering_context.rs
// occt: Aspect_RenderingContext

/// Rendering context (opaque handle).
pub type AspectRenderingContext = *mut std::ffi::c_void;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rendering_context() {
        let _ctx: AspectRenderingContext = std::ptr::null_mut();
    }
}

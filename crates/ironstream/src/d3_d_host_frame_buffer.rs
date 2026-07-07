// FILE: d3_d_host_frame_buffer.rs
// occt: D3DHost_FrameBuffer

//! Implements a bridge FBO (Frame Buffer Object) for direct rendering to Direct3D surfaces.
//! This module provides interoperability between OpenGL and Direct3D 9.

use std::ptr;

/// Represents a Direct3D device (opaque pointer).
pub type D3DDevice = *mut std::ffi::c_void;

/// Represents a Direct3D surface (opaque pointer).
pub type D3DSurface = *mut std::ffi::c_void;

/// OpenGL context placeholder.
#[derive(Clone, Debug)]
pub struct OpenGlContext {
    id: u32,
}

impl OpenGlContext {
    pub fn new(id: u32) -> Self {
        OpenGlContext { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// D3DHost_FrameBuffer: bridges OpenGL and Direct3D for framebuffer rendering.
/// Implements shared resources between the two APIs.
#[derive(Clone, Debug)]
pub struct D3DHostFrameBuffer {
    /// D3D surface used as color buffer
    d3d_surf: Option<D3DSurface>,
    /// D3D surface share handle in WDDM
    d3d_surf_share: Option<*mut std::ffi::c_void>,
    /// WGL/D3D device handle
    gl_d3d_device: Option<*mut std::ffi::c_void>,
    /// WGL/D3D surface handle
    gl_d3d_surf: Option<*mut std::ffi::c_void>,
    /// Locking counter for resource synchronization
    lock_count: i32,
    /// Indicates that FBO has been initialized without WGL/D3D interop (fallback mode)
    d3d_fallback: bool,
    /// Indicates that color buffer is sRGB ready
    is_srgb_ready: bool,
    /// Framebuffer dimensions
    width: i32,
    height: i32,
    /// Depth format (0 = no depth attachment)
    depth_format: i32,
}

impl D3DHostFrameBuffer {
    /// Create an empty framebuffer.
    pub fn new() -> Self {
        D3DHostFrameBuffer {
            d3d_surf: None,
            d3d_surf_share: None,
            gl_d3d_device: None,
            gl_d3d_surf: None,
            lock_count: 0,
            d3d_fallback: false,
            is_srgb_ready: false,
            width: 0,
            height: 0,
            depth_format: 0,
        }
    }

    /// Releases D3D and OpenGL resources.
    pub fn release(&mut self) {
        self.d3d_surf = None;
        self.d3d_surf_share = None;
        self.gl_d3d_device = None;
        self.gl_d3d_surf = None;
        self.lock_count = 0;
    }

    /// Initializes OpenGL FBO for Direct3D interoperability or in fallback mode.
    /// Color pixel format is always GL_RGBA8/D3DFMT_X8R8G8B8, no MSAA.
    /// Depth-stencil is GL_DEPTH24_STENCIL8 by default.
    pub fn init(
        &mut self,
        _ctx: &OpenGlContext,
        _d3d_device: D3DDevice,
        _is_d3d_ex: bool,
        size_x: i32,
        size_y: i32,
    ) -> bool {
        if size_x <= 0 || size_y <= 0 {
            return false;
        }
        self.width = size_x;
        self.height = size_y;
        self.depth_format = 24; // GL_DEPTH24_STENCIL8
        true
    }

    /// Initializes OpenGL FBO for Direct3D interoperability with custom depth format.
    pub fn init_d3d_interop(
        &mut self,
        _ctx: &OpenGlContext,
        _d3d_device: D3DDevice,
        _is_d3d_ex: bool,
        size_x: i32,
        size_y: i32,
        depth_format: i32,
    ) -> bool {
        if size_x <= 0 || size_y <= 0 {
            return false;
        }
        self.width = size_x;
        self.height = size_y;
        self.depth_format = depth_format;
        self.d3d_fallback = false;
        true
    }

    /// Initializes OpenGL FBO + Direct3D surface for copying memory using fallback.
    pub fn init_d3d_fallback(
        &mut self,
        _ctx: &OpenGlContext,
        _d3d_device: D3DDevice,
        _is_d3d_ex: bool,
        size_x: i32,
        size_y: i32,
        depth_format: i32,
    ) -> bool {
        if size_x <= 0 || size_y <= 0 {
            return false;
        }
        self.width = size_x;
        self.height = size_y;
        self.depth_format = depth_format;
        self.d3d_fallback = true;
        true
    }

    /// Binds Direct3D color buffer to OpenGL texture.
    pub fn register_d3d_buffer(&mut self, _ctx: &OpenGlContext) -> bool {
        if self.width <= 0 || self.height <= 0 {
            return false;
        }
        // In a real implementation, this would register D3D resources with OpenGL
        true
    }

    /// Binds Direct3D objects for OpenGL drawing.
    pub fn bind_buffer(&mut self, _ctx: &OpenGlContext) {
        if self.lock_count == 0 {
            // Acquire D3D resource
            self.lock_count += 1;
        }
    }

    /// Acquires D3D resource for OpenGL usage (lock).
    pub fn lock_surface(&mut self, _ctx: &OpenGlContext) {
        self.lock_count += 1;
    }

    /// Releases D3D resource (unlock).
    pub fn unlock_surface(&mut self, _ctx: &OpenGlContext) {
        if self.lock_count > 0 {
            self.lock_count -= 1;
        }
    }

    /// Returns the D3D surface used as color buffer.
    pub fn d3d_color_surface(&self) -> Option<D3DSurface> {
        self.d3d_surf
    }

    /// Returns the WDDM handle for D3D color surface.
    pub fn d3d_color_surface_share(&self) -> Option<*mut std::ffi::c_void> {
        self.d3d_surf_share
    }

    /// Returns TRUE if FBO has been initialized without WGL/D3D interop (fallback mode).
    pub fn d3d_fallback(&self) -> bool {
        self.d3d_fallback
    }

    /// Returns TRUE if color buffer is sRGB ready.
    pub fn is_srgb_ready(&self) -> bool {
        self.is_srgb_ready
    }

    /// Set if color buffer is sRGB ready.
    pub fn set_srgb_ready(&mut self, is_ready: bool) {
        self.is_srgb_ready = is_ready;
    }

    /// Get framebuffer width.
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Get framebuffer height.
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Get depth format.
    pub fn depth_format(&self) -> i32 {
        self.depth_format
    }

    /// Get current lock count.
    pub fn lock_count(&self) -> i32 {
        self.lock_count
    }
}

impl Default for D3DHostFrameBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framebuffer_creation() {
        let fb = D3DHostFrameBuffer::new();
        assert_eq!(fb.width(), 0);
        assert_eq!(fb.height(), 0);
        assert!(!fb.d3d_fallback());
        assert!(!fb.is_srgb_ready());
        assert_eq!(fb.lock_count(), 0);
    }

    #[test]
    fn test_framebuffer_init() {
        let mut fb = D3DHostFrameBuffer::new();
        let ctx = OpenGlContext::new(1);
        let device = ptr::null_mut();

        let result = fb.init(&ctx, device, false, 800, 600);
        assert!(result);
        assert_eq!(fb.width(), 800);
        assert_eq!(fb.height(), 600);
    }

    #[test]
    fn test_framebuffer_init_invalid_size() {
        let mut fb = D3DHostFrameBuffer::new();
        let ctx = OpenGlContext::new(1);
        let device = ptr::null_mut();

        assert!(!fb.init(&ctx, device, false, 0, 600));
        assert!(!fb.init(&ctx, device, false, 800, -1));
    }

    #[test]
    fn test_framebuffer_init_d3d_interop() {
        let mut fb = D3DHostFrameBuffer::new();
        let ctx = OpenGlContext::new(1);
        let device = ptr::null_mut();

        let result = fb.init_d3d_interop(&ctx, device, true, 1024, 768, 24);
        assert!(result);
        assert_eq!(fb.width(), 1024);
        assert_eq!(fb.height(), 768);
        assert_eq!(fb.depth_format(), 24);
        assert!(!fb.d3d_fallback());
    }

    #[test]
    fn test_framebuffer_init_d3d_fallback() {
        let mut fb = D3DHostFrameBuffer::new();
        let ctx = OpenGlContext::new(1);
        let device = ptr::null_mut();

        let result = fb.init_d3d_fallback(&ctx, device, false, 512, 512, 0);
        assert!(result);
        assert!(fb.d3d_fallback());
        assert_eq!(fb.depth_format(), 0);
    }

    #[test]
    fn test_framebuffer_register_d3d_buffer() {
        let mut fb = D3DHostFrameBuffer::new();
        let ctx = OpenGlContext::new(1);

        // Should fail on uninitialized framebuffer
        assert!(!fb.register_d3d_buffer(&ctx));

        // Initialize and try again
        fb.init(&ctx, ptr::null_mut(), false, 256, 256);
        assert!(fb.register_d3d_buffer(&ctx));
    }

    #[test]
    fn test_framebuffer_lock_unlock() {
        let mut fb = D3DHostFrameBuffer::new();
        let ctx = OpenGlContext::new(1);

        assert_eq!(fb.lock_count(), 0);

        fb.lock_surface(&ctx);
        assert_eq!(fb.lock_count(), 1);

        fb.lock_surface(&ctx);
        assert_eq!(fb.lock_count(), 2);

        fb.unlock_surface(&ctx);
        assert_eq!(fb.lock_count(), 1);

        fb.unlock_surface(&ctx);
        assert_eq!(fb.lock_count(), 0);
    }

    #[test]
    fn test_framebuffer_srgb() {
        let mut fb = D3DHostFrameBuffer::new();
        assert!(!fb.is_srgb_ready());

        fb.set_srgb_ready(true);
        assert!(fb.is_srgb_ready());

        fb.set_srgb_ready(false);
        assert!(!fb.is_srgb_ready());
    }

    #[test]
    fn test_framebuffer_release() {
        let mut fb = D3DHostFrameBuffer::new();
        let ctx = OpenGlContext::new(1);

        fb.init(&ctx, ptr::null_mut(), false, 512, 512);
        fb.lock_surface(&ctx);
        assert_eq!(fb.lock_count(), 1);

        fb.release();
        assert_eq!(fb.lock_count(), 0);
        assert_eq!(fb.width(), 512);
    }
}

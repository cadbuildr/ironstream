// FILE: open_gl_workspace.rs
// occt: OpenGl_Workspace

//! Rendering workspace.
//! Provides methods to render primitives and maintain GL state.

use std::collections::HashMap;

/// Frame buffer placeholder
#[derive(Debug, Clone)]
pub struct FrameBuffer {
    id: u32,
    width: i32,
    height: i32,
}

impl FrameBuffer {
    pub fn new(id: u32, width: i32, height: i32) -> Self {
        Self { id, width, height }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}

/// Buffer type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    BackBuffer,
    FrontBuffer,
    DepthBuffer,
    StencilBuffer,
}

/// Polygon offset structure
#[derive(Debug, Clone, Copy)]
pub struct PolygonOffset {
    factor: f32,
    units: f32,
}

impl PolygonOffset {
    pub fn new(factor: f32, units: f32) -> Self {
        Self { factor, units }
    }

    pub fn factor(&self) -> f32 {
        self.factor
    }

    pub fn units(&self) -> f32 {
        self.units
    }
}

impl Default for PolygonOffset {
    fn default() -> Self {
        Self {
            factor: 0.0,
            units: 0.0,
        }
    }
}

/// OpenGL Context placeholder
#[derive(Debug, Clone)]
pub struct GlContext {
    id: u32,
}

impl GlContext {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// OpenGL View placeholder
#[derive(Debug, Clone)]
pub struct GlView {
    id: u32,
}

impl GlView {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// OpenGL Window placeholder
#[derive(Debug, Clone)]
pub struct GlWindow {
    width: i32,
    height: i32,
    gl_context: GlContext,
}

impl GlWindow {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            gl_context: GlContext::new(1),
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn get_gl_context(&self) -> &GlContext {
        &self.gl_context
    }
}

/// Rendering workspace
#[derive(Debug)]
pub struct OpenGlWorkspace {
    view: Option<GlView>,
    window: Option<GlWindow>,
    gl_context: Option<GlContext>,
    use_z_buffer: bool,
    use_depth_write: bool,
    polygon_offset: PolygonOffset,
    allow_face_culling: bool,
    highlight_enabled: bool,
    frame_buffers: HashMap<u32, FrameBuffer>,
    fbo_counter: u32,
}

impl OpenGlWorkspace {
    /// Constructor
    pub fn new(view: Option<GlView>, window: Option<GlWindow>) -> Self {
        let gl_context = window.as_ref().map(|w| w.get_gl_context().clone());

        Self {
            view,
            window,
            gl_context,
            use_z_buffer: true,
            use_depth_write: true,
            polygon_offset: PolygonOffset::default(),
            allow_face_culling: true,
            highlight_enabled: false,
            frame_buffers: HashMap::new(),
            fbo_counter: 0,
        }
    }

    /// Activate rendering context
    pub fn activate(&mut self) -> bool {
        self.gl_context.is_some()
    }

    /// Get view
    pub fn view(&self) -> Option<&GlView> {
        self.view.as_ref()
    }

    /// Get GL context
    pub fn get_gl_context(&self) -> Option<&GlContext> {
        self.gl_context.as_ref()
    }

    /// Create frame buffer
    pub fn fbo_create(&mut self, width: i32, height: i32) -> Option<FrameBuffer> {
        self.fbo_counter += 1;
        let fbo = FrameBuffer::new(self.fbo_counter, width, height);
        self.frame_buffers.insert(self.fbo_counter, fbo.clone());
        Some(fbo)
    }

    /// Release frame buffer
    pub fn fbo_release(&mut self, fbo_id: u32) -> bool {
        self.frame_buffers.remove(&fbo_id).is_some()
    }

    /// Dump frame buffer to image (mock)
    pub fn buffer_dump(&self, fbo_id: u32, buffer_type: BufferType) -> bool {
        // In real implementation: glReadPixels from frame buffer
        self.frame_buffers.contains_key(&fbo_id)
    }

    /// Get width
    pub fn width(&self) -> i32 {
        self.window.as_ref().map(|w| w.width()).unwrap_or(0)
    }

    /// Get height
    pub fn height(&self) -> i32 {
        self.window.as_ref().map(|w| w.height()).unwrap_or(0)
    }

    /// Set Z-buffer usage flag
    pub fn set_use_z_buffer(&mut self, to_use: bool) -> bool {
        let was_used = self.use_z_buffer;
        self.use_z_buffer = to_use;
        was_used
    }

    /// Get Z-buffer usage flag (mutable ref)
    pub fn use_z_buffer_mut(&mut self) -> &mut bool {
        &mut self.use_z_buffer
    }

    /// Get Z-buffer usage flag
    pub fn use_z_buffer(&self) -> bool {
        self.use_z_buffer
    }

    /// Get depth write flag (mutable ref)
    pub fn use_depth_write_mut(&mut self) -> &mut bool {
        &mut self.use_depth_write
    }

    /// Get depth write flag
    pub fn use_depth_write(&self) -> bool {
        self.use_depth_write
    }

    /// Set default polygon offset
    pub fn set_default_polygon_offset(&mut self, offset: PolygonOffset) -> PolygonOffset {
        let prev = self.polygon_offset;
        self.polygon_offset = offset;
        prev
    }

    /// Get polygon offset
    pub fn polygon_offset(&self) -> PolygonOffset {
        self.polygon_offset
    }

    /// Check if face culling is allowed
    pub fn to_allow_face_culling(&self) -> bool {
        self.allow_face_culling
    }

    /// Set face culling allowance
    pub fn set_allow_face_culling(&mut self, allow: bool) -> bool {
        let was_allowed = self.allow_face_culling;
        self.allow_face_culling = allow;
        was_allowed
    }

    /// Check if highlighting is enabled
    pub fn to_highlight(&self) -> bool {
        self.highlight_enabled
    }

    /// Set highlighting
    pub fn set_highlight(&mut self, enabled: bool) {
        self.highlight_enabled = enabled;
    }

    /// Get number of frame buffers
    pub fn fbo_count(&self) -> usize {
        self.frame_buffers.len()
    }
}

impl Default for OpenGlWorkspace {
    fn default() -> Self {
        Self::new(None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let workspace = OpenGlWorkspace::new(None, None);
        assert!(workspace.view().is_none());
        assert!(workspace.get_gl_context().is_none());
    }

    #[test]
    fn test_create_with_window() {
        let window = GlWindow::new(800, 600);
        let workspace = OpenGlWorkspace::new(None, Some(window));

        assert_eq!(workspace.width(), 800);
        assert_eq!(workspace.height(), 600);
        assert!(workspace.get_gl_context().is_some());
    }

    #[test]
    fn test_z_buffer_flag() {
        let mut workspace = OpenGlWorkspace::new(None, None);
        assert!(workspace.use_z_buffer());

        let was_used = workspace.set_use_z_buffer(false);
        assert!(was_used);
        assert!(!workspace.use_z_buffer());
    }

    #[test]
    fn test_depth_write_flag() {
        let mut workspace = OpenGlWorkspace::new(None, None);
        *workspace.use_depth_write_mut() = false;
        assert!(!workspace.use_depth_write());
    }

    #[test]
    fn test_polygon_offset() {
        let mut workspace = OpenGlWorkspace::new(None, None);
        let offset = PolygonOffset::new(1.0, 2.0);
        let prev = workspace.set_default_polygon_offset(offset);

        assert_eq!(workspace.polygon_offset().factor(), 1.0);
        assert_eq!(workspace.polygon_offset().units(), 2.0);
    }

    #[test]
    fn test_face_culling() {
        let mut workspace = OpenGlWorkspace::new(None, None);
        assert!(workspace.to_allow_face_culling());

        let was_allowed = workspace.set_allow_face_culling(false);
        assert!(was_allowed);
        assert!(!workspace.to_allow_face_culling());
    }

    #[test]
    fn test_highlight() {
        let mut workspace = OpenGlWorkspace::new(None, None);
        assert!(!workspace.to_highlight());

        workspace.set_highlight(true);
        assert!(workspace.to_highlight());
    }

    #[test]
    fn test_fbo_create() {
        let mut workspace = OpenGlWorkspace::new(None, None);
        let fbo = workspace.fbo_create(512, 512);

        assert!(fbo.is_some());
        assert_eq!(workspace.fbo_count(), 1);

        if let Some(fbo) = fbo {
            assert_eq!(fbo.width(), 512);
            assert_eq!(fbo.height(), 512);
        }
    }

    #[test]
    fn test_fbo_release() {
        let mut workspace = OpenGlWorkspace::new(None, None);
        let fbo = workspace.fbo_create(512, 512);

        if let Some(fbo) = fbo {
            assert!(workspace.fbo_release(fbo.id()));
            assert_eq!(workspace.fbo_count(), 0);
        }
    }

    #[test]
    fn test_activate() {
        let window = GlWindow::new(640, 480);
        let mut workspace = OpenGlWorkspace::new(None, Some(window));

        assert!(workspace.activate());
    }

    #[test]
    fn test_default() {
        let workspace = OpenGlWorkspace::default();
        assert_eq!(workspace.width(), 0);
        assert_eq!(workspace.height(), 0);
    }
}

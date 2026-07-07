// FILE: open_gl_window.rs
// occt: OpenGl_Window

//! Low-level wrapper over window with GL context.
//! The window itself should be provided to constructor.

/// Platform window reference
#[derive(Debug, Clone)]
pub struct PlatformWindow {
    id: u32,
}

impl PlatformWindow {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// Size window reference
#[derive(Debug, Clone)]
pub struct SizeWindow {
    id: u32,
    width: i32,
    height: i32,
}

impl SizeWindow {
    pub fn new(id: u32, width: i32, height: i32) -> Self {
        Self { id, width, height }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// Rendering context
#[derive(Debug, Clone)]
pub struct RenderingContext {
    id: u32,
}

impl RenderingContext {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// OpenGL capabilities
#[derive(Debug, Clone)]
pub struct GlCaps {
    version_major: i32,
    version_minor: i32,
    vbo_support: bool,
}

impl GlCaps {
    pub fn new() -> Self {
        Self {
            version_major: 2,
            version_minor: 0,
            vbo_support: true,
        }
    }

    pub fn version(&self) -> (i32, i32) {
        (self.version_major, self.version_minor)
    }

    pub fn has_vbo_support(&self) -> bool {
        self.vbo_support
    }
}

impl Default for GlCaps {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenGL Context placeholder
#[derive(Debug, Clone)]
pub struct GlContext {
    id: u32,
    active: bool,
}

impl GlContext {
    pub fn new(id: u32) -> Self {
        Self { id, active: false }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

impl Default for GlContext {
    fn default() -> Self {
        Self::new(0)
    }
}

/// OpenGL Window
#[derive(Debug)]
pub struct OpenGlWindow {
    platform_window: Option<PlatformWindow>,
    size_window: Option<SizeWindow>,
    gl_context: Option<GlContext>,
    width: i32,
    height: i32,
    initialized: bool,
}

impl OpenGlWindow {
    /// Empty constructor
    pub fn new() -> Self {
        Self {
            platform_window: None,
            size_window: None,
            gl_context: None,
            width: 0,
            height: 0,
            initialized: false,
        }
    }

    /// Initialize the new window
    pub fn init(
        &mut self,
        platform_window: PlatformWindow,
        size_window: SizeWindow,
        _context: RenderingContext,
        _caps: &GlCaps,
        _share_ctx: Option<&GlContext>,
    ) -> bool {
        self.platform_window = Some(platform_window);
        self.size_window = Some(size_window.clone());
        self.width = size_window.width();
        self.height = size_window.height();
        self.gl_context = Some(GlContext::new(1));
        self.initialized = true;
        true
    }

    /// Resize the window
    pub fn resize(&mut self, width: i32, height: i32) {
        self.width = width.max(1);
        self.height = height.max(1);
    }

    /// Return platform window
    pub fn platform_window(&self) -> Option<&PlatformWindow> {
        self.platform_window.as_ref()
    }

    /// Return size window
    pub fn size_window(&self) -> Option<&SizeWindow> {
        self.size_window.as_ref()
    }

    /// Get width
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Get height
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Return OpenGL context
    pub fn get_gl_context(&self) -> Option<&GlContext> {
        self.gl_context.as_ref()
    }

    /// Makes GL context for this window active in current thread
    pub fn activate(&mut self) -> bool {
        if let Some(ctx) = &mut self.gl_context {
            ctx.set_active(true);
            true
        } else {
            false
        }
    }

    /// Deactivate GL context
    pub fn deactivate(&mut self) -> bool {
        if let Some(ctx) = &mut self.gl_context {
            ctx.set_active(false);
            true
        } else {
            false
        }
    }

    /// Check if window is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Release resources
    pub fn release(&mut self) {
        self.platform_window = None;
        self.size_window = None;
        self.gl_context = None;
        self.initialized = false;
    }

    /// Get window size
    pub fn size(&self) -> (i32, i32) {
        (self.width, self.height)
    }
}

impl Default for OpenGlWindow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let window = OpenGlWindow::new();
        assert!(!window.is_initialized());
        assert_eq!(window.width(), 0);
        assert_eq!(window.height(), 0);
    }

    #[test]
    fn test_init() {
        let mut window = OpenGlWindow::new();
        let platform = PlatformWindow::new(1);
        let size = SizeWindow::new(2, 800, 600);
        let ctx = RenderingContext::new(1);
        let caps = GlCaps::new();

        assert!(window.init(platform, size, ctx, &caps, None));
        assert!(window.is_initialized());
        assert_eq!(window.width(), 800);
        assert_eq!(window.height(), 600);
    }

    #[test]
    fn test_resize() {
        let mut window = OpenGlWindow::new();
        window.resize(1024, 768);

        assert_eq!(window.width(), 1024);
        assert_eq!(window.height(), 768);
    }

    #[test]
    fn test_resize_invalid() {
        let mut window = OpenGlWindow::new();
        window.resize(0, -1);

        assert_eq!(window.width(), 1);
        assert_eq!(window.height(), 1);
    }

    #[test]
    fn test_activate_deactivate() {
        let mut window = OpenGlWindow::new();
        let platform = PlatformWindow::new(1);
        let size = SizeWindow::new(2, 640, 480);
        let ctx = RenderingContext::new(1);
        let caps = GlCaps::new();

        window.init(platform, size, ctx, &caps, None);

        assert!(window.activate());
        assert!(window.get_gl_context().unwrap().is_active());

        assert!(window.deactivate());
        assert!(!window.get_gl_context().unwrap().is_active());
    }

    #[test]
    fn test_platform_window() {
        let mut window = OpenGlWindow::new();
        let platform = PlatformWindow::new(42);
        let size = SizeWindow::new(2, 640, 480);
        let ctx = RenderingContext::new(1);
        let caps = GlCaps::new();

        window.init(platform, size, ctx, &caps, None);

        assert!(window.platform_window().is_some());
        assert_eq!(window.platform_window().unwrap().id(), 42);
    }

    #[test]
    fn test_release() {
        let mut window = OpenGlWindow::new();
        let platform = PlatformWindow::new(1);
        let size = SizeWindow::new(2, 640, 480);
        let ctx = RenderingContext::new(1);
        let caps = GlCaps::new();

        window.init(platform, size, ctx, &caps, None);
        assert!(window.is_initialized());

        window.release();
        assert!(!window.is_initialized());
    }

    #[test]
    fn test_gl_caps() {
        let caps = GlCaps::new();
        let (maj, min) = caps.version();
        assert_eq!(maj, 2);
        assert_eq!(min, 0);
        assert!(caps.has_vbo_support());
    }

    #[test]
    fn test_default() {
        let window = OpenGlWindow::default();
        assert!(!window.is_initialized());
    }
}

// FILE: cocoa_window.rs
// occt: Cocoa_Window

/// Cocoa window abstraction for macOS/iOS.
///
/// This class wraps either NSWindow/NSView (macOS) or UIView/UIWindow (iOS)
/// and provides a Rust-friendly interface for window management.
///
/// On macOS, it can either create a new native window or wrap an existing NSView.
/// On iOS, it wraps an existing UIView.
pub struct CocoaWindow {
    /// Native window handle (void pointer in Rust, NSWindow* on macOS)
    window_ptr: *mut std::ffi::c_void,
    /// Native view handle (NSView* on macOS, UIView* on iOS)
    view_ptr: *mut std::ffi::c_void,
    /// Window position and size tracking
    x_left: i32,
    y_top: i32,
    x_right: i32,
    y_bottom: i32,
    /// Title of the window
    title: String,
    /// Whether window is currently mapped (visible)
    is_mapped: bool,
}

impl CocoaWindow {
    /// Create a new Cocoa window with the specified title and dimensions in pixels.
    ///
    /// # Arguments
    /// * `title` - Window title
    /// * `px_left` - Left edge position in pixels
    /// * `px_top` - Top edge position in pixels
    /// * `px_width` - Window width in pixels
    /// * `px_height` - Window height in pixels
    pub fn new(title: &str, px_left: i32, px_top: i32, px_width: i32, px_height: i32) -> Self {
        Self {
            window_ptr: std::ptr::null_mut(),
            view_ptr: std::ptr::null_mut(),
            x_left: px_left,
            y_top: px_top,
            x_right: px_left + px_width,
            y_bottom: px_top + px_height,
            title: title.to_string(),
            is_mapped: false,
        }
    }

    /// Get the window title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set the window title.
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    /// Get the window position and size.
    ///
    /// Returns (x1, y1, x2, y2) where (x1, y1) is top-left and (x2, y2) is bottom-right.
    pub fn position(&self) -> (i32, i32, i32, i32) {
        (self.x_left, self.y_top, self.x_right, self.y_bottom)
    }

    /// Get the window size in pixels.
    ///
    /// Returns (width, height).
    pub fn size(&self) -> (i32, i32) {
        let width = self.x_right - self.x_left;
        let height = self.y_bottom - self.y_top;
        (width, height)
    }

    /// Get the window aspect ratio (width / height).
    pub fn ratio(&self) -> f64 {
        let (width, height) = self.size();
        if height != 0 {
            width as f64 / height as f64
        } else {
            1.0
        }
    }

    /// Map (show) the window.
    pub fn map(&mut self) {
        self.is_mapped = true;
    }

    /// Unmap (hide) the window.
    pub fn unmap(&mut self) {
        self.is_mapped = false;
    }

    /// Check if the window is mapped (visible).
    pub fn is_mapped(&self) -> bool {
        self.is_mapped
    }

    /// Get the native window handle.
    pub fn native_handle(&self) -> *mut std::ffi::c_void {
        self.view_ptr
    }

    /// Get the native parent window handle.
    pub fn native_parent_handle(&self) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }

    /// Get native frame buffer configuration (macOS specific).
    pub fn native_fb_config(&self) -> *const std::ffi::c_void {
        std::ptr::null()
    }

    /// Invalidate the window content (request redraw).
    pub fn invalidate_content(&self) {
        // TODO: Call setNeedsDisplay on the underlying NSView
    }
}

impl Default for CocoaWindow {
    fn default() -> Self {
        Self::new("Cocoa Window", 0, 0, 800, 600)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cocoa_window_creation() {
        let window = CocoaWindow::new("Test Window", 100, 200, 800, 600);
        assert_eq!(window.title(), "Test Window");
        assert!(!window.is_mapped());
    }

    #[test]
    fn test_cocoa_window_position() {
        let window = CocoaWindow::new("Test", 100, 200, 800, 600);
        let (x1, y1, x2, y2) = window.position();
        assert_eq!(x1, 100);
        assert_eq!(y1, 200);
        assert_eq!(x2, 900);
        assert_eq!(y2, 800);
    }

    #[test]
    fn test_cocoa_window_size() {
        let window = CocoaWindow::new("Test", 100, 200, 800, 600);
        let (width, height) = window.size();
        assert_eq!(width, 800);
        assert_eq!(height, 600);
    }

    #[test]
    fn test_cocoa_window_ratio() {
        let window = CocoaWindow::new("Test", 0, 0, 1600, 800);
        let ratio = window.ratio();
        assert!((ratio - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_cocoa_window_ratio_square() {
        let window = CocoaWindow::new("Test", 0, 0, 800, 800);
        let ratio = window.ratio();
        assert!((ratio - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_cocoa_window_set_title() {
        let mut window = CocoaWindow::new("Original", 0, 0, 800, 600);
        window.set_title("New Title");
        assert_eq!(window.title(), "New Title");
    }

    #[test]
    fn test_cocoa_window_mapping() {
        let mut window = CocoaWindow::new("Test", 0, 0, 800, 600);
        assert!(!window.is_mapped());
        window.map();
        assert!(window.is_mapped());
        window.unmap();
        assert!(!window.is_mapped());
    }

    #[test]
    fn test_cocoa_window_default() {
        let window = CocoaWindow::default();
        assert_eq!(window.title(), "Cocoa Window");
        let (width, height) = window.size();
        assert_eq!(width, 800);
        assert_eq!(height, 600);
    }

    #[test]
    fn test_cocoa_window_native_handles() {
        let window = CocoaWindow::new("Test", 0, 0, 800, 600);
        // Handles should be null by default (not initialized)
        assert!(window.native_parent_handle().is_null());
        assert!(window.native_fb_config().is_null());
    }
}

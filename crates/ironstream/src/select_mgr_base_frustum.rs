// FILE: select_mgr_base_frustum.rs
// occt: SelectMgr_BaseFrustum

/// Represents a base selecting frustum for different selection types
/// (point, box, polyline selection).
pub struct SelectMgrBaseFrustum {
    pixel_tolerance: i32,
    window_width: i32,
    window_height: i32,
    viewport_x: f64,
    viewport_y: f64,
    viewport_width: f64,
    viewport_height: f64,
}

impl SelectMgrBaseFrustum {
    /// Creates a new selecting volume with pixel tolerance set to 2.
    pub fn new() -> Self {
        SelectMgrBaseFrustum {
            pixel_tolerance: 2,
            window_width: 0,
            window_height: 0,
            viewport_x: 0.0,
            viewport_y: 0.0,
            viewport_width: 0.0,
            viewport_height: 0.0,
        }
    }

    /// Sets the pixel tolerance.
    pub fn set_pixel_tolerance(&mut self, tol: i32) {
        self.pixel_tolerance = tol;
    }

    /// Gets the pixel tolerance.
    pub fn pixel_tolerance(&self) -> i32 {
        self.pixel_tolerance
    }

    /// Sets the window size.
    pub fn set_window_size(&mut self, width: i32, height: i32) {
        self.window_width = width;
        self.window_height = height;
    }

    /// Gets the window size.
    pub fn window_size(&self) -> (i32, i32) {
        (self.window_width, self.window_height)
    }

    /// Sets the viewport parameters.
    pub fn set_viewport(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.viewport_x = x;
        self.viewport_y = y;
        self.viewport_width = width;
        self.viewport_height = height;
    }

    /// Gets the viewport parameters.
    pub fn viewport(&self) -> (f64, f64, f64, f64) {
        (
            self.viewport_x,
            self.viewport_y,
            self.viewport_width,
            self.viewport_height,
        )
    }
}

impl Default for SelectMgrBaseFrustum {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_frustum_creation() {
        let frustum = SelectMgrBaseFrustum::new();
        assert_eq!(frustum.pixel_tolerance(), 2);
        let (w, h) = frustum.window_size();
        assert_eq!(w, 0);
        assert_eq!(h, 0);
    }

    #[test]
    fn test_set_pixel_tolerance() {
        let mut frustum = SelectMgrBaseFrustum::new();
        assert_eq!(frustum.pixel_tolerance(), 2);

        frustum.set_pixel_tolerance(5);
        assert_eq!(frustum.pixel_tolerance(), 5);
    }

    #[test]
    fn test_set_window_size() {
        let mut frustum = SelectMgrBaseFrustum::new();
        frustum.set_window_size(1024, 768);

        let (width, height) = frustum.window_size();
        assert_eq!(width, 1024);
        assert_eq!(height, 768);
    }

    #[test]
    fn test_set_viewport() {
        let mut frustum = SelectMgrBaseFrustum::new();
        frustum.set_viewport(10.0, 20.0, 800.0, 600.0);

        let (x, y, w, h) = frustum.viewport();
        assert_eq!(x, 10.0);
        assert_eq!(y, 20.0);
        assert_eq!(w, 800.0);
        assert_eq!(h, 600.0);
    }

    #[test]
    fn test_default_frustum() {
        let frustum = SelectMgrBaseFrustum::default();
        assert_eq!(frustum.pixel_tolerance(), 2);
    }

    #[test]
    fn test_frustum_state_persistence() {
        let mut frustum = SelectMgrBaseFrustum::new();
        frustum.set_pixel_tolerance(3);
        frustum.set_window_size(512, 512);
        frustum.set_viewport(5.0, 5.0, 400.0, 400.0);

        assert_eq!(frustum.pixel_tolerance(), 3);
        let (w, h) = frustum.window_size();
        assert_eq!(w, 512);
        assert_eq!(h, 512);
        let (x, y, vw, vh) = frustum.viewport();
        assert_eq!(x, 5.0);
        assert_eq!(y, 5.0);
        assert_eq!(vw, 400.0);
        assert_eq!(vh, 400.0);
    }
}

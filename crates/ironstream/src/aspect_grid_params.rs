// FILE: aspect_grid_params.rs
// occt: Aspect_GridParams

/// Grid appearance parameters: color, scale, bounds, arc, draw mode, background and adaptive flags.
pub struct AspectGridParams {
    color: (f32, f32, f32),                 // RGB
    accent_color: (f32, f32, f32),          // RGB
    origin: (f64, f64, f64),                // x, y, z
    scale: f64,
    scale_y: f64,
    accent_scale_x: f64,
    accent_scale_y: f64,
    accent_angular_scale: f64,
    line_thickness: f64,
    rotation_angle: f64,
    size_x: f64,
    size_y: f64,
    radius: f64,
    z_offset: f64,
    angle_start: f64,
    angle_end: f64,
    angular_divisions: i32,
    draw_mode: GridDrawMode,
    is_background: bool,
    is_draw_axis: bool,
    is_view_adaptive: bool,
}

/// Grid draw mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridDrawMode {
    Lines = 0,
    Points = 1,
    None = 2,
}

impl Default for AspectGridParams {
    fn default() -> Self {
        AspectGridParams {
            color: (0.7, 0.7, 0.7),
            accent_color: (0.5, 0.5, 0.5),
            origin: (0.0, 0.0, 0.0),
            scale: 0.01,
            scale_y: 0.0,
            accent_scale_x: 0.0,
            accent_scale_y: 0.0,
            accent_angular_scale: 0.0,
            line_thickness: 0.01,
            rotation_angle: 0.0,
            size_x: 0.0,
            size_y: 0.0,
            radius: 0.0,
            z_offset: 0.0,
            angle_start: 0.0,
            angle_end: 0.0,
            angular_divisions: 0,
            draw_mode: GridDrawMode::Lines,
            is_background: false,
            is_draw_axis: true,
            is_view_adaptive: false,
        }
    }
}

impl AspectGridParams {
    /// Create with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get grid color.
    pub fn color(&self) -> (f32, f32, f32) {
        self.color
    }

    /// Set grid color.
    pub fn set_color(&mut self, color: (f32, f32, f32)) {
        self.color = color;
    }

    /// Get grid origin.
    pub fn origin(&self) -> (f64, f64, f64) {
        self.origin
    }

    /// Set grid origin.
    pub fn set_origin(&mut self, origin: (f64, f64, f64)) {
        self.origin = origin;
    }

    /// Get scale.
    pub fn scale(&self) -> f64 {
        self.scale
    }

    /// Set scale (must be non-negative).
    pub fn set_scale(&mut self, scale: f64) {
        assert!(scale >= 0.0, "scale must be non-negative");
        self.scale = scale;
    }

    /// Get Y scale.
    pub fn scale_y(&self) -> f64 {
        self.scale_y
    }

    /// Set Y scale.
    pub fn set_scale_y(&mut self, scale_y: f64) {
        assert!(scale_y >= 0.0, "scale_y must be non-negative");
        self.scale_y = scale_y;
    }

    /// Get effective Y scale.
    pub fn effective_scale_y(&self) -> f64 {
        if self.scale_y > 0.0 {
            self.scale_y
        } else {
            self.scale
        }
    }

    /// Get angular divisions.
    pub fn angular_divisions(&self) -> i32 {
        self.angular_divisions
    }

    /// Set angular divisions.
    pub fn set_angular_divisions(&mut self, divisions: i32) {
        self.angular_divisions = divisions;
    }

    /// Is circular grid.
    pub fn is_circular(&self) -> bool {
        self.angular_divisions > 0
    }

    /// Get radius.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Set radius.
    pub fn set_radius(&mut self, radius: f64) {
        assert!(radius >= 0.0, "radius must be non-negative");
        self.radius = radius;
    }

    /// Get draw mode.
    pub fn draw_mode(&self) -> GridDrawMode {
        self.draw_mode
    }

    /// Set draw mode.
    pub fn set_draw_mode(&mut self, mode: GridDrawMode) {
        self.draw_mode = mode;
    }

    /// Get Z offset.
    pub fn z_offset(&self) -> f64 {
        self.z_offset
    }

    /// Set Z offset.
    pub fn set_z_offset(&mut self, offset: f64) {
        self.z_offset = offset;
    }

    /// Is bounded.
    pub fn is_bounded(&self) -> bool {
        self.size_x > 0.0 || self.size_y > 0.0 || self.radius > 0.0
    }

    /// Is arc.
    pub fn is_arc(&self) -> bool {
        (self.angle_start - self.angle_end).abs() > 1e-10
    }

    /// Set arc range.
    pub fn set_arc_range(&mut self, start: f64, end: f64) {
        self.angle_start = start;
        self.angle_end = end;
    }

    /// Get view adaptive flag.
    pub fn is_view_adaptive(&self) -> bool {
        self.is_view_adaptive
    }

    /// Set view adaptive.
    pub fn set_is_view_adaptive(&mut self, adaptive: bool) {
        self.is_view_adaptive = adaptive;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let params = AspectGridParams::new();
        assert_eq!(params.scale(), 0.01);
        assert!(params.is_draw_axis);
        assert_eq!(params.draw_mode(), GridDrawMode::Lines);
    }

    #[test]
    fn test_set_scale() {
        let mut params = AspectGridParams::new();
        params.set_scale(0.05);
        assert_eq!(params.scale(), 0.05);
    }

    #[test]
    fn test_effective_scale_y() {
        let mut params = AspectGridParams::new();
        params.set_scale(0.1);
        assert_eq!(params.effective_scale_y(), 0.1);
        params.set_scale_y(0.2);
        assert_eq!(params.effective_scale_y(), 0.2);
    }

    #[test]
    fn test_is_circular() {
        let mut params = AspectGridParams::new();
        assert!(!params.is_circular());
        params.set_angular_divisions(12);
        assert!(params.is_circular());
    }

    #[test]
    fn test_is_bounded() {
        let mut params = AspectGridParams::new();
        assert!(!params.is_bounded());
        params.set_radius(1.0);
        assert!(params.is_bounded());
    }

    #[test]
    #[should_panic]
    fn test_negative_scale_panics() {
        let mut params = AspectGridParams::new();
        params.set_scale(-0.1);
    }
}

// FILE: graphic3d_presentation_attributes.rs
// occt: Graphic3d_PresentationAttributes

/// Highlight method types.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightMethod {
    /// Color highlighting (default)
    Color = 0,
    /// Box highlighting
    Box = 1,
}

/// Color representation (RGB).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Color { red, green, blue }
    }

    /// White color (1.0, 1.0, 1.0)
    pub fn white() -> Self {
        Color {
            red: 1.0f32,
            green: 1.0f32,
            blue: 1.0f32,
        }
    }
}

/// Color representation with alpha channel (RGBA).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorRgba {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl ColorRgba {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        ColorRgba {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// White color with full opacity (1.0, 1.0, 1.0, 1.0)
    pub fn white() -> Self {
        ColorRgba {
            red: 1.0f32,
            green: 1.0f32,
            blue: 1.0f32,
            alpha: 1.0f32,
        }
    }

    /// Returns the RGB components (ignores alpha).
    pub fn get_rgb(&self) -> Color {
        Color {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }

    /// Changes RGB components, preserving alpha.
    pub fn set_rgb(&mut self, color: &Color) {
        self.red = color.red;
        self.green = color.green;
        self.blue = color.blue;
    }

    /// Returns the alpha component.
    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    /// Sets the alpha component.
    pub fn set_alpha(&mut self, alpha: f32) {
        self.alpha = alpha;
    }
}

/// Z-layer identifier type.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZLayerId {
    /// Unknown/undefined layer
    Unknown = -1,
    /// Default layer
    Default = 0,
}

impl ZLayerId {
    pub fn as_i32(self) -> i32 {
        self as i32
    }

    pub fn from_i32(value: i32) -> Self {
        match value {
            -1 => ZLayerId::Unknown,
            _ => ZLayerId::Default,
        }
    }
}

/// Presentation attributes.
#[derive(Debug, Clone)]
pub struct PresentationAttributes {
    basic_color: ColorRgba,
    hi_method: HighlightMethod,
    z_layer: ZLayerId,
    disp_mode: i32,
    // basic_fill_area_aspect: Option<Arc<AspectFillArea3d>>, // stub for now
}

impl PresentationAttributes {
    /// Creates a new presentation attributes with default values.
    /// Default: white color, color highlighting method, default z-layer, display mode 0.
    pub fn new() -> Self {
        PresentationAttributes {
            basic_color: ColorRgba::white(),
            hi_method: HighlightMethod::Color,
            z_layer: ZLayerId::Default,
            disp_mode: 0,
        }
    }

    /// Returns the highlight method (Aspect_TOHM_COLOR by default).
    pub fn method(&self) -> HighlightMethod {
        self.hi_method
    }

    /// Sets the highlight method.
    pub fn set_method(&mut self, method: HighlightMethod) {
        self.hi_method = method;
    }

    /// Returns the basic presentation color including alpha channel.
    pub fn color_rgba(&self) -> ColorRgba {
        self.basic_color
    }

    /// Returns the basic presentation color (RGB only, Quantity_NOC_WHITE by default).
    pub fn color(&self) -> Color {
        self.basic_color.get_rgb()
    }

    /// Sets the basic presentation color (RGB components only, does not modify transparency).
    pub fn set_color(&mut self, color: &Color) {
        self.basic_color.set_rgb(color);
    }

    /// Returns the basic presentation transparency.
    /// 0 = opaque, 1 = fully transparent. Default is 0 (opaque).
    pub fn transparency(&self) -> f32 {
        1.0f32 - self.basic_color.alpha()
    }

    /// Sets the basic presentation transparency (0 = opaque, 1 = fully transparent).
    pub fn set_transparency(&mut self, transparency: f32) {
        self.basic_color.set_alpha(1.0f32 - transparency);
    }

    /// Returns the presentation Z-layer (Graphic3d_ZLayerId_Default by default).
    /// ZLayerId::Unknown means undefined (main presentation layer to be used).
    pub fn z_layer(&self) -> ZLayerId {
        self.z_layer
    }

    /// Sets the presentation Z-layer.
    pub fn set_z_layer(&mut self, layer: ZLayerId) {
        self.z_layer = layer;
    }

    /// Returns the display mode (0 by default).
    /// -1 means undefined (main display mode of presentation to be used).
    pub fn display_mode(&self) -> i32 {
        self.disp_mode
    }

    /// Sets the display mode.
    pub fn set_display_mode(&mut self, mode: i32) {
        self.disp_mode = mode;
    }
}

impl Default for PresentationAttributes {
    fn default() -> Self {
        PresentationAttributes::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::new(0.5f32, 0.6f32, 0.7f32);
        assert_eq!(color.red, 0.5f32);
        assert_eq!(color.green, 0.6f32);
        assert_eq!(color.blue, 0.7f32);
    }

    #[test]
    fn test_color_white() {
        let color = Color::white();
        assert_eq!(color.red, 1.0f32);
        assert_eq!(color.green, 1.0f32);
        assert_eq!(color.blue, 1.0f32);
    }

    #[test]
    fn test_color_rgba_creation() {
        let color = ColorRgba::new(0.5f32, 0.6f32, 0.7f32, 0.8f32);
        assert_eq!(color.red, 0.5f32);
        assert_eq!(color.green, 0.6f32);
        assert_eq!(color.blue, 0.7f32);
        assert_eq!(color.alpha, 0.8f32);
    }

    #[test]
    fn test_color_rgba_white() {
        let color = ColorRgba::white();
        assert_eq!(color.alpha(), 1.0f32);
    }

    #[test]
    fn test_color_rgba_get_rgb() {
        let rgba = ColorRgba::new(0.5f32, 0.6f32, 0.7f32, 0.8f32);
        let rgb = rgba.get_rgb();
        assert_eq!(rgb.red, 0.5f32);
        assert_eq!(rgb.green, 0.6f32);
        assert_eq!(rgb.blue, 0.7f32);
    }

    #[test]
    fn test_color_rgba_set_rgb() {
        let mut rgba = ColorRgba::white();
        let new_color = Color::new(0.1f32, 0.2f32, 0.3f32);
        rgba.set_rgb(&new_color);
        assert_eq!(rgba.red, 0.1f32);
        assert_eq!(rgba.green, 0.2f32);
        assert_eq!(rgba.blue, 0.3f32);
        assert_eq!(rgba.alpha, 1.0f32); // alpha unchanged
    }

    #[test]
    fn test_color_rgba_set_alpha() {
        let mut rgba = ColorRgba::white();
        rgba.set_alpha(0.5f32);
        assert_eq!(rgba.alpha, 0.5f32);
    }

    #[test]
    fn test_z_layer_id_values() {
        assert_eq!(ZLayerId::Unknown.as_i32(), -1);
        assert_eq!(ZLayerId::Default.as_i32(), 0);
    }

    #[test]
    fn test_z_layer_id_from_i32() {
        assert_eq!(ZLayerId::from_i32(-1), ZLayerId::Unknown);
        assert_eq!(ZLayerId::from_i32(0), ZLayerId::Default);
        assert_eq!(ZLayerId::from_i32(5), ZLayerId::Default); // unknown values map to default
    }

    #[test]
    fn test_presentation_attributes_default() {
        let attrs = PresentationAttributes::new();
        assert_eq!(attrs.method(), HighlightMethod::Color);
        assert_eq!(attrs.transparency(), 0.0f32); // opaque
        assert_eq!(attrs.z_layer(), ZLayerId::Default);
        assert_eq!(attrs.display_mode(), 0);
    }

    #[test]
    fn test_presentation_attributes_color() {
        let mut attrs = PresentationAttributes::new();
        let new_color = Color::new(0.5f32, 0.5f32, 0.5f32);
        attrs.set_color(&new_color);
        let retrieved = attrs.color();
        assert_eq!(retrieved.red, 0.5f32);
        assert_eq!(retrieved.green, 0.5f32);
        assert_eq!(retrieved.blue, 0.5f32);
    }

    #[test]
    fn test_presentation_attributes_transparency() {
        let mut attrs = PresentationAttributes::new();
        attrs.set_transparency(0.5f32);
        assert_eq!(attrs.transparency(), 0.5f32);
        // alpha should be 1.0 - 0.5 = 0.5
        assert_eq!(attrs.color_rgba().alpha(), 0.5f32);
    }

    #[test]
    fn test_presentation_attributes_method() {
        let mut attrs = PresentationAttributes::new();
        attrs.set_method(HighlightMethod::Box);
        assert_eq!(attrs.method(), HighlightMethod::Box);
    }

    #[test]
    fn test_presentation_attributes_z_layer() {
        let mut attrs = PresentationAttributes::new();
        attrs.set_z_layer(ZLayerId::Unknown);
        assert_eq!(attrs.z_layer(), ZLayerId::Unknown);
    }

    #[test]
    fn test_presentation_attributes_display_mode() {
        let mut attrs = PresentationAttributes::new();
        attrs.set_display_mode(5);
        assert_eq!(attrs.display_mode(), 5);
    }

    #[test]
    fn test_presentation_attributes_default_trait() {
        let attrs = PresentationAttributes::default();
        assert_eq!(attrs.method(), HighlightMethod::Color);
        assert_eq!(attrs.display_mode(), 0);
    }
}

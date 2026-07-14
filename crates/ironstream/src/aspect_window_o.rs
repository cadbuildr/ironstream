// FILE: aspect_window_o.rs
// occt-ref: Aspect_Window

use std::sync::Arc;

/// Local model of Aspect_Background: a solid background color (RGB in [0,1]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AspectBackground {
  pub color: (f64, f64, f64),
}

impl AspectBackground {
  pub fn new(color: (f64, f64, f64)) -> Self {
    AspectBackground { color }
  }
}

impl Default for AspectBackground {
  fn default() -> Self {
    AspectBackground { color: (0.0, 0.0, 0.0) }
  }
}

/// Local model of Aspect_GradientBackground: two colors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AspectGradientBackground {
  pub color1: (f64, f64, f64),
  pub color2: (f64, f64, f64),
}

impl AspectGradientBackground {
  pub fn new(color1: (f64, f64, f64), color2: (f64, f64, f64)) -> Self {
    AspectGradientBackground { color1, color2 }
  }
}

impl Default for AspectGradientBackground {
  fn default() -> Self {
    AspectGradientBackground { color1: (0.0, 0.0, 0.0), color2: (0.0, 0.0, 0.0) }
  }
}

/// Placeholder for Aspect_FillMethod (stub for now)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectFillMethod {
  Centered = 0,
  Tiled = 1,
  Stretched = 2,
}

/// Simple 2D vector type for position and dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2Int {
  pub x: i32,
  pub y: i32,
}

impl Vec2Int {
  pub fn new(x: i32, y: i32) -> Self {
    Vec2Int { x, y }
  }
}

/// Placeholder for Aspect_DisplayConnection
pub type AspectDisplayConnection = ();

/// Placeholder for Aspect_Drawable
pub type AspectDrawable = usize;

/// Placeholder for Aspect_FBConfig
pub type AspectFBConfig = usize;

/// Placeholder for Aspect_TypeOfResize
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectTypeOfResize {
  NotResized = 0,
  Resized = 1,
}

/// Defines a window.
/// This is an abstract base class representing a window in the display system.
pub struct AspectWindow {
  my_display: Option<Arc<AspectDisplayConnection>>,
  my_background: AspectBackground,
  my_gradient_background: AspectGradientBackground,
  my_background_fill_method: AspectFillMethod,
  my_is_virtual: bool,
}

impl AspectWindow {
  /// Creates a new window with default initialization.
  pub fn new() -> Self {
    AspectWindow {
      my_display: None,
      my_background: AspectBackground::default(),
      my_gradient_background: AspectGradientBackground::default(),
      my_background_fill_method: AspectFillMethod::Centered,
      my_is_virtual: false,
    }
  }

  /// Returns true if the window is virtual.
  pub fn is_virtual(&self) -> bool {
    self.my_is_virtual
  }

  /// Sets the virtual state of the window.
  pub fn set_virtual(&mut self, is_virtual: bool) {
    self.my_is_virtual = is_virtual;
  }

  /// Returns the window's top-left corner position.
  pub fn top_left(&self) -> Vec2Int {
    // This would call Position() on a concrete implementation
    // For the base abstract class, we return a default
    Vec2Int::new(0, 0)
  }

  /// Returns the window's dimensions.
  pub fn dimensions(&self) -> Vec2Int {
    // This would call Size() on a concrete implementation
    // For the base abstract class, we return a default
    Vec2Int::new(0, 0)
  }

  /// Returns the display connection, or None.
  pub fn display_connection(&self) -> &Option<Arc<AspectDisplayConnection>> {
    &self.my_display
  }

  /// Returns the window's background.
  pub fn background(&self) -> &AspectBackground {
    &self.my_background
  }

  /// Returns the current background fill method.
  pub fn background_fill_method(&self) -> AspectFillMethod {
    self.my_background_fill_method
  }

  /// Returns the window's gradient background.
  pub fn gradient_background(&self) -> &AspectGradientBackground {
    &self.my_gradient_background
  }

  /// Sets the window's background.
  pub fn set_background(&mut self, background: AspectBackground) {
    self.my_background = background;
  }

  /// Sets the window's gradient background.
  pub fn set_gradient_background(&mut self, background: AspectGradientBackground) {
    self.my_gradient_background = background;
  }

  /// Sets the background fill method.
  pub fn set_background_fill_method(&mut self, method: AspectFillMethod) {
    self.my_background_fill_method = method;
  }

  /// Returns the device pixel ratio (logical to backing store scale factor).
  /// Default is 1.0.
  pub fn device_pixel_ratio(&self) -> f64 {
    1.0
  }

  /// Converts a point from logical units into backing store units.
  pub fn convert_point_to_backing(&self, point: (f64, f64)) -> (f64, f64) {
    let ratio = self.device_pixel_ratio();
    (point.0 * ratio, point.1 * ratio)
  }

  /// Converts a point from backing store units to logical units.
  pub fn convert_point_from_backing(&self, point: (f64, f64)) -> (f64, f64) {
    let ratio = self.device_pixel_ratio();
    (point.0 / ratio, point.1 / ratio)
  }

  // Abstract methods (would be implemented by concrete subclasses):
  // is_mapped() -> bool
  // map()
  // unmap()
  // do_resize() -> AspectTypeOfResize
  // do_mapping() -> bool
  // ratio() -> f64
  // position(x1, y1, x2, y2)
  // size(width, height)
  // native_handle() -> AspectDrawable
  // native_parent_handle() -> AspectDrawable
  // native_fb_config() -> AspectFBConfig
}

impl Default for AspectWindow {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_window_creation() {
    let window = AspectWindow::new();
    assert!(!window.is_virtual());
  }

  #[test]
  fn test_set_virtual() {
    let mut window = AspectWindow::new();
    assert!(!window.is_virtual());
    window.set_virtual(true);
    assert!(window.is_virtual());
    window.set_virtual(false);
    assert!(!window.is_virtual());
  }

  #[test]
  fn test_device_pixel_ratio() {
    let window = AspectWindow::new();
    assert_eq!(window.device_pixel_ratio(), 1.0);
  }

  #[test]
  fn test_convert_point_to_backing() {
    let window = AspectWindow::new();
    let point = (100.0, 200.0);
    let converted = window.convert_point_to_backing(point);
    // With default ratio of 1.0, should be unchanged
    assert_eq!(converted, (100.0, 200.0));
  }

  #[test]
  fn test_convert_point_from_backing() {
    let window = AspectWindow::new();
    let point = (100.0, 200.0);
    let converted = window.convert_point_from_backing(point);
    // With default ratio of 1.0, should be unchanged
    assert_eq!(converted, (100.0, 200.0));
  }

  #[test]
  fn test_round_trip_conversion() {
    let window = AspectWindow::new();
    let original = (150.5, 250.5);
    let to_backing = window.convert_point_to_backing(original);
    let back = window.convert_point_from_backing(to_backing);
    assert_eq!(original, back);
  }

  #[test]
  fn test_top_left_default() {
    let window = AspectWindow::new();
    let tl = window.top_left();
    assert_eq!(tl.x, 0);
    assert_eq!(tl.y, 0);
  }

  #[test]
  fn test_dimensions_default() {
    let window = AspectWindow::new();
    let dims = window.dimensions();
    assert_eq!(dims.x, 0);
    assert_eq!(dims.y, 0);
  }

  #[test]
  fn test_background_operations() {
    let mut window = AspectWindow::new();
    assert_eq!(*window.background(), AspectBackground::default());
    let bg = AspectBackground::new((0.1, 0.2, 0.3));
    window.set_background(bg);
    assert_eq!(*window.background(), bg);
    let grad = AspectGradientBackground::new((1.0, 0.0, 0.0), (0.0, 0.0, 1.0));
    window.set_gradient_background(grad);
    assert_eq!(*window.gradient_background(), grad);
  }

  #[test]
  fn test_background_fill_method() {
    let mut window = AspectWindow::new();
    assert_eq!(window.background_fill_method(), AspectFillMethod::Centered);
    window.set_background_fill_method(AspectFillMethod::Tiled);
    assert_eq!(window.background_fill_method(), AspectFillMethod::Tiled);
  }

  #[test]
  fn test_display_connection() {
    let window = AspectWindow::new();
    assert!(window.display_connection().is_none());
  }

  #[test]
  fn test_vec2int() {
    let v = Vec2Int::new(10, 20);
    assert_eq!(v.x, 10);
    assert_eq!(v.y, 20);

    let v2 = Vec2Int::new(10, 20);
    assert_eq!(v, v2);
  }
}

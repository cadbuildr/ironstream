// FILE: graphic3d_graduated_trihedron.rs
// occt: Graphic3d_GraduatedTrihedron
// occt: Graphic3d_GraduatedTrihedron // ::AxisAspect

use core::fmt;

/// Simple string type for compatibility (using String internally)
#[derive(Debug, Clone, PartialEq)]
struct ExtendedString {
    data: String,
}

impl ExtendedString {
    fn new(s: &str) -> Self {
        ExtendedString {
            data: s.to_string(),
        }
    }

    fn empty() -> Self {
        ExtendedString {
            data: String::new(),
        }
    }

    fn as_str(&self) -> &str {
        &self.data
    }
}

impl Default for ExtendedString {
    fn default() -> Self {
        ExtendedString::empty()
    }
}

/// Simple Color type (RGB)
#[derive(Debug, Clone, Copy, PartialEq)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    fn black() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    fn red() -> Self {
        Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        }
    }

    fn green() -> Self {
        Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        }
    }

    fn blue() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 1.0,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::black()
    }
}

/// Font aspect enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontAspect {
    Regular = 0,
    Bold = 1,
}

impl Default for FontAspect {
    fn default() -> Self {
        FontAspect::Regular
    }
}

/// Class that stores style for one graduated trihedron axis
#[derive(Debug, Clone)]
pub struct AxisAspect {
    name: ExtendedString,
    name_color: Color,
    color: Color,
    values_offset: i32,
    name_offset: i32,
    tickmarks_number: i32,
    tickmarks_length: i32,
    to_draw_name: bool,
    to_draw_values: bool,
    to_draw_tickmarks: bool,
}

impl AxisAspect {
    pub fn new(
        name: &str,
        name_color: Color,
        color: Color,
        values_offset: i32,
        name_offset: i32,
        tickmarks_number: i32,
        tickmarks_length: i32,
        to_draw_name: bool,
        to_draw_values: bool,
        to_draw_tickmarks: bool,
    ) -> Self {
        AxisAspect {
            name: ExtendedString::new(name),
            name_color,
            color,
            values_offset,
            name_offset,
            tickmarks_number,
            tickmarks_length,
            to_draw_name,
            to_draw_values,
            to_draw_tickmarks,
        }
    }

    pub fn default_with_name(name: &str) -> Self {
        AxisAspect::new(
            name,
            Color::black(),
            Color::black(),
            10,
            30,
            5,
            10,
            true,
            true,
            true,
        )
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = ExtendedString::new(name);
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn to_draw_name(&self) -> bool {
        self.to_draw_name
    }

    pub fn set_draw_name(&mut self, to_draw: bool) {
        self.to_draw_name = to_draw;
    }

    pub fn to_draw_tickmarks(&self) -> bool {
        self.to_draw_tickmarks
    }

    pub fn set_draw_tickmarks(&mut self, to_draw: bool) {
        self.to_draw_tickmarks = to_draw;
    }

    pub fn to_draw_values(&self) -> bool {
        self.to_draw_values
    }

    pub fn set_draw_values(&mut self, to_draw: bool) {
        self.to_draw_values = to_draw;
    }

    pub fn name_color(&self) -> Color {
        self.name_color
    }

    pub fn set_name_color(&mut self, color: Color) {
        self.name_color = color;
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn tickmarks_number(&self) -> i32 {
        self.tickmarks_number
    }

    pub fn set_tickmarks_number(&mut self, value: i32) {
        self.tickmarks_number = value;
    }

    pub fn tickmarks_length(&self) -> i32 {
        self.tickmarks_length
    }

    pub fn set_tickmarks_length(&mut self, value: i32) {
        self.tickmarks_length = value;
    }

    pub fn values_offset(&self) -> i32 {
        self.values_offset
    }

    pub fn set_values_offset(&mut self, value: i32) {
        self.values_offset = value;
    }

    pub fn name_offset(&self) -> i32 {
        self.name_offset
    }

    pub fn set_name_offset(&mut self, value: i32) {
        self.name_offset = value;
    }
}

impl Default for AxisAspect {
    fn default() -> Self {
        AxisAspect::new(
            "",
            Color::black(),
            Color::black(),
            10,
            30,
            5,
            10,
            true,
            true,
            true,
        )
    }
}

/// Defines the class of a graduated trihedron
pub struct GraduatedTrihedron {
    cubic_axes_callback: Option<fn()>,
    names_font: String,
    names_style: FontAspect,
    names_size: i32,
    values_font: String,
    values_style: FontAspect,
    values_size: i32,
    arrows_length: f32,
    grid_color: Color,
    to_draw_grid: bool,
    to_draw_axes: bool,
    axes: [AxisAspect; 3],
}

impl GraduatedTrihedron {
    pub fn new(
        names_font: &str,
        names_style: FontAspect,
        names_size: i32,
        values_font: &str,
        values_style: FontAspect,
        values_size: i32,
        arrows_length: f32,
        grid_color: Color,
        to_draw_grid: bool,
        to_draw_axes: bool,
    ) -> Self {
        GraduatedTrihedron {
            cubic_axes_callback: None,
            names_font: names_font.to_string(),
            names_style,
            names_size,
            values_font: values_font.to_string(),
            values_style,
            values_size,
            arrows_length,
            grid_color,
            to_draw_grid,
            to_draw_axes,
            axes: [
                AxisAspect::default_with_name("X"),
                AxisAspect::default_with_name("Y"),
                AxisAspect::default_with_name("Z"),
            ],
        }
    }

    pub fn default() -> Self {
        let mut tri = GraduatedTrihedron {
            cubic_axes_callback: None,
            names_font: "Arial".to_string(),
            names_style: FontAspect::Bold,
            names_size: 12,
            values_font: "Arial".to_string(),
            values_style: FontAspect::Regular,
            values_size: 12,
            arrows_length: 30.0,
            grid_color: Color::black(),
            to_draw_grid: true,
            to_draw_axes: true,
            axes: [
                AxisAspect::default_with_name("X"),
                AxisAspect::default_with_name("Y"),
                AxisAspect::default_with_name("Z"),
            ],
        };
        tri.axes[0].color = Color::red();
        tri.axes[0].name_color = Color::red();
        tri.axes[1].color = Color::green();
        tri.axes[1].name_color = Color::green();
        tri.axes[2].color = Color::blue();
        tri.axes[2].name_color = Color::blue();
        tri
    }

    pub fn change_x_axis_aspect(&mut self) -> &mut AxisAspect {
        &mut self.axes[0]
    }

    pub fn change_y_axis_aspect(&mut self) -> &mut AxisAspect {
        &mut self.axes[1]
    }

    pub fn change_z_axis_aspect(&mut self) -> &mut AxisAspect {
        &mut self.axes[2]
    }

    pub fn change_axis_aspect(&mut self, index: usize) -> Option<&mut AxisAspect> {
        if index < 3 {
            Some(&mut self.axes[index])
        } else {
            None
        }
    }

    pub fn x_axis_aspect(&self) -> &AxisAspect {
        &self.axes[0]
    }

    pub fn y_axis_aspect(&self) -> &AxisAspect {
        &self.axes[1]
    }

    pub fn z_axis_aspect(&self) -> &AxisAspect {
        &self.axes[2]
    }

    pub fn axis_aspect_at(&self, index: usize) -> Option<&AxisAspect> {
        if index < 3 {
            Some(&self.axes[index])
        } else {
            None
        }
    }

    pub fn arrows_length(&self) -> f32 {
        self.arrows_length
    }

    pub fn set_arrows_length(&mut self, value: f32) {
        self.arrows_length = value;
    }

    pub fn grid_color(&self) -> Color {
        self.grid_color
    }

    pub fn set_grid_color(&mut self, color: Color) {
        self.grid_color = color;
    }

    pub fn to_draw_grid(&self) -> bool {
        self.to_draw_grid
    }

    pub fn set_draw_grid(&mut self, to_draw: bool) {
        self.to_draw_grid = to_draw;
    }

    pub fn to_draw_axes(&self) -> bool {
        self.to_draw_axes
    }

    pub fn set_draw_axes(&mut self, to_draw: bool) {
        self.to_draw_axes = to_draw;
    }

    pub fn names_font(&self) -> &str {
        &self.names_font
    }

    pub fn set_names_font(&mut self, font: &str) {
        self.names_font = font.to_string();
    }

    pub fn names_font_aspect(&self) -> FontAspect {
        self.names_style
    }

    pub fn set_names_font_aspect(&mut self, aspect: FontAspect) {
        self.names_style = aspect;
    }

    pub fn names_size(&self) -> i32 {
        self.names_size
    }

    pub fn set_names_size(&mut self, value: i32) {
        self.names_size = value;
    }

    pub fn values_font(&self) -> &str {
        &self.values_font
    }

    pub fn set_values_font(&mut self, font: &str) {
        self.values_font = font.to_string();
    }

    pub fn values_font_aspect(&self) -> FontAspect {
        self.values_style
    }

    pub fn set_values_font_aspect(&mut self, aspect: FontAspect) {
        self.values_style = aspect;
    }

    pub fn values_size(&self) -> i32 {
        self.values_size
    }

    pub fn set_values_size(&mut self, value: i32) {
        self.values_size = value;
    }

    pub fn cubic_axes_callback(&self) -> bool {
        self.cubic_axes_callback.is_some()
    }

    pub fn set_cubic_axes_callback(&mut self, callback: Option<fn()>) {
        self.cubic_axes_callback = callback;
    }
}

impl Default for GraduatedTrihedron {
    fn default() -> Self {
        GraduatedTrihedron::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axis_aspect_default() {
        let aspect = AxisAspect::default();
        assert_eq!(aspect.name(), "");
        assert_eq!(aspect.to_draw_name(), true);
        assert_eq!(aspect.to_draw_values(), true);
        assert_eq!(aspect.to_draw_tickmarks(), true);
        assert_eq!(aspect.tickmarks_number(), 5);
        assert_eq!(aspect.tickmarks_length(), 10);
    }

    #[test]
    fn test_axis_aspect_with_name() {
        let aspect = AxisAspect::default_with_name("TestAxis");
        assert_eq!(aspect.name(), "TestAxis");
        assert_eq!(aspect.to_draw_name(), true);
    }

    #[test]
    fn test_axis_aspect_setters() {
        let mut aspect = AxisAspect::default();
        aspect.set_name("X");
        assert_eq!(aspect.name(), "X");

        aspect.set_draw_name(false);
        assert_eq!(aspect.to_draw_name(), false);

        aspect.set_draw_values(false);
        assert_eq!(aspect.to_draw_values(), false);

        aspect.set_draw_tickmarks(false);
        assert_eq!(aspect.to_draw_tickmarks(), false);

        aspect.set_tickmarks_number(10);
        assert_eq!(aspect.tickmarks_number(), 10);

        aspect.set_tickmarks_length(20);
        assert_eq!(aspect.tickmarks_length(), 20);

        aspect.set_values_offset(15);
        assert_eq!(aspect.values_offset(), 15);

        aspect.set_name_offset(40);
        assert_eq!(aspect.name_offset(), 40);
    }

    #[test]
    fn test_graduated_trihedron_default() {
        let tri = GraduatedTrihedron::default();
        assert_eq!(tri.names_font(), "Arial");
        assert_eq!(tri.names_size(), 12);
        assert_eq!(tri.values_font(), "Arial");
        assert_eq!(tri.values_size(), 12);
        assert!(tri.to_draw_grid());
        assert!(tri.to_draw_axes());
        assert_eq!(tri.arrows_length(), 30.0);
    }

    #[test]
    fn test_graduated_trihedron_axes() {
        let tri = GraduatedTrihedron::default();
        assert_eq!(tri.x_axis_aspect().name(), "X");
        assert_eq!(tri.y_axis_aspect().name(), "Y");
        assert_eq!(tri.z_axis_aspect().name(), "Z");
    }

    #[test]
    fn test_graduated_trihedron_setters() {
        let mut tri = GraduatedTrihedron::default();

        tri.set_names_font("Courier");
        assert_eq!(tri.names_font(), "Courier");

        tri.set_names_size(14);
        assert_eq!(tri.names_size(), 14);

        tri.set_draw_grid(false);
        assert!(!tri.to_draw_grid());

        tri.set_draw_axes(false);
        assert!(!tri.to_draw_axes());

        tri.set_arrows_length(50.0);
        assert_eq!(tri.arrows_length(), 50.0);
    }

    #[test]
    fn test_graduated_trihedron_change_axis() {
        let mut tri = GraduatedTrihedron::default();
        tri.change_x_axis_aspect().set_name("CustomX");
        assert_eq!(tri.x_axis_aspect().name(), "CustomX");

        if let Some(aspect) = tri.change_axis_aspect(1) {
            aspect.set_name("CustomY");
        }
        assert_eq!(tri.y_axis_aspect().name(), "CustomY");
    }
}

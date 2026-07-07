// FILE: draw_tr_surf_b_spline_curve.rs
// occt: DrawTrSurf_BSplineCurve

//! A drawable B-spline curve for the Draw interface.
//!
//! This class wraps a B-spline curve with visualization parameters such as
//! control point markers and knot display options.

use std::fmt;

/// A point in 3D space.
#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// Draw color enumeration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawColor {
    White,
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
}

impl Default for DrawColor {
    fn default() -> Self {
        DrawColor::White
    }
}

/// Marker shape for knot/pole display.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkerShape {
    Circle,
    Square,
    Diamond,
    Cross,
}

impl Default for MarkerShape {
    fn default() -> Self {
        MarkerShape::Circle
    }
}

/// Display object for drawing operations.
#[derive(Clone, Debug)]
pub struct Display {
    // Simplified display stub
}

impl Display {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}

/// A B-spline curve for drawing.
#[derive(Clone, Debug)]
pub struct BSplineCurve {
    pub control_points: Vec<Point>,
    pub knots: Vec<f64>,
    pub degree: usize,
}

impl BSplineCurve {
    pub fn new(control_points: Vec<Point>, knots: Vec<f64>, degree: usize) -> Self {
        Self {
            control_points,
            knots,
            degree,
        }
    }

    pub fn evaluate(&self, u: f64) -> Point {
        // Stub: return first control point
        self.control_points.first().cloned().unwrap_or_default()
    }
}

impl Default for BSplineCurve {
    fn default() -> Self {
        Self {
            control_points: Vec::new(),
            knots: Vec::new(),
            degree: 3,
        }
    }
}

/// Parent class: DrawTrSurf_Curve (simplified).
#[derive(Clone, Debug)]
pub struct Curve {
    pub curve: BSplineCurve,
    pub color: DrawColor,
    pub discretization: usize,
    pub deflection: f64,
}

impl Curve {
    pub fn new(curve: BSplineCurve) -> Self {
        Self {
            curve,
            color: DrawColor::default(),
            discretization: 50,
            deflection: 0.01,
        }
    }
}

impl Default for Curve {
    fn default() -> Self {
        Self {
            curve: BSplineCurve::default(),
            color: DrawColor::default(),
            discretization: 50,
            deflection: 0.01,
        }
    }
}

/// A drawable B-spline curve with control points and knots.
#[derive(Clone, Debug)]
pub struct DrawTrSurfBSplineCurve {
    base: Curve,
    curve_color: DrawColor,
    poles_color: DrawColor,
    knots_color: DrawColor,
    knots_shape: MarkerShape,
    knots_size: usize,
    show_poles: bool,
    show_knots: bool,
    draw_mode: usize,
}

impl DrawTrSurfBSplineCurve {
    /// Create a drawable B-spline curve.
    pub fn new(curve: BSplineCurve) -> Self {
        let base = Curve::new(curve);
        Self {
            base,
            curve_color: DrawColor::Red,
            poles_color: DrawColor::Green,
            knots_color: DrawColor::Blue,
            knots_shape: MarkerShape::Circle,
            knots_size: 3,
            show_poles: true,
            show_knots: true,
            draw_mode: 0,
        }
    }

    /// Create a drawable B-spline curve with custom colors and options.
    pub fn with_params(
        curve: BSplineCurve,
        curve_color: DrawColor,
        poles_color: DrawColor,
        knots_color: DrawColor,
        knots_shape: MarkerShape,
        knots_size: usize,
        show_poles: bool,
        show_knots: bool,
        discretization: usize,
        deflection: f64,
        draw_mode: usize,
    ) -> Self {
        let mut base = Curve::new(curve);
        base.discretization = discretization;
        base.deflection = deflection;

        Self {
            base,
            curve_color,
            poles_color,
            knots_color,
            knots_shape,
            knots_size,
            show_poles,
            show_knots,
            draw_mode,
        }
    }

    /// Draw the curve on a display.
    pub fn draw_on(&self, _display: &mut Display) {
        // Real implementation would discretize the curve and draw line segments
    }

    /// Draw with custom pole and knot visibility.
    pub fn draw_on_with_visibility(&self, _display: &mut Display, _show_poles: bool, _show_knots: bool) {
        // Real implementation would draw with specified visibility
    }

    /// Draw a portion of the curve.
    pub fn draw_portion(
        &self,
        _display: &mut Display,
        _u1: f64,
        _u2: f64,
        _pole_index: usize,
        _show_poles: bool,
        _show_knots: bool,
    ) {
        // Real implementation would draw a trimmed portion of the curve
    }

    /// Show poles in drawings.
    pub fn show_poles(&mut self) {
        self.show_poles = true;
    }

    /// Show knots in drawings.
    pub fn show_knots(&mut self) {
        self.show_knots = true;
    }

    /// Hide poles in drawings.
    pub fn clear_poles(&mut self) {
        self.show_poles = false;
    }

    /// Hide knots in drawings.
    pub fn clear_knots(&mut self) {
        self.show_knots = false;
    }

    /// Find a pole near screen coordinates (X, Y).
    pub fn find_pole(&self, x: f64, y: f64, _display: &Display, precision: f64) -> Option<usize> {
        let screen_point = Point::new(x, y, 0.0);

        for (index, pole) in self.base.curve.control_points.iter().enumerate() {
            if pole.distance_to(&screen_point) < precision {
                return Some(index + 1);
            }
        }
        None
    }

    /// Find a knot near screen coordinates (X, Y).
    pub fn find_knot(&self, x: f64, y: f64, _display: &Display, precision: f64) -> Option<usize> {
        let screen_point = Point::new(x, y, 0.0);

        for (index, knot_val) in self.base.curve.knots.iter().enumerate() {
            // Project knot parameter to screen space (stub)
            let knot_point = Point::new(*knot_val, 0.0, 0.0);
            if knot_point.distance_to(&screen_point) < precision {
                return Some(index);
            }
        }
        None
    }

    pub fn curve_color(&self) -> DrawColor {
        self.curve_color
    }

    pub fn set_curve_color(&mut self, color: DrawColor) {
        self.curve_color = color;
    }

    pub fn poles_color(&self) -> DrawColor {
        self.poles_color
    }

    pub fn set_poles_color(&mut self, color: DrawColor) {
        self.poles_color = color;
    }

    pub fn knots_color(&self) -> DrawColor {
        self.knots_color
    }

    pub fn set_knots_color(&mut self, color: DrawColor) {
        self.knots_color = color;
    }
}

impl fmt::Display for DrawTrSurfBSplineCurve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DrawTrSurfBSplineCurve {{ degree: {}, poles: {}, show_poles: {}, show_knots: {} }}",
            self.base.curve.degree,
            self.base.curve.control_points.len(),
            self.show_poles,
            self.show_knots
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bspline_curve() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(2.0, 0.0, 0.0),
        ];
        let curve = BSplineCurve::new(points, vec![0.0, 1.0], 2);

        let drawable = DrawTrSurfBSplineCurve::new(curve);

        assert_eq!(drawable.base.curve.degree, 2);
        assert_eq!(drawable.base.curve.control_points.len(), 3);
        assert!(drawable.show_poles);
        assert!(drawable.show_knots);
    }

    #[test]
    fn test_show_hide_poles() {
        let curve = BSplineCurve::default();
        let mut drawable = DrawTrSurfBSplineCurve::new(curve);

        assert!(drawable.show_poles);

        drawable.clear_poles();
        assert!(!drawable.show_poles);

        drawable.show_poles();
        assert!(drawable.show_poles);
    }

    #[test]
    fn test_show_hide_knots() {
        let curve = BSplineCurve::default();
        let mut drawable = DrawTrSurfBSplineCurve::new(curve);

        assert!(drawable.show_knots);

        drawable.clear_knots();
        assert!(!drawable.show_knots);

        drawable.show_knots();
        assert!(drawable.show_knots);
    }

    #[test]
    fn test_set_colors() {
        let curve = BSplineCurve::default();
        let mut drawable = DrawTrSurfBSplineCurve::new(curve);

        drawable.set_curve_color(DrawColor::Blue);
        assert_eq!(drawable.curve_color(), DrawColor::Blue);

        drawable.set_poles_color(DrawColor::Cyan);
        assert_eq!(drawable.poles_color(), DrawColor::Cyan);

        drawable.set_knots_color(DrawColor::Magenta);
        assert_eq!(drawable.knots_color(), DrawColor::Magenta);
    }

    #[test]
    fn test_find_pole() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(2.0, 0.0, 0.0),
        ];
        let curve = BSplineCurve::new(points, vec![0.0, 1.0], 2);
        let drawable = DrawTrSurfBSplineCurve::new(curve);

        let display = Display::new();

        let pole = drawable.find_pole(0.0, 0.0, &display, 0.5);
        assert_eq!(pole, Some(1));

        let pole = drawable.find_pole(1.0, 0.0, &display, 0.5);
        assert_eq!(pole, Some(2));

        let pole = drawable.find_pole(10.0, 10.0, &display, 0.5);
        assert_eq!(pole, None);
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(3.0, 4.0, 0.0);

        let dist = p1.distance_to(&p2);
        assert!((dist - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_with_params() {
        let curve = BSplineCurve::default();

        let drawable = DrawTrSurfBSplineCurve::with_params(
            curve,
            DrawColor::Red,
            DrawColor::Green,
            DrawColor::Blue,
            MarkerShape::Square,
            5,
            false,
            false,
            100,
            0.05,
            1,
        );

        assert_eq!(drawable.curve_color(), DrawColor::Red);
        assert_eq!(drawable.poles_color(), DrawColor::Green);
        assert_eq!(drawable.knots_color(), DrawColor::Blue);
        assert_eq!(drawable.knots_shape, MarkerShape::Square);
        assert_eq!(drawable.knots_size, 5);
        assert!(!drawable.show_poles);
        assert!(!drawable.show_knots);
        assert_eq!(drawable.base.discretization, 100);
    }

    #[test]
    fn test_display_format() {
        let curve = BSplineCurve::default();
        let drawable = DrawTrSurfBSplineCurve::new(curve);

        let output = format!("{}", drawable);
        assert!(output.contains("BSplineCurve"));
        assert!(output.contains("show_poles"));
    }
}

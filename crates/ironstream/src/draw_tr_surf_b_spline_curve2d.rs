// FILE: draw_tr_surf_b_spline_curve2d.rs
// occt: DrawTrSurf_BSplineCurve2d

//! A drawable 2D B-spline curve for the Draw interface.

use std::fmt;

/// A point in 2D space.
#[derive(Clone, Copy, Debug, Default)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

impl Point2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Point2d) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
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
}

impl Default for MarkerShape {
    fn default() -> Self {
        MarkerShape::Circle
    }
}

/// A 2D B-spline curve.
#[derive(Clone, Debug)]
pub struct BSplineCurve2d {
    pub control_points: Vec<Point2d>,
    pub knots: Vec<f64>,
    pub degree: usize,
}

impl BSplineCurve2d {
    pub fn new(control_points: Vec<Point2d>, knots: Vec<f64>, degree: usize) -> Self {
        Self {
            control_points,
            knots,
            degree,
        }
    }
}

impl Default for BSplineCurve2d {
    fn default() -> Self {
        Self {
            control_points: Vec::new(),
            knots: Vec::new(),
            degree: 3,
        }
    }
}

/// Display stub.
#[derive(Clone, Debug)]
pub struct Display;

impl Display {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}

/// A drawable 2D B-spline curve.
#[derive(Clone, Debug)]
pub struct DrawTrSurfBSplineCurve2d {
    curve: BSplineCurve2d,
    curve_color: DrawColor,
    poles_color: DrawColor,
    knots_color: DrawColor,
    show_poles: bool,
    show_knots: bool,
}

impl DrawTrSurfBSplineCurve2d {
    pub fn new(curve: BSplineCurve2d) -> Self {
        Self {
            curve,
            curve_color: DrawColor::Red,
            poles_color: DrawColor::Green,
            knots_color: DrawColor::Blue,
            show_poles: true,
            show_knots: true,
        }
    }

    pub fn draw_on(&self, _display: &mut Display) {
        // Draw the curve
    }

    pub fn show_poles(&mut self) {
        self.show_poles = true;
    }

    pub fn show_knots(&mut self) {
        self.show_knots = true;
    }

    pub fn clear_poles(&mut self) {
        self.show_poles = false;
    }

    pub fn clear_knots(&mut self) {
        self.show_knots = false;
    }

    pub fn find_pole(&self, x: f64, y: f64, _precision: f64) -> Option<usize> {
        let point = Point2d::new(x, y);
        for (index, pole) in self.curve.control_points.iter().enumerate() {
            if pole.distance_to(&point) < _precision {
                return Some(index + 1);
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
}

impl fmt::Display for DrawTrSurfBSplineCurve2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DrawTrSurfBSplineCurve2d {{ degree: {}, poles: {} }}",
            self.curve.degree,
            self.curve.control_points.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let points = vec![Point2d::new(0.0, 0.0), Point2d::new(1.0, 1.0)];
        let curve = BSplineCurve2d::new(points, vec![0.0, 1.0], 2);
        let drawable = DrawTrSurfBSplineCurve2d::new(curve);

        assert_eq!(drawable.curve.degree, 2);
        assert!(drawable.show_poles);
    }

    #[test]
    fn test_visibility() {
        let curve = BSplineCurve2d::default();
        let mut drawable = DrawTrSurfBSplineCurve2d::new(curve);

        drawable.clear_poles();
        assert!(!drawable.show_poles);

        drawable.show_poles();
        assert!(drawable.show_poles);
    }

    #[test]
    fn test_find_pole() {
        let points = vec![Point2d::new(0.0, 0.0), Point2d::new(2.0, 2.0)];
        let curve = BSplineCurve2d::new(points, vec![0.0, 1.0], 1);
        let drawable = DrawTrSurfBSplineCurve2d::new(curve);

        assert_eq!(drawable.find_pole(0.0, 0.0, 0.5), Some(1));
        assert_eq!(drawable.find_pole(10.0, 10.0, 0.5), None);
    }

    #[test]
    fn test_colors() {
        let curve = BSplineCurve2d::default();
        let mut drawable = DrawTrSurfBSplineCurve2d::new(curve);

        drawable.set_curve_color(DrawColor::Blue);
        assert_eq!(drawable.curve_color(), DrawColor::Blue);
    }
}

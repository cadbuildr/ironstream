// FILE: draw_tr_surf_bezier_curve.rs
// occt: DrawTrSurf_BezierCurve

//! A drawable Bezier curve for the Draw interface.

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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawColor {
    Red,
    Green,
    Blue,
    White,
}

impl Default for DrawColor {
    fn default() -> Self {
        DrawColor::White
    }
}

#[derive(Clone, Debug)]
pub struct Display;

#[derive(Clone, Debug)]
pub struct BezierCurve {
    pub control_points: Vec<Point>,
}

impl BezierCurve {
    pub fn new(control_points: Vec<Point>) -> Self {
        Self { control_points }
    }

    pub fn degree(&self) -> usize {
        if self.control_points.is_empty() {
            0
        } else {
            self.control_points.len() - 1
        }
    }
}

impl Default for BezierCurve {
    fn default() -> Self {
        Self {
            control_points: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DrawTrSurfBezierCurve {
    curve: BezierCurve,
    color: DrawColor,
    show_poles: bool,
}

impl DrawTrSurfBezierCurve {
    pub fn new(curve: BezierCurve) -> Self {
        Self {
            curve,
            color: DrawColor::default(),
            show_poles: true,
        }
    }

    pub fn draw_on(&self, _display: &mut Display) {}

    pub fn show_poles(&mut self) {
        self.show_poles = true;
    }

    pub fn clear_poles(&mut self) {
        self.show_poles = false;
    }

    pub fn degree(&self) -> usize {
        self.curve.degree()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let points = vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 0.0)];
        let curve = BezierCurve::new(points);
        let drawable = DrawTrSurfBezierCurve::new(curve);

        assert_eq!(drawable.degree(), 1);
        assert!(drawable.show_poles);
    }

    #[test]
    fn test_visibility() {
        let curve = BezierCurve::default();
        let mut drawable = DrawTrSurfBezierCurve::new(curve);

        drawable.clear_poles();
        assert!(!drawable.show_poles);

        drawable.show_poles();
        assert!(drawable.show_poles);
    }
}

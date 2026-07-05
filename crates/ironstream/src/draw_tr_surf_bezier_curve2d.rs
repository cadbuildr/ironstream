// FILE: draw_tr_surf_bezier_curve2d.rs
// occt: DrawTrSurf_BezierCurve2d

//! A drawable 2D Bezier curve for the Draw interface.

#[derive(Clone, Copy, Debug, Default)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

impl Point2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug)]
pub struct BezierCurve2d {
    pub control_points: Vec<Point2d>,
}

impl BezierCurve2d {
    pub fn new(control_points: Vec<Point2d>) -> Self {
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

impl Default for BezierCurve2d {
    fn default() -> Self {
        Self {
            control_points: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Display;

#[derive(Clone, Debug)]
pub struct DrawTrSurfBezierCurve2d {
    curve: BezierCurve2d,
    show_poles: bool,
}

impl DrawTrSurfBezierCurve2d {
    pub fn new(curve: BezierCurve2d) -> Self {
        Self {
            curve,
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
        let points = vec![Point2d::new(0.0, 0.0), Point2d::new(1.0, 1.0)];
        let curve = BezierCurve2d::new(points);
        let drawable = DrawTrSurfBezierCurve2d::new(curve);

        assert_eq!(drawable.degree(), 1);
    }

    #[test]
    fn test_visibility() {
        let curve = BezierCurve2d::default();
        let mut drawable = DrawTrSurfBezierCurve2d::new(curve);

        drawable.clear_poles();
        assert!(!drawable.show_poles);

        drawable.show_poles();
        assert!(drawable.show_poles);
    }
}

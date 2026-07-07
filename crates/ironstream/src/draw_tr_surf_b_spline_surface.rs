// FILE: draw_tr_surf_b_spline_surface.rs
// occt: DrawTrSurf_BSplineSurface

//! A drawable B-spline surface for the Draw interface.

#[derive(Clone, Debug)]
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
    Black,
    Yellow,
}

impl Default for DrawColor {
    fn default() -> Self {
        DrawColor::White
    }
}

#[derive(Clone, Debug)]
pub struct Display;

#[derive(Clone, Debug)]
pub struct BSplineSurface {
    pub control_points: Vec<Vec<Point>>,
    pub u_knots: Vec<f64>,
    pub v_knots: Vec<f64>,
    pub u_degree: usize,
    pub v_degree: usize,
}

impl BSplineSurface {
    pub fn new(
        control_points: Vec<Vec<Point>>,
        u_knots: Vec<f64>,
        v_knots: Vec<f64>,
        u_degree: usize,
        v_degree: usize,
    ) -> Self {
        Self {
            control_points,
            u_knots,
            v_knots,
            u_degree,
            v_degree,
        }
    }
}

impl Default for BSplineSurface {
    fn default() -> Self {
        Self {
            control_points: Vec::new(),
            u_knots: Vec::new(),
            v_knots: Vec::new(),
            u_degree: 3,
            v_degree: 3,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DrawTrSurfBSplineSurface {
    surface: BSplineSurface,
    color: DrawColor,
    show_poles: bool,
    show_knots: bool,
}

impl DrawTrSurfBSplineSurface {
    pub fn new(surface: BSplineSurface) -> Self {
        Self {
            surface,
            color: DrawColor::default(),
            show_poles: true,
            show_knots: true,
        }
    }

    pub fn draw_on(&self, _display: &mut Display) {}

    pub fn show_poles(&mut self) {
        self.show_poles = true;
    }

    pub fn clear_poles(&mut self) {
        self.show_poles = false;
    }

    pub fn show_knots(&mut self) {
        self.show_knots = true;
    }

    pub fn clear_knots(&mut self) {
        self.show_knots = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_surface() {
        let surface = BSplineSurface::default();
        let drawable = DrawTrSurfBSplineSurface::new(surface);
        assert!(drawable.show_poles);
    }

    #[test]
    fn test_visibility() {
        let surface = BSplineSurface::default();
        let mut drawable = DrawTrSurfBSplineSurface::new(surface);

        drawable.clear_poles();
        assert!(!drawable.show_poles);

        drawable.show_poles();
        assert!(drawable.show_poles);
    }
}

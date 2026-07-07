// FILE: draw_tr_surf_curve.rs
// occt: DrawTrSurf_Curve

//! Base class for drawable 3D curves.

#[derive(Clone, Copy, Debug)]
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
    White,
    Black,
    Red,
    Green,
    Blue,
}

impl Default for DrawColor {
    fn default() -> Self {
        DrawColor::White
    }
}

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

#[derive(Clone, Debug)]
pub struct DrawTrSurfCurve {
    color: DrawColor,
    discretization: usize,
    deflection: f64,
}

impl DrawTrSurfCurve {
    pub fn new() -> Self {
        Self {
            color: DrawColor::default(),
            discretization: 50,
            deflection: 0.01,
        }
    }

    pub fn draw_on(&self, _display: &mut Display) {}

    pub fn set_color(&mut self, color: DrawColor) {
        self.color = color;
    }

    pub fn color(&self) -> DrawColor {
        self.color
    }

    pub fn set_discretization(&mut self, num: usize) {
        self.discretization = num;
    }

    pub fn discretization(&self) -> usize {
        self.discretization
    }
}

impl Default for DrawTrSurfCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let curve = DrawTrSurfCurve::new();
        assert_eq!(curve.color(), DrawColor::White);
        assert_eq!(curve.discretization(), 50);
    }

    #[test]
    fn test_set_color() {
        let mut curve = DrawTrSurfCurve::new();
        curve.set_color(DrawColor::Red);
        assert_eq!(curve.color(), DrawColor::Red);
    }

    #[test]
    fn test_discretization() {
        let mut curve = DrawTrSurfCurve::new();
        curve.set_discretization(100);
        assert_eq!(curve.discretization(), 100);
    }
}

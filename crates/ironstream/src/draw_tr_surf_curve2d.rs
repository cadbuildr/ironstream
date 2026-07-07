// FILE: draw_tr_surf_curve2d.rs
// occt: DrawTrSurf_Curve2d

//! Base class for drawable 2D curves.

#[derive(Clone, Copy, Debug)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

impl Point2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawColor {
    White,
    Black,
    Red,
}

impl Default for DrawColor {
    fn default() -> Self {
        DrawColor::White
    }
}

#[derive(Clone, Debug)]
pub struct Display;

#[derive(Clone, Debug)]
pub struct DrawTrSurfCurve2d {
    color: DrawColor,
    discretization: usize,
}

impl DrawTrSurfCurve2d {
    pub fn new() -> Self {
        Self {
            color: DrawColor::default(),
            discretization: 50,
        }
    }

    pub fn draw_on(&self, _display: &mut Display) {}

    pub fn set_color(&mut self, color: DrawColor) {
        self.color = color;
    }

    pub fn color(&self) -> DrawColor {
        self.color
    }
}

impl Default for DrawTrSurfCurve2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let curve = DrawTrSurfCurve2d::new();
        assert_eq!(curve.color(), DrawColor::White);
    }

    #[test]
    fn test_color() {
        let mut curve = DrawTrSurfCurve2d::new();
        curve.set_color(DrawColor::Red);
        assert_eq!(curve.color(), DrawColor::Red);
    }
}

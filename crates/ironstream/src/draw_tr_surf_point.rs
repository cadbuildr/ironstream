// FILE: draw_tr_surf_point.rs
// occt: DrawTrSurf_Point

//! A drawable 3D point for the Draw interface.

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
    White,
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

#[derive(Clone, Debug)]
pub struct DrawTrSurfPoint {
    point: Point,
    color: DrawColor,
    size: usize,
}

impl DrawTrSurfPoint {
    pub fn new(point: Point) -> Self {
        Self {
            point,
            color: DrawColor::default(),
            size: 3,
        }
    }

    pub fn draw_on(&self, _display: &mut Display) {}

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn set_point(&mut self, p: Point) {
        self.point = p;
    }

    pub fn set_color(&mut self, color: DrawColor) {
        self.color = color;
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let pt = Point::new(1.0, 2.0, 3.0);
        let drawable = DrawTrSurfPoint::new(pt);

        assert_eq!(drawable.point().x, 1.0);
        assert_eq!(drawable.point().y, 2.0);
        assert_eq!(drawable.point().z, 3.0);
    }

    #[test]
    fn test_set_point() {
        let pt1 = Point::new(0.0, 0.0, 0.0);
        let mut drawable = DrawTrSurfPoint::new(pt1);

        let pt2 = Point::new(5.0, 5.0, 5.0);
        drawable.set_point(pt2);

        assert_eq!(drawable.point().x, 5.0);
    }

    #[test]
    fn test_color() {
        let pt = Point::new(0.0, 0.0, 0.0);
        let mut drawable = DrawTrSurfPoint::new(pt);

        drawable.set_color(DrawColor::Red);
        assert_eq!(drawable.color, DrawColor::Red);
    }

    #[test]
    fn test_size() {
        let pt = Point::new(0.0, 0.0, 0.0);
        let mut drawable = DrawTrSurfPoint::new(pt);

        drawable.set_size(5);
        assert_eq!(drawable.size, 5);
    }
}

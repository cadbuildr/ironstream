// FILE: draw_axis2_d.rs
// occt: Draw_Axis2D

//! A drawable 2D axis system.

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

#[derive(Clone, Copy, Debug)]
pub struct Vector2d {
    pub dx: f64,
    pub dy: f64,
}

impl Vector2d {
    pub fn new(dx: f64, dy: f64) -> Self {
        Self { dx, dy }
    }

    pub fn normalize(&self) -> Self {
        let len = (self.dx * self.dx + self.dy * self.dy).sqrt();
        if len > 0.0 {
            Self {
                dx: self.dx / len,
                dy: self.dy / len,
            }
        } else {
            Self { dx: 1.0, dy: 0.0 }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Display;

#[derive(Clone, Debug)]
pub struct DrawAxis2d {
    origin: Point2d,
    x_axis: Vector2d,
    y_axis: Vector2d,
}

impl DrawAxis2d {
    pub fn new(origin: Point2d, x_axis: Vector2d, y_axis: Vector2d) -> Self {
        Self {
            origin,
            x_axis: x_axis.normalize(),
            y_axis: y_axis.normalize(),
        }
    }

    pub fn draw_on(&self, _display: &mut Display) {}

    pub fn origin(&self) -> Point2d {
        self.origin
    }

    pub fn x_axis(&self) -> Vector2d {
        self.x_axis
    }

    pub fn y_axis(&self) -> Vector2d {
        self.y_axis
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let origin = Point2d::new(0.0, 0.0);
        let x_axis = Vector2d::new(1.0, 0.0);
        let y_axis = Vector2d::new(0.0, 1.0);

        let axis = DrawAxis2d::new(origin, x_axis, y_axis);

        assert_eq!(axis.origin().x, 0.0);
        assert_eq!(axis.x_axis().dx, 1.0);
    }

    #[test]
    fn test_normalize() {
        let v = Vector2d::new(3.0, 4.0);
        let normalized = v.normalize();

        assert!((normalized.dx - 0.6).abs() < 1e-10);
        assert!((normalized.dy - 0.8).abs() < 1e-10);
    }
}

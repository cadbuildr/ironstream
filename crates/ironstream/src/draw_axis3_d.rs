// FILE: draw_axis3_d.rs
// occt: Draw_Axis3D

//! A drawable 3D axis system.

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

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

impl Vector {
    pub fn new(dx: f64, dy: f64, dz: f64) -> Self {
        Self { dx, dy, dz }
    }

    pub fn normalize(&self) -> Self {
        let len = (self.dx * self.dx + self.dy * self.dy + self.dz * self.dz).sqrt();
        if len > 0.0 {
            Self {
                dx: self.dx / len,
                dy: self.dy / len,
                dz: self.dz / len,
            }
        } else {
            Self {
                dx: 1.0,
                dy: 0.0,
                dz: 0.0,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Display;

#[derive(Clone, Debug)]
pub struct DrawAxis3d {
    origin: Point,
    x_axis: Vector,
    y_axis: Vector,
    z_axis: Vector,
}

impl DrawAxis3d {
    pub fn new(origin: Point, x_axis: Vector, y_axis: Vector, z_axis: Vector) -> Self {
        Self {
            origin,
            x_axis: x_axis.normalize(),
            y_axis: y_axis.normalize(),
            z_axis: z_axis.normalize(),
        }
    }

    pub fn draw_on(&self, _display: &mut Display) {}

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn x_axis(&self) -> Vector {
        self.x_axis
    }

    pub fn y_axis(&self) -> Vector {
        self.y_axis
    }

    pub fn z_axis(&self) -> Vector {
        self.z_axis
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let origin = Point::new(0.0, 0.0, 0.0);
        let x_axis = Vector::new(1.0, 0.0, 0.0);
        let y_axis = Vector::new(0.0, 1.0, 0.0);
        let z_axis = Vector::new(0.0, 0.0, 1.0);

        let axis = DrawAxis3d::new(origin, x_axis, y_axis, z_axis);

        assert_eq!(axis.origin().x, 0.0);
        assert_eq!(axis.x_axis().dx, 1.0);
    }

    #[test]
    fn test_normalize_vector() {
        let v = Vector::new(2.0, 2.0, 1.0);
        let normalized = v.normalize();

        let len_sq = normalized.dx * normalized.dx + normalized.dy * normalized.dy + normalized.dz * normalized.dz;
        assert!((len_sq - 1.0).abs() < 1e-10);
    }
}

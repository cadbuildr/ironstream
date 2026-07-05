// FILE: draw_box.rs
// occt: Draw_Box

//! A drawable bounding box.

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

#[derive(Clone, Debug)]
pub struct Display;

#[derive(Clone, Debug)]
pub struct DrawBox {
    x_min: f64,
    y_min: f64,
    z_min: f64,
    x_max: f64,
    y_max: f64,
    z_max: f64,
}

impl DrawBox {
    pub fn new(x_min: f64, y_min: f64, z_min: f64, x_max: f64, y_max: f64, z_max: f64) -> Self {
        Self {
            x_min: x_min.min(x_max),
            y_min: y_min.min(y_max),
            z_min: z_min.min(z_max),
            x_max: x_min.max(x_max),
            y_max: y_min.max(y_max),
            z_max: z_min.max(z_max),
        }
    }

    pub fn from_points(p_min: Point, p_max: Point) -> Self {
        Self::new(
            p_min.x,
            p_min.y,
            p_min.z,
            p_max.x,
            p_max.y,
            p_max.z,
        )
    }

    pub fn draw_on(&self, _display: &mut Display) {}

    pub fn min(&self) -> Point {
        Point::new(self.x_min, self.y_min, self.z_min)
    }

    pub fn max(&self) -> Point {
        Point::new(self.x_max, self.y_max, self.z_max)
    }

    pub fn size_x(&self) -> f64 {
        self.x_max - self.x_min
    }

    pub fn size_y(&self) -> f64 {
        self.y_max - self.y_min
    }

    pub fn size_z(&self) -> f64 {
        self.z_max - self.z_min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let bbox = DrawBox::new(0.0, 0.0, 0.0, 10.0, 10.0, 10.0);

        assert_eq!(bbox.size_x(), 10.0);
        assert_eq!(bbox.size_y(), 10.0);
        assert_eq!(bbox.size_z(), 10.0);
    }

    #[test]
    fn test_from_points() {
        let p_min = Point::new(1.0, 2.0, 3.0);
        let p_max = Point::new(4.0, 5.0, 6.0);

        let bbox = DrawBox::from_points(p_min, p_max);

        assert_eq!(bbox.min().x, 1.0);
        assert_eq!(bbox.max().z, 6.0);
    }

    #[test]
    fn test_order_swap() {
        let bbox = DrawBox::new(10.0, 10.0, 10.0, 0.0, 0.0, 0.0);

        assert_eq!(bbox.min().x, 0.0);
        assert_eq!(bbox.max().x, 10.0);
        assert_eq!(bbox.size_x(), 10.0);
    }
}

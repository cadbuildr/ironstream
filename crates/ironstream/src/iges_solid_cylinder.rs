// FILE: iges_solid_cylinder.rs
// occt: IGESSolid_Cylinder

//! Cylinder entity (IGES Type 155, Form 0).
//!
//! Right circular cylinder defined by height, radius, center point, and axis direction.

#[derive(Clone, Copy, Debug)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3D { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn unit_z() -> Self {
        Vec3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    pub fn normalized(&self) -> Vec3D {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len > 0.0 {
            Vec3D {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            *self
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }

    pub fn from_vec(v: Vec3D) -> Self {
        Point3D {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

/// Cylinder solid entity
pub struct IGESSolidCylinder {
    height: f64,
    radius: f64,
    center: Vec3D,
    axis: Vec3D,
}

impl IGESSolidCylinder {
    /// Creates a new cylinder
    pub fn new() -> Self {
        IGESSolidCylinder {
            height: 0.0,
            radius: 0.0,
            center: Vec3D::zero(),
            axis: Vec3D::unit_z(),
        }
    }

    /// Initializes the cylinder
    pub fn init(&mut self, ht: f64, radius: f64, center: Vec3D, axis: Vec3D) {
        self.height = ht;
        self.radius = radius;
        self.center = center;
        self.axis = axis.normalized();
    }

    /// Returns the height of the cylinder
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Returns the radius of the cylinder
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Returns the center point
    pub fn center(&self) -> Point3D {
        Point3D::from_vec(self.center)
    }

    /// Returns the axis direction
    pub fn axis(&self) -> Vec3D {
        self.axis.normalized()
    }

    /// Returns the center after transformation (stub)
    pub fn transformed_center(&self) -> Point3D {
        Point3D::from_vec(self.center)
    }

    /// Returns the axis after transformation (stub)
    pub fn transformed_axis(&self) -> Vec3D {
        self.axis.normalized()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cylinder_creation() {
        let cyl = IGESSolidCylinder::new();
        assert_eq!(cyl.height(), 0.0);
        assert_eq!(cyl.radius(), 0.0);
    }

    #[test]
    fn test_cylinder_init() {
        let mut cyl = IGESSolidCylinder::new();
        cyl.init(10.0, 5.0, Vec3D::zero(), Vec3D::unit_z());

        assert_eq!(cyl.height(), 10.0);
        assert_eq!(cyl.radius(), 5.0);
    }

    #[test]
    fn test_cylinder_center() {
        let mut cyl = IGESSolidCylinder::new();
        let center = Vec3D::new(1.0, 2.0, 3.0);
        cyl.init(10.0, 5.0, center, Vec3D::unit_z());

        let c = cyl.center();
        assert_eq!(c.x, 1.0);
        assert_eq!(c.y, 2.0);
        assert_eq!(c.z, 3.0);
    }

    #[test]
    fn test_cylinder_axis() {
        let mut cyl = IGESSolidCylinder::new();
        cyl.init(10.0, 5.0, Vec3D::zero(), Vec3D::unit_z());

        let axis = cyl.axis();
        assert!((axis.z - 1.0).abs() < 1e-10);
    }
}

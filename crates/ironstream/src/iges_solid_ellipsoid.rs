// FILE: iges_solid_ellipsoid.rs
// occt: IGESSolid_Ellipsoid

//! Ellipsoid entity (IGES Type 158, Form 0).
//!
//! An ellipsoid defined by a center point, three semi-axes radii,
//! and their corresponding directions.

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

/// Ellipsoid solid entity
pub struct IGESSolidEllipsoid {
    center: Vec3D,
    x_radius: f64,
    y_radius: f64,
    z_radius: f64,
    x_axis: Vec3D,
    z_axis: Vec3D,
}

impl IGESSolidEllipsoid {
    /// Creates a new ellipsoid
    pub fn new() -> Self {
        IGESSolidEllipsoid {
            center: Vec3D::zero(),
            x_radius: 0.0,
            y_radius: 0.0,
            z_radius: 0.0,
            x_axis: Vec3D::new(1.0, 0.0, 0.0),
            z_axis: Vec3D::new(0.0, 0.0, 1.0),
        }
    }

    /// Initializes the ellipsoid
    pub fn init(
        &mut self,
        center: Vec3D,
        x_radius: f64,
        y_radius: f64,
        z_radius: f64,
        x_axis: Vec3D,
        z_axis: Vec3D,
    ) {
        self.center = center;
        self.x_radius = x_radius;
        self.y_radius = y_radius;
        self.z_radius = z_radius;
        self.x_axis = x_axis.normalized();
        self.z_axis = z_axis.normalized();
    }

    /// Returns the center point
    pub fn center(&self) -> Point3D {
        Point3D::from_vec(self.center)
    }

    /// Returns the X semi-axis radius
    pub fn x_radius(&self) -> f64 {
        self.x_radius
    }

    /// Returns the Y semi-axis radius
    pub fn y_radius(&self) -> f64 {
        self.y_radius
    }

    /// Returns the Z semi-axis radius
    pub fn z_radius(&self) -> f64 {
        self.z_radius
    }

    /// Returns the X-axis direction
    pub fn x_axis(&self) -> Vec3D {
        self.x_axis.normalized()
    }

    /// Returns the Z-axis direction
    pub fn z_axis(&self) -> Vec3D {
        self.z_axis.normalized()
    }

    /// Computes Y-axis as cross product of Z and X
    pub fn y_axis(&self) -> Vec3D {
        self.z_axis.cross(&self.x_axis)
    }

    /// Returns the center after transformation (stub)
    pub fn transformed_center(&self) -> Point3D {
        Point3D::from_vec(self.center)
    }
}

impl Vec3D {
    pub fn cross(&self, other: &Vec3D) -> Vec3D {
        Vec3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ellipsoid_creation() {
        let e = IGESSolidEllipsoid::new();
        assert_eq!(e.x_radius(), 0.0);
        assert_eq!(e.y_radius(), 0.0);
        assert_eq!(e.z_radius(), 0.0);
    }

    #[test]
    fn test_ellipsoid_init() {
        let mut e = IGESSolidEllipsoid::new();
        let center = Vec3D::new(1.0, 2.0, 3.0);

        e.init(
            center,
            5.0,
            4.0,
            3.0,
            Vec3D::new(1.0, 0.0, 0.0),
            Vec3D::new(0.0, 0.0, 1.0),
        );

        assert_eq!(e.x_radius(), 5.0);
        assert_eq!(e.y_radius(), 4.0);
        assert_eq!(e.z_radius(), 3.0);
    }

    #[test]
    fn test_ellipsoid_center() {
        let mut e = IGESSolidEllipsoid::new();
        let center = Vec3D::new(1.0, 2.0, 3.0);

        e.init(center, 5.0, 4.0, 3.0, Vec3D::new(1.0, 0.0, 0.0), Vec3D::new(0.0, 0.0, 1.0));

        let c = e.center();
        assert_eq!(c.x, 1.0);
        assert_eq!(c.y, 2.0);
        assert_eq!(c.z, 3.0);
    }

    #[test]
    fn test_ellipsoid_axes() {
        let mut e = IGESSolidEllipsoid::new();
        e.init(
            Vec3D::zero(),
            1.0,
            1.0,
            1.0,
            Vec3D::new(1.0, 0.0, 0.0),
            Vec3D::new(0.0, 0.0, 1.0),
        );

        let x = e.x_axis();
        let z = e.z_axis();

        assert!((x.x - 1.0).abs() < 1e-10);
        assert!((z.z - 1.0).abs() < 1e-10);
    }
}

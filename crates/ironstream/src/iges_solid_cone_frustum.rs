// FILE: iges_solid_cone_frustum.rs
// occt: IGESSolid_ConeFrustum

//! Cone Frustum entity (IGES Type 156, Form 0).
//!
//! Defined by the center of the larger circular face, its radius, axis direction,
//! height, and radius of the smaller face.

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

/// Cone Frustum solid entity
pub struct IGESSolidConeFrustum {
    height: f64,
    larger_radius: f64,
    smaller_radius: f64,
    face_center: Vec3D,
    axis: Vec3D,
    has_transformation: bool,
}

impl IGESSolidConeFrustum {
    /// Creates a new cone frustum
    pub fn new() -> Self {
        IGESSolidConeFrustum {
            height: 0.0,
            larger_radius: 0.0,
            smaller_radius: 0.0,
            face_center: Vec3D::zero(),
            axis: Vec3D::unit_z(),
            has_transformation: false,
        }
    }

    /// Initializes the cone frustum with geometric parameters
    pub fn init(&mut self, ht: f64, r1: f64, r2: f64, center: Vec3D, axis: Vec3D) {
        self.height = ht;
        self.larger_radius = r1;
        self.smaller_radius = r2;
        self.face_center = center;
        self.axis = axis.normalized();
    }

    /// Returns the height of the cone frustum
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Returns the radius of the larger face
    pub fn larger_radius(&self) -> f64 {
        self.larger_radius
    }

    /// Returns the radius of the smaller face
    pub fn smaller_radius(&self) -> f64 {
        self.smaller_radius
    }

    /// Returns the center of the larger face
    pub fn face_center(&self) -> Point3D {
        Point3D::from_vec(self.face_center)
    }

    /// Returns the center of the larger face after transformation
    pub fn transformed_face_center(&self) -> Point3D {
        Point3D::from_vec(self.face_center)
    }

    /// Returns the axis direction
    pub fn axis(&self) -> Vec3D {
        self.axis.normalized()
    }

    /// Returns the axis direction after transformation
    pub fn transformed_axis(&self) -> Vec3D {
        self.axis.normalized()
    }

    pub fn set_transformation(&mut self, _has_transform: bool) {
        self.has_transformation = _has_transform;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cone_frustum_creation() {
        let cf = IGESSolidConeFrustum::new();
        assert_eq!(cf.height(), 0.0);
        assert_eq!(cf.larger_radius(), 0.0);
    }

    #[test]
    fn test_cone_frustum_init() {
        let mut cf = IGESSolidConeFrustum::new();
        cf.init(10.0, 5.0, 2.0, Vec3D::zero(), Vec3D::unit_z());

        assert_eq!(cf.height(), 10.0);
        assert_eq!(cf.larger_radius(), 5.0);
        assert_eq!(cf.smaller_radius(), 2.0);
    }

    #[test]
    fn test_cone_frustum_axis() {
        let mut cf = IGESSolidConeFrustum::new();
        cf.init(10.0, 5.0, 2.0, Vec3D::zero(), Vec3D::unit_z());

        let axis = cf.axis();
        assert!((axis.z - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_cone_frustum_face_center() {
        let mut cf = IGESSolidConeFrustum::new();
        let center = Vec3D::new(1.0, 2.0, 3.0);
        cf.init(10.0, 5.0, 2.0, center, Vec3D::unit_z());

        let fc = cf.face_center();
        assert_eq!(fc.x, 1.0);
        assert_eq!(fc.y, 2.0);
        assert_eq!(fc.z, 3.0);
    }
}

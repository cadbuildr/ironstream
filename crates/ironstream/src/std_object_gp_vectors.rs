// FILE: std_object_gp_vectors.rs
// occt: StdObject_gp_Vectors

/// Persistent representation of geometric vectors
#[derive(Clone, Debug)]
pub struct GpVector {
    x: f64,
    y: f64,
    z: f64,
}

impl GpVector {
    /// Create a new vector
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        GpVector { x, y, z }
    }

    /// Create a zero vector
    pub fn zero() -> Self {
        GpVector { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Get X component
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Get Y component
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Get Z component
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Compute magnitude
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalize the vector
    pub fn normalize(&self) -> GpVector {
        let mag = self.magnitude();
        if mag > 1e-10 {
            GpVector {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        } else {
            GpVector::zero()
        }
    }

    /// Compute dot product
    pub fn dot(&self, other: &GpVector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Compute cross product
    pub fn cross(&self, other: &GpVector) -> GpVector {
        GpVector {
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
    fn test_create() {
        let vec = GpVector::new(1.0, 2.0, 3.0);
        assert_eq!(vec.x(), 1.0);
        assert_eq!(vec.y(), 2.0);
        assert_eq!(vec.z(), 3.0);
    }

    #[test]
    fn test_zero() {
        let vec = GpVector::zero();
        assert_eq!(vec.magnitude(), 0.0);
    }

    #[test]
    fn test_magnitude() {
        let vec = GpVector::new(3.0, 4.0, 0.0);
        assert_eq!(vec.magnitude(), 5.0);
    }

    #[test]
    fn test_normalize() {
        let vec = GpVector::new(3.0, 4.0, 0.0);
        let normalized = vec.normalize();
        assert!((normalized.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_dot_product() {
        let v1 = GpVector::new(1.0, 0.0, 0.0);
        let v2 = GpVector::new(0.0, 1.0, 0.0);
        assert_eq!(v1.dot(&v2), 0.0);

        let v3 = GpVector::new(1.0, 0.0, 0.0);
        assert_eq!(v1.dot(&v3), 1.0);
    }

    #[test]
    fn test_cross_product() {
        let v1 = GpVector::new(1.0, 0.0, 0.0);
        let v2 = GpVector::new(0.0, 1.0, 0.0);
        let cross = v1.cross(&v2);

        assert_eq!(cross.x(), 0.0);
        assert_eq!(cross.y(), 0.0);
        assert_eq!(cross.z(), 1.0);
    }
}

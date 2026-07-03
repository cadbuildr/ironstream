// FILE: geom_vector_with_magnitude.rs
// occt: Geom_VectorWithMagnitude

//! A vector with arbitrary magnitude (as opposed to a Direction which has unit magnitude).

pub struct VectorWithMagnitude {
    x: f64,
    y: f64,
    z: f64,
}

impl VectorWithMagnitude {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        VectorWithMagnitude { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn coord(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn square_magnitude(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > 1e-12 {
            self.x /= mag;
            self.y /= mag;
            self.z /= mag;
        }
    }

    pub fn normalized(&self) -> VectorWithMagnitude {
        let mag = self.magnitude();
        if mag > 1e-12 {
            VectorWithMagnitude {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        } else {
            VectorWithMagnitude {
                x: self.x,
                y: self.y,
                z: self.z,
            }
        }
    }

    pub fn reverse(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn reversed(&self) -> VectorWithMagnitude {
        VectorWithMagnitude {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn dot(&self, other: &VectorWithMagnitude) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &VectorWithMagnitude) -> VectorWithMagnitude {
        VectorWithMagnitude {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn scale(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }

    pub fn scaled(&self, factor: f64) -> VectorWithMagnitude {
        VectorWithMagnitude {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let vec = VectorWithMagnitude::new(3.0, 4.0, 0.0);
        assert_eq!(vec.x(), 3.0);
        assert_eq!(vec.y(), 4.0);
        assert_eq!(vec.z(), 0.0);
    }

    #[test]
    fn test_magnitude() {
        let vec = VectorWithMagnitude::new(3.0, 4.0, 0.0);
        assert_eq!(vec.magnitude(), 5.0);
    }

    #[test]
    fn test_normalize() {
        let mut vec = VectorWithMagnitude::new(3.0, 4.0, 0.0);
        vec.normalize();
        assert!((vec.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_normalized() {
        let vec = VectorWithMagnitude::new(3.0, 4.0, 0.0);
        let normalized = vec.normalized();
        assert!((normalized.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_reverse() {
        let mut vec = VectorWithMagnitude::new(1.0, 2.0, 3.0);
        vec.reverse();
        assert_eq!(vec.coord(), (-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_reversed() {
        let vec = VectorWithMagnitude::new(1.0, 2.0, 3.0);
        let rev = vec.reversed();
        assert_eq!(rev.coord(), (-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_dot() {
        let v1 = VectorWithMagnitude::new(1.0, 2.0, 3.0);
        let v2 = VectorWithMagnitude::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn test_cross() {
        let v1 = VectorWithMagnitude::new(1.0, 0.0, 0.0);
        let v2 = VectorWithMagnitude::new(0.0, 1.0, 0.0);
        let cross = v1.cross(&v2);
        assert_eq!(cross.coord(), (0.0, 0.0, 1.0));
    }

    #[test]
    fn test_scale() {
        let mut vec = VectorWithMagnitude::new(1.0, 2.0, 3.0);
        vec.scale(2.0);
        assert_eq!(vec.coord(), (2.0, 4.0, 6.0));
    }

    #[test]
    fn test_scaled() {
        let vec = VectorWithMagnitude::new(1.0, 2.0, 3.0);
        let scaled = vec.scaled(3.0);
        assert_eq!(scaled.coord(), (3.0, 6.0, 9.0));
    }
}

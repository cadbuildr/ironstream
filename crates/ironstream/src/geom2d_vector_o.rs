// FILE: geom2d_vector_o.rs
// occt: Geom2d_Vector

use std::f64::consts::PI;

/// Abstract base class for 2D geometric vectors.
pub struct Geom2dVectorAbstract {
    x: f64,
    y: f64,
}

impl Geom2dVectorAbstract {
    pub fn new(x: f64, y: f64) -> Self {
        Geom2dVectorAbstract { x, y }
    }

    /// Returns the X coordinate.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Returns the Y coordinate.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Returns the coordinates.
    pub fn coord(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    /// Returns the magnitude.
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns the square magnitude.
    pub fn square_magnitude(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// Reverses the vector in place.
    pub fn reverse(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

    /// Returns a reversed copy.
    pub fn reversed(&self) -> Geom2dVectorAbstract {
        Geom2dVectorAbstract {
            x: -self.x,
            y: -self.y,
        }
    }

    /// Computes the cross product (scalar in 2D).
    pub fn crossed(&self, other: &Geom2dVectorAbstract) -> f64 {
        self.x * other.y - self.y * other.x
    }

    /// Computes the scalar dot product.
    pub fn dot(&self, other: &Geom2dVectorAbstract) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Computes the angle between this vector and another.
    /// Returns a value between -PI and PI.
    pub fn angle(&self, other: &Geom2dVectorAbstract) -> f64 {
        let mag1 = self.magnitude();
        let mag2 = other.magnitude();

        if mag1 < 1e-15 || mag2 < 1e-15 {
            return 0.0;
        }

        let dot = self.dot(other);
        let cross = self.crossed(other);

        let cos_angle = dot / (mag1 * mag2);
        let cos_clamped = cos_angle.max(-1.0).min(1.0);
        let angle = cos_clamped.acos();

        if cross < 0.0 {
            -angle
        } else {
            angle
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_vector_new() {
        let v = Geom2dVectorAbstract::new(3.0, 4.0);
        assert!((v.x() - 3.0).abs() < PRECISION);
        assert!((v.y() - 4.0).abs() < PRECISION);
    }

    #[test]
    fn test_vector_magnitude() {
        let v = Geom2dVectorAbstract::new(3.0, 4.0);
        assert!((v.magnitude() - 5.0).abs() < PRECISION);
    }

    #[test]
    fn test_vector_square_magnitude() {
        let v = Geom2dVectorAbstract::new(3.0, 4.0);
        assert!((v.square_magnitude() - 25.0).abs() < PRECISION);
    }

    #[test]
    fn test_vector_reverse() {
        let mut v = Geom2dVectorAbstract::new(3.0, 4.0);
        v.reverse();
        assert!((v.x() + 3.0).abs() < PRECISION);
        assert!((v.y() + 4.0).abs() < PRECISION);
    }

    #[test]
    fn test_vector_reversed() {
        let v = Geom2dVectorAbstract::new(3.0, 4.0);
        let vr = v.reversed();
        assert!((vr.x() + 3.0).abs() < PRECISION);
        assert!((vr.y() + 4.0).abs() < PRECISION);
    }

    #[test]
    fn test_vector_crossed() {
        let v1 = Geom2dVectorAbstract::new(1.0, 0.0);
        let v2 = Geom2dVectorAbstract::new(0.0, 1.0);
        assert!((v1.crossed(&v2) - 1.0).abs() < PRECISION);
    }

    #[test]
    fn test_vector_dot() {
        let v1 = Geom2dVectorAbstract::new(1.0, 2.0);
        let v2 = Geom2dVectorAbstract::new(3.0, 4.0);
        assert!((v1.dot(&v2) - 11.0).abs() < PRECISION);
    }

    #[test]
    fn test_vector_angle_perpendicular() {
        let v1 = Geom2dVectorAbstract::new(1.0, 0.0);
        let v2 = Geom2dVectorAbstract::new(0.0, 1.0);
        let angle = v1.angle(&v2);
        assert!((angle - PI / 2.0).abs() < PRECISION);
    }

    #[test]
    fn test_vector_angle_opposite() {
        let v1 = Geom2dVectorAbstract::new(1.0, 0.0);
        let v2 = Geom2dVectorAbstract::new(-1.0, 0.0);
        let angle = v1.angle(&v2);
        assert!((angle.abs() - PI).abs() < PRECISION);
    }

    #[test]
    fn test_vector_angle_same() {
        let v1 = Geom2dVectorAbstract::new(1.0, 0.0);
        let v2 = Geom2dVectorAbstract::new(1.0, 0.0);
        let angle = v1.angle(&v2);
        assert!(angle.abs() < PRECISION);
    }
}

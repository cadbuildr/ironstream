// FILE: geom2d_point_o.rs
// occt: Geom2d_Point

/// Abstract base class for 2D geometric points.
pub struct Geom2dPointAbstract {
    x: f64,
    y: f64,
}

impl Geom2dPointAbstract {
    pub fn new(x: f64, y: f64) -> Self {
        Geom2dPointAbstract { x, y }
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

    /// Computes distance between this point and another.
    pub fn distance(&self, other: &Geom2dPointAbstract) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Computes squared distance between this point and another.
    pub fn square_distance(&self, other: &Geom2dPointAbstract) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_point_new() {
        let p = Geom2dPointAbstract::new(3.0, 4.0);
        assert!((p.x() - 3.0).abs() < PRECISION);
        assert!((p.y() - 4.0).abs() < PRECISION);
    }

    #[test]
    fn test_point_coord() {
        let p = Geom2dPointAbstract::new(1.0, 2.0);
        let (x, y) = p.coord();
        assert!((x - 1.0).abs() < PRECISION);
        assert!((y - 2.0).abs() < PRECISION);
    }

    #[test]
    fn test_point_distance() {
        let p1 = Geom2dPointAbstract::new(0.0, 0.0);
        let p2 = Geom2dPointAbstract::new(3.0, 4.0);
        let dist = p1.distance(&p2);
        assert!((dist - 5.0).abs() < PRECISION);
    }

    #[test]
    fn test_point_square_distance() {
        let p1 = Geom2dPointAbstract::new(0.0, 0.0);
        let p2 = Geom2dPointAbstract::new(3.0, 4.0);
        let sq_dist = p1.square_distance(&p2);
        assert!((sq_dist - 25.0).abs() < PRECISION);
    }

    #[test]
    fn test_point_distance_to_self() {
        let p = Geom2dPointAbstract::new(5.0, 7.0);
        let dist = p.distance(&p);
        assert!(dist.abs() < PRECISION);
    }
}

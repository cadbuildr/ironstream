// FILE: bisector_point_on_bis.rs
// occt: Bisector_PointOnBis

/// Represents a point on a bisector curve with geometric and parametric data.
#[derive(Debug, Clone)]
pub struct BisectorPointOnBis {
    param1: f64,
    param2: f64,
    param_bis: f64,
    distance: f64,
    infinite: bool,
    point: (f64, f64), // 2D point as (x, y)
}

impl BisectorPointOnBis {
    /// Create an empty point on bisector.
    pub fn new() -> Self {
        BisectorPointOnBis {
            param1: 0.0,
            param2: 0.0,
            param_bis: 0.0,
            distance: 0.0,
            infinite: false,
            point: (0.0, 0.0),
        }
    }

    /// Create a point on bisector with parameters.
    pub fn with_params(
        param1: f64,
        param2: f64,
        param_bis: f64,
        distance: f64,
        point: (f64, f64),
    ) -> Self {
        BisectorPointOnBis {
            param1,
            param2,
            param_bis,
            distance,
            infinite: false,
            point,
        }
    }

    pub fn set_param_on_c1(&mut self, param: f64) {
        self.param1 = param;
    }

    pub fn set_param_on_c2(&mut self, param: f64) {
        self.param2 = param;
    }

    pub fn set_param_on_bis(&mut self, param: f64) {
        self.param_bis = param;
    }

    pub fn set_distance(&mut self, distance: f64) {
        self.distance = distance;
    }

    pub fn set_infinite(&mut self, infinite: bool) {
        self.infinite = infinite;
    }

    pub fn set_point(&mut self, point: (f64, f64)) {
        self.point = point;
    }

    pub fn param_on_c1(&self) -> f64 {
        self.param1
    }

    pub fn param_on_c2(&self) -> f64 {
        self.param2
    }

    pub fn param_on_bis(&self) -> f64 {
        self.param_bis
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn point(&self) -> (f64, f64) {
        self.point
    }

    pub fn is_infinite(&self) -> bool {
        self.infinite
    }
}

impl Default for BisectorPointOnBis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_on_bis_creation() {
        let p = BisectorPointOnBis::new();
        assert_eq!(p.param_on_c1(), 0.0);
        assert_eq!(p.distance(), 0.0);
        assert!(!p.is_infinite());
    }

    #[test]
    fn test_point_on_bis_with_params() {
        let p = BisectorPointOnBis::with_params(1.0, 2.0, 3.0, 4.5, (5.0, 6.0));
        assert_eq!(p.param_on_c1(), 1.0);
        assert_eq!(p.param_on_c2(), 2.0);
        assert_eq!(p.param_on_bis(), 3.0);
        assert_eq!(p.distance(), 4.5);
        assert_eq!(p.point(), (5.0, 6.0));
    }

    #[test]
    fn test_point_on_bis_setters() {
        let mut p = BisectorPointOnBis::new();
        p.set_param_on_c1(10.0);
        p.set_distance(20.0);
        p.set_infinite(true);
        assert_eq!(p.param_on_c1(), 10.0);
        assert_eq!(p.distance(), 20.0);
        assert!(p.is_infinite());
    }
}

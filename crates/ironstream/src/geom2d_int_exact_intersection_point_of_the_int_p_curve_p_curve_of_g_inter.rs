// FILE: geom2d_int_exact_intersection_point_of_the_int_p_curve_p_curve_of_g_inter.rs
// occt: Geom2dInt_ExactIntersectionPointOfTheIntPCurvePCurveOfGInter

/// Exact intersection point between two parametric curves.
#[derive(Debug, Clone)]
pub struct Geom2dIntExactIntersectionPoint {
    pub x: f64,
    pub y: f64,
    pub param1: f64,
    pub param2: f64,
}

impl Geom2dIntExactIntersectionPoint {
    pub fn new(x: f64, y: f64, param1: f64, param2: f64) -> Self {
        Geom2dIntExactIntersectionPoint { x, y, param1, param2 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_point() {
        let pt = Geom2dIntExactIntersectionPoint::new(1.0, 2.0, 0.5, 0.3);
        assert_eq!(pt.x, 1.0);
        assert_eq!(pt.y, 2.0);
        assert_eq!(pt.param1, 0.5);
    }
}

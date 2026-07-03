// FILE: geom2d_int_the_polygon2d_of_the_int_p_curve_p_curve_of_g_inter.rs
// occt: Geom2dInt_ThePolygon2dOfTheIntPCurvePCurveOfGInter

/// Polygon representing a parametric 2D curve for intersection testing
pub struct ThePolygon2dOfTheIntPCurvePCurve {
    nb_segments: usize,
    closed: bool,
    deflection: f64,
    inf_param: f64,
    sup_param: f64,
}

impl ThePolygon2dOfTheIntPCurvePCurve {
    pub fn new(
        _curve: (),
        nb_pnt: usize,
        _domain: (),
        tol: f64,
    ) -> Self {
        ThePolygon2dOfTheIntPCurvePCurve {
            nb_segments: nb_pnt.saturating_sub(1),
            closed: false,
            deflection: tol,
            inf_param: 0.0,
            sup_param: 1.0,
        }
    }

    pub fn nb_segments(&self) -> usize {
        self.nb_segments
    }

    pub fn closed(&self) -> bool {
        self.closed
    }

    pub fn set_closed(&mut self, clos: bool) {
        self.closed = clos;
    }

    pub fn deflection_over_estimation(&self) -> f64 {
        self.deflection
    }

    pub fn set_deflection_over_estimation(&mut self, d: f64) {
        self.deflection = d;
    }

    pub fn inf_parameter(&self) -> f64 {
        self.inf_param
    }

    pub fn sup_parameter(&self) -> f64 {
        self.sup_param
    }

    pub fn auto_intersection_is_possible(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let poly = ThePolygon2dOfTheIntPCurvePCurve::new((), 10, (), 0.01);
        assert_eq!(poly.nb_segments(), 9);
        assert!(!poly.closed());
    }

    #[test]
    fn test_set_closed() {
        let mut poly = ThePolygon2dOfTheIntPCurvePCurve::new((), 5, (), 0.01);
        poly.set_closed(true);
        assert!(poly.closed());
    }

    #[test]
    fn test_parameters() {
        let poly = ThePolygon2dOfTheIntPCurvePCurve::new((), 10, (), 0.01);
        assert_eq!(poly.inf_parameter(), 0.0);
        assert_eq!(poly.sup_parameter(), 1.0);
    }

    #[test]
    fn test_deflection() {
        let mut poly = ThePolygon2dOfTheIntPCurvePCurve::new((), 10, (), 0.01);
        assert_eq!(poly.deflection_over_estimation(), 0.01);
        poly.set_deflection_over_estimation(0.05);
        assert_eq!(poly.deflection_over_estimation(), 0.05);
    }
}

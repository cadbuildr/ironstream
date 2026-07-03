// FILE: geom2d_int_the_int_conic_curve_of_g_inter.rs
// occt: Geom2dInt_TheIntConicCurveOfGInter

/// Intersection between a conic and a parametric 2D curve
pub struct TheIntConicCurve {
    is_done: bool,
    nb_points: usize,
}

impl TheIntConicCurve {
    pub fn new() -> Self {
        TheIntConicCurve {
            is_done: false,
            nb_points: 0,
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_points(&self) -> usize {
        self.nb_points
    }
}

impl Default for TheIntConicCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let inter = TheIntConicCurve::new();
        assert!(!inter.is_done());
        assert_eq!(inter.nb_points(), 0);
    }

    #[test]
    fn test_default() {
        let inter = TheIntConicCurve::default();
        assert!(!inter.is_done());
    }
}

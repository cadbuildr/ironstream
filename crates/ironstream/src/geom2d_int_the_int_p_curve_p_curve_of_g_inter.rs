// FILE: geom2d_int_the_int_p_curve_p_curve_of_g_inter.rs
// occt: Geom2dInt_TheIntPCurvePCurveOfGInter

/// Intersection between two parametric 2D curves
pub struct TheIntPCurvePCurve {
    is_done: bool,
    nb_points: usize,
}

impl TheIntPCurvePCurve {
    pub fn new() -> Self {
        TheIntPCurvePCurve {
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

impl Default for TheIntPCurvePCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let inter = TheIntPCurvePCurve::new();
        assert!(!inter.is_done());
    }

    #[test]
    fn test_default() {
        let inter = TheIntPCurvePCurve::default();
        assert_eq!(inter.nb_points(), 0);
    }
}

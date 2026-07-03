// FILE: geom2d_int_the_intersector_of_the_int_conic_curve_of_g_inter.rs
// occt: Geom2dInt_TheIntersectorOfTheIntConicCurveOfGInter

/// Intersector for conic and parametric curve
pub struct TheIntersectorOfTheIntConicCurve {
    is_done: bool,
}

impl TheIntersectorOfTheIntConicCurve {
    pub fn new() -> Self {
        TheIntersectorOfTheIntConicCurve { is_done: false }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

impl Default for TheIntersectorOfTheIntConicCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let inter = TheIntersectorOfTheIntConicCurve::new();
        assert!(!inter.is_done());
    }

    #[test]
    fn test_default() {
        let _inter = TheIntersectorOfTheIntConicCurve::default();
    }
}

// FILE: hlrb_rep_exact_intersection_point_of_the_int_p_curve_p_curve_of_c_inter.rs
// occt: HLRBRep_ExactIntersectionPointOfTheIntPCurvePCurveOfCInter

pub struct HlrbrépExactIntersectionPointOfTheIntPCurvePCurveOfCInter {
    u1: f64,
    u2: f64,
    xyz: [f64; 3],
}

impl HlrbrépExactIntersectionPointOfTheIntPCurvePCurveOfCInter {
    pub fn new() -> Self {
        HlrbrépExactIntersectionPointOfTheIntPCurvePCurveOfCInter {
            u1: 0.0, u2: 0.0, xyz: [0.0, 0.0, 0.0],
        }
    }

    pub fn set_parameters(&mut self, u1: f64, u2: f64) {
        self.u1 = u1;
        self.u2 = u2;
    }

    pub fn u1(&self) -> f64 { self.u1 }
    pub fn u2(&self) -> f64 { self.u2 }
}

impl Default for HlrbrépExactIntersectionPointOfTheIntPCurvePCurveOfCInter {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let p = HlrbrépExactIntersectionPointOfTheIntPCurvePCurveOfCInter::new();
        assert_eq!(p.u1(), 0.0);
        assert_eq!(p.u2(), 0.0);
    }
}

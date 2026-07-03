// FILE: hlrb_rep_the_int_p_curve_p_curve_of_c_inter.rs
// occt: HLRBRep_TheIntPCurvePCurveOfCInter

#[derive(Clone, Debug)]
pub struct HLRBRepTheIntPCurvePCurve {
    intersections: usize,
}

impl HLRBRepTheIntPCurvePCurve {
    pub fn new() -> Self {
        HLRBRepTheIntPCurvePCurve { intersections: 0 }
    }

    pub fn nb_intersections(&self) -> usize {
        self.intersections
    }
}

impl Default for HLRBRepTheIntPCurvePCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let pc = HLRBRepTheIntPCurvePCurve::new();
        assert_eq!(pc.nb_intersections(), 0);
    }
}

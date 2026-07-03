// FILE: hlrb_rep_the_int_conic_curve_of_c_inter.rs
// occt: HLRBRep_TheIntConicCurveOfCInter

#[derive(Clone, Debug)]
pub struct HLRBRepTheIntConicCurve {
    intersections: usize,
}

impl HLRBRepTheIntConicCurve {
    pub fn new() -> Self {
        HLRBRepTheIntConicCurve { intersections: 0 }
    }

    pub fn nb_intersections(&self) -> usize {
        self.intersections
    }
}

impl Default for HLRBRepTheIntConicCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let ic = HLRBRepTheIntConicCurve::new();
        assert_eq!(ic.nb_intersections(), 0);
    }
}

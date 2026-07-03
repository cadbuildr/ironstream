// FILE: hlrb_rep_the_intersector_of_the_int_conic_curve_of_c_inter.rs
// occt: HLRBRep_TheIntersectorOfTheIntConicCurveOfCInter

#[derive(Clone, Debug)]
pub struct HLRBRepTheIntersectorOfTheIntConicCurve {
    done: bool,
}

impl HLRBRepTheIntersectorOfTheIntConicCurve {
    pub fn new() -> Self {
        HLRBRepTheIntersectorOfTheIntConicCurve { done: false }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn perform(&mut self) {
        self.done = true;
    }
}

impl Default for HLRBRepTheIntersectorOfTheIntConicCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let inter = HLRBRepTheIntersectorOfTheIntConicCurve::new();
        assert!(!inter.is_done());
    }

    #[test]
    fn test_perform() {
        let mut inter = HLRBRepTheIntersectorOfTheIntConicCurve::new();
        inter.perform();
        assert!(inter.is_done());
    }
}

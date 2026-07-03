// FILE: hlrb_rep_the_dist_between_p_curves_of_the_int_p_curve_p_curve_of_c_inter.rs
// occt: HLRBRep_TheDistBetweenPCurvesOfTheIntPCurvePCurveOfCInter

#[derive(Clone, Debug)]
pub struct HLRBRepTheDistBetweenPCurves {
    distance: f64,
}

impl HLRBRepTheDistBetweenPCurves {
    pub fn new() -> Self {
        HLRBRepTheDistBetweenPCurves { distance: 0.0 }
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn set_distance(&mut self, d: f64) {
        self.distance = d;
    }
}

impl Default for HLRBRepTheDistBetweenPCurves {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let d = HLRBRepTheDistBetweenPCurves::new();
        assert_eq!(d.distance(), 0.0);
    }

    #[test]
    fn test_set_distance() {
        let mut d = HLRBRepTheDistBetweenPCurves::new();
        d.set_distance(2.5);
        assert_eq!(d.distance(), 2.5);
    }
}

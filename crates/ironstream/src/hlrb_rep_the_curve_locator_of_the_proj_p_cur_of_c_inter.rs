// FILE: hlrb_rep_the_curve_locator_of_the_proj_p_cur_of_c_inter.rs
// occt: HLRBRep_TheCurveLocatorOfTheProjPCurOfCInter

#[derive(Clone, Debug)]
pub struct HLRBRepTheCurveLocatorOfTheProjPCurOfCInter {
    curve_index: i32,
}

impl HLRBRepTheCurveLocatorOfTheProjPCurOfCInter {
    pub fn new() -> Self {
        HLRBRepTheCurveLocatorOfTheProjPCurOfCInter { curve_index: 0 }
    }

    pub fn locate(&self, _param: f64) -> i32 {
        self.curve_index
    }
}

impl Default for HLRBRepTheCurveLocatorOfTheProjPCurOfCInter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let loc = HLRBRepTheCurveLocatorOfTheProjPCurOfCInter::new();
        assert_eq!(loc.locate(0.5), 0);
    }
}

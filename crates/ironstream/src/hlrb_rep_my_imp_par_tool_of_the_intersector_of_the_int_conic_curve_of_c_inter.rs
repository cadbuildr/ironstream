// FILE: hlrb_rep_my_imp_par_tool_of_the_intersector_of_the_int_conic_curve_of_c_inter.rs
// occt: HLRBRep_MyImpParToolOfTheIntersectorOfTheIntConicCurveOfCInter

pub struct HlrbrépMyImpParToolOfTheIntersectorOfTheIntConicCurveOfCInter;

impl HlrbrépMyImpParToolOfTheIntersectorOfTheIntConicCurveOfCInter {
    pub fn tolerance() -> f64 { 1e-7 }
    pub fn max_iterations() -> usize { 100 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_constants() {
        assert!(HlrbrépMyImpParToolOfTheIntersectorOfTheIntConicCurveOfCInter::tolerance() > 0.0);
    }
}

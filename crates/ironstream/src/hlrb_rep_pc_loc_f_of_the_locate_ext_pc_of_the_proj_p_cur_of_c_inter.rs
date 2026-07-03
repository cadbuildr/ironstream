// FILE: hlrb_rep_pc_loc_f_of_the_locate_ext_pc_of_the_proj_p_cur_of_c_inter.rs
// occt: HLRBRep_PCLocFOfTheLocateExtPCOfTheProjPCurOfCInter

pub struct HlrbrépPCLocFOfTheLocateExtPCOfTheProjPCurOfCInter {
    param: f64,
}

impl HlrbrépPCLocFOfTheLocateExtPCOfTheProjPCurOfCInter {
    pub fn new() -> Self {
        HlrbrépPCLocFOfTheLocateExtPCOfTheProjPCurOfCInter { param: 0.0 }
    }

    pub fn set_parameter(&mut self, p: f64) { self.param = p; }
    pub fn parameter(&self) -> f64 { self.param }
}

impl Default for HlrbrépPCLocFOfTheLocateExtPCOfTheProjPCurOfCInter {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let p = HlrbrépPCLocFOfTheLocateExtPCOfTheProjPCurOfCInter::new();
        assert_eq!(p.parameter(), 0.0);
    }
}

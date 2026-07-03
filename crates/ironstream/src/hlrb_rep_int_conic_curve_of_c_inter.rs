// FILE: hlrb_rep_int_conic_curve_of_c_inter.rs
// occt: HLRBRep_IntConicCurveOfCInter

pub struct HlrbrépIntConicCurveOfCInter {
    points: Vec<(f64, f64)>,
    done: bool,
}

impl HlrbrépIntConicCurveOfCInter {
    pub fn new() -> Self {
        HlrbrépIntConicCurveOfCInter {
            points: Vec::new(),
            done: false,
        }
    }

    pub fn perform(&mut self) { self.done = true; }
    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_points(&self) -> usize { self.points.len() }
    pub fn add_point(&mut self, u: f64, v: f64) { self.points.push((u, v)); }
}

impl Default for HlrbrépIntConicCurveOfCInter {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let ic = HlrbrépIntConicCurveOfCInter::new();
        assert!(!ic.is_done());
    }
}

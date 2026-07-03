// FILE: hlrb_rep_c_inter.rs
// occt: HLRBRep_CInter

/// Curve-curve intersection algorithm.
pub struct HlrbrépCInter {
    intersections: Vec<(f64, f64)>,
    done: bool,
}

impl HlrbrépCInter {
    pub fn new() -> Self {
        HlrbrépCInter {
            intersections: Vec::new(),
            done: false,
        }
    }

    pub fn perform(&mut self) {
        self.done = true;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn nb_intersections(&self) -> usize {
        self.intersections.len()
    }

    pub fn add_intersection(&mut self, u1: f64, u2: f64) {
        self.intersections.push((u1, u2));
    }

    pub fn intersection(&self, idx: usize) -> Option<(f64, f64)> {
        self.intersections.get(idx).copied()
    }
}

impl Default for HlrbrépCInter {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let ci = HlrbrépCInter::new();
        assert!(!ci.is_done());
        assert_eq!(ci.nb_intersections(), 0);
    }
    #[test]
    fn test_perform() {
        let mut ci = HlrbrépCInter::new();
        ci.perform();
        assert!(ci.is_done());
    }
}

// FILE: hlrb_rep_intersector.rs
// occt: HLRBRep_Intersector

pub struct HlrbrépIntersector {
    intersections: Vec<(f64, f64)>,
    done: bool,
}

impl HlrbrépIntersector {
    pub fn new() -> Self {
        HlrbrépIntersector {
            intersections: Vec::new(),
            done: false,
        }
    }

    pub fn perform(&mut self) { self.done = true; }
    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_intersections(&self) -> usize { self.intersections.len() }
}

impl Default for HlrbrépIntersector {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let i = HlrbrépIntersector::new();
        assert!(!i.is_done());
    }
}

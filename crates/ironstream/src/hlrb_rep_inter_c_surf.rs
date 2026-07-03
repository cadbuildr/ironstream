// FILE: hlrb_rep_inter_c_surf.rs
// occt: HLRBRep_InterCSurf

pub struct HlrbrépInterCSurf {
    intersections: Vec<(f64, f64)>,
    done: bool,
}

impl HlrbrépInterCSurf {
    pub fn new() -> Self {
        HlrbrépInterCSurf {
            intersections: Vec::new(),
            done: false,
        }
    }

    pub fn perform(&mut self) { self.done = true; }
    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_intersections(&self) -> usize { self.intersections.len() }
    pub fn add_intersection(&mut self, u: f64, v: f64) { self.intersections.push((u, v)); }
}

impl Default for HlrbrépInterCSurf {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let ics = HlrbrépInterCSurf::new();
        assert!(!ics.is_done());
    }
}

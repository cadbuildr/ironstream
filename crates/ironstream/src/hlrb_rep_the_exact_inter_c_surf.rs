// FILE: hlrb_rep_the_exact_inter_c_surf.rs
// occt: HLRBRep_TheExactInterCSurf

#[derive(Clone, Debug)]
pub struct HLRBRepTheExactInterCSurf {
    points: Vec<(f64, f64, f64)>,
}

impl HLRBRepTheExactInterCSurf {
    pub fn new() -> Self {
        HLRBRepTheExactInterCSurf { points: Vec::new() }
    }

    pub fn nb_points(&self) -> usize {
        self.points.len()
    }

    pub fn point(&self, idx: usize) -> Option<(f64, f64, f64)> {
        self.points.get(idx).copied()
    }

    pub fn add_point(&mut self, p: (f64, f64, f64)) {
        self.points.push(p);
    }
}

impl Default for HLRBRepTheExactInterCSurf {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let inter = HLRBRepTheExactInterCSurf::new();
        assert_eq!(inter.nb_points(), 0);
    }

    #[test]
    fn test_add_point() {
        let mut inter = HLRBRepTheExactInterCSurf::new();
        inter.add_point((1.0, 2.0, 3.0));
        assert_eq!(inter.nb_points(), 1);
    }
}

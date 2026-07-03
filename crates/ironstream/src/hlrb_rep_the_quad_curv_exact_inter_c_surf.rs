// FILE: hlrb_rep_the_quad_curv_exact_inter_c_surf.rs
// occt: HLRBRep_TheQuadCurvExactInterCSurf

#[derive(Clone, Debug)]
pub struct HLRBRepTheQuadCurvExactInterCSurf {
    nb_points: usize,
}

impl HLRBRepTheQuadCurvExactInterCSurf {
    pub fn new() -> Self {
        HLRBRepTheQuadCurvExactInterCSurf { nb_points: 0 }
    }

    pub fn nb_points(&self) -> usize {
        self.nb_points
    }

    pub fn set_nb_points(&mut self, n: usize) {
        self.nb_points = n;
    }
}

impl Default for HLRBRepTheQuadCurvExactInterCSurf {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let inter = HLRBRepTheQuadCurvExactInterCSurf::new();
        assert_eq!(inter.nb_points(), 0);
    }

    #[test]
    fn test_set_nb_points() {
        let mut inter = HLRBRepTheQuadCurvExactInterCSurf::new();
        inter.set_nb_points(5);
        assert_eq!(inter.nb_points(), 5);
    }
}

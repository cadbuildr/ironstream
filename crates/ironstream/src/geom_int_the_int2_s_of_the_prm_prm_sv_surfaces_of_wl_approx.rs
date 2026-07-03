// FILE: geom_int_the_int2_s_of_the_prm_prm_sv_surfaces_of_wl_approx.rs
// occt: GeomInt_TheInt2SOfThePrmPrmSvSurfacesOfWLApprox

pub struct GeomIntTheInt2S {
    solutions: usize,
}

impl GeomIntTheInt2S {
    pub fn new() -> Self {
        GeomIntTheInt2S { solutions: 0 }
    }

    pub fn nb_solutions(&self) -> usize {
        self.solutions
    }

    pub fn set_nb_solutions(&mut self, n: usize) {
        self.solutions = n;
    }
}

impl Default for GeomIntTheInt2S {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let int2s = GeomIntTheInt2S::new();
        assert_eq!(int2s.nb_solutions(), 0);
    }

    #[test]
    fn test_set_nb_solutions() {
        let mut int2s = GeomIntTheInt2S::new();
        int2s.set_nb_solutions(3);
        assert_eq!(int2s.nb_solutions(), 3);
    }
}

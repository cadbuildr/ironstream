// FILE: int_curve_surface_the_quad_curv_exact_h_inter.rs
// occt: IntCurveSurface_TheQuadCurvExactHInter

pub struct IntCurveSurfaceTheQuadCurvExactHInter {
    solutions: usize,
}

impl IntCurveSurfaceTheQuadCurvExactHInter {
    pub fn new() -> Self {
        IntCurveSurfaceTheQuadCurvExactHInter { solutions: 0 }
    }

    pub fn nb_solutions(&self) -> usize {
        self.solutions
    }

    pub fn set_nb_solutions(&mut self, n: usize) {
        self.solutions = n;
    }
}

impl Default for IntCurveSurfaceTheQuadCurvExactHInter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let inter = IntCurveSurfaceTheQuadCurvExactHInter::new();
        assert_eq!(inter.nb_solutions(), 0);
    }

    #[test]
    fn test_set_nb_solutions() {
        let mut inter = IntCurveSurfaceTheQuadCurvExactHInter::new();
        inter.set_nb_solutions(2);
        assert_eq!(inter.nb_solutions(), 2);
    }
}

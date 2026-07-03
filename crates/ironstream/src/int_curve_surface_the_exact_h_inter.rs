// FILE: int_curve_surface_the_exact_h_inter.rs
// occt: IntCurveSurface_TheExactHInter

pub struct IntCurveSurfaceTheExactHInter {
    done: bool,
}

impl IntCurveSurfaceTheExactHInter {
    pub fn new() -> Self {
        IntCurveSurfaceTheExactHInter { done: false }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self, d: bool) {
        self.done = d;
    }
}

impl Default for IntCurveSurfaceTheExactHInter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let inter = IntCurveSurfaceTheExactHInter::new();
        assert!(!inter.is_done());
    }

    #[test]
    fn test_set_done() {
        let mut inter = IntCurveSurfaceTheExactHInter::new();
        inter.set_done(true);
        assert!(inter.is_done());
    }
}

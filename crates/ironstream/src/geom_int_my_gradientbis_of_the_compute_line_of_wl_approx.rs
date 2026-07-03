// FILE: geom_int_my_gradientbis_of_the_compute_line_of_wl_approx.rs
// occt: GeomInt_MyGradientbisOfTheComputeLineOfWLApprox

pub struct GeomIntMyGradientbis {
    done: bool,
}

impl GeomIntMyGradientbis {
    pub fn new() -> Self {
        GeomIntMyGradientbis { done: false }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self, d: bool) {
        self.done = d;
    }
}

impl Default for GeomIntMyGradientbis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let grad = GeomIntMyGradientbis::new();
        assert!(!grad.is_done());
    }

    #[test]
    fn test_set_done() {
        let mut grad = GeomIntMyGradientbis::new();
        grad.set_done(true);
        assert!(grad.is_done());
    }
}

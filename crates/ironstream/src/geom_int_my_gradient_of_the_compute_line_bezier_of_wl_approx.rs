// FILE: geom_int_my_gradient_of_the_compute_line_bezier_of_wl_approx.rs
// occt: GeomInt_MyGradientOfTheComputeLineBezierOfWLApprox

pub struct GeomIntMyGradientBezier {
    done: bool,
}

impl GeomIntMyGradientBezier {
    pub fn new() -> Self {
        GeomIntMyGradientBezier { done: false }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self, d: bool) {
        self.done = d;
    }
}

impl Default for GeomIntMyGradientBezier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let grad = GeomIntMyGradientBezier::new();
        assert!(!grad.is_done());
    }

    #[test]
    fn test_set_done() {
        let mut grad = GeomIntMyGradientBezier::new();
        grad.set_done(true);
        assert!(grad.is_done());
    }
}

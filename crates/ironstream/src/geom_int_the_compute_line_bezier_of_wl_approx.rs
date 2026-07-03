// FILE: geom_int_the_compute_line_bezier_of_wl_approx.rs
// occt: GeomInt_TheComputeLineBezierOfWLApprox

pub struct GeomIntTheComputeLineBezier {
    done: bool,
}

impl GeomIntTheComputeLineBezier {
    pub fn new() -> Self {
        GeomIntTheComputeLineBezier { done: false }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self, d: bool) {
        self.done = d;
    }
}

impl Default for GeomIntTheComputeLineBezier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let compute = GeomIntTheComputeLineBezier::new();
        assert!(!compute.is_done());
    }

    #[test]
    fn test_set_done() {
        let mut compute = GeomIntTheComputeLineBezier::new();
        compute.set_done(true);
        assert!(compute.is_done());
    }
}

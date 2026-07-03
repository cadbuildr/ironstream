// FILE: geom_int_the_compute_line_of_wl_approx.rs
// occt: GeomInt_TheComputeLineOfWLApprox

pub struct GeomIntTheComputeLine {
    done: bool,
}

impl GeomIntTheComputeLine {
    pub fn new() -> Self {
        GeomIntTheComputeLine { done: false }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self, d: bool) {
        self.done = d;
    }
}

impl Default for GeomIntTheComputeLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let compute = GeomIntTheComputeLine::new();
        assert!(!compute.is_done());
    }

    #[test]
    fn test_set_done() {
        let mut compute = GeomIntTheComputeLine::new();
        compute.set_done(true);
        assert!(compute.is_done());
    }
}

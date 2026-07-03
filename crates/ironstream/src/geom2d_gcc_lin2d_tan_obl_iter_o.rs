// FILE: geom2d_gcc_lin2d_tan_obl_iter_o.rs
// occt: Geom2dGcc_Lin2dTanOblIter

/// Iterative solver for line with oblique tangency.
pub struct Lin2dTanOblIter {
    done: bool,
}

impl Lin2dTanOblIter {
    pub fn new() -> Self {
        Lin2dTanOblIter { done: false }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}

impl Default for Lin2dTanOblIter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solver = Lin2dTanOblIter::new();
        assert!(!solver.is_done());
    }
}

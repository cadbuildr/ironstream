// FILE: geom2d_gcc_lin2d2_tan_iter_o.rs
// occt: Geom2dGcc_Lin2d2TanIter

/// Iterative solver for line tangent to two curves.
pub struct Lin2d2TanIter {
    done: bool,
}

impl Lin2d2TanIter {
    pub fn new() -> Self {
        Lin2d2TanIter { done: false }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}

impl Default for Lin2d2TanIter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solver = Lin2d2TanIter::new();
        assert!(!solver.is_done());
    }
}

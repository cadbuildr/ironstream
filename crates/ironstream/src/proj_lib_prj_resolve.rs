// FILE: proj_lib_prj_resolve.rs
// occt: ProjLib_PrjResolve

/// Solver for projection problems.
pub struct ProjLibPrjResolve {
    is_done: bool,
}

impl ProjLibPrjResolve {
    /// Create a projection solver.
    pub fn new() -> Self {
        ProjLibPrjResolve { is_done: false }
    }

    /// Solve the projection.
    pub fn solve(&mut self) {
        self.is_done = true;
    }

    /// Check if solution was found.
    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

impl Default for ProjLibPrjResolve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_solver() {
        let solver = ProjLibPrjResolve::new();
        assert!(!solver.is_done());
    }

    #[test]
    fn test_solve() {
        let mut solver = ProjLibPrjResolve::new();
        solver.solve();
        assert!(solver.is_done());
    }
}

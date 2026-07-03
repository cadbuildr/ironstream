// FILE: geom2d_gcc_circ2d2_tan_on_iter_o.rs
// occt: Geom2dGcc_Circ2d2TanOnIter

/// Iterative solver for circle tangent to two geometries and on another.
pub struct Circ2d2TanOnIter {
    done: bool,
}

impl Circ2d2TanOnIter {
    /// Initialize solver.
    pub fn new() -> Self {
        Circ2d2TanOnIter { done: false }
    }

    /// Returns computation status.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

impl Default for Circ2d2TanOnIter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solver = Circ2d2TanOnIter::new();
        assert!(!solver.is_done());
    }

    #[test]
    fn test_default() {
        let solver = Circ2d2TanOnIter::default();
        assert!(!solver.is_done());
    }
}

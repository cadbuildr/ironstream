// FILE: geom2d_gcc_circ2d3_tan_iter_o.rs
// occt: Geom2dGcc_Circ2d3TanIter

/// Iterative solver for circle tangent to three geometries.
pub struct Circ2d3TanIter {
    done: bool,
}

impl Circ2d3TanIter {
    pub fn new() -> Self {
        Circ2d3TanIter { done: false }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}

impl Default for Circ2d3TanIter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solver = Circ2d3TanIter::new();
        assert!(!solver.is_done());
    }
}

// FILE: b_rep_sweep_num_linear_regular_sweep.rs
// occt: BRepSweep_NumLinearRegularSweep

/// Generic class used to build swept primitives.
#[derive(Debug, Clone)]
pub struct BRepSweepNumLinearRegularSweep;

impl BRepSweepNumLinearRegularSweep {
    pub fn new() -> Self {
        BRepSweepNumLinearRegularSweep
    }

    pub fn closed(&self) -> bool {
        false // Placeholder
    }
}

impl Default for BRepSweepNumLinearRegularSweep {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_linear_sweep() {
        let _nls = BRepSweepNumLinearRegularSweep::new();
    }
}

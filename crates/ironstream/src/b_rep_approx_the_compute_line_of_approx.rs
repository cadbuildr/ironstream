// FILE: b_rep_approx_the_compute_line_of_approx.rs
// occt: BRepApprox_TheComputeLineOfApprox

pub struct BrepapproxThecomputelineofapprox;

impl BrepapproxThecomputelineofapprox {
    pub fn new() -> Self {
        BrepapproxThecomputelineofapprox
    }
}

impl Default for BrepapproxThecomputelineofapprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxThecomputelineofapprox::new();
    }
}

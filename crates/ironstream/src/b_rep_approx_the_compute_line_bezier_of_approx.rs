// FILE: b_rep_approx_the_compute_line_bezier_of_approx.rs
// occt: BRepApprox_TheComputeLineBezierOfApprox

pub struct BrepapproxThecomputelinebezierofapprox;

impl BrepapproxThecomputelinebezierofapprox {
    pub fn new() -> Self {
        BrepapproxThecomputelinebezierofapprox
    }
}

impl Default for BrepapproxThecomputelinebezierofapprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxThecomputelinebezierofapprox::new();
    }
}

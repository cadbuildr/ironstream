// FILE: b_rep_approx_res_constraint_of_my_gradientbis_of_the_compute_line_of_approx.rs
// occt: BRepApprox_ResConstraintOfMyGradientbisOfTheComputeLineOfApprox

pub struct BrepapproxResconstraintofmygradientbisofthecomputelineofapprox;

impl BrepapproxResconstraintofmygradientbisofthecomputelineofapprox {
    pub fn new() -> Self {
        BrepapproxResconstraintofmygradientbisofthecomputelineofapprox
    }
}

impl Default for BrepapproxResconstraintofmygradientbisofthecomputelineofapprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxResconstraintofmygradientbisofthecomputelineofapprox::new();
    }
}

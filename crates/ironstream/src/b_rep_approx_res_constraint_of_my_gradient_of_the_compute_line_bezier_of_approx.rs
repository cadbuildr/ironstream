// FILE: b_rep_approx_res_constraint_of_my_gradient_of_the_compute_line_bezier_of_approx.rs
// occt: BRepApprox_ResConstraintOfMyGradientOfTheComputeLineBezierOfApprox

pub struct BrepapproxResconstraintofmygradientofthecomputelinebezierofapprox;

impl BrepapproxResconstraintofmygradientofthecomputelinebezierofapprox {
    pub fn new() -> Self {
        BrepapproxResconstraintofmygradientofthecomputelinebezierofapprox
    }
}

impl Default for BrepapproxResconstraintofmygradientofthecomputelinebezierofapprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxResconstraintofmygradientofthecomputelinebezierofapprox::new();
    }
}

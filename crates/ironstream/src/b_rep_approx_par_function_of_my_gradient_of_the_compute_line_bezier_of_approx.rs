// FILE: b_rep_approx_par_function_of_my_gradient_of_the_compute_line_bezier_of_approx.rs
// occt: BRepApprox_ParFunctionOfMyGradientOfTheComputeLineBezierOfApprox

pub struct BrepapproxParfunctionofmygradientofthecomputelinebezierofapprox;

impl BrepapproxParfunctionofmygradientofthecomputelinebezierofapprox {
    pub fn new() -> Self {
        BrepapproxParfunctionofmygradientofthecomputelinebezierofapprox
    }
}

impl Default for BrepapproxParfunctionofmygradientofthecomputelinebezierofapprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxParfunctionofmygradientofthecomputelinebezierofapprox::new();
    }
}

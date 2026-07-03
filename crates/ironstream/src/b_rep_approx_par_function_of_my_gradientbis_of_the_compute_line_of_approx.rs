// FILE: b_rep_approx_par_function_of_my_gradientbis_of_the_compute_line_of_approx.rs
// occt: BRepApprox_ParFunctionOfMyGradientbisOfTheComputeLineOfApprox

pub struct BrepapproxParfunctionofmygradientbisofthecomputelineofapprox;

impl BrepapproxParfunctionofmygradientbisofthecomputelineofapprox {
    pub fn new() -> Self {
        BrepapproxParfunctionofmygradientbisofthecomputelineofapprox
    }
}

impl Default for BrepapproxParfunctionofmygradientbisofthecomputelineofapprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxParfunctionofmygradientbisofthecomputelineofapprox::new();
    }
}

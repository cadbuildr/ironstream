// FILE: b_rep_approx_par_least_square_of_my_gradient_of_the_compute_line_bezier_of_approx.rs
// occt: BRepApprox_ParLeastSquareOfMyGradientOfTheComputeLineBezierOfApprox

pub struct BrepapproxParleastsquareofmygradientofthecomputelinebezierofapprox;

impl BrepapproxParleastsquareofmygradientofthecomputelinebezierofapprox {
    pub fn new() -> Self {
        BrepapproxParleastsquareofmygradientofthecomputelinebezierofapprox
    }
}

impl Default for BrepapproxParleastsquareofmygradientofthecomputelinebezierofapprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxParleastsquareofmygradientofthecomputelinebezierofapprox::new();
    }
}

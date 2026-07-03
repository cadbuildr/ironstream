// FILE: b_rep_approx_par_least_square_of_my_gradientbis_of_the_compute_line_of_approx.rs
// occt: BRepApprox_ParLeastSquareOfMyGradientbisOfTheComputeLineOfApprox

pub struct BrepapproxParleastsquareofmygradientbisofthecomputelineofapprox;

impl BrepapproxParleastsquareofmygradientbisofthecomputelineofapprox {
    pub fn new() -> Self {
        BrepapproxParleastsquareofmygradientbisofthecomputelineofapprox
    }
}

impl Default for BrepapproxParleastsquareofmygradientbisofthecomputelineofapprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxParleastsquareofmygradientbisofthecomputelineofapprox::new();
    }
}

// FILE: b_rep_approx_my_gradientbis_of_the_compute_line_of_approx.rs
// occt: BRepApprox_MyGradientbisOfTheComputeLineOfApprox

pub struct BrepapproxMygradientbisofthecomputelineofapprox;

impl BrepapproxMygradientbisofthecomputelineofapprox {
    pub fn new() -> Self {
        BrepapproxMygradientbisofthecomputelineofapprox
    }
}

impl Default for BrepapproxMygradientbisofthecomputelineofapprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxMygradientbisofthecomputelineofapprox::new();
    }
}

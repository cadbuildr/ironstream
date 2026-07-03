// FILE: b_rep_approx_my_b_spl_gradient_of_the_compute_line_of_approx.rs
// occt: BRepApprox_MyBSplGradientOfTheComputeLineOfApprox

pub struct BrepapproxMybsplgradientofthecomputelineofapprox;

impl BrepapproxMybsplgradientofthecomputelineofapprox {
    pub fn new() -> Self {
        BrepapproxMybsplgradientofthecomputelineofapprox
    }
}

impl Default for BrepapproxMybsplgradientofthecomputelineofapprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxMybsplgradientofthecomputelineofapprox::new();
    }
}

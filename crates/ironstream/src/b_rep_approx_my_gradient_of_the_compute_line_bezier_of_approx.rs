// FILE: b_rep_approx_my_gradient_of_the_compute_line_bezier_of_approx.rs
// occt: BRepApprox_MyGradientOfTheComputeLineBezierOfApprox

pub struct BrepapproxMygradientofthecomputelinebezierofapprox;

impl BrepapproxMygradientofthecomputelinebezierofapprox {
    pub fn new() -> Self {
        BrepapproxMygradientofthecomputelinebezierofapprox
    }
}

impl Default for BrepapproxMygradientofthecomputelinebezierofapprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxMygradientofthecomputelinebezierofapprox::new();
    }
}

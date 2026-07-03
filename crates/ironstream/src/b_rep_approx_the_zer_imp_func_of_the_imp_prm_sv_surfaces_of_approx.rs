// FILE: b_rep_approx_the_zer_imp_func_of_the_imp_prm_sv_surfaces_of_approx.rs
// occt: BRepApprox_TheZerImpFuncOfTheImpPrmSvSurfacesOfApprox

pub struct BrepapproxThezerimpfuncoftheimpprmsvsurfacesofapprox;

impl BrepapproxThezerimpfuncoftheimpprmsvsurfacesofapprox {
    pub fn new() -> Self {
        BrepapproxThezerimpfuncoftheimpprmsvsurfacesofapprox
    }
}

impl Default for BrepapproxThezerimpfuncoftheimpprmsvsurfacesofapprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxThezerimpfuncoftheimpprmsvsurfacesofapprox::new();
    }
}

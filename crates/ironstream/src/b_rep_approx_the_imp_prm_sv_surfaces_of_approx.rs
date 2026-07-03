// FILE: b_rep_approx_the_imp_prm_sv_surfaces_of_approx.rs
// occt: BRepApprox_TheImpPrmSvSurfacesOfApprox

pub struct BrepapproxTheimpprmsvsurfacesofapprox;

impl BrepapproxTheimpprmsvsurfacesofapprox {
    pub fn new() -> Self {
        BrepapproxTheimpprmsvsurfacesofapprox
    }
}

impl Default for BrepapproxTheimpprmsvsurfacesofapprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxTheimpprmsvsurfacesofapprox::new();
    }
}

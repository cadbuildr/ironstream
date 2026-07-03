// FILE: b_rep_approx_the_prm_prm_sv_surfaces_of_approx.rs
// occt: BRepApprox_ThePrmPrmSvSurfacesOfApprox

pub struct BrepapproxTheprmprmsvsurfacesofapprox;

impl BrepapproxTheprmprmsvsurfacesofapprox {
    pub fn new() -> Self {
        BrepapproxTheprmprmsvsurfacesofapprox
    }
}

impl Default for BrepapproxTheprmprmsvsurfacesofapprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxTheprmprmsvsurfacesofapprox::new();
    }
}

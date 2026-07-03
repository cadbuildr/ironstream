// FILE: b_rep_approx_the_int2_s_of_the_prm_prm_sv_surfaces_of_approx.rs
// occt: BRepApprox_TheInt2SOfThePrmPrmSvSurfacesOfApprox

pub struct BrepapproxTheint2softheprmprmsvsurfacesofapprox;

impl BrepapproxTheint2softheprmprmsvsurfacesofapprox {
    pub fn new() -> Self {
        BrepapproxTheint2softheprmprmsvsurfacesofapprox
    }
}

impl Default for BrepapproxTheint2softheprmprmsvsurfacesofapprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxTheint2softheprmprmsvsurfacesofapprox::new();
    }
}

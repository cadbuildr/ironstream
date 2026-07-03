// FILE: geom_int_the_function_of_the_int2_s_of_the_prm_prm_sv_surfaces_of_wl_approx.rs
// occt: GeomInt_TheFunctionOfTheInt2SOfThePrmPrmSvSurfacesOfWLApprox

pub struct GeomIntTheFunctionOfTheInt2S {
    nb_vars: i32,
}

impl GeomIntTheFunctionOfTheInt2S {
    pub fn new() -> Self {
        GeomIntTheFunctionOfTheInt2S { nb_vars: 2 }
    }

    pub fn nb_vars(&self) -> i32 {
        self.nb_vars
    }
}

impl Default for GeomIntTheFunctionOfTheInt2S {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let func = GeomIntTheFunctionOfTheInt2S::new();
        assert_eq!(func.nb_vars(), 2);
    }
}

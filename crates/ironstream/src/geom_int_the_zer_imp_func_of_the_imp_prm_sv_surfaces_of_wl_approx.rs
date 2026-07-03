// FILE: geom_int_the_zer_imp_func_of_the_imp_prm_sv_surfaces_of_wl_approx.rs
// occt: GeomInt_TheZerImpFuncOfTheImpPrmSvSurfacesOfWLApprox

pub struct GeomIntTheZerImpFunc {
    value: f64,
}

impl GeomIntTheZerImpFunc {
    pub fn new() -> Self {
        GeomIntTheZerImpFunc { value: 0.0 }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }
}

impl Default for GeomIntTheZerImpFunc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let func = GeomIntTheZerImpFunc::new();
        assert_eq!(func.value(), 0.0);
    }

    #[test]
    fn test_set_value() {
        let mut func = GeomIntTheZerImpFunc::new();
        func.set_value(1.5);
        assert_eq!(func.value(), 1.5);
    }
}

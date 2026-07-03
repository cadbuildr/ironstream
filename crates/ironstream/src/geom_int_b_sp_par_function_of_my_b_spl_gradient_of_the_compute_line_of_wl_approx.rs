// FILE: geom_int_b_sp_par_function_of_my_b_spl_gradient_of_the_compute_line_of_wl_approx.rs
// occt: GeomInt_BSpParFunctionOfMyBSplGradientOfTheComputeLineOfWLApprox

pub struct GeomIntBSpParFunction {
    nb_vars: i32,
}

impl GeomIntBSpParFunction {
    pub fn new() -> Self {
        GeomIntBSpParFunction { nb_vars: 0 }
    }

    pub fn nb_vars(&self) -> i32 {
        self.nb_vars
    }
}

impl Default for GeomIntBSpParFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let func = GeomIntBSpParFunction::new();
        assert_eq!(func.nb_vars(), 0);
    }
}

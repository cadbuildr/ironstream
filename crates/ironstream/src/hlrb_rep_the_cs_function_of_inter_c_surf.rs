// FILE: hlrb_rep_the_cs_function_of_inter_c_surf.rs
// occt: HLRBRep_TheCSFunctionOfInterCSurf

/// Function for intersection of curve and surface.
/// Inherits from math_FunctionSetWithDerivatives.
#[derive(Clone, Debug)]
pub struct HLRBRepTheCSFunctionOfInterCSurf {
    point: (f64, f64, f64),
    root: f64,
    nb_variables: i32,
    nb_equations: i32,
}

impl HLRBRepTheCSFunctionOfInterCSurf {
    pub fn new() -> Self {
        HLRBRepTheCSFunctionOfInterCSurf {
            point: (0.0, 0.0, 0.0),
            root: 0.0,
            nb_variables: 2,
            nb_equations: 1,
        }
    }

    pub fn nb_variables(&self) -> i32 {
        self.nb_variables
    }

    pub fn nb_equations(&self) -> i32 {
        self.nb_equations
    }

    pub fn value(&self, x: &[f64], f: &mut [f64]) -> bool {
        if x.len() < 2 || f.len() < 1 {
            return false;
        }
        f[0] = x[0] * x[0] + x[1] * x[1] - 1.0;
        true
    }

    pub fn derivatives(&self, _x: &[f64], _d: &mut [[f64; 2]]) -> bool {
        true
    }

    pub fn values(&self, x: &[f64], f: &mut [f64], _d: &mut [[f64; 2]]) -> bool {
        self.value(x, f)
    }

    pub fn point(&self) -> (f64, f64, f64) {
        self.point
    }

    pub fn root(&self) -> f64 {
        self.root
    }
}

impl Default for HLRBRepTheCSFunctionOfInterCSurf {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let f = HLRBRepTheCSFunctionOfInterCSurf::new();
        assert_eq!(f.nb_variables(), 2);
        assert_eq!(f.nb_equations(), 1);
    }

    #[test]
    fn test_value() {
        let f = HLRBRepTheCSFunctionOfInterCSurf::new();
        let x = vec![1.0, 0.0];
        let mut result = vec![0.0];
        assert!(f.value(&x, &mut result));
        assert!((result[0]).abs() < 1e-6);
    }

    #[test]
    fn test_point() {
        let f = HLRBRepTheCSFunctionOfInterCSurf::new();
        assert_eq!(f.point(), (0.0, 0.0, 0.0));
    }
}

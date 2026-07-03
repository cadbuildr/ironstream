// FILE: hlrb_rep_the_quad_curv_func_of_the_quad_curv_exact_inter_c_surf.rs
// occt: HLRBRep_TheQuadCurvFuncOfTheQuadCurvExactInterCSurf

#[derive(Clone, Debug)]
pub struct HLRBRepTheQuadCurvFunc {
    value: f64,
}

impl HLRBRepTheQuadCurvFunc {
    pub fn new() -> Self {
        HLRBRepTheQuadCurvFunc { value: 0.0 }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn evaluate(&mut self, x: f64, y: f64) -> f64 {
        self.value = x * x + y * y;
        self.value
    }
}

impl Default for HLRBRepTheQuadCurvFunc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let func = HLRBRepTheQuadCurvFunc::new();
        assert_eq!(func.value(), 0.0);
    }

    #[test]
    fn test_evaluate() {
        let mut func = HLRBRepTheQuadCurvFunc::new();
        let result = func.evaluate(3.0, 4.0);
        assert_eq!(result, 25.0);
    }
}

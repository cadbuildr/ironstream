// FILE: int_curve_surface_the_quad_curv_func_of_the_quad_curv_exact_h_inter.rs
// occt: IntCurveSurface_TheQuadCurvFuncOfTheQuadCurvExactHInter

pub struct IntCurveSurfaceTheQuadCurvFunc {
    value: f64,
}

impl IntCurveSurfaceTheQuadCurvFunc {
    pub fn new() -> Self {
        IntCurveSurfaceTheQuadCurvFunc { value: 0.0 }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }
}

impl Default for IntCurveSurfaceTheQuadCurvFunc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let func = IntCurveSurfaceTheQuadCurvFunc::new();
        assert_eq!(func.value(), 0.0);
    }

    #[test]
    fn test_set_value() {
        let mut func = IntCurveSurfaceTheQuadCurvFunc::new();
        func.set_value(2.5);
        assert_eq!(func.value(), 2.5);
    }
}

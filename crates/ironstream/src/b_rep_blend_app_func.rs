// FILE: b_rep_blend_app_func.rs
// occt: BRepBlend_AppFunc

#[derive(Clone, Debug)]
pub struct BRepBlendAppFunc {
    nb_variables: i32,
    nb_equations: i32,
}

impl BRepBlendAppFunc {
    pub fn new() -> Self {
        BRepBlendAppFunc {
            nb_variables: 3,
            nb_equations: 3,
        }
    }

    pub fn nb_variables(&self) -> i32 {
        self.nb_variables
    }

    pub fn nb_equations(&self) -> i32 {
        self.nb_equations
    }

    pub fn value(&self, _x: &[f64], _f: &mut [f64]) -> bool {
        true
    }
}

impl Default for BRepBlendAppFunc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let func = BRepBlendAppFunc::new();
        assert_eq!(func.nb_variables(), 3);
    }
}

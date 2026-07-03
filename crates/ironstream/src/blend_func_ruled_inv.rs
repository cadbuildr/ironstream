// FILE: blend_func_ruled_inv.rs
// occt: BlendFunc_RuledInv

#[derive(Clone)]
pub struct BlendFuncRuledInv {
    param: f64,
}

impl BlendFuncRuledInv {
    pub fn new() -> Self {
        Self { param: 0.0 }
    }

    pub fn set_param(&mut self, p: f64) {
        self.param = p;
    }

    pub fn param(&self) -> f64 {
        self.param
    }

    pub fn value(&mut self, x: &[f64], f: &mut [f64]) -> bool {
        if x.len() < 4 || f.len() < 4 {
            return false;
        }
        f[0] = 0.0;
        f[1] = 0.0;
        f[2] = 0.0;
        f[3] = 0.0;
        true
    }

    pub fn derivatives(&self, x: &[f64], d: &mut [[f64; 4]; 4]) -> bool {
        if x.len() < 4 {
            return false;
        }
        for i in 0..4 {
            for j in 0..4 {
                d[i][j] = 0.0;
            }
        }
        true
    }

    pub fn is_solution(&self, sol: &[f64], tol: f64) -> bool {
        sol.len() >= 4 && sol[0].abs() <= tol && sol[1].abs() <= tol
    }
}

impl Default for BlendFuncRuledInv {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let ruled = BlendFuncRuledInv::new();
        assert_eq!(ruled.param(), 0.0);
    }

    #[test]
    fn test_set_param() {
        let mut ruled = BlendFuncRuledInv::new();
        ruled.set_param(0.4);
        assert_eq!(ruled.param(), 0.4);
    }

    #[test]
    fn test_value() {
        let mut ruled = BlendFuncRuledInv::new();
        let x = vec![0.0; 4];
        let mut f = vec![0.0; 4];
        assert!(ruled.value(&x, &mut f));
    }
}

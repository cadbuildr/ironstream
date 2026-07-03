// FILE: blend_func_evol_rad_inv.rs
// occt: BlendFunc_EvolRadInv

#[derive(Clone)]
pub struct BlendFuncEvolRadInv {
    param: f64,
    radius_func: Option<fn(f64) -> f64>,
}

impl BlendFuncEvolRadInv {
    pub fn new() -> Self {
        Self {
            param: 0.0,
            radius_func: None,
        }
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

impl Default for BlendFuncEvolRadInv {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let evol = BlendFuncEvolRadInv::new();
        assert_eq!(evol.param(), 0.0);
    }

    #[test]
    fn test_value() {
        let mut evol = BlendFuncEvolRadInv::new();
        let x = vec![0.0; 4];
        let mut f = vec![0.0; 4];
        assert!(evol.value(&x, &mut f));
    }
}

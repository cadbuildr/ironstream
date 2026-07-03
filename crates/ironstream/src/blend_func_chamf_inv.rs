// FILE: blend_func_chamf_inv.rs
// occt: BlendFunc_ChamfInv

#[derive(Clone)]
pub struct BlendFuncChamfInv {
    dist1: f64,
    dist2: f64,
}

impl BlendFuncChamfInv {
    pub fn new() -> Self {
        Self {
            dist1: 0.0,
            dist2: 0.0,
        }
    }

    pub fn set(&mut self, d1: f64, d2: f64, _choix: i32) {
        self.dist1 = d1;
        self.dist2 = d2;
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

impl Default for BlendFuncChamfInv {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let chamf = BlendFuncChamfInv::new();
        assert_eq!(chamf.dist1, 0.0);
    }

    #[test]
    fn test_set() {
        let mut chamf = BlendFuncChamfInv::new();
        chamf.set(1.0, 2.0, 1);
        assert_eq!(chamf.dist1, 1.0);
        assert_eq!(chamf.dist2, 2.0);
    }

    #[test]
    fn test_value() {
        let mut chamf = BlendFuncChamfInv::new();
        let x = vec![0.0; 4];
        let mut f = vec![0.0; 4];
        assert!(chamf.value(&x, &mut f));
    }
}

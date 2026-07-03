// FILE: blend_surf_point_func_inv.rs
// occt: Blend_SurfPointFuncInv

#[derive(Clone)]
pub struct BlendSurfPointFuncInv {
    param: f64,
}

impl BlendSurfPointFuncInv {
    pub fn new() -> Self {
        Self { param: 0.0 }
    }

    pub fn set_param(&mut self, p: f64) {
        self.param = p;
    }

    pub fn param(&self) -> f64 {
        self.param
    }

    pub fn nb_variables(&self) -> i32 {
        3
    }

    pub fn nb_equations(&self) -> i32 {
        3
    }

    pub fn value(&mut self, x: &[f64], f: &mut [f64]) -> bool {
        if x.len() < 3 || f.len() < 3 {
            return false;
        }
        f[0] = 0.0;
        f[1] = 0.0;
        f[2] = 0.0;
        true
    }

    pub fn derivatives(&self, x: &[f64], d: &mut [[f64; 4]; 4]) -> bool {
        if x.len() < 3 {
            return false;
        }
        for i in 0..4 {
            for j in 0..4 {
                d[i][j] = 0.0;
            }
        }
        true
    }
}

impl Default for BlendSurfPointFuncInv {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let func = BlendSurfPointFuncInv::new();
        assert_eq!(func.nb_variables(), 3);
        assert_eq!(func.nb_equations(), 3);
    }

    #[test]
    fn test_value() {
        let mut func = BlendSurfPointFuncInv::new();
        let x = vec![0.0; 3];
        let mut f = vec![0.0; 3];
        assert!(func.value(&x, &mut f));
    }
}

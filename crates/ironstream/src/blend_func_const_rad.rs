// FILE: blend_func_const_rad.rs
// occt: BlendFunc_ConstRad

#[derive(Clone)]
pub struct BlendFuncConstRad {
    radius: f64,
    param: f64,
    sign1: f64,
    sign2: f64,
}

impl BlendFuncConstRad {
    pub fn new() -> Self {
        Self {
            radius: 0.0,
            param: 0.0,
            sign1: 0.0,
            sign2: 0.0,
        }
    }

    pub fn set(&mut self, rad: f64, _unused: f64, choix: i32) {
        self.radius = rad;
        match choix {
            1 | 2 => {
                self.sign1 = -1.0;
                self.sign2 = -1.0;
            }
            3 | 4 => {
                self.sign1 = 1.0;
                self.sign2 = -1.0;
            }
            _ => {
                self.sign1 = -1.0;
                self.sign2 = -1.0;
            }
        }
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

impl Default for BlendFuncConstRad {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let rad = BlendFuncConstRad::new();
        assert_eq!(rad.radius, 0.0);
    }

    #[test]
    fn test_set() {
        let mut rad = BlendFuncConstRad::new();
        rad.set(5.0, 0.0, 1);
        assert_eq!(rad.radius, 5.0);
    }

    #[test]
    fn test_value() {
        let mut rad = BlendFuncConstRad::new();
        let x = vec![0.0; 4];
        let mut f = vec![0.0; 4];
        assert!(rad.value(&x, &mut f));
    }
}

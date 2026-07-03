// FILE: blend_func_inv.rs
// occt: Blend_FuncInv

#[derive(Clone)]
pub struct BlendFuncInv {
    nb_eqs: i32,
}

impl BlendFuncInv {
    pub fn new() -> Self {
        Self { nb_eqs: 4 }
    }

    pub fn set_nb_equations(&mut self, n: i32) {
        self.nb_eqs = n;
    }

    pub fn nb_variables(&self) -> i32 {
        4
    }

    pub fn nb_equations(&self) -> i32 {
        self.nb_eqs
    }

    pub fn value(&mut self, x: &[f64], f: &mut [f64]) -> bool {
        if x.len() < 4 || f.len() < self.nb_eqs as usize {
            return false;
        }
        for v in f.iter_mut() {
            *v = 0.0;
        }
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

    pub fn values(
        &mut self,
        x: &[f64],
        f: &mut [f64],
        d: &mut [[f64; 4]; 4],
    ) -> bool {
        self.value(x, f) && self.derivatives(x, d)
    }

    pub fn is_solution(&self, sol: &[f64], tol: f64) -> bool {
        sol.len() >= 4 && sol[0].abs() <= tol && sol[1].abs() <= tol
    }
}

impl Default for BlendFuncInv {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let func = BlendFuncInv::new();
        assert_eq!(func.nb_variables(), 4);
        assert_eq!(func.nb_equations(), 4);
    }

    #[test]
    fn test_set_nb_equations() {
        let mut func = BlendFuncInv::new();
        func.set_nb_equations(3);
        assert_eq!(func.nb_equations(), 3);
    }

    #[test]
    fn test_value() {
        let mut func = BlendFuncInv::new();
        let x = vec![0.0; 4];
        let mut f = vec![0.0; 4];
        assert!(func.value(&x, &mut f));
    }

    #[test]
    fn test_is_solution() {
        let func = BlendFuncInv::new();
        let sol = vec![0.0, 0.0, 0.0, 0.0];
        assert!(func.is_solution(&sol, 0.1));
    }
}

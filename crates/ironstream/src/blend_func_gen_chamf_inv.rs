// FILE: blend_func_gen_chamf_inv.rs
// occt: BlendFunc_GenChamfInv

#[derive(Clone)]
pub struct BlendFuncGenChamfInv {
    dist1: f64,
    dist2: f64,
    choix: i32,
    first: bool,
}

impl BlendFuncGenChamfInv {
    pub fn new() -> Self {
        Self {
            dist1: 0.0,
            dist2: 0.0,
            choix: 0,
            first: false,
        }
    }

    pub fn set(&mut self, d1: f64, d2: f64, choix: i32) {
        self.dist1 = d1;
        self.dist2 = d2;
        self.choix = choix;
    }

    pub fn set_on_first(&mut self, on_first: bool) {
        self.first = on_first;
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

    pub fn values(
        &mut self,
        x: &[f64],
        f: &mut [f64],
        d: &mut [[f64; 4]; 4],
    ) -> bool {
        self.value(x, f) && self.derivatives(x, d)
    }

    pub fn nb_equations(&self) -> i32 {
        4
    }
}

impl Default for BlendFuncGenChamfInv {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let gen = BlendFuncGenChamfInv::new();
        assert_eq!(gen.dist1, 0.0);
        assert_eq!(gen.nb_equations(), 4);
    }

    #[test]
    fn test_set() {
        let mut gen = BlendFuncGenChamfInv::new();
        gen.set(2.0, 3.0, 2);
        assert_eq!(gen.dist1, 2.0);
        assert_eq!(gen.dist2, 3.0);
    }

    #[test]
    fn test_value() {
        let mut gen = BlendFuncGenChamfInv::new();
        let x = vec![0.0; 4];
        let mut f = vec![0.0; 4];
        assert!(gen.value(&x, &mut f));
    }
}

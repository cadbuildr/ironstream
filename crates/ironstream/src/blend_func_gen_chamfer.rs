// FILE: blend_func_gen_chamfer.rs
// occt: BlendFunc_GenChamfer

#[derive(Clone)]
pub struct BlendFuncGenChamfer {
    dist1: f64,
    dist2: f64,
    choix: i32,
}

impl BlendFuncGenChamfer {
    pub fn new() -> Self {
        Self {
            dist1: 0.0,
            dist2: 0.0,
            choix: 0,
        }
    }

    pub fn set(&mut self, d1: f64, d2: f64, choix: i32) {
        self.dist1 = d1;
        self.dist2 = d2;
        self.choix = choix;
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

    pub fn nb_equations(&self) -> i32 {
        4
    }
}

impl Default for BlendFuncGenChamfer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let gen = BlendFuncGenChamfer::new();
        assert_eq!(gen.dist1, 0.0);
    }

    #[test]
    fn test_set() {
        let mut gen = BlendFuncGenChamfer::new();
        gen.set(1.5, 2.5, 1);
        assert_eq!(gen.dist1, 1.5);
        assert_eq!(gen.choix, 1);
    }

    #[test]
    fn test_nb_equations() {
        let gen = BlendFuncGenChamfer::new();
        assert_eq!(gen.nb_equations(), 4);
    }
}

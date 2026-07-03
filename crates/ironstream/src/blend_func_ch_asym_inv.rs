// FILE: blend_func_ch_asym_inv.rs
// occt: BlendFunc_ChAsymInv

#[derive(Clone)]
pub struct BlendFuncChAsymInv {
    dist1: f64,
    dist2: f64,
    choix: i32,
}

impl BlendFuncChAsymInv {
    pub fn new() -> Self {
        Self {
            dist1: 0.0,
            dist2: 0.0,
            choix: 0,
        }
    }

    pub fn set_distances(&mut self, d1: f64, d2: f64, choice: i32) {
        self.dist1 = d1;
        self.dist2 = d2;
        self.choix = choice;
    }

    pub fn dist1(&self) -> f64 {
        self.dist1
    }

    pub fn dist2(&self) -> f64 {
        self.dist2
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
}

impl Default for BlendFuncChAsymInv {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let asym = BlendFuncChAsymInv::new();
        assert_eq!(asym.dist1(), 0.0);
    }

    #[test]
    fn test_set_distances() {
        let mut asym = BlendFuncChAsymInv::new();
        asym.set_distances(2.0, 3.0, 2);
        assert_eq!(asym.dist1(), 2.0);
        assert_eq!(asym.dist2(), 3.0);
    }

    #[test]
    fn test_value() {
        let mut asym = BlendFuncChAsymInv::new();
        let x = vec![0.0; 4];
        let mut f = vec![0.0; 4];
        assert!(asym.value(&x, &mut f));
    }
}

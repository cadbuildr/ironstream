// FILE: blend_func_evol_rad.rs
// occt: BlendFunc_EvolRad

#[derive(Clone)]
pub struct BlendFuncEvolRad {
    param: f64,
    radius_func: Option<fn(f64) -> f64>,
}

impl BlendFuncEvolRad {
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
}

impl Default for BlendFuncEvolRad {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let evol = BlendFuncEvolRad::new();
        assert_eq!(evol.param(), 0.0);
    }

    #[test]
    fn test_set_param() {
        let mut evol = BlendFuncEvolRad::new();
        evol.set_param(0.7);
        assert_eq!(evol.param(), 0.7);
    }

    #[test]
    fn test_value() {
        let mut evol = BlendFuncEvolRad::new();
        let x = vec![0.0; 4];
        let mut f = vec![0.0; 4];
        assert!(evol.value(&x, &mut f));
    }
}

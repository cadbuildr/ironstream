// FILE: blend_rst_rst_function.rs
// occt: Blend_RstRstFunction

#[derive(Clone)]
pub struct BlendRstRstFunction {
    radius: f64,
}

impl BlendRstRstFunction {
    pub fn new() -> Self {
        Self { radius: 0.0 }
    }

    pub fn set_radius(&mut self, r: f64) {
        self.radius = r;
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn nb_variables(&self) -> i32 {
        4
    }

    pub fn nb_equations(&self) -> i32 {
        3
    }

    pub fn value(&mut self, x: &[f64], f: &mut [f64]) -> bool {
        if x.len() < 4 || f.len() < 3 {
            return false;
        }
        f[0] = 0.0;
        f[1] = 0.0;
        f[2] = 0.0;
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

impl Default for BlendRstRstFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let func = BlendRstRstFunction::new();
        assert_eq!(func.radius(), 0.0);
    }

    #[test]
    fn test_set_radius() {
        let mut func = BlendRstRstFunction::new();
        func.set_radius(2.5);
        assert_eq!(func.radius(), 2.5);
    }

    #[test]
    fn test_value() {
        let mut func = BlendRstRstFunction::new();
        let x = vec![0.0; 4];
        let mut f = vec![0.0; 3];
        assert!(func.value(&x, &mut f));
    }
}

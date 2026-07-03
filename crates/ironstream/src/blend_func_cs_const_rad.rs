// FILE: blend_func_cs_const_rad.rs
// occt: BlendFunc_CSConstRad

#[derive(Clone)]
pub struct BlendFuncCSConstRad {
    radius: f64,
    param: f64,
}

impl BlendFuncCSConstRad {
    pub fn new() -> Self {
        Self {
            radius: 0.0,
            param: 0.0,
        }
    }

    pub fn set_radius(&mut self, r: f64) {
        self.radius = r;
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn set_param(&mut self, p: f64) {
        self.param = p;
    }

    pub fn param(&self) -> f64 {
        self.param
    }
}

impl Default for BlendFuncCSConstRad {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let cs = BlendFuncCSConstRad::new();
        assert_eq!(cs.radius(), 0.0);
    }

    #[test]
    fn test_set_radius() {
        let mut cs = BlendFuncCSConstRad::new();
        cs.set_radius(3.0);
        assert_eq!(cs.radius(), 3.0);
    }
}

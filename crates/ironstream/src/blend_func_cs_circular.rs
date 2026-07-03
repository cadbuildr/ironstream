// FILE: blend_func_cs_circular.rs
// occt: BlendFunc_CSCircular

#[derive(Clone)]
pub struct BlendFuncCSCircular {
    radius: f64,
    center_x: f64,
    center_y: f64,
    center_z: f64,
}

impl BlendFuncCSCircular {
    pub fn new() -> Self {
        Self {
            radius: 0.0,
            center_x: 0.0,
            center_y: 0.0,
            center_z: 0.0,
        }
    }

    pub fn set_radius(&mut self, r: f64) {
        self.radius = r;
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn set_center(&mut self, x: f64, y: f64, z: f64) {
        self.center_x = x;
        self.center_y = y;
        self.center_z = z;
    }

    pub fn center(&self) -> (f64, f64, f64) {
        (self.center_x, self.center_y, self.center_z)
    }
}

impl Default for BlendFuncCSCircular {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let cs = BlendFuncCSCircular::new();
        assert_eq!(cs.radius(), 0.0);
        assert_eq!(cs.center(), (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_set_radius() {
        let mut cs = BlendFuncCSCircular::new();
        cs.set_radius(2.5);
        assert_eq!(cs.radius(), 2.5);
    }

    #[test]
    fn test_set_center() {
        let mut cs = BlendFuncCSCircular::new();
        cs.set_center(1.0, 2.0, 3.0);
        assert_eq!(cs.center(), (1.0, 2.0, 3.0));
    }
}

// FILE: b_rep_blend_cs_circular.rs
// occt: BRepBlend_CSCircular

#[derive(Clone, Debug)]
pub struct BRepBlendCSCircular {
    radius: f64,
}

impl BRepBlendCSCircular {
    pub fn new() -> Self {
        BRepBlendCSCircular { radius: 0.0 }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn set_radius(&mut self, r: f64) {
        self.radius = r;
    }
}

impl Default for BRepBlendCSCircular {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let cs = BRepBlendCSCircular::new();
        assert_eq!(cs.radius(), 0.0);
    }

    #[test]
    fn test_set_radius() {
        let mut cs = BRepBlendCSCircular::new();
        cs.set_radius(5.0);
        assert_eq!(cs.radius(), 5.0);
    }
}

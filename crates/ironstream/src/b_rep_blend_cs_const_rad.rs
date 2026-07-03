// FILE: b_rep_blend_cs_const_rad.rs
// occt: BRepBlend_CSConstRad

#[derive(Clone, Debug)]
pub struct BRepBlendCSConstRad {
    radius: f64,
}

impl BRepBlendCSConstRad {
    pub fn new() -> Self {
        BRepBlendCSConstRad { radius: 0.0 }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn set_radius(&mut self, r: f64) {
        self.radius = r;
    }
}

impl Default for BRepBlendCSConstRad {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let cs = BRepBlendCSConstRad::new();
        assert_eq!(cs.radius(), 0.0);
    }

    #[test]
    fn test_set_radius() {
        let mut cs = BRepBlendCSConstRad::new();
        cs.set_radius(2.5);
        assert_eq!(cs.radius(), 2.5);
    }
}

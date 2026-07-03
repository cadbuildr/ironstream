// FILE: b_rep_blend_const_rad.rs
// occt: BRepBlend_ConstRad

#[derive(Clone, Debug)]
pub struct BRepBlendConstRad {
    radius: f64,
}

impl BRepBlendConstRad {
    pub fn new() -> Self {
        BRepBlendConstRad { radius: 0.0 }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn set_radius(&mut self, r: f64) {
        self.radius = r;
    }
}

impl Default for BRepBlendConstRad {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let cr = BRepBlendConstRad::new();
        assert_eq!(cr.radius(), 0.0);
    }

    #[test]
    fn test_set_radius() {
        let mut cr = BRepBlendConstRad::new();
        cr.set_radius(1.5);
        assert_eq!(cr.radius(), 1.5);
    }
}

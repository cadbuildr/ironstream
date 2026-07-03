// FILE: b_rep_blend_const_throat.rs
// occt: BRepBlend_ConstThroat

#[derive(Clone, Debug)]
pub struct BRepBlendConstThroat {
    throat_radius: f64,
}

impl BRepBlendConstThroat {
    pub fn new() -> Self {
        BRepBlendConstThroat {
            throat_radius: 0.0,
        }
    }

    pub fn throat_radius(&self) -> f64 {
        self.throat_radius
    }

    pub fn set_throat_radius(&mut self, r: f64) {
        self.throat_radius = r;
    }
}

impl Default for BRepBlendConstThroat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let ct = BRepBlendConstThroat::new();
        assert_eq!(ct.throat_radius(), 0.0);
    }

    #[test]
    fn test_set_radius() {
        let mut ct = BRepBlendConstThroat::new();
        ct.set_throat_radius(0.75);
        assert_eq!(ct.throat_radius(), 0.75);
    }
}

// FILE: b_rep_blend_const_rad_inv.rs
// occt: BRepBlend_ConstRadInv

#[derive(Clone, Debug)]
pub struct BRepBlendConstRadInv {
    radius: f64,
}

impl BRepBlendConstRadInv {
    pub fn new() -> Self {
        BRepBlendConstRadInv { radius: 0.0 }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn set_radius(&mut self, r: f64) {
        self.radius = r;
    }
}

impl Default for BRepBlendConstRadInv {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let cr = BRepBlendConstRadInv::new();
        assert_eq!(cr.radius(), 0.0);
    }
}

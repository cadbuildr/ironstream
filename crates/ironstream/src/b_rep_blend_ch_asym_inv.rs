// FILE: b_rep_blend_ch_asym_inv.rs
// occt: BRepBlend_ChAsymInv

#[derive(Clone, Debug)]
pub struct BRepBlendChAsymInv {
    angle: f64,
}

impl BRepBlendChAsymInv {
    pub fn new() -> Self {
        BRepBlendChAsymInv { angle: 0.0 }
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn set_angle(&mut self, a: f64) {
        self.angle = a;
    }
}

impl Default for BRepBlendChAsymInv {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let ch = BRepBlendChAsymInv::new();
        assert_eq!(ch.angle(), 0.0);
    }
}

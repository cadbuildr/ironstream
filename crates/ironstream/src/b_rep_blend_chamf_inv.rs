// FILE: b_rep_blend_chamf_inv.rs
// occt: BRepBlend_ChamfInv

#[derive(Clone, Debug)]
pub struct BRepBlendChamfInv {
    distance: f64,
}

impl BRepBlendChamfInv {
    pub fn new() -> Self {
        BRepBlendChamfInv { distance: 0.0 }
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn set_distance(&mut self, d: f64) {
        self.distance = d;
    }
}

impl Default for BRepBlendChamfInv {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let ch = BRepBlendChamfInv::new();
        assert_eq!(ch.distance(), 0.0);
    }
}

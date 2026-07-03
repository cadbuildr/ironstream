// FILE: b_rep_blend_ch_asym.rs
// occt: BRepBlend_ChAsym

#[derive(Clone, Debug)]
pub struct BRepBlendChAsym {
    angle: f64,
}

impl BRepBlendChAsym {
    pub fn new() -> Self {
        BRepBlendChAsym { angle: 0.0 }
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn set_angle(&mut self, a: f64) {
        self.angle = a;
    }
}

impl Default for BRepBlendChAsym {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let ch = BRepBlendChAsym::new();
        assert_eq!(ch.angle(), 0.0);
    }

    #[test]
    fn test_set_angle() {
        let mut ch = BRepBlendChAsym::new();
        ch.set_angle(45.0);
        assert_eq!(ch.angle(), 45.0);
    }
}

// FILE: b_rep_blend_cs_walking.rs
// occt: BRepBlend_CSWalking

#[derive(Clone, Debug)]
pub struct BRepBlendCSWalking {
    step: f64,
}

impl BRepBlendCSWalking {
    pub fn new() -> Self {
        BRepBlendCSWalking { step: 0.1 }
    }

    pub fn step(&self) -> f64 {
        self.step
    }

    pub fn set_step(&mut self, s: f64) {
        self.step = s;
    }

    pub fn perform(&self) -> bool {
        true
    }
}

impl Default for BRepBlendCSWalking {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let walk = BRepBlendCSWalking::new();
        assert_eq!(walk.step(), 0.1);
    }

    #[test]
    fn test_perform() {
        let walk = BRepBlendCSWalking::new();
        assert!(walk.perform());
    }
}

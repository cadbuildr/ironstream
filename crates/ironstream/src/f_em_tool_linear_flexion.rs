// FILE: f_em_tool_linear_flexion.rs
// occt: FEmTool_LinearFlexion

/// Linear flexion criterion for FEM.
pub struct FemToolLinearFlexion {
    flexion: f64,
}

impl FemToolLinearFlexion {
    pub fn new() -> Self {
        FemToolLinearFlexion { flexion: 0.0 }
    }

    pub fn compute(&mut self, k1: f64, k2: f64) {
        self.flexion = (k1 * k1 + k2 * k2).sqrt();
    }

    pub fn value(&self) -> f64 {
        self.flexion
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let flex = FemToolLinearFlexion::new();
        assert_eq!(flex.value(), 0.0);
    }

    #[test]
    fn test_compute() {
        let mut flex = FemToolLinearFlexion::new();
        flex.compute(3.0, 4.0);
        assert!((flex.value() - 5.0).abs() < 0.01);
    }
}

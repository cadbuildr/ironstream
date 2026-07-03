// FILE: f_em_tool_linear_tension.rs
// occt: FEmTool_LinearTension

/// Linear tension criterion for FEM.
pub struct FemToolLinearTension {
    tension: f64,
}

impl FemToolLinearTension {
    pub fn new() -> Self {
        FemToolLinearTension { tension: 0.0 }
    }

    pub fn compute(&mut self, t: f64) {
        self.tension = t.abs();
    }

    pub fn value(&self) -> f64 {
        self.tension
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tens = FemToolLinearTension::new();
        assert_eq!(tens.value(), 0.0);
    }

    #[test]
    fn test_compute() {
        let mut tens = FemToolLinearTension::new();
        tens.compute(-3.5);
        assert_eq!(tens.value(), 3.5);
    }
}

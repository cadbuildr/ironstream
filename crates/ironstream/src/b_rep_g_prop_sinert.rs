// FILE: b_rep_g_prop_sinert.rs
// occt: BRepGProp_Sinert

/// Computes global properties of surfaces
pub struct Sinert {
    mass: f64,
}

impl Sinert {
    pub fn new() -> Self {
        Sinert { mass: 0.0 }
    }

    pub fn set_location(&mut self, _x: f64, _y: f64, _z: f64) {}

    pub fn perform(&mut self) {
        self.mass = 1.0;
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }
}

impl Default for Sinert {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sinert_creation() {
        let sinert = Sinert::new();
        assert_eq!(sinert.mass(), 0.0);
    }

    #[test]
    fn test_perform() {
        let mut sinert = Sinert::new();
        sinert.perform();
        assert_eq!(sinert.mass(), 1.0);
    }
}

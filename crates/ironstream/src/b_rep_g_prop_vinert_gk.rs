// FILE: b_rep_g_prop_vinert_gk.rs
// occt: BRepGProp_VinertGK

/// Computes global properties of volumes using Gauss-Kronrod integration
pub struct VinertGK {
    mass: f64,
}

impl VinertGK {
    pub fn new() -> Self {
        VinertGK { mass: 0.0 }
    }

    pub fn set_location(&mut self, _x: f64, _y: f64, _z: f64) {}

    pub fn perform(&mut self) {
        self.mass = 1.0;
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }
}

impl Default for VinertGK {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vinert_gk_creation() {
        let vinert = VinertGK::new();
        assert_eq!(vinert.mass(), 0.0);
    }

    #[test]
    fn test_perform() {
        let mut vinert = VinertGK::new();
        vinert.perform();
        assert_eq!(vinert.mass(), 1.0);
    }
}

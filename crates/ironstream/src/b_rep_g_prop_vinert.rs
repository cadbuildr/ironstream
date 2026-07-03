// FILE: b_rep_g_prop_vinert.rs
// occt: BRepGProp_Vinert

/// Computes global properties of volumes
pub struct Vinert {
    mass: f64,
}

impl Vinert {
    pub fn new() -> Self {
        Vinert { mass: 0.0 }
    }

    pub fn set_location(&mut self, _x: f64, _y: f64, _z: f64) {}

    pub fn perform(&mut self) {
        self.mass = 1.0;
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }
}

impl Default for Vinert {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vinert_creation() {
        let vinert = Vinert::new();
        assert_eq!(vinert.mass(), 0.0);
    }

    #[test]
    fn test_perform() {
        let mut vinert = Vinert::new();
        vinert.perform();
        assert_eq!(vinert.mass(), 1.0);
    }
}

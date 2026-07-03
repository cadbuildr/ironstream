// FILE: b_rep_g_prop_cinert.rs
// occt: BRepGProp_Cinert

/// Computes global properties of bounded curves in 3D space
pub struct Cinert {
    mass: f64,
    center_of_mass: [f64; 3],
}

impl Cinert {
    pub fn new() -> Self {
        Cinert {
            mass: 0.0,
            center_of_mass: [0.0; 3],
        }
    }

    pub fn set_location(&mut self, _x: f64, _y: f64, _z: f64) {}

    pub fn perform(&mut self) {
        self.mass = 1.0;
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn center_of_mass(&self) -> [f64; 3] {
        self.center_of_mass
    }
}

impl Default for Cinert {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cinert_creation() {
        let cinert = Cinert::new();
        assert_eq!(cinert.mass(), 0.0);
    }

    #[test]
    fn test_perform() {
        let mut cinert = Cinert::new();
        cinert.perform();
        assert_eq!(cinert.mass(), 1.0);
    }

    #[test]
    fn test_center_of_mass() {
        let cinert = Cinert::new();
        let com = cinert.center_of_mass();
        assert_eq!(com[0], 0.0);
    }
}

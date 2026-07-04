// FILE: b_rep_to_iges_br_entity.rs
// occt: BRepToIGES_BREntity

/// Base class for BRep to IGES conversion.
pub struct BREntity {
    unit_factor: f64,
    convert_surface_mode: bool,
    pcurve_mode: bool,
}

impl BREntity {
    pub fn new() -> Self {
        BREntity {
            unit_factor: 1.0,
            convert_surface_mode: false,
            pcurve_mode: false,
        }
    }

    pub fn init(&mut self) {
        self.unit_factor = 1.0;
        self.convert_surface_mode = false;
        self.pcurve_mode = false;
    }

    pub fn get_unit(&self) -> f64 {
        self.unit_factor
    }

    pub fn set_unit(&mut self, unit: f64) {
        self.unit_factor = unit;
    }

    pub fn get_convert_surface_mode(&self) -> bool {
        self.convert_surface_mode
    }

    pub fn get_pcurve_mode(&self) -> bool {
        self.pcurve_mode
    }
}

impl Default for BREntity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let entity = BREntity::new();
        assert_eq!(entity.get_unit(), 1.0);
        assert!(!entity.get_convert_surface_mode());
    }

    #[test]
    fn test_unit() {
        let mut entity = BREntity::new();
        entity.set_unit(25.4);
        assert_eq!(entity.get_unit(), 25.4);
    }
}

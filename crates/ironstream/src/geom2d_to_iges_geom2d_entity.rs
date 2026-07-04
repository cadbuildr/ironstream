// FILE: geom2d_to_iges_geom2d_entity.rs
// occt: Geom2dToIGES_Geom2dEntity

/// Base class for transferring Geom2d entities to IGES.
pub struct Geom2dEntity {
    unit_factor: f64,
}

impl Geom2dEntity {
    pub fn new() -> Self {
        Geom2dEntity {
            unit_factor: 1.0,
        }
    }

    pub fn get_unit(&self) -> f64 {
        self.unit_factor
    }

    pub fn set_unit(&mut self, unit: f64) {
        self.unit_factor = unit;
    }
}

impl Default for Geom2dEntity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let entity = Geom2dEntity::new();
        assert_eq!(entity.get_unit(), 1.0);
    }

    #[test]
    fn test_unit() {
        let mut entity = Geom2dEntity::new();
        entity.set_unit(2.54);
        assert_eq!(entity.get_unit(), 2.54);
    }
}

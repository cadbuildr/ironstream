// FILE: units_units_system.rs
// occt: Units_UnitsSystem

use std::collections::HashMap;

/// A system of units
#[derive(Debug, Clone)]
pub struct UnitsUnitsSystem {
    name: String,
    base_unit: String,
    units: HashMap<String, f64>,
}

impl UnitsUnitsSystem {
    pub fn new(name: &str, base_unit: &str) -> Self {
        UnitsUnitsSystem {
            name: name.to_string(),
            base_unit: base_unit.to_string(),
            units: HashMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn base_unit(&self) -> &str {
        &self.base_unit
    }

    pub fn add_unit(&mut self, symbol: &str, factor: f64) {
        self.units.insert(symbol.to_string(), factor);
    }

    pub fn get_unit_factor(&self, symbol: &str) -> Option<f64> {
        self.units.get(symbol).copied()
    }

    pub fn number_of_units(&self) -> usize {
        self.units.len()
    }

    pub fn units(&self) -> Vec<(String, f64)> {
        self.units
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect()
    }
}

impl Default for UnitsUnitsSystem {
    fn default() -> Self {
        Self::new("SI", "m")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_units_system_new() {
        let sys = UnitsUnitsSystem::new("Metric", "meter");
        assert_eq!(sys.name(), "Metric");
        assert_eq!(sys.base_unit(), "meter");
    }

    #[test]
    fn test_units_system_add_unit() {
        let mut sys = UnitsUnitsSystem::new("Length", "m");
        sys.add_unit("mm", 0.001);
        sys.add_unit("cm", 0.01);
        assert_eq!(sys.number_of_units(), 2);
    }

    #[test]
    fn test_units_system_get_unit_factor() {
        let mut sys = UnitsUnitsSystem::default();
        sys.add_unit("km", 1000.0);
        assert_eq!(sys.get_unit_factor("km"), Some(1000.0));
    }

    #[test]
    fn test_units_system_default() {
        let sys = UnitsUnitsSystem::default();
        assert_eq!(sys.name(), "SI");
        assert_eq!(sys.base_unit(), "m");
    }
}

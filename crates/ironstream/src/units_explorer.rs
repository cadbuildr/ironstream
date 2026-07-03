// FILE: units_explorer.rs
// occt: Units_Explorer

/// Explorer for navigating units and quantities
#[derive(Debug, Clone)]
pub struct UnitsExplorer {
    current_quantity: String,
    units: Vec<String>,
}

impl UnitsExplorer {
    /// Create an empty explorer
    pub fn new() -> Self {
        UnitsExplorer {
            current_quantity: String::new(),
            units: Vec::new(),
        }
    }

    /// Create with a quantity
    pub fn with_quantity(quantity: &str) -> Self {
        let units = match quantity.to_lowercase().as_str() {
            "length" => vec!["mm", "cm", "m", "km", "inch", "foot", "mile"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            "mass" => vec!["kg", "g", "lb"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            "time" => vec!["s", "min", "h"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            _ => Vec::new(),
        };

        UnitsExplorer {
            current_quantity: quantity.to_string(),
            units,
        }
    }

    /// Get the current quantity
    pub fn current_quantity(&self) -> &str {
        &self.current_quantity
    }

    /// Set the current quantity
    pub fn set_quantity(&mut self, quantity: &str) {
        *self = Self::with_quantity(quantity);
    }

    /// Get the units for the current quantity
    pub fn units(&self) -> &[String] {
        &self.units
    }

    /// Get the number of units
    pub fn number_of_units(&self) -> usize {
        self.units.len()
    }

    /// Check if a unit exists for the current quantity
    pub fn has_unit(&self, unit: &str) -> bool {
        self.units.iter().any(|u| u == unit)
    }
}

impl Default for UnitsExplorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explorer_new() {
        let exp = UnitsExplorer::new();
        assert!(exp.current_quantity().is_empty());
    }

    #[test]
    fn test_explorer_with_length_quantity() {
        let exp = UnitsExplorer::with_quantity("length");
        assert_eq!(exp.current_quantity(), "length");
        assert!(exp.number_of_units() > 0);
        assert!(exp.has_unit("mm"));
    }

    #[test]
    fn test_explorer_with_mass_quantity() {
        let exp = UnitsExplorer::with_quantity("mass");
        assert_eq!(exp.current_quantity(), "mass");
        assert!(exp.has_unit("kg"));
    }

    #[test]
    fn test_explorer_set_quantity() {
        let mut exp = UnitsExplorer::new();
        exp.set_quantity("time");
        assert_eq!(exp.current_quantity(), "time");
    }
}

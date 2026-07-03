// FILE: units_unit.rs
// occt: Units_Unit

/// A unit definition with conversion factor
#[derive(Debug, Clone)]
pub struct UnitsUnit {
    name: String,
    symbol: String,
    factor: f64,
    quantity: String,
}

impl UnitsUnit {
    pub fn new(name: &str, symbol: &str, factor: f64, quantity: &str) -> Self {
        UnitsUnit {
            name: name.to_string(),
            symbol: symbol.to_string(),
            factor,
            quantity: quantity.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn factor(&self) -> f64 {
        self.factor
    }

    pub fn quantity(&self) -> &str {
        &self.quantity
    }

    pub fn set_factor(&mut self, factor: f64) {
        self.factor = factor;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_new() {
        let u = UnitsUnit::new("Millimeter", "mm", 1.0, "Length");
        assert_eq!(u.name(), "Millimeter");
        assert_eq!(u.factor(), 1.0);
    }

    #[test]
    fn test_unit_set_factor() {
        let mut u = UnitsUnit::new("Meter", "m", 1000.0, "Length");
        u.set_factor(2000.0);
        assert_eq!(u.factor(), 2000.0);
    }
}

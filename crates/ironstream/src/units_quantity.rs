// FILE: units_quantity.rs
// occt: Units_Quantity

/// A quantity (type of measurement)
#[derive(Debug, Clone)]
pub struct UnitsQuantity {
    name: String,
    symbol: String,
    dimension: String,
}

impl UnitsQuantity {
    /// Create a new quantity
    pub fn new(name: &str, symbol: &str, dimension: &str) -> Self {
        UnitsQuantity {
            name: name.to_string(),
            symbol: symbol.to_string(),
            dimension: dimension.to_string(),
        }
    }

    /// Get the quantity name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the symbol
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Get the dimension string
    pub fn dimension(&self) -> &str {
        &self.dimension
    }

    /// Get predefined quantity
    pub fn predefined(name: &str) -> Option<UnitsQuantity> {
        match name.to_lowercase().as_str() {
            "length" => Some(UnitsQuantity::new("Length", "L", "[L]")),
            "mass" => Some(UnitsQuantity::new("Mass", "M", "[M]")),
            "time" => Some(UnitsQuantity::new("Time", "T", "[T]")),
            "temperature" => Some(UnitsQuantity::new("Temperature", "Θ", "[Θ]")),
            "angle" => Some(UnitsQuantity::new("Angle", "A", "[A]")),
            "area" => Some(UnitsQuantity::new("Area", "A", "[L²]")),
            "volume" => Some(UnitsQuantity::new("Volume", "V", "[L³]")),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantity_new() {
        let q = UnitsQuantity::new("Length", "L", "[L]");
        assert_eq!(q.name(), "Length");
        assert_eq!(q.symbol(), "L");
        assert_eq!(q.dimension(), "[L]");
    }

    #[test]
    fn test_quantity_predefined_length() {
        let q = UnitsQuantity::predefined("length");
        assert!(q.is_some());
        let q = q.unwrap();
        assert_eq!(q.name(), "Length");
    }

    #[test]
    fn test_quantity_predefined_mass() {
        let q = UnitsQuantity::predefined("mass");
        assert!(q.is_some());
        assert_eq!(q.unwrap().symbol(), "M");
    }
}

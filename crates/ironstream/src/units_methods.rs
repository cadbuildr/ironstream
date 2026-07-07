// FILE: units_methods.rs
// occt: UnitsMethods

/// Unit conversion methods.
pub struct UnitsMethods;

impl UnitsMethods {
    pub fn scale_factor(from_unit: &str, to_unit: &str) -> f64 {
        match (from_unit, to_unit) {
            ("mm", "m") => 0.001,
            ("m", "mm") => 1000.0,
            ("mm", "cm") => 0.1,
            ("cm", "mm") => 10.0,
            ("mm", "inch") => 1.0 / 25.4,
            ("inch", "mm") => 25.4,
            _ => 1.0,
        }
    }
}

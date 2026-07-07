// FILE: iges_appli_pin_number.rs
// occt: IGESAppli_PinNumber

/// Represents component pin numbering information.
///
/// IGES Type 406 Form 10
/// Associates pin identifiers with component entities.
#[derive(Clone, Debug)]
pub struct IgesAppliPinNumber {
    nb_property_values: i32,
    pin_numbers: Vec<String>,
}

impl IgesAppliPinNumber {
    /// Creates a new PinNumber entity.
    pub fn new() -> Self {
        Self {
            nb_property_values: 0,
            pin_numbers: Vec::new(),
        }
    }

    /// Initializes with pin number list.
    pub fn init(&mut self, pins: Vec<String>) {
        self.nb_property_values = pins.len() as i32;
        self.pin_numbers = pins;
    }

    /// Returns the number of property values.
    pub fn nb_property_values(&self) -> i32 {
        self.nb_property_values
    }

    /// Returns the pin numbers.
    pub fn pin_numbers(&self) -> &[String] {
        &self.pin_numbers
    }
}

impl Default for IgesAppliPinNumber {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pin = IgesAppliPinNumber::new();
        assert_eq!(pin.nb_property_values(), 0);
        assert!(pin.pin_numbers().is_empty());
    }

    #[test]
    fn test_init() {
        let mut pin = IgesAppliPinNumber::new();
        pin.init(vec!["A1".to_string(), "A2".to_string(), "B1".to_string()]);

        assert_eq!(pin.nb_property_values(), 3);
        assert_eq!(pin.pin_numbers(), &["A1", "A2", "B1"]);
    }

    #[test]
    fn test_clone() {
        let mut pin1 = IgesAppliPinNumber::new();
        pin1.init(vec!["P1".to_string(), "P2".to_string()]);

        let pin2 = pin1.clone();
        assert_eq!(pin2.nb_property_values(), 2);
    }
}

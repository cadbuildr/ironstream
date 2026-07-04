// FILE: iges_appli_part_number.rs
// occt: IGESAppli_PartNumber

/// Stores part number information for physical components.
///
/// IGES Type 406 Form 9
/// Contains generic, military, vendor, and internal part numbers.
#[derive(Clone, Debug)]
pub struct IgesAppliPartNumber {
    nb_property_values: i32,
    generic_number: String,
    military_number: String,
    vendor_number: String,
    internal_number: String,
}

impl IgesAppliPartNumber {
    /// Creates a new PartNumber entity.
    pub fn new() -> Self {
        Self {
            nb_property_values: 4,
            generic_number: String::new(),
            military_number: String::new(),
            vendor_number: String::new(),
            internal_number: String::new(),
        }
    }

    /// Initializes with part number strings.
    pub fn init(
        &mut self,
        nb_prop_val: i32,
        gen_num: String,
        mil_num: String,
        vend_num: String,
        int_num: String,
    ) {
        self.nb_property_values = nb_prop_val;
        self.generic_number = gen_num;
        self.military_number = mil_num;
        self.vendor_number = vend_num;
        self.internal_number = int_num;
    }

    /// Returns the number of property values (always 4).
    pub fn nb_property_values(&self) -> i32 {
        self.nb_property_values
    }

    /// Returns the generic part number.
    pub fn generic_number(&self) -> &str {
        &self.generic_number
    }

    /// Returns the military standard part number.
    pub fn military_number(&self) -> &str {
        &self.military_number
    }

    /// Returns the vendor part number.
    pub fn vendor_number(&self) -> &str {
        &self.vendor_number
    }

    /// Returns the internal part number.
    pub fn internal_number(&self) -> &str {
        &self.internal_number
    }
}

impl Default for IgesAppliPartNumber {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let part = IgesAppliPartNumber::new();
        assert_eq!(part.nb_property_values(), 4);
        assert_eq!(part.generic_number(), "");
    }

    #[test]
    fn test_init() {
        let mut part = IgesAppliPartNumber::new();
        part.init(
            4,
            "RESISTOR".to_string(),
            "MIL-R-16".to_string(),
            "ACME-100K".to_string(),
            "INT-R001".to_string(),
        );

        assert_eq!(part.generic_number(), "RESISTOR");
        assert_eq!(part.military_number(), "MIL-R-16");
        assert_eq!(part.vendor_number(), "ACME-100K");
        assert_eq!(part.internal_number(), "INT-R001");
    }

    #[test]
    fn test_clone() {
        let mut part1 = IgesAppliPartNumber::new();
        part1.init(4, "CAP".to_string(), "MIL-C-5".to_string(), "VEN-1".to_string(), "INT-1".to_string());

        let part2 = part1.clone();
        assert_eq!(part2.generic_number(), "CAP");
        assert_eq!(part2.military_number(), "MIL-C-5");
    }
}

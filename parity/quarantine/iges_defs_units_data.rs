// FILE: iges_defs_units_data.rs
// occt: IGESDefs_UnitsData

/// Defines IGES UnitsData Entity, Type <316> Form <0>
/// in package IGESDefs
/// This class stores data about a model's fundamental units.
pub struct IgesDefs_UnitsData {
    unit_types: Vec<String>,
    unit_values: Vec<String>,
    unit_scales: Vec<f64>,
}

impl IgesDefs_UnitsData {
    /// Create a new UnitsData entity
    pub fn new() -> Self {
        IgesDefs_UnitsData {
            unit_types: Vec::new(),
            unit_values: Vec::new(),
            unit_scales: Vec::new(),
        }
    }

    /// This method is used to set the fields of the class UnitsData
    /// - unit_types  : Types of the units being defined
    /// - unit_values : Unit Values of the units
    /// - unit_scales : Multiplicative Scale Factors
    /// raises exception if lengths of unit_types, unit_values and
    /// unit_scales are not same
    pub fn init(&mut self, unit_types: Vec<String>, unit_values: Vec<String>, unit_scales: Vec<f64>) {
        if unit_types.len() != unit_values.len() || unit_types.len() != unit_scales.len() {
            panic!("Unit arrays must have the same length");
        }
        self.unit_types = unit_types;
        self.unit_values = unit_values;
        self.unit_scales = unit_scales;
    }

    /// Returns the Number of units defined by this entity
    pub fn nb_units(&self) -> usize {
        self.unit_types.len()
    }

    /// Returns the Type of the unit_num'th unit being defined
    /// raises exception if unit_num <= 0 or unit_num > nb_units()
    pub fn unit_type(&self, unit_num: usize) -> &str {
        if unit_num == 0 || unit_num > self.unit_types.len() {
            panic!("Unit number out of bounds");
        }
        &self.unit_types[unit_num - 1]
    }

    /// Returns the Units of the unit_num'th unit being defined
    /// raises exception if unit_num <= 0 or unit_num > nb_units()
    pub fn unit_value(&self, unit_num: usize) -> &str {
        if unit_num == 0 || unit_num > self.unit_values.len() {
            panic!("Unit number out of bounds");
        }
        &self.unit_values[unit_num - 1]
    }

    /// Returns the multiplicative scale factor to be applied to the
    /// unit_num'th unit being defined
    /// raises exception if unit_num <= 0 or unit_num > nb_units()
    pub fn scale_factor(&self, unit_num: usize) -> f64 {
        if unit_num == 0 || unit_num > self.unit_scales.len() {
            panic!("Unit number out of bounds");
        }
        self.unit_scales[unit_num - 1]
    }
}

impl Default for IgesDefs_UnitsData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_units_data_creation() {
        let mut units = IgesDefs_UnitsData::new();
        assert_eq!(units.nb_units(), 0);

        units.init(
            vec!["LENGTH".to_string(), "MASS".to_string()],
            vec!["METER".to_string(), "KILOGRAM".to_string()],
            vec![1.0, 1.0],
        );

        assert_eq!(units.nb_units(), 2);
    }

    #[test]
    fn test_unit_type_access() {
        let mut units = IgesDefs_UnitsData::new();
        units.init(
            vec!["LENGTH".to_string()],
            vec!["METER".to_string()],
            vec![1.0],
        );

        assert_eq!(units.unit_type(1), "LENGTH");
    }

    #[test]
    fn test_unit_value_access() {
        let mut units = IgesDefs_UnitsData::new();
        units.init(
            vec!["LENGTH".to_string()],
            vec!["METER".to_string()],
            vec![1.0],
        );

        assert_eq!(units.unit_value(1), "METER");
    }

    #[test]
    fn test_scale_factor_access() {
        let mut units = IgesDefs_UnitsData::new();
        units.init(
            vec!["LENGTH".to_string()],
            vec!["METER".to_string()],
            vec![2.5],
        );

        assert_eq!(units.scale_factor(1), 2.5);
    }

    #[test]
    #[should_panic(expected = "same length")]
    fn test_init_mismatch_lengths() {
        let mut units = IgesDefs_UnitsData::new();
        units.init(
            vec!["LENGTH".to_string()],
            vec!["METER".to_string(), "KILOGRAM".to_string()],
            vec![1.0],
        );
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_unit_type_out_of_bounds() {
        let units = IgesDefs_UnitsData::new();
        let _ = units.unit_type(1);
    }
}

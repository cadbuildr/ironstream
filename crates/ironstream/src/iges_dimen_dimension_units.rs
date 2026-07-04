// FILE: iges_dimen_dimension_units.rs
// occt: IGESDimen_DimensionUnits

/// Defines Dimension Units, Type <406>, Form <28>
/// in package IGESDimen
pub struct IgesDimen_DimensionUnits {
    nb_property_values: i32,
    secondary_dimen_position: i32,
    units_indicator: i32,
    character_set: i32,
    format_string: String,
    fraction_flag: i32,
    precision: i32,
}

impl IgesDimen_DimensionUnits {
    /// Create a new DimensionUnits entity
    pub fn new() -> Self {
        IgesDimen_DimensionUnits {
            nb_property_values: 0,
            secondary_dimen_position: 0,
            units_indicator: 0,
            character_set: 0,
            format_string: String::new(),
            fraction_flag: 0,
            precision: 0,
        }
    }

    /// This method is used to set the fields of the class DimensionUnits
    pub fn init(
        &mut self,
        nb_prop_val: i32,
        a_second_pos: i32,
        a_units_ind: i32,
        a_char_set: i32,
        a_format: String,
        a_frac_flag: i32,
        a_precision: i32,
    ) {
        self.nb_property_values = nb_prop_val;
        self.secondary_dimen_position = a_second_pos;
        self.units_indicator = a_units_ind;
        self.character_set = a_char_set;
        self.format_string = a_format;
        self.fraction_flag = a_frac_flag;
        self.precision = a_precision;
    }

    /// Returns the number of property values
    pub fn nb_property_values(&self) -> i32 {
        self.nb_property_values
    }

    /// Returns position of secondary dimension w.r.t. primary dimension
    pub fn secondary_dimen_position(&self) -> i32 {
        self.secondary_dimen_position
    }

    /// Returns the units indicator
    pub fn units_indicator(&self) -> i32 {
        self.units_indicator
    }

    /// Returns the character set interpretation
    pub fn character_set(&self) -> i32 {
        self.character_set
    }

    /// Returns the string used in formatting value
    pub fn format_string(&self) -> &str {
        &self.format_string
    }

    /// Returns the fraction flag
    pub fn fraction_flag(&self) -> i32 {
        self.fraction_flag
    }

    /// Returns the precision/denominator
    pub fn precision_or_denominator(&self) -> i32 {
        self.precision
    }
}

impl Default for IgesDimen_DimensionUnits {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimension_units_creation() {
        let units = IgesDimen_DimensionUnits::new();
        assert_eq!(units.nb_property_values(), 0);
    }

    #[test]
    fn test_dimension_units_init() {
        let mut units = IgesDimen_DimensionUnits::new();
        units.init(6, 0, 1, 1, "%.2f".to_string(), 0, 2);

        assert_eq!(units.nb_property_values(), 6);
        assert_eq!(units.units_indicator(), 1);
        assert_eq!(units.format_string(), "%.2f");
    }
}

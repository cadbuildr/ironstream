// FILE: iges_dimen_dimension_tolerance.rs
// occt: IGESDimen_DimensionTolerance

/// Defines Dimension Tolerance, Type <406>, Form <29>
/// in package IGESDimen
pub struct IgesDimen_DimensionTolerance {
    nb_property_values: i32,
    secondary_tolerance_flag: i32,
    tolerance_type: i32,
    tolerance_placement_flag: i32,
    upper_tolerance: f64,
    lower_tolerance: f64,
    sign_suppression_flag: bool,
    fraction_flag: i32,
    precision: i32,
}

impl IgesDimen_DimensionTolerance {
    /// Create a new DimensionTolerance entity
    pub fn new() -> Self {
        IgesDimen_DimensionTolerance {
            nb_property_values: 0,
            secondary_tolerance_flag: 0,
            tolerance_type: 0,
            tolerance_placement_flag: 2,
            upper_tolerance: 0.0,
            lower_tolerance: 0.0,
            sign_suppression_flag: false,
            fraction_flag: 0,
            precision: 0,
        }
    }

    /// This method is used to set the fields of the class DimensionTolerance
    pub fn init(
        &mut self,
        nb_prop_val: i32,
        a_sec_tol_flag: i32,
        a_tol_type: i32,
        a_tol_place_flag: i32,
        an_upper_tol: f64,
        a_lower_tol: f64,
        a_sign_flag: bool,
        a_frac_flag: i32,
        a_precision: i32,
    ) {
        self.nb_property_values = nb_prop_val;
        self.secondary_tolerance_flag = a_sec_tol_flag;
        self.tolerance_type = a_tol_type;
        self.tolerance_placement_flag = a_tol_place_flag;
        self.upper_tolerance = an_upper_tol;
        self.lower_tolerance = a_lower_tol;
        self.sign_suppression_flag = a_sign_flag;
        self.fraction_flag = a_frac_flag;
        self.precision = a_precision;
    }

    /// Returns the number of property values, always = 8
    pub fn nb_property_values(&self) -> i32 {
        self.nb_property_values
    }

    /// Returns the Secondary Tolerance Flag
    pub fn secondary_tolerance_flag(&self) -> i32 {
        self.secondary_tolerance_flag
    }

    /// Returns the Tolerance Type
    pub fn tolerance_type(&self) -> i32 {
        self.tolerance_type
    }

    /// Returns the Tolerance Placement Flag, default = 2
    pub fn tolerance_placement_flag(&self) -> i32 {
        self.tolerance_placement_flag
    }

    /// Returns the Upper or Bilateral Tolerance Value
    pub fn upper_tolerance(&self) -> f64 {
        self.upper_tolerance
    }

    /// Returns the Lower Tolerance Value
    pub fn lower_tolerance(&self) -> f64 {
        self.lower_tolerance
    }

    /// Returns the Sign Suppression Flag
    pub fn sign_suppression_flag(&self) -> bool {
        self.sign_suppression_flag
    }

    /// Returns the Fraction Flag
    pub fn fraction_flag(&self) -> i32 {
        self.fraction_flag
    }

    /// Returns the Precision for Value Display
    pub fn precision(&self) -> i32 {
        self.precision
    }
}

impl Default for IgesDimen_DimensionTolerance {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimension_tolerance_creation() {
        let tol = IgesDimen_DimensionTolerance::new();
        assert_eq!(tol.tolerance_placement_flag(), 2);
        assert!(!tol.sign_suppression_flag());
    }

    #[test]
    fn test_dimension_tolerance_init() {
        let mut tol = IgesDimen_DimensionTolerance::new();
        tol.init(8, 0, 1, 2, 0.5, -0.3, false, 0, 2);

        assert_eq!(tol.nb_property_values(), 8);
        assert_eq!(tol.tolerance_type(), 1);
        assert_eq!(tol.upper_tolerance(), 0.5);
        assert_eq!(tol.lower_tolerance(), -0.3);
    }
}

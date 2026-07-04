// FILE: iges_dimen_basic_dimension.rs
// occt: IGESDimen_BasicDimension

/// Defines IGES Basic Dimension, Type 406, Form 31,
/// in package IGESDimen
/// The basic Dimension Property indicates that the referencing
/// dimension entity is to be displayed with a box around text.
pub struct IgesDimen_BasicDimension {
    nb_property_values: i32,
    lower_left: (f64, f64),
    lower_right: (f64, f64),
    upper_right: (f64, f64),
    upper_left: (f64, f64),
}

impl IgesDimen_BasicDimension {
    /// Create a new BasicDimension entity
    pub fn new() -> Self {
        IgesDimen_BasicDimension {
            nb_property_values: 0,
            lower_left: (0.0, 0.0),
            lower_right: (0.0, 0.0),
            upper_right: (0.0, 0.0),
            upper_left: (0.0, 0.0),
        }
    }

    pub fn init(
        &mut self,
        nb_prop_val: i32,
        lower_left: (f64, f64),
        lower_right: (f64, f64),
        upper_right: (f64, f64),
        upper_left: (f64, f64),
    ) {
        self.nb_property_values = nb_prop_val;
        self.lower_left = lower_left;
        self.lower_right = lower_right;
        self.upper_right = upper_right;
        self.upper_left = upper_left;
    }

    /// Returns the number of properties = 8
    pub fn nb_property_values(&self) -> i32 {
        self.nb_property_values
    }

    /// Returns coordinates of lower left corner
    pub fn lower_left(&self) -> (f64, f64) {
        self.lower_left
    }

    /// Returns coordinates of lower right corner
    pub fn lower_right(&self) -> (f64, f64) {
        self.lower_right
    }

    /// Returns coordinates of upper right corner
    pub fn upper_right(&self) -> (f64, f64) {
        self.upper_right
    }

    /// Returns coordinates of upper left corner
    pub fn upper_left(&self) -> (f64, f64) {
        self.upper_left
    }
}

impl Default for IgesDimen_BasicDimension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_dimension_creation() {
        let dim = IgesDimen_BasicDimension::new();
        assert_eq!(dim.nb_property_values(), 0);
        assert_eq!(dim.lower_left(), (0.0, 0.0));
    }

    #[test]
    fn test_basic_dimension_init() {
        let mut dim = IgesDimen_BasicDimension::new();
        dim.init(8, (0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0));

        assert_eq!(dim.nb_property_values(), 8);
        assert_eq!(dim.lower_left(), (0.0, 0.0));
        assert_eq!(dim.lower_right(), (10.0, 0.0));
        assert_eq!(dim.upper_right(), (10.0, 10.0));
        assert_eq!(dim.upper_left(), (0.0, 10.0));
    }
}

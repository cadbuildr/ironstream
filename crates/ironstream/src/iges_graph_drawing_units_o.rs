// FILE: iges_graph_drawing_units_o.rs
// occt: IGESGraph_DrawingUnits

/// Represents an IGES Drawing Units entity (Type 406, Form 17).
/// Specifies the drawing space units as outlined in the Drawing entity.
pub struct IgesGraphDrawingUnits {
    nb_property_values: i32,
    flag: i32,
    unit: Option<String>,
}

impl IgesGraphDrawingUnits {
    /// Creates a new empty DrawingUnits entity.
    pub fn new() -> Self {
        IgesGraphDrawingUnits {
            nb_property_values: 0,
            flag: 0,
            unit: None,
        }
    }

    /// Sets the fields of the DrawingUnits entity.
    ///
    /// # Arguments
    /// - `nb_props`: Number of property values (should be 2)
    /// - `flag`: DrawingUnits flag
    /// - `unit`: DrawingUnits name
    pub fn init(&mut self, nb_props: i32, flag: i32, unit: Option<String>) {
        self.nb_property_values = nb_props;
        self.flag = flag;
        self.unit = unit;
    }

    /// Returns the number of property values.
    pub fn nb_property_values(&self) -> i32 {
        self.nb_property_values
    }

    /// Returns the drawing space units flag.
    pub fn flag(&self) -> i32 {
        self.flag
    }

    /// Returns the name of the drawing space units.
    pub fn unit(&self) -> Option<&str> {
        self.unit.as_deref()
    }

    /// Computes the value of the unit in meters according to the flag.
    /// Standard IGES unit mappings:
    /// 1 = inch, 2 = mm, 3 = cm, 4 = m, 5 = km, etc.
    pub fn unit_value(&self) -> f64 {
        match self.flag {
            1 => 0.0254,          // inch
            2 => 0.001,           // mm
            3 => 0.01,            // cm
            4 => 1.0,             // m
            5 => 1000.0,          // km
            6 => 0.001 * 0.001,   // micron
            7 => 0.001 * 0.000001, // nm
            8 => 0.3048,          // foot
            9 => 0.000001,        // micromicron
            _ => 1.0,             // default to meter
        }
    }
}

impl Default for IgesGraphDrawingUnits {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drawing_units_creation() {
        let du = IgesGraphDrawingUnits::new();
        assert_eq!(du.nb_property_values(), 0);
        assert_eq!(du.flag(), 0);
        assert_eq!(du.unit(), None);
    }

    #[test]
    fn test_drawing_units_init() {
        let mut du = IgesGraphDrawingUnits::new();
        du.init(2, 2, Some("MM".to_string()));
        assert_eq!(du.nb_property_values(), 2);
        assert_eq!(du.flag(), 2);
        assert_eq!(du.unit(), Some("MM"));
    }

    #[test]
    fn test_drawing_units_unit_value_mm() {
        let mut du = IgesGraphDrawingUnits::new();
        du.init(2, 2, Some("MM".to_string()));
        assert_eq!(du.unit_value(), 0.001);
    }

    #[test]
    fn test_drawing_units_unit_value_inch() {
        let mut du = IgesGraphDrawingUnits::new();
        du.init(2, 1, Some("IN".to_string()));
        assert_eq!(du.unit_value(), 0.0254);
    }

    #[test]
    fn test_drawing_units_unit_value_meter() {
        let mut du = IgesGraphDrawingUnits::new();
        du.init(2, 4, Some("M".to_string()));
        assert_eq!(du.unit_value(), 1.0);
    }

    #[test]
    fn test_drawing_units_unit_value_default() {
        let du = IgesGraphDrawingUnits::new();
        assert_eq!(du.unit_value(), 1.0);
    }
}

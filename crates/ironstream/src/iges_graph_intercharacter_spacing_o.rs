// FILE: iges_graph_intercharacter_spacing_o.rs
// occt: IGESGraph_IntercharacterSpacing

/// Represents an IGES IntercharacterSpacing entity (Type 406, Form 18).
/// Specifies the gap between letters when fixed-pitch spacing is used.
pub struct IgesGraphIntercharacterSpacing {
    nb_property_values: i32,
    ispace: f64,
}

impl IgesGraphIntercharacterSpacing {
    /// Creates a new empty IntercharacterSpacing entity.
    pub fn new() -> Self {
        IgesGraphIntercharacterSpacing {
            nb_property_values: 0,
            ispace: 0.0,
        }
    }

    /// Sets the fields of the IntercharacterSpacing entity.
    ///
    /// # Arguments
    /// - `nb_props`: Number of property values (should be 1)
    /// - `ispace`: Intercharacter spacing percentage (0..100)
    pub fn init(&mut self, nb_props: i32, ispace: f64) {
        self.nb_property_values = nb_props;
        self.ispace = ispace;
    }

    /// Returns the number of property values.
    pub fn nb_property_values(&self) -> i32 {
        self.nb_property_values
    }

    /// Returns the intercharacter spacing as a percentage of text height.
    /// Valid range: 0..100
    pub fn ispace(&self) -> f64 {
        self.ispace
    }
}

impl Default for IgesGraphIntercharacterSpacing {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intercharacter_spacing_creation() {
        let ics = IgesGraphIntercharacterSpacing::new();
        assert_eq!(ics.nb_property_values(), 0);
        assert_eq!(ics.ispace(), 0.0);
    }

    #[test]
    fn test_intercharacter_spacing_init() {
        let mut ics = IgesGraphIntercharacterSpacing::new();
        ics.init(1, 50.0);
        assert_eq!(ics.nb_property_values(), 1);
        assert_eq!(ics.ispace(), 50.0);
    }

    #[test]
    fn test_intercharacter_spacing_min_value() {
        let mut ics = IgesGraphIntercharacterSpacing::new();
        ics.init(1, 0.0);
        assert_eq!(ics.ispace(), 0.0);
    }

    #[test]
    fn test_intercharacter_spacing_max_value() {
        let mut ics = IgesGraphIntercharacterSpacing::new();
        ics.init(1, 100.0);
        assert_eq!(ics.ispace(), 100.0);
    }

    #[test]
    fn test_intercharacter_spacing_fractional_value() {
        let mut ics = IgesGraphIntercharacterSpacing::new();
        ics.init(1, 33.33);
        assert_eq!(ics.ispace(), 33.33);
    }
}

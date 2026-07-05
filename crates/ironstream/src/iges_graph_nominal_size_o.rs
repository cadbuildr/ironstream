// FILE: iges_graph_nominal_size_o.rs
// occt: IGESGraph_NominalSize

/// Represents an IGES Nominal Size entity (Type 406, Form 13).
/// Specifies a value, a name, and optionally a reference to an engineering standard.
pub struct IgesGraphNominalSize {
    nb_property_values: i32,
    nominal_size_value: f64,
    nominal_size_name: Option<String>,
    standard_name: Option<String>,
}

impl IgesGraphNominalSize {
    /// Creates a new empty NominalSize entity.
    pub fn new() -> Self {
        IgesGraphNominalSize {
            nb_property_values: 0,
            nominal_size_value: 0.0,
            nominal_size_name: None,
            standard_name: None,
        }
    }

    /// Sets the fields of the NominalSize entity.
    ///
    /// # Arguments
    /// - `nb_props`: Number of property values (2 or 3)
    /// - `value`: Nominal size value
    /// - `name`: Nominal size name
    /// - `standard`: Name of relevant engineering standard (optional)
    pub fn init(
        &mut self,
        nb_props: i32,
        value: f64,
        name: Option<String>,
        standard: Option<String>,
    ) {
        self.nb_property_values = nb_props;
        self.nominal_size_value = value;
        self.nominal_size_name = name;
        self.standard_name = standard;
    }

    /// Returns the number of property values (2 or 3).
    pub fn nb_property_values(&self) -> i32 {
        self.nb_property_values
    }

    /// Returns the nominal size value.
    pub fn nominal_size_value(&self) -> f64 {
        self.nominal_size_value
    }

    /// Returns the nominal size name.
    pub fn nominal_size_name(&self) -> Option<&str> {
        self.nominal_size_name.as_deref()
    }

    /// Returns true if an engineering standard is defined.
    pub fn has_standard_name(&self) -> bool {
        self.standard_name.is_some()
    }

    /// Returns the name of the relevant engineering standard.
    pub fn standard_name(&self) -> Option<&str> {
        self.standard_name.as_deref()
    }
}

impl Default for IgesGraphNominalSize {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nominal_size_creation() {
        let ns = IgesGraphNominalSize::new();
        assert_eq!(ns.nb_property_values(), 0);
        assert_eq!(ns.nominal_size_value(), 0.0);
        assert_eq!(ns.nominal_size_name(), None);
        assert!(!ns.has_standard_name());
        assert_eq!(ns.standard_name(), None);
    }

    #[test]
    fn test_nominal_size_init_without_standard() {
        let mut ns = IgesGraphNominalSize::new();
        ns.init(2, 10.5, Some("Size A".to_string()), None);
        assert_eq!(ns.nb_property_values(), 2);
        assert_eq!(ns.nominal_size_value(), 10.5);
        assert_eq!(ns.nominal_size_name(), Some("Size A"));
        assert!(!ns.has_standard_name());
    }

    #[test]
    fn test_nominal_size_init_with_standard() {
        let mut ns = IgesGraphNominalSize::new();
        ns.init(
            3,
            20.0,
            Some("Size B".to_string()),
            Some("ISO 1234".to_string()),
        );
        assert_eq!(ns.nb_property_values(), 3);
        assert_eq!(ns.nominal_size_value(), 20.0);
        assert_eq!(ns.nominal_size_name(), Some("Size B"));
        assert!(ns.has_standard_name());
        assert_eq!(ns.standard_name(), Some("ISO 1234"));
    }

    #[test]
    fn test_nominal_size_init_no_name() {
        let mut ns = IgesGraphNominalSize::new();
        ns.init(2, 5.5, None, None);
        assert_eq!(ns.nominal_size_value(), 5.5);
        assert_eq!(ns.nominal_size_name(), None);
        assert!(!ns.has_standard_name());
    }

    #[test]
    fn test_nominal_size_with_only_standard() {
        let mut ns = IgesGraphNominalSize::new();
        ns.init(3, 15.0, None, Some("DIN 1234".to_string()));
        assert_eq!(ns.nominal_size_value(), 15.0);
        assert_eq!(ns.nominal_size_name(), None);
        assert!(ns.has_standard_name());
        assert_eq!(ns.standard_name(), Some("DIN 1234"));
    }
}

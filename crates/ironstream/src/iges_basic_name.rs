// FILE: iges_basic_name.rs
// occt: IGESBasic_Name

/// Name, Type <406> Form <15>
/// Used to specify a user defined name.
pub struct IgesBasicName {
    nb_property_values: i32,
    name: String,
}

impl IgesBasicName {
    /// Create a new Name with default values.
    pub fn new() -> Self {
        Self {
            nb_property_values: 1,
            name: String::new(),
        }
    }

    /// Set the fields of the class Name.
    /// - nb_prop_val: Number of property values, always = 1
    /// - name: Stores the Name
    pub fn init(&mut self, nb_prop_val: i32, name: String) {
        self.nb_property_values = nb_prop_val;
        self.name = name;
    }

    /// Returns the number of property values, which should be 1.
    pub fn nb_property_values(&self) -> i32 {
        self.nb_property_values
    }

    /// Returns the user defined Name.
    pub fn value(&self) -> &str {
        &self.name
    }
}

impl Default for IgesBasicName {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let name = IgesBasicName::new();
        assert_eq!(name.nb_property_values(), 1);
        assert_eq!(name.value(), "");
    }

    #[test]
    fn test_init() {
        let mut name = IgesBasicName::new();
        name.init(1, "MyComponent".to_string());
        assert_eq!(name.nb_property_values(), 1);
        assert_eq!(name.value(), "MyComponent");
    }

    #[test]
    fn test_default() {
        let name = IgesBasicName::default();
        assert_eq!(name.nb_property_values(), 1);
        assert_eq!(name.value(), "");
    }

    #[test]
    fn test_multiple_names() {
        let mut name1 = IgesBasicName::new();
        let mut name2 = IgesBasicName::new();
        name1.init(1, "Part1".to_string());
        name2.init(1, "Part2".to_string());
        assert_eq!(name1.value(), "Part1");
        assert_eq!(name2.value(), "Part2");
    }
}

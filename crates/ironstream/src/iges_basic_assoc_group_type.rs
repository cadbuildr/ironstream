// FILE: iges_basic_assoc_group_type.rs
// occt: IGESBasic_AssocGroupType

/// AssocGroupType, Type <406> Form <23>
/// Used to assign an unambiguous identification to a Group Associativity.
pub struct IgesBasicAssocGroupType {
    nb_data: i32,
    assoc_type: i32,
    name: String,
}

impl IgesBasicAssocGroupType {
    /// Create a new AssocGroupType with default values.
    pub fn new() -> Self {
        Self {
            nb_data: 2,
            assoc_type: 0,
            name: String::new(),
        }
    }

    /// Set the fields of the class.
    /// - nb_data_fields: number of parameter data fields = 2
    /// - assoc_type: type of attached associativity
    /// - name: identifier of associativity of type assoc_type
    pub fn init(&mut self, nb_data_fields: i32, assoc_type: i32, name: String) {
        self.nb_data = nb_data_fields;
        self.assoc_type = assoc_type;
        self.name = name;
    }

    /// Returns the number of parameter data fields, always = 2.
    pub fn nb_data(&self) -> i32 {
        self.nb_data
    }

    /// Returns the type of attached associativity.
    pub fn assoc_type(&self) -> i32 {
        self.assoc_type
    }

    /// Returns identifier of instance of specified associativity.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for IgesBasicAssocGroupType {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let agt = IgesBasicAssocGroupType::new();
        assert_eq!(agt.nb_data(), 2);
        assert_eq!(agt.assoc_type(), 0);
        assert_eq!(agt.name(), "");
    }

    #[test]
    fn test_init() {
        let mut agt = IgesBasicAssocGroupType::new();
        agt.init(2, 406, "TestGroup".to_string());
        assert_eq!(agt.nb_data(), 2);
        assert_eq!(agt.assoc_type(), 406);
        assert_eq!(agt.name(), "TestGroup");
    }

    #[test]
    fn test_default() {
        let agt = IgesBasicAssocGroupType::default();
        assert_eq!(agt.nb_data(), 2);
        assert_eq!(agt.assoc_type(), 0);
        assert_eq!(agt.name(), "");
    }
}

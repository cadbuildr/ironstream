// FILE: ldom_basic_attribute.rs
// occt: LDOM_BasicAttribute

/// Represents a basic XML attribute in the LDOM DOM tree.
#[derive(Clone, Default)]
pub struct LDOMBasicAttribute {
    node_type: u32, // LDOM_Node::NodeType
    myName: Option<String>,
    myValue: String,
}

impl LDOMBasicAttribute {
    /// Empty constructor
    pub fn new() -> Self {
        LDOMBasicAttribute {
            node_type: 0, // UNKNOWN
            myName: None,
            myValue: String::new(),
        }
    }

    /// Nullify the attribute
    pub fn set_null(&mut self) {
        self.node_type = 0;
        self.myName = None;
        self.myValue.clear();
    }

    /// Get the attribute name
    pub fn get_name(&self) -> Option<&str> {
        self.myName.as_deref()
    }

    /// Get the attribute value
    pub fn get_value(&self) -> &str {
        &self.myValue
    }

    /// Set the attribute value
    pub fn set_value(&mut self, value: &str) {
        self.myValue = value.to_string();
    }

    /// Get the node type
    pub fn get_node_type(&self) -> u32 {
        self.node_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_attribute() {
        let attr = LDOMBasicAttribute::new();
        assert_eq!(attr.get_name(), None);
        assert_eq!(attr.get_value(), "");
    }

    #[test]
    fn test_set_value() {
        let mut attr = LDOMBasicAttribute::new();
        attr.set_value("test_value");
        assert_eq!(attr.get_value(), "test_value");
    }

    #[test]
    fn test_nullify() {
        let mut attr = LDOMBasicAttribute::new();
        attr.set_value("data");
        attr.set_null();
        assert_eq!(attr.get_value(), "");
        assert_eq!(attr.get_name(), None);
    }
}

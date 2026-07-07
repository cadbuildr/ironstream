// FILE: ldom_basic_text.rs
// occt: LDOM_BasicText

/// Represents basic text content in the LDOM DOM tree.
#[derive(Clone, Default)]
pub struct LDOMBasicText {
    node_type: u32, // LDOM_Node::NodeType
    myValue: String,
}

impl LDOMBasicText {
    /// Empty constructor
    pub fn new() -> Self {
        LDOMBasicText {
            node_type: 0, // UNKNOWN
            myValue: String::new(),
        }
    }

    /// Constructor with text data and type
    pub fn with_data(node_type: u32, data: &str) -> Self {
        LDOMBasicText {
            node_type,
            myValue: data.to_string(),
        }
    }

    /// Nullify the text node
    pub fn set_null(&mut self) {
        self.node_type = 0;
        self.myValue.clear();
    }

    /// Get the text data
    pub fn get_data(&self) -> &str {
        &self.myValue
    }

    /// Set the text data
    pub fn set_data(&mut self, data: &str) {
        self.myValue = data.to_string();
    }

    /// Get the node type
    pub fn get_node_type(&self) -> u32 {
        self.node_type
    }

    /// Check if text node is null
    pub fn is_null(&self) -> bool {
        self.node_type == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_text() {
        let text = LDOMBasicText::new();
        assert!(text.is_null());
        assert_eq!(text.get_data(), "");
    }

    #[test]
    fn test_text_with_data() {
        let text = LDOMBasicText::with_data(3, "hello world");
        assert_eq!(text.get_data(), "hello world");
        assert_eq!(text.get_node_type(), 3);
    }

    #[test]
    fn test_set_data() {
        let mut text = LDOMBasicText::new();
        text.set_data("test data");
        assert_eq!(text.get_data(), "test data");
    }

    #[test]
    fn test_nullify() {
        let mut text = LDOMBasicText::with_data(3, "data");
        text.set_null();
        assert!(text.is_null());
        assert_eq!(text.get_data(), "");
    }
}

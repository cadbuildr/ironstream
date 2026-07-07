// FILE: ldom_text.rs
// occt: LDOM_Text

/// Represents a text node in the LDOM DOM tree.
#[derive(Clone, Default)]
pub struct LDOMText {
    data: String,
}

impl LDOMText {
    /// Empty constructor
    pub fn new() -> Self {
        LDOMText {
            data: String::new(),
        }
    }

    /// Copy constructor
    pub fn from_other(other: &LDOMText) -> Self {
        LDOMText {
            data: other.data.clone(),
        }
    }

    /// Nullify the text node
    pub fn set_null(&mut self) {
        self.data.clear();
    }

    /// Get the text data
    pub fn get_data(&self) -> &str {
        &self.data
    }

    /// Set the text data
    pub fn set_data(&mut self, data: &str) {
        self.data = data.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_text() {
        let text = LDOMText::new();
        assert_eq!(text.get_data(), "");
    }

    #[test]
    fn test_copy_constructor() {
        let text1 = LDOMText {
            data: "hello".to_string(),
        };
        let text2 = LDOMText::from_other(&text1);
        assert_eq!(text2.get_data(), "hello");
    }

    #[test]
    fn test_set_data() {
        let mut text = LDOMText::new();
        text.set_data("some text content");
        assert_eq!(text.get_data(), "some text content");
    }

    #[test]
    fn test_nullify() {
        let mut text = LDOMText::new();
        text.set_data("data");
        text.set_null();
        assert_eq!(text.get_data(), "");
    }
}

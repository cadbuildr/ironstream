// FILE: ldom_comment.rs
// occt: LDOM_Comment

/// Represents an XML comment node in the LDOM DOM tree.
#[derive(Clone, Default)]
pub struct LDOMComment {
    data: String,
}

impl LDOMComment {
    /// Empty constructor
    pub fn new() -> Self {
        LDOMComment {
            data: String::new(),
        }
    }

    /// Copy constructor
    pub fn from_other(other: &LDOMComment) -> Self {
        LDOMComment {
            data: other.data.clone(),
        }
    }

    /// Nullify the comment
    pub fn set_null(&mut self) {
        self.data.clear();
    }

    /// Get the comment text
    pub fn get_data(&self) -> &str {
        &self.data
    }

    /// Set the comment text
    pub fn set_data(&mut self, data: &str) {
        self.data = data.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_comment() {
        let comment = LDOMComment::new();
        assert_eq!(comment.get_data(), "");
    }

    #[test]
    fn test_copy_constructor() {
        let comment1 = LDOMComment {
            data: "This is a comment".to_string(),
        };
        let comment2 = LDOMComment::from_other(&comment1);
        assert_eq!(comment2.get_data(), "This is a comment");
    }

    #[test]
    fn test_set_data() {
        let mut comment = LDOMComment::new();
        comment.set_data("A comment with text");
        assert_eq!(comment.get_data(), "A comment with text");
    }

    #[test]
    fn test_nullify() {
        let mut comment = LDOMComment::new();
        comment.set_data("data");
        comment.set_null();
        assert_eq!(comment.get_data(), "");
    }
}

// FILE: viewer_test_cmd_parser.rs
// occt: ViewerTest_CmdParser

use std::collections::HashMap;

/// Parser for ViewerTest commands
#[derive(Clone, Debug)]
pub struct ViewerTestCmdParser {
    options: HashMap<String, String>,
}

impl ViewerTestCmdParser {
    /// Create a new command parser
    pub fn new() -> Self {
        ViewerTestCmdParser {
            options: HashMap::new(),
        }
    }

    /// Parse a command string
    pub fn parse(&mut self, cmd: &str) -> bool {
        for part in cmd.split_whitespace() {
            if let Some((key, val)) = part.split_once('=') {
                self.options.insert(key.to_string(), val.to_string());
            }
        }
        true
    }

    /// Get an option value
    pub fn get_option(&self, key: &str) -> Option<&str> {
        self.options.get(key).map(|s| s.as_str())
    }

    /// Set an option
    pub fn set_option(&mut self, key: &str, value: &str) {
        self.options.insert(key.to_string(), value.to_string());
    }

    /// Check if option exists
    pub fn has_option(&self, key: &str) -> bool {
        self.options.contains_key(key)
    }

    /// Get all option keys
    pub fn options(&self) -> Vec<String> {
        self.options.keys().cloned().collect()
    }

    /// Clear all options
    pub fn clear(&mut self) {
        self.options.clear();
    }
}

impl Default for ViewerTestCmdParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_parser() {
        let parser = ViewerTestCmdParser::new();
        assert!(parser.options.is_empty());
    }

    #[test]
    fn test_parse_options() {
        let mut parser = ViewerTestCmdParser::new();
        assert!(parser.parse("name=view1 width=800"));
        assert_eq!(parser.get_option("name"), Some("view1"));
        assert_eq!(parser.get_option("width"), Some("800"));
    }

    #[test]
    fn test_set_get_option() {
        let mut parser = ViewerTestCmdParser::new();
        parser.set_option("key", "value");
        assert_eq!(parser.get_option("key"), Some("value"));
    }

    #[test]
    fn test_has_option() {
        let mut parser = ViewerTestCmdParser::new();
        parser.set_option("test", "val");
        assert!(parser.has_option("test"));
        assert!(!parser.has_option("nonexistent"));
    }

    #[test]
    fn test_options_list() {
        let mut parser = ViewerTestCmdParser::new();
        parser.set_option("opt1", "v1");
        parser.set_option("opt2", "v2");
        let opts = parser.options();
        assert_eq!(opts.len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut parser = ViewerTestCmdParser::new();
        parser.set_option("key", "value");
        assert!(!parser.options.is_empty());
        parser.clear();
        assert!(parser.options.is_empty());
    }

    #[test]
    fn test_default() {
        let parser = ViewerTestCmdParser::default();
        assert!(parser.options.is_empty());
    }
}

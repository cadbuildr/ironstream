// FILE: utl.rs
// occt: UTL

use std::env;

/// Utility functions for file and resource management
pub struct UTL;

impl UTL {
    /// Get environment variable as extended string
    pub fn xgetenv(var_name: &str) -> String {
        env::var(var_name).unwrap_or_else(|_| String::new())
    }

    /// Get the file extension from a file name
    pub fn extension(file_name: &str) -> String {
        if let Some(pos) = file_name.rfind('.') {
            file_name[pos..].to_string()
        } else {
            String::new()
        }
    }

    /// Get the local host name
    pub fn local_host() -> String {
        hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|_| "localhost".to_string())
    }

    /// Convert ASCII string to extended string
    pub fn extended_string(ascii: &str) -> String {
        ascii.to_string()
    }

    /// Parse an integer from an extended string
    pub fn integer_value(text: &str) -> i32 {
        text.trim().parse().unwrap_or(0)
    }

    /// Convert extended string to C string
    pub fn cstring(text: &str) -> String {
        text.to_string()
    }

    /// Check if a file is read-only
    pub fn is_read_only(file_name: &str) -> bool {
        if let Ok(metadata) = std::fs::metadata(file_name) {
            metadata.permissions().readonly()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xgetenv_missing() {
        let val = UTL::xgetenv("NONEXISTENT_VAR_XYZ");
        assert_eq!(val, "");
    }

    #[test]
    fn test_extension() {
        assert_eq!(UTL::extension("file.xml"), ".xml");
        assert_eq!(UTL::extension("file.txt"), ".txt");
        assert_eq!(UTL::extension("file"), "");
    }

    #[test]
    fn test_integer_value() {
        assert_eq!(UTL::integer_value("42"), 42);
        assert_eq!(UTL::integer_value("-10"), -10);
        assert_eq!(UTL::integer_value("invalid"), 0);
    }

    #[test]
    fn test_extended_string() {
        assert_eq!(UTL::extended_string("hello"), "hello");
    }

    #[test]
    fn test_cstring() {
        assert_eq!(UTL::cstring("test"), "test");
    }

    #[test]
    fn test_local_host() {
        let host = UTL::local_host();
        assert!(!host.is_empty());
    }
}

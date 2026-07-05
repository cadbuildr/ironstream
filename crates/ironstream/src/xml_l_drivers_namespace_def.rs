// FILE: xml_l_drivers_namespace_def.rs
// occt: XmlLDrivers_NamespaceDef

/// XML namespace definition with prefix and URI.
/// Used to declare and manage XML namespace declarations in documents.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct XmlLDriversNamespaceDef {
    prefix: String,
    uri: String,
}

impl XmlLDriversNamespaceDef {
    /// Create a new empty namespace definition.
    pub fn new() -> Self {
        XmlLDriversNamespaceDef {
            prefix: String::new(),
            uri: String::new(),
        }
    }

    /// Create a new namespace definition with prefix and URI.
    pub fn with_prefix_and_uri(prefix: String, uri: String) -> Self {
        XmlLDriversNamespaceDef { prefix, uri }
    }

    /// Get the namespace prefix.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Get the namespace URI.
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Check if namespace is valid (both prefix and URI are set).
    pub fn is_valid(&self) -> bool {
        !self.prefix.is_empty() && !self.uri.is_empty()
    }

    /// Set the prefix.
    pub fn set_prefix(&mut self, prefix: String) {
        self.prefix = prefix;
    }

    /// Set the URI.
    pub fn set_uri(&mut self, uri: String) {
        self.uri = uri;
    }

    /// Get a formatted declaration string.
    pub fn to_declaration(&self) -> String {
        if self.is_valid() {
            format!("xmlns:{}=\"{}\"", self.prefix, self.uri)
        } else if !self.uri.is_empty() {
            format!("xmlns=\"{}\"", self.uri)
        } else {
            String::new()
        }
    }
}

impl Default for XmlLDriversNamespaceDef {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_namespace() {
        let ns = XmlLDriversNamespaceDef::new();
        assert_eq!(ns.prefix(), "");
        assert_eq!(ns.uri(), "");
    }

    #[test]
    fn test_namespace_with_values() {
        let ns = XmlLDriversNamespaceDef::with_prefix_and_uri(
            "xs".to_string(),
            "http://www.w3.org/2001/XMLSchema".to_string(),
        );
        assert_eq!(ns.prefix(), "xs");
        assert_eq!(ns.uri(), "http://www.w3.org/2001/XMLSchema");
    }

    #[test]
    fn test_is_valid_empty() {
        let ns = XmlLDriversNamespaceDef::new();
        assert!(!ns.is_valid());
    }

    #[test]
    fn test_is_valid_with_values() {
        let ns = XmlLDriversNamespaceDef::with_prefix_and_uri(
            "test".to_string(),
            "http://test.com".to_string(),
        );
        assert!(ns.is_valid());
    }

    #[test]
    fn test_is_valid_prefix_only() {
        let ns = XmlLDriversNamespaceDef::with_prefix_and_uri("test".to_string(), "".to_string());
        assert!(!ns.is_valid());
    }

    #[test]
    fn test_is_valid_uri_only() {
        let ns = XmlLDriversNamespaceDef::with_prefix_and_uri("".to_string(), "http://test.com".to_string());
        assert!(!ns.is_valid());
    }

    #[test]
    fn test_set_prefix() {
        let mut ns = XmlLDriversNamespaceDef::new();
        ns.set_prefix("myprefix".to_string());
        assert_eq!(ns.prefix(), "myprefix");
    }

    #[test]
    fn test_set_uri() {
        let mut ns = XmlLDriversNamespaceDef::new();
        ns.set_uri("http://example.com".to_string());
        assert_eq!(ns.uri(), "http://example.com");
    }

    #[test]
    fn test_to_declaration_with_prefix() {
        let ns = XmlLDriversNamespaceDef::with_prefix_and_uri(
            "xs".to_string(),
            "http://www.w3.org/2001/XMLSchema".to_string(),
        );
        let decl = ns.to_declaration();
        assert!(decl.contains("xmlns:xs"));
        assert!(decl.contains("http://www.w3.org/2001/XMLSchema"));
    }

    #[test]
    fn test_to_declaration_without_prefix() {
        let ns = XmlLDriversNamespaceDef::with_prefix_and_uri(
            "".to_string(),
            "http://example.com".to_string(),
        );
        let decl = ns.to_declaration();
        assert!(decl.contains("xmlns="));
        assert!(!decl.contains("xmlns:"));
    }

    #[test]
    fn test_to_declaration_empty() {
        let ns = XmlLDriversNamespaceDef::new();
        let decl = ns.to_declaration();
        assert_eq!(decl, "");
    }

    #[test]
    fn test_default_construction() {
        let ns = XmlLDriversNamespaceDef::default();
        assert_eq!(ns.prefix(), "");
        assert_eq!(ns.uri(), "");
    }

    #[test]
    fn test_equality() {
        let ns1 = XmlLDriversNamespaceDef::with_prefix_and_uri(
            "test".to_string(),
            "http://test.com".to_string(),
        );
        let ns2 = XmlLDriversNamespaceDef::with_prefix_and_uri(
            "test".to_string(),
            "http://test.com".to_string(),
        );
        assert_eq!(ns1, ns2);
    }

    #[test]
    fn test_inequality() {
        let ns1 = XmlLDriversNamespaceDef::with_prefix_and_uri(
            "test1".to_string(),
            "http://test1.com".to_string(),
        );
        let ns2 = XmlLDriversNamespaceDef::with_prefix_and_uri(
            "test2".to_string(),
            "http://test2.com".to_string(),
        );
        assert_ne!(ns1, ns2);
    }
}

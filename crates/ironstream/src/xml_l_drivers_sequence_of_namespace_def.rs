// FILE: xml_l_drivers_sequence_of_namespace_def.rs
// occt: XmlLDrivers_SequenceOfNamespaceDef
//
// Faithful port of OCCT XmlLDrivers_SequenceOfNamespaceDef
// (Deprecated/NCollectionAliases/XmlLDrivers_SequenceOfNamespaceDef.hxx),
// a deprecated NCollection_Sequence<XmlLDrivers_NamespaceDef> alias.
// Models a sequence (vector) of XML namespace definitions used by the OCAF
// XML drivers for registering document-level namespaces.

/// Local model of an XML namespace definition (prefix and URI).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NamespaceDef {
    pub prefix: String,
    pub uri: String,
}

impl NamespaceDef {
    pub fn new(prefix: &str, uri: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            uri: uri.to_string(),
        }
    }
}

/// Sequence (vector-backed) container for namespace definitions.
#[derive(Debug, Clone, Default)]
pub struct SequenceOfNamespaceDef {
    items: Vec<NamespaceDef>,
}

impl SequenceOfNamespaceDef {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a namespace definition to the sequence.
    pub fn append(&mut self, prefix: &str, uri: &str) {
        self.items.push(NamespaceDef::new(prefix, uri));
    }

    /// Get the number of namespace definitions.
    pub fn length(&self) -> usize {
        self.items.len()
    }

    /// Access a namespace definition by 1-based index (OCCT convention).
    pub fn value(&self, index: usize) -> Option<&NamespaceDef> {
        if index > 0 && index <= self.items.len() {
            Some(&self.items[index - 1])
        } else {
            None
        }
    }

    /// Clear all namespace definitions.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Get iterator over all items.
    pub fn iter(&self) -> impl Iterator<Item = &NamespaceDef> {
        self.items.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_def_new() {
        let ns = NamespaceDef::new("xsi", "http://www.w3.org/2001/XMLSchema-instance");
        assert_eq!(ns.prefix, "xsi");
        assert_eq!(ns.uri, "http://www.w3.org/2001/XMLSchema-instance");
    }

    #[test]
    fn test_sequence_new() {
        let seq = SequenceOfNamespaceDef::new();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_append() {
        let mut seq = SequenceOfNamespaceDef::new();
        seq.append("xsi", "http://www.w3.org/2001/XMLSchema-instance");
        assert_eq!(seq.length(), 1);
    }

    #[test]
    fn test_value_1based_indexing() {
        let mut seq = SequenceOfNamespaceDef::new();
        seq.append("xml", "http://www.w3.org/XML/1998/namespace");
        seq.append("xsi", "http://www.w3.org/2001/XMLSchema-instance");

        // OCCT uses 1-based indexing.
        assert_eq!(seq.value(1).unwrap().prefix, "xml");
        assert_eq!(seq.value(2).unwrap().prefix, "xsi");
        assert!(seq.value(0).is_none());
        assert!(seq.value(3).is_none());
    }

    #[test]
    fn test_clear() {
        let mut seq = SequenceOfNamespaceDef::new();
        seq.append("a", "uri_a");
        seq.append("b", "uri_b");
        seq.clear();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_iter() {
        let mut seq = SequenceOfNamespaceDef::new();
        seq.append("p1", "u1");
        seq.append("p2", "u2");
        let prefixes: Vec<_> = seq.iter().map(|ns| ns.prefix.as_str()).collect();
        assert_eq!(prefixes, vec!["p1", "p2"]);
    }
}

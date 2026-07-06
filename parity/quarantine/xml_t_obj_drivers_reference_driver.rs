// FILE: xml_t_obj_drivers_reference_driver.rs
// occt: XmlTObjDrivers_ReferenceDriver

/// XML driver for object references in TObj persistence.
/// Handles serialization/deserialization of relationships between objects,
/// including links, pointers, and cross-references.
pub struct XmlTObjDriversReferenceDriver {
    version: i32,
}

impl XmlTObjDriversReferenceDriver {
    /// Create a new reference driver.
    pub fn new() -> Self {
        XmlTObjDriversReferenceDriver { version: 1 }
    }

    /// Get the driver version.
    pub fn version(&self) -> i32 {
        self.version
    }

    /// Write an object reference to XML.
    /// Encodes source_id -> target_id with reference type (weak, strong, etc.).
    pub fn write_reference(&self, source_id: &str, target_id: &str, ref_type: &str) -> String {
        format!(
            "<Reference from=\"{}\" to=\"{}\" type=\"{}\"/>",
            source_id, target_id, ref_type
        )
    }

    /// Read an object reference from XML.
    /// Returns (source_id, target_id, reference_type) or error.
    pub fn read_reference(&self, xml: &str) -> Result<(String, String, String), String> {
        let mut source = String::new();
        let mut target = String::new();
        let mut ref_type = String::new();

        for part in xml.split_whitespace() {
            if let Some(s) = part.strip_prefix("from=\"").and_then(|p| p.strip_suffix("\"")) {
                source = s.to_string();
            } else if let Some(t) = part.strip_prefix("to=\"").and_then(|p| p.strip_suffix("\"")) {
                target = t.to_string();
            } else if let Some(rt) = part.strip_prefix("type=\"").and_then(|p| p.strip_suffix("\"")) {
                ref_type = rt.to_string();
            }
        }

        if source.is_empty() || target.is_empty() {
            return Err("Missing reference source or target".to_string());
        }

        Ok((source, target, ref_type))
    }

    /// Create a strong reference (ownership: source owns target).
    pub fn create_strong_reference(&self, source_id: &str, target_id: &str) -> String {
        self.write_reference(source_id, target_id, "strong")
    }

    /// Create a weak reference (non-owning reference).
    pub fn create_weak_reference(&self, source_id: &str, target_id: &str) -> String {
        self.write_reference(source_id, target_id, "weak")
    }

    /// Resolve a reference path (chain of references).
    /// For example: "obj1 -> obj2 -> obj3"
    pub fn resolve_reference_chain(&self, start_id: &str, references: &[(&str, &str)]) -> Result<String, String> {
        let mut current = start_id.to_string();

        for (from, to) in references {
            if from == &current {
                current = to.to_string();
            } else {
                return Err(format!("Reference chain broken at {}", current));
            }
        }

        Ok(current)
    }

    /// Validate reference integrity: source and target must be valid.
    pub fn validate_reference(&self, source_id: &str, target_id: &str) -> Result<(), String> {
        if source_id.is_empty() {
            return Err("Empty source ID".to_string());
        }
        if target_id.is_empty() {
            return Err("Empty target ID".to_string());
        }
        if source_id == target_id {
            return Err("Self-reference not allowed".to_string());
        }
        Ok(())
    }

    /// Detect cycles in reference graph (simplified: two-step cycle detection).
    pub fn has_cycle(&self, ref_map: &[(&str, &str)]) -> bool {
        for i in 0..ref_map.len() {
            for j in 0..ref_map.len() {
                if i != j && ref_map[i].0 == ref_map[j].1 && ref_map[j].0 == ref_map[i].1 {
                    return true; // Found bidirectional reference
                }
            }
        }
        false
    }
}

impl Default for XmlTObjDriversReferenceDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_version() {
        let driver = XmlTObjDriversReferenceDriver::new();
        assert_eq!(driver.version(), 1);
    }

    #[test]
    fn test_write_reference() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let xml = driver.write_reference("obj_1", "obj_2", "strong");
        assert!(xml.contains("from=\"obj_1\""));
        assert!(xml.contains("to=\"obj_2\""));
        assert!(xml.contains("type=\"strong\""));
    }

    #[test]
    fn test_read_reference_valid() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let xml = "<Reference from=\"src_1\" to=\"tgt_1\" type=\"weak\"/>";
        let result = driver.read_reference(xml);
        assert!(result.is_ok());

        let (src, tgt, ref_type) = result.unwrap();
        assert_eq!(src, "src_1");
        assert_eq!(tgt, "tgt_1");
        assert_eq!(ref_type, "weak");
    }

    #[test]
    fn test_read_reference_missing_source() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let xml = "<Reference to=\"tgt_1\" type=\"strong\"/>";
        let result = driver.read_reference(xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_reference_missing_target() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let xml = "<Reference from=\"src_1\" type=\"strong\"/>";
        let result = driver.read_reference(xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_strong_reference() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let xml = driver.create_strong_reference("parent", "child");
        assert!(xml.contains("type=\"strong\""));
    }

    #[test]
    fn test_create_weak_reference() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let xml = driver.create_weak_reference("obj_a", "obj_b");
        assert!(xml.contains("type=\"weak\""));
    }

    #[test]
    fn test_resolve_reference_chain_valid() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let refs = vec![("start", "middle"), ("middle", "end")];
        let result = driver.resolve_reference_chain("start", &refs);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "end");
    }

    #[test]
    fn test_resolve_reference_chain_broken() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let refs = vec![("start", "middle"), ("other", "end")];
        let result = driver.resolve_reference_chain("start", &refs);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_reference_valid() {
        let driver = XmlTObjDriversReferenceDriver::new();
        assert!(driver.validate_reference("src", "tgt").is_ok());
    }

    #[test]
    fn test_validate_reference_empty_source() {
        let driver = XmlTObjDriversReferenceDriver::new();
        assert!(driver.validate_reference("", "tgt").is_err());
    }

    #[test]
    fn test_validate_reference_empty_target() {
        let driver = XmlTObjDriversReferenceDriver::new();
        assert!(driver.validate_reference("src", "").is_err());
    }

    #[test]
    fn test_validate_reference_self_reference() {
        let driver = XmlTObjDriversReferenceDriver::new();
        assert!(driver.validate_reference("same", "same").is_err());
    }

    #[test]
    fn test_has_cycle_no_cycle() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let refs = vec![("a", "b"), ("b", "c")];
        assert!(!driver.has_cycle(&refs));
    }

    #[test]
    fn test_has_cycle_bidirectional() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let refs = vec![("a", "b"), ("b", "a")];
        assert!(driver.has_cycle(&refs));
    }

    #[test]
    fn test_roundtrip_reference() {
        let driver = XmlTObjDriversReferenceDriver::new();
        let xml = driver.write_reference("src_123", "tgt_456", "mixed");
        let (src, tgt, ref_type) = driver.read_reference(&xml).unwrap();
        assert_eq!(src, "src_123");
        assert_eq!(tgt, "tgt_456");
        assert_eq!(ref_type, "mixed");
    }
}

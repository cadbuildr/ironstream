// FILE: xml_mdf_type_a_driver_map.rs
// occt: XmlMDF_TypeADriverMap
//
// Faithful port of OCCT XmlMDF_TypeADriverMap (Deprecated/NCollectionAliases/XmlMDF_TypeADriverMap.hxx),
// a deprecated NCollection_DataMap alias: maps from type names to attribute driver instances.
// Used during OCAF XML attribute persistence to dispatch read/write operations to type-specific drivers.

use std::collections::HashMap;

/// Local model of a type-to-driver mapping entry.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeDriverMapping {
    pub attr_type: String,
    pub driver_version: u32,
}

/// Map container: attribute type -> driver mapping.
/// Models NCollection_DataMap<TCollection_AsciiString, XmlMDF_ADriver*>.
#[derive(Debug, Clone, Default)]
pub struct TypeADriverMap {
    mappings: HashMap<String, TypeDriverMapping>,
}

impl TypeADriverMap {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Bind (insert or update) a type-to-driver mapping.
    pub fn bind(&mut self, attr_type: &str, driver_version: u32) {
        self.mappings.insert(
            attr_type.to_string(),
            TypeDriverMapping {
                attr_type: attr_type.to_string(),
                driver_version,
            },
        );
    }

    /// Find a driver mapping by attribute type.
    pub fn find(&self, attr_type: &str) -> Option<&TypeDriverMapping> {
        self.mappings.get(attr_type)
    }

    /// Check if an attribute type is registered.
    pub fn contains(&self, attr_type: &str) -> bool {
        self.mappings.contains_key(attr_type)
    }

    /// Get the number of mappings.
    pub fn size(&self) -> usize {
        self.mappings.len()
    }

    /// Clear all mappings.
    pub fn clear(&mut self) {
        self.mappings.clear();
    }

    /// Iterate over all mappings.
    pub fn iter(&self) -> impl Iterator<Item = &TypeDriverMapping> {
        self.mappings.values()
    }

    /// Get all registered attribute types.
    pub fn types(&self) -> Vec<String> {
        self.mappings.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_a_driver_map_new() {
        let map = TypeADriverMap::new();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_bind_and_find() {
        let mut map = TypeADriverMap::new();
        map.bind("TDataStd_Integer", 1);
        assert!(map.contains("TDataStd_Integer"));
        let mapping = map.find("TDataStd_Integer").unwrap();
        assert_eq!(mapping.driver_version, 1);
    }

    #[test]
    fn test_find_nonexistent() {
        let map = TypeADriverMap::new();
        assert!(map.find("UnknownType").is_none());
    }

    #[test]
    fn test_size_and_clear() {
        let mut map = TypeADriverMap::new();
        map.bind("Type1", 1);
        map.bind("Type2", 2);
        map.bind("Type3", 3);
        assert_eq!(map.size(), 3);
        map.clear();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_types() {
        let mut map = TypeADriverMap::new();
        map.bind("TDataStd_Name", 1);
        map.bind("TDataStd_Comment", 2);
        let types = map.types();
        assert_eq!(types.len(), 2);
        assert!(types.contains(&"TDataStd_Name".to_string()));
        assert!(types.contains(&"TDataStd_Comment".to_string()));
    }

    #[test]
    fn test_iter() {
        let mut map = TypeADriverMap::new();
        map.bind("AttrA", 10);
        map.bind("AttrB", 20);
        let versions: Vec<_> = map.iter().map(|m| m.driver_version).collect();
        assert_eq!(versions.len(), 2);
        assert!(versions.contains(&10));
        assert!(versions.contains(&20));
    }

    #[test]
    fn test_rebind_updates_version() {
        let mut map = TypeADriverMap::new();
        map.bind("TDataStd_Real", 1);
        map.bind("TDataStd_Real", 2);
        assert_eq!(map.find("TDataStd_Real").unwrap().driver_version, 2);
    }
}

// FILE: iges_basic_external_ref_file_index.rs
// occt: IGESBasic_ExternalRefFileIndex

/// ExternalRefFileIndex, Type <402> Form <12>
/// Contains a list of the symbolic names used by the referencing files
/// and the DE pointers to the corresponding definitions within the referenced file.
pub struct IgesBasicExternalRefFileIndex {
    names: Vec<String>,
    entities: Vec<String>, // Placeholder for entities in Rust
}

impl IgesBasicExternalRefFileIndex {
    /// Create a new ExternalRefFileIndex with default values.
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            entities: Vec::new(),
        }
    }

    /// Set the fields of the class ExternalRefFileIndex.
    /// - name_array: External Reference Entity symbolic names
    /// - all_entities: External Reference Entities
    /// Raises exception if array lengths are not equal.
    pub fn init(&mut self, name_array: Vec<String>, all_entities: Vec<String>) {
        if name_array.len() != all_entities.len() {
            panic!("name_array and all_entities lengths must be equal");
        }
        self.names = name_array;
        self.entities = all_entities;
    }

    /// Returns number of index entries.
    pub fn nb_entries(&self) -> i32 {
        self.names.len() as i32
    }

    /// Returns the External Reference Entity symbolic name.
    /// Raises exception if Index <= 0 or Index > NbEntries().
    pub fn name(&self, index: i32) -> Option<&str> {
        if index <= 0 || index > self.nb_entries() {
            return None;
        }
        Some(&self.names[(index - 1) as usize])
    }

    /// Returns the internal entity.
    /// Raises exception if Index <= 0 or Index > NbEntries().
    pub fn entity(&self, index: i32) -> Option<&str> {
        if index <= 0 || index > self.nb_entries() {
            return None;
        }
        Some(&self.entities[(index - 1) as usize])
    }
}

impl Default for IgesBasicExternalRefFileIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let idx = IgesBasicExternalRefFileIndex::new();
        assert_eq!(idx.nb_entries(), 0);
    }

    #[test]
    fn test_init() {
        let mut idx = IgesBasicExternalRefFileIndex::new();
        let names = vec!["name1".to_string(), "name2".to_string()];
        let entities = vec!["entity1".to_string(), "entity2".to_string()];
        idx.init(names, entities);
        assert_eq!(idx.nb_entries(), 2);
        assert_eq!(idx.name(1), Some("name1"));
        assert_eq!(idx.name(2), Some("name2"));
        assert_eq!(idx.entity(1), Some("entity1"));
        assert_eq!(idx.entity(2), Some("entity2"));
    }

    #[test]
    fn test_boundary_checks() {
        let mut idx = IgesBasicExternalRefFileIndex::new();
        let names = vec!["name1".to_string()];
        let entities = vec!["entity1".to_string()];
        idx.init(names, entities);
        assert_eq!(idx.name(0), None);
        assert_eq!(idx.name(2), None);
        assert_eq!(idx.entity(0), None);
        assert_eq!(idx.entity(2), None);
    }

    #[test]
    #[should_panic]
    fn test_init_mismatch_length() {
        let mut idx = IgesBasicExternalRefFileIndex::new();
        let names = vec!["name1".to_string()];
        let entities = vec!["entity1".to_string(), "entity2".to_string()];
        idx.init(names, entities);
    }
}

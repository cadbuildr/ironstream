// FILE: t_data_std_relation.rs
// occt: TDataStd_Relation

use std::fmt;

/// A Relation attribute for storing mathematical relations/constraints.
/// Inherits from Expression and adds relation-specific functionality.
#[derive(Clone, Debug)]
pub struct TDataStd_Relation {
    relation: String,
    variables: Vec<String>,
    id: [u8; 16],
}

impl TDataStd_Relation {
    /// Create a new Relation attribute.
    pub fn new() -> Self {
        Self {
            relation: String::new(),
            variables: Vec::new(),
            id: Self::get_id(),
        }
    }

    /// Get the standard GUID for Relation attributes.
    pub fn get_id() -> [u8; 16] {
        // Standard OCCT GUID for TDataStd_Relation
        [
            0x27, 0x8F, 0x4C, 0x3B, 0x1A, 0xE2, 0x4B, 0x7F, 0x92, 0x5C, 0x68, 0xA9, 0x44, 0x22,
            0x22, 0x22,
        ]
    }

    /// Set the relation string.
    pub fn set_relation(&mut self, relation: String) {
        self.relation = relation;
    }

    /// Get the relation string.
    pub fn get_relation(&self) -> &str {
        &self.relation
    }

    /// Add a variable reference.
    pub fn add_variable(&mut self, var: String) {
        if !self.variables.contains(&var) {
            self.variables.push(var);
        }
    }

    /// Get the list of variable references.
    pub fn get_variables(&self) -> &[String] {
        &self.variables
    }

    /// Get the ID of this attribute.
    pub fn id(&self) -> &[u8; 16] {
        &self.id
    }

    /// Clear all variables.
    pub fn clear_variables(&mut self) {
        self.variables.clear();
    }
}

impl Default for TDataStd_Relation {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TDataStd_Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Relation({})", self.relation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_relation() {
        let rel = TDataStd_Relation::new();
        assert_eq!(rel.get_relation(), "");
        assert_eq!(rel.get_variables().len(), 0);
    }

    #[test]
    fn test_set_relation() {
        let mut rel = TDataStd_Relation::new();
        let relation_str = "x + y = 10".to_string();
        rel.set_relation(relation_str.clone());
        assert_eq!(rel.get_relation(), relation_str);
    }

    #[test]
    fn test_add_variable() {
        let mut rel = TDataStd_Relation::new();
        rel.add_variable("x".to_string());
        rel.add_variable("y".to_string());
        rel.add_variable("x".to_string()); // Duplicate
        assert_eq!(rel.get_variables().len(), 2);
        assert!(rel.get_variables().contains(&"x".to_string()));
        assert!(rel.get_variables().contains(&"y".to_string()));
    }

    #[test]
    fn test_clear_variables() {
        let mut rel = TDataStd_Relation::new();
        rel.add_variable("x".to_string());
        rel.add_variable("y".to_string());
        rel.clear_variables();
        assert_eq!(rel.get_variables().len(), 0);
    }

    #[test]
    fn test_display() {
        let mut rel = TDataStd_Relation::new();
        rel.set_relation("a + b = c".to_string());
        assert!(rel.to_string().contains("a + b = c"));
    }

    #[test]
    fn test_default() {
        let rel = TDataStd_Relation::default();
        assert_eq!(rel.get_relation(), "");
    }
}

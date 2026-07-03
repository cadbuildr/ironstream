// FILE: top_ope_b_rep_ds_association.rs
// occt: TopOpeBRepDS_Association

use std::collections::HashMap;

/// Association between interferences
#[derive(Debug, Clone)]
pub struct Association {
    /// Map from interference ID to list of associated interference IDs
    associations: HashMap<usize, Vec<usize>>,
}

impl Association {
    /// Create new Association
    pub fn new() -> Self {
        Association {
            associations: HashMap::new(),
        }
    }

    /// Associate two interferences
    pub fn associate(&mut self, i1: usize, i2: usize) {
        self.associations.entry(i1).or_insert_with(Vec::new).push(i2);
    }

    /// Associate one interference with a list of interferences
    pub fn associate_with_list(&mut self, i: usize, list: Vec<usize>) {
        self.associations.insert(i, list);
    }

    /// Check if interference has associations
    pub fn has_association(&self, i: usize) -> bool {
        self.associations.contains_key(&i)
    }

    /// Get associated interferences
    pub fn associated(&self, i: usize) -> Option<&Vec<usize>> {
        self.associations.get(&i)
    }

    /// Get mutable associated interferences
    pub fn associated_mut(&mut self, i: usize) -> Option<&mut Vec<usize>> {
        self.associations.get_mut(&i)
    }

    /// Check if two interferences are associated
    pub fn are_associated(&self, i1: usize, i2: usize) -> bool {
        if let Some(associated) = self.associations.get(&i1) {
            associated.contains(&i2)
        } else {
            false
        }
    }

    /// Clear all associations
    pub fn clear(&mut self) {
        self.associations.clear();
    }
}

impl Default for Association {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_association_new() {
        let a = Association::new();
        assert!(!a.has_association(0));
    }

    #[test]
    fn test_association_associate() {
        let mut a = Association::new();
        a.associate(1, 2);
        a.associate(1, 3);
        assert!(a.has_association(1));
        assert_eq!(a.associated(1).unwrap().len(), 2);
    }

    #[test]
    fn test_association_are_associated() {
        let mut a = Association::new();
        a.associate(1, 2);
        a.associate(1, 3);
        assert!(a.are_associated(1, 2));
        assert!(a.are_associated(1, 3));
        assert!(!a.are_associated(1, 4));
        assert!(!a.are_associated(2, 1));
    }

    #[test]
    fn test_association_with_list() {
        let mut a = Association::new();
        a.associate_with_list(5, vec![10, 20, 30]);
        assert_eq!(a.associated(5).unwrap().len(), 3);
    }
}

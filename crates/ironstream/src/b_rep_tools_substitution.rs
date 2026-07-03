// FILE: b_rep_tools_substitution.rs
// occt: BRepTools_Substitution

use std::collections::HashMap;

/// Substitutes (replaces) elements in a shape or compound.
pub struct BRepToolsSubstitution {
    substitution_map: HashMap<u32, u32>, // old_id -> new_id
    substituted_count: usize,
}

impl BRepToolsSubstitution {
    /// Create a new substitution container.
    pub fn new() -> Self {
        Self {
            substitution_map: HashMap::new(),
            substituted_count: 0,
        }
    }

    /// Add a substitution rule.
    pub fn add_substitution(&mut self, old_id: u32, new_id: u32) {
        if !self.substitution_map.contains_key(&old_id) {
            self.substituted_count += 1;
        }
        self.substitution_map.insert(old_id, new_id);
    }

    /// Get the substitution for an item.
    pub fn get_substitution(&self, old_id: u32) -> Option<u32> {
        self.substitution_map.get(&old_id).copied()
    }

    /// Check if an item has been substituted.
    pub fn has_substitution(&self, old_id: u32) -> bool {
        self.substitution_map.contains_key(&old_id)
    }

    /// Get the number of substitutions.
    pub fn substitution_count(&self) -> usize {
        self.substitution_map.len()
    }

    /// Get all old ids.
    pub fn old_ids(&self) -> Vec<u32> {
        self.substitution_map.keys().copied().collect()
    }

    /// Remove a substitution.
    pub fn remove_substitution(&mut self, old_id: u32) {
        if self.substitution_map.remove(&old_id).is_some() {
            self.substituted_count = self.substituted_count.saturating_sub(1);
        }
    }

    /// Clear all substitutions.
    pub fn clear(&mut self) {
        self.substitution_map.clear();
        self.substituted_count = 0;
    }

    /// Build the resulting shape after applying substitutions.
    pub fn build(&self) -> Vec<(u32, u32)> {
        self.substitution_map
            .iter()
            .map(|(&old, &new)| (old, new))
            .collect()
    }
}

impl Default for BRepToolsSubstitution {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = BRepToolsSubstitution::new();
        assert_eq!(s.substitution_count(), 0);
    }

    #[test]
    fn test_add_substitution() {
        let mut s = BRepToolsSubstitution::new();
        s.add_substitution(1, 10);

        assert!(s.has_substitution(1));
        assert_eq!(s.get_substitution(1), Some(10));
        assert_eq!(s.substitution_count(), 1);
    }

    #[test]
    fn test_remove_substitution() {
        let mut s = BRepToolsSubstitution::new();
        s.add_substitution(1, 10);
        s.remove_substitution(1);

        assert!(!s.has_substitution(1));
        assert_eq!(s.substitution_count(), 0);
    }

    #[test]
    fn test_old_ids() {
        let mut s = BRepToolsSubstitution::new();
        s.add_substitution(1, 10);
        s.add_substitution(2, 20);

        let ids = s.old_ids();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&1));
        assert!(ids.contains(&2));
    }

    #[test]
    fn test_build() {
        let mut s = BRepToolsSubstitution::new();
        s.add_substitution(1, 10);
        s.add_substitution(2, 20);

        let result = s.build();
        assert_eq!(result.len(), 2);
        assert!(result.contains(&(1, 10)));
        assert!(result.contains(&(2, 20)));
    }
}

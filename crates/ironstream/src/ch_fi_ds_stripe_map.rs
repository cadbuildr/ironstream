// FILE: ch_fi_ds_stripe_map.rs
// occt: ChFiDS_StripeMap

use std::collections::HashMap;

/// Encapsulation of a map from vertices to lists of stripe handles.
/// Provides indexed access to vertex-stripe associations.
#[derive(Debug, Clone)]
pub struct ChFiDsStripeMap {
    /// Map from vertex id to list of stripe indices
    vertex_to_stripes: HashMap<usize, Vec<usize>>,
    /// Counter for extent tracking
    extent: usize,
}

impl ChFiDsStripeMap {
    /// Create a new empty StripeMap
    pub fn new() -> Self {
        Self {
            vertex_to_stripes: HashMap::new(),
            extent: 0,
        }
    }

    /// Add a vertex-stripe association
    /// vertex_id should be a stable hash of the vertex
    /// stripe_id should be a unique index of the stripe
    pub fn add(&mut self, vertex_id: usize, stripe_id: usize) {
        self.vertex_to_stripes
            .entry(vertex_id)
            .or_insert_with(Vec::new)
            .push(stripe_id);
        self.extent += 1;
    }

    /// Get the number of associations
    pub fn extent(&self) -> usize {
        self.extent
    }

    /// Find stripes by vertex id
    pub fn find_from_key(&self, vertex_id: usize) -> Option<&Vec<usize>> {
        self.vertex_to_stripes.get(&vertex_id)
    }

    /// Clear all associations
    pub fn clear(&mut self) {
        self.vertex_to_stripes.clear();
        self.extent = 0;
    }

    /// Get number of unique vertices
    pub fn len(&self) -> usize {
        self.vertex_to_stripes.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.vertex_to_stripes.is_empty()
    }
}

impl Default for ChFiDsStripeMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let map = ChFiDsStripeMap::new();
        assert_eq!(map.extent(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_add() {
        let mut map = ChFiDsStripeMap::new();
        map.add(1, 10);
        assert_eq!(map.extent(), 1);
        assert!(!map.is_empty());
    }

    #[test]
    fn test_find_from_key() {
        let mut map = ChFiDsStripeMap::new();
        map.add(5, 20);
        map.add(5, 21);
        let stripes = map.find_from_key(5);
        assert!(stripes.is_some());
        assert_eq!(stripes.unwrap().len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut map = ChFiDsStripeMap::new();
        map.add(1, 10);
        map.add(2, 20);
        assert_eq!(map.len(), 2);
        map.clear();
        assert!(map.is_empty());
        assert_eq!(map.extent(), 0);
    }

    #[test]
    fn test_default() {
        let map = ChFiDsStripeMap::default();
        assert!(map.is_empty());
    }
}

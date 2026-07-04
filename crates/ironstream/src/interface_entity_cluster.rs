// FILE: interface_entity_cluster.rs
// occt: Interface_EntityCluster

/// A cluster of entities for efficient grouping.
#[derive(Clone, Debug)]
pub struct InterfaceEntityCluster {
    entities: Vec<usize>,
}

impl InterfaceEntityCluster {
    /// Creates an empty cluster
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    /// Adds an entity to the cluster
    pub fn add(&mut self, entity_id: usize) {
        self.entities.push(entity_id);
    }

    /// Returns the count of entities
    pub fn count(&self) -> usize {
        self.entities.len()
    }

    /// Gets an entity by index (1-indexed)
    pub fn get(&self, num: usize) -> Option<usize> {
        if num >= 1 && num <= self.entities.len() {
            Some(self.entities[num - 1])
        } else {
            None
        }
    }
}

impl Default for InterfaceEntityCluster {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let cluster = InterfaceEntityCluster::new();
        assert_eq!(cluster.count(), 0);
    }

    #[test]
    fn test_add() {
        let mut cluster = InterfaceEntityCluster::new();
        cluster.add(1);
        cluster.add(2);
        assert_eq!(cluster.count(), 2);
        assert_eq!(cluster.get(1), Some(1));
        assert_eq!(cluster.get(2), Some(2));
    }
}

// FILE: mesh_vs_map_of_two_nodes.rs
// occt: MeshVS_MapOfTwoNodes, MeshVS_MapIteratorOfMapOfTwoNodes

use std::collections::HashSet;

/// MeshVS_TwoNodes represents a pair of node IDs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MeshVsTwoNodes {
    pub node1: i32,
    pub node2: i32,
}

impl MeshVsTwoNodes {
    pub fn new(node1: i32, node2: i32) -> Self {
        MeshVsTwoNodes { node1, node2 }
    }

    pub fn first(&self) -> i32 {
        self.node1
    }

    pub fn second(&self) -> i32 {
        self.node2
    }
}

/// Deprecated typedef alias for backward compatibility.
/// Original OCCT: `NCollection_Map<MeshVS_TwoNodes>`
pub type MeshVsMapOfTwoNodes = HashSet<MeshVsTwoNodes>;

/// Deprecated typedef alias for the iterator.
/// Original OCCT: `NCollection_Map<MeshVS_TwoNodes>::Iterator`
pub type MeshVsMapIteratorOfMapOfTwoNodes = std::collections::hash_set::IntoIter<MeshVsTwoNodes>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_nodes_creation() {
        let nodes = MeshVsTwoNodes::new(1, 2);
        assert_eq!(nodes.first(), 1);
        assert_eq!(nodes.second(), 2);
    }

    #[test]
    fn test_two_nodes_equality() {
        let nodes1 = MeshVsTwoNodes::new(1, 2);
        let nodes2 = MeshVsTwoNodes::new(1, 2);
        let nodes3 = MeshVsTwoNodes::new(2, 1);

        assert_eq!(nodes1, nodes2);
        assert_ne!(nodes1, nodes3);
    }

    #[test]
    fn test_two_nodes_hash() {
        let nodes1 = MeshVsTwoNodes::new(1, 2);
        let nodes2 = MeshVsTwoNodes::new(1, 2);

        let mut set = HashSet::new();
        set.insert(nodes1);
        set.insert(nodes2);

        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_map_creation() {
        let map: MeshVsMapOfTwoNodes = HashSet::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_insert_and_contains() {
        let mut map: MeshVsMapOfTwoNodes = HashSet::new();

        let nodes1 = MeshVsTwoNodes::new(1, 2);
        let nodes2 = MeshVsTwoNodes::new(3, 4);
        let nodes3 = MeshVsTwoNodes::new(5, 6);

        assert!(map.insert(nodes1));
        assert!(map.insert(nodes2));
        assert!(map.insert(nodes3));

        assert!(map.contains(&nodes1));
        assert!(map.contains(&nodes2));
        assert!(map.contains(&nodes3));
        assert!(!map.contains(&MeshVsTwoNodes::new(0, 1)));
    }

    #[test]
    fn test_map_remove() {
        let mut map: MeshVsMapOfTwoNodes = HashSet::new();

        let nodes = MeshVsTwoNodes::new(10, 20);
        assert!(map.insert(nodes));
        assert!(map.contains(&nodes));

        assert!(map.remove(&nodes));
        assert!(!map.contains(&nodes));
        assert!(!map.remove(&nodes));
    }

    #[test]
    fn test_map_size() {
        let mut map: MeshVsMapOfTwoNodes = HashSet::new();
        assert_eq!(map.len(), 0);

        let nodes1 = MeshVsTwoNodes::new(1, 2);
        let nodes2 = MeshVsTwoNodes::new(3, 4);
        let nodes3 = MeshVsTwoNodes::new(5, 6);

        map.insert(nodes1);
        assert_eq!(map.len(), 1);

        map.insert(nodes2);
        map.insert(nodes3);
        assert_eq!(map.len(), 3);

        map.remove(&nodes1);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_map_iteration() {
        let mut map: MeshVsMapOfTwoNodes = HashSet::new();

        let nodes1 = MeshVsTwoNodes::new(1, 2);
        let nodes2 = MeshVsTwoNodes::new(3, 4);
        let nodes3 = MeshVsTwoNodes::new(5, 6);

        map.insert(nodes1);
        map.insert(nodes2);
        map.insert(nodes3);

        let collected: Vec<MeshVsTwoNodes> = map.into_iter().collect();
        assert_eq!(collected.len(), 3);
    }
}

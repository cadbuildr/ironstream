// FILE: draft_indexed_data_map_of_edge_edge_info.rs
// occt: Draft_IndexedDataMapOfEdgeEdgeInfo

/// Draft_IndexedDataMapOfEdgeEdgeInfo: Deprecated typedef for backward compatibility.
///
/// This is a deprecated alias for:
/// `NCollection_IndexedDataMap<TopoDS_Edge, Draft_EdgeInfo, TopTools_ShapeMapHasher>`
///
/// Since OCCT 8.0.0, users should use the direct template instantiation instead.
/// This module provides the alias for compatibility.
///
/// An indexed data map maps edges to edge information with shape hashing.

// In Rust, we use a type alias wrapper. However, since we don't have the full
// TopoDS_Edge and Draft_EdgeInfo types in this minimal kernel, we provide
// the semantic structure:

/// Edge information stored in indexed data map.
#[derive(Clone, Debug)]
pub struct EdgeInfo {
    // Edge-specific data would go here
}

/// Indexed data map: Edge -> EdgeInfo with fast lookup and iteration order preservation
#[derive(Clone, Debug)]
pub struct DraftIndexedDataMapOfEdgeEdgeInfo {
    entries: Vec<(usize, EdgeInfo)>, // (edge_id, info)
}

impl DraftIndexedDataMapOfEdgeEdgeInfo {
    /// Creates a new empty indexed data map.
    pub fn new() -> Self {
        DraftIndexedDataMapOfEdgeEdgeInfo {
            entries: Vec::new(),
        }
    }

    /// Adds or updates an entry at the given index.
    pub fn bind(&mut self, edge_id: usize, info: EdgeInfo) {
        if let Some(pos) = self.entries.iter().position(|(id, _)| *id == edge_id) {
            self.entries[pos].1 = info;
        } else {
            self.entries.push((edge_id, info));
        }
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for DraftIndexedDataMapOfEdgeEdgeInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let map = DraftIndexedDataMapOfEdgeEdgeInfo::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_insert() {
        let mut map = DraftIndexedDataMapOfEdgeEdgeInfo::new();
        let info = EdgeInfo {};
        map.bind(1, info);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_default() {
        let map = DraftIndexedDataMapOfEdgeEdgeInfo::default();
        assert!(map.is_empty());
    }
}

// FILE: draft_indexed_data_map_of_vertex_vertex_info.rs
// occt: Draft_IndexedDataMapOfVertexVertexInfo

/// Draft_IndexedDataMapOfVertexVertexInfo: Deprecated typedef for backward compatibility.
///
/// This is a deprecated alias for:
/// `NCollection_IndexedDataMap<TopoDS_Vertex, Draft_VertexInfo, TopTools_ShapeMapHasher>`
///
/// Since OCCT 8.0.0, users should use the direct template instantiation instead.
/// This module provides the alias for compatibility.

/// Vertex information stored in indexed data map.
#[derive(Clone, Debug)]
pub struct VertexInfo {
    // Vertex-specific data would go here
}

/// Indexed data map: Vertex -> VertexInfo with fast lookup and iteration order preservation
#[derive(Clone, Debug)]
pub struct DraftIndexedDataMapOfVertexVertexInfo {
    entries: Vec<(usize, VertexInfo)>, // (vertex_id, info)
}

impl DraftIndexedDataMapOfVertexVertexInfo {
    /// Creates a new empty indexed data map.
    pub fn new() -> Self {
        DraftIndexedDataMapOfVertexVertexInfo {
            entries: Vec::new(),
        }
    }

    /// Adds or updates an entry at the given index.
    pub fn bind(&mut self, vertex_id: usize, info: VertexInfo) {
        if let Some(pos) = self.entries.iter().position(|(id, _)| *id == vertex_id) {
            self.entries[pos].1 = info;
        } else {
            self.entries.push((vertex_id, info));
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

impl Default for DraftIndexedDataMapOfVertexVertexInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let map = DraftIndexedDataMapOfVertexVertexInfo::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_insert() {
        let mut map = DraftIndexedDataMapOfVertexVertexInfo::new();
        let info = VertexInfo {};
        map.bind(1, info);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_default() {
        let map = DraftIndexedDataMapOfVertexVertexInfo::default();
        assert!(map.is_empty());
    }
}

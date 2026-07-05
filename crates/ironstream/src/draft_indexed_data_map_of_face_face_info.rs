// FILE: draft_indexed_data_map_of_face_face_info.rs
// occt: Draft_IndexedDataMapOfFaceFaceInfo

/// Draft_IndexedDataMapOfFaceFaceInfo: Deprecated typedef for backward compatibility.
///
/// This is a deprecated alias for:
/// `NCollection_IndexedDataMap<TopoDS_Face, Draft_FaceInfo, TopTools_ShapeMapHasher>`
///
/// Since OCCT 8.0.0, users should use the direct template instantiation instead.
/// This module provides the alias for compatibility.

/// Face information stored in indexed data map.
#[derive(Clone, Debug)]
pub struct FaceInfo {
    // Face-specific data would go here
}

/// Indexed data map: Face -> FaceInfo with fast lookup and iteration order preservation
#[derive(Clone, Debug)]
pub struct DraftIndexedDataMapOfFaceFaceInfo {
    entries: Vec<(usize, FaceInfo)>, // (face_id, info)
}

impl DraftIndexedDataMapOfFaceFaceInfo {
    /// Creates a new empty indexed data map.
    pub fn new() -> Self {
        DraftIndexedDataMapOfFaceFaceInfo {
            entries: Vec::new(),
        }
    }

    /// Adds or updates an entry at the given index.
    pub fn bind(&mut self, face_id: usize, info: FaceInfo) {
        if let Some(pos) = self.entries.iter().position(|(id, _)| *id == face_id) {
            self.entries[pos].1 = info;
        } else {
            self.entries.push((face_id, info));
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

impl Default for DraftIndexedDataMapOfFaceFaceInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let map = DraftIndexedDataMapOfFaceFaceInfo::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_insert() {
        let mut map = DraftIndexedDataMapOfFaceFaceInfo::new();
        let info = FaceInfo {};
        map.bind(1, info);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_default() {
        let map = DraftIndexedDataMapOfFaceFaceInfo::default();
        assert!(map.is_empty());
    }
}

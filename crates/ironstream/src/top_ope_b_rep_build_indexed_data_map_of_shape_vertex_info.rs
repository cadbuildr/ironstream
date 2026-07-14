// FILE: top_ope_b_rep_build_indexed_data_map_of_shape_vertex_info.rs
// occt: TopOpeBRepBuild_IndexedDataMapOfShapeVertexInfo
// occt-ref: TopOpeBRepBuild_VertexInfo

use std::collections::HashMap;

/// ShapeKey: Simplified Shape for hashing.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShapeKey {
    id: usize,
}

impl ShapeKey {
    pub fn new(id: usize) -> Self {
        ShapeKey { id }
    }
}

/// VertexInfo: Information about a vertex.
#[derive(Clone, Debug)]
pub struct VertexInfo {
    index: i32,
    point_x: f64,
    point_y: f64,
    point_z: f64,
}

impl VertexInfo {
    pub fn new() -> Self {
        VertexInfo {
            index: 0,
            point_x: 0.0,
            point_y: 0.0,
            point_z: 0.0,
        }
    }

    pub fn with_index_and_point(index: i32, x: f64, y: f64, z: f64) -> Self {
        VertexInfo {
            index,
            point_x: x,
            point_y: y,
            point_z: z,
        }
    }

    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn set_index(&mut self, index: i32) {
        self.index = index;
    }

    pub fn point(&self) -> (f64, f64, f64) {
        (self.point_x, self.point_y, self.point_z)
    }

    pub fn set_point(&mut self, x: f64, y: f64, z: f64) {
        self.point_x = x;
        self.point_y = y;
        self.point_z = z;
    }
}

impl Default for VertexInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// IndexedDataMapOfShapeVertexInfo: Indexed map (1-based) from Shape to VertexInfo.
///
/// Unlike regular DataMap, IndexedDataMap maintains insertion order and provides
/// 1-based indexing (OCCT style).
#[derive(Clone, Debug)]
pub struct IndexedDataMapOfShapeVertexInfo {
    entries: Vec<(ShapeKey, VertexInfo)>, // Maintains order, 1-based via position
}

impl IndexedDataMapOfShapeVertexInfo {
    /// Creates a new empty indexed map.
    pub fn new() -> Self {
        IndexedDataMapOfShapeVertexInfo {
            entries: Vec::new(),
        }
    }

    /// Adds or updates a shape-vertexinfo pair.
    /// Returns the 1-based index where it was inserted/updated.
    pub fn add(&mut self, shape: ShapeKey, info: VertexInfo) -> usize {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == &shape) {
            self.entries[pos] = (shape, info);
            pos + 1 // 1-based
        } else {
            self.entries.push((shape, info));
            self.entries.len() // 1-based
        }
    }

    /// Binds a shape to vertex info. Returns true if newly inserted.
    pub fn bind(&mut self, shape: ShapeKey, info: VertexInfo) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|(k, _)| k == &shape) {
            entry.1 = info;
            false
        } else {
            self.entries.push((shape, info));
            true
        }
    }

    /// Returns true if the map contains the shape.
    pub fn contains(&self, shape: &ShapeKey) -> bool {
        self.entries.iter().any(|(k, _)| k == shape)
    }

    /// Returns a reference to the vertex info (0-based internal index).
    pub fn find(&self, shape: &ShapeKey) -> Option<&VertexInfo> {
        self.entries.iter().find(|(k, _)| k == shape).map(|(_, v)| v)
    }

    /// Returns a mutable reference to the vertex info.
    pub fn find_mut(&mut self, shape: &ShapeKey) -> Option<&mut VertexInfo> {
        self.entries.iter_mut().find(|(k, _)| k == shape).map(|(_, v)| v)
    }

    /// Gets value at 1-based index.
    pub fn value_at(&self, index_1based: usize) -> Option<&VertexInfo> {
        if index_1based == 0 {
            None
        } else {
            self.entries.get(index_1based - 1).map(|(_, v)| v)
        }
    }

    /// Gets key at 1-based index.
    pub fn key_at(&self, index_1based: usize) -> Option<&ShapeKey> {
        if index_1based == 0 {
            None
        } else {
            self.entries.get(index_1based - 1).map(|(k, _)| k)
        }
    }

    /// Gets mutable value at 1-based index.
    pub fn value_at_mut(&mut self, index_1based: usize) -> Option<&mut VertexInfo> {
        if index_1based == 0 {
            None
        } else {
            self.entries.get_mut(index_1based - 1).map(|(_, v)| v)
        }
    }

    /// Removes a shape from the map.
    pub fn remove(&mut self, shape: &ShapeKey) -> bool {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == shape) {
            self.entries.remove(pos);
            true
        } else {
            false
        }
    }

    /// Returns the size of the map.
    pub fn size(&self) -> usize {
        self.entries.len()
    }

    /// Clears the map.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns the lower bound (always 1 for OCCT IndexedDataMap).
    pub fn lower(&self) -> usize {
        1
    }

    /// Returns the upper bound (same as size).
    pub fn upper(&self) -> usize {
        self.entries.len()
    }

    /// Returns an iterator.
    pub fn iter(&self) -> impl Iterator<Item = (&ShapeKey, &VertexInfo)> {
        self.entries.iter().map(|(k, v)| (k, v))
    }

    /// Returns a mutable iterator.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&ShapeKey, &mut VertexInfo)> {
        self.entries.iter_mut().map(|(k, v)| (&*k, v))
    }
}

impl Default for IndexedDataMapOfShapeVertexInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_info_new() {
        let info = VertexInfo::new();
        assert_eq!(info.index(), 0);
        let (x, y, z) = info.point();
        assert_eq!((x, y, z), (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_vertex_info_with_values() {
        let info = VertexInfo::with_index_and_point(42, 1.0, 2.0, 3.0);
        assert_eq!(info.index(), 42);
        assert_eq!(info.point(), (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_indexed_map_add() {
        let mut map = IndexedDataMapOfShapeVertexInfo::new();
        let shape1 = ShapeKey::new(1);
        let info1 = VertexInfo::with_index_and_point(10, 0.5, 0.5, 0.5);

        let idx1 = map.add(shape1.clone(), info1);
        assert_eq!(idx1, 1); // 1-based

        let shape2 = ShapeKey::new(2);
        let info2 = VertexInfo::with_index_and_point(20, 1.5, 1.5, 1.5);
        let idx2 = map.add(shape2, info2);
        assert_eq!(idx2, 2);

        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_indexed_map_contains() {
        let mut map = IndexedDataMapOfShapeVertexInfo::new();
        let shape = ShapeKey::new(5);
        assert!(!map.contains(&shape));

        map.bind(shape.clone(), VertexInfo::new());
        assert!(map.contains(&shape));
    }

    #[test]
    fn test_indexed_map_find() {
        let mut map = IndexedDataMapOfShapeVertexInfo::new();
        let shape = ShapeKey::new(3);
        let info = VertexInfo::with_index_and_point(99, 10.0, 20.0, 30.0);

        map.bind(shape.clone(), info);
        let found = map.find(&shape).unwrap();
        assert_eq!(found.index(), 99);
    }

    #[test]
    fn test_indexed_map_value_at() {
        let mut map = IndexedDataMapOfShapeVertexInfo::new();
        let shape1 = ShapeKey::new(1);
        let info1 = VertexInfo::with_index_and_point(100, 1.0, 1.0, 1.0);
        let shape2 = ShapeKey::new(2);
        let info2 = VertexInfo::with_index_and_point(200, 2.0, 2.0, 2.0);

        map.add(shape1, info1);
        map.add(shape2, info2);

        assert!(map.value_at(0).is_none()); // 0 is invalid
        let val1 = map.value_at(1).unwrap();
        assert_eq!(val1.index(), 100);
        let val2 = map.value_at(2).unwrap();
        assert_eq!(val2.index(), 200);
    }

    #[test]
    fn test_indexed_map_remove() {
        let mut map = IndexedDataMapOfShapeVertexInfo::new();
        let shape = ShapeKey::new(7);
        map.bind(shape.clone(), VertexInfo::new());

        assert_eq!(map.size(), 1);
        assert!(map.remove(&shape));
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_indexed_map_bounds() {
        let mut map = IndexedDataMapOfShapeVertexInfo::new();
        map.add(ShapeKey::new(1), VertexInfo::new());
        map.add(ShapeKey::new(2), VertexInfo::new());

        assert_eq!(map.lower(), 1);
        assert_eq!(map.upper(), 2);
    }
}

// FILE: topo_shape_iter.rs
//! Indexed and unordered map containers for topological shapes, mirroring the
//! OCCT `TopTools_IndexedMapOfShape` and `TopTools_MapOfShape` APIs.
//!
//! These containers store shapes as opaque string keys (shape labels / hash
//! representations).  In a full OCCT reimplementation the key type would be a
//! richer `TopoDS_Shape` handle; the string representation is a placeholder
//! that keeps the module dependency-free while preserving the public API
//! contract.
//!
//! # Key types
//!
//! * [`IndexedMapOfShape`] — ordered map: shapes are accessible both by their
//!   string key and by a 1-based integer index.
//! * [`MapOfShape`]        — unordered set: membership queries only.
//!
//! # Free functions
//!
//! * [`map_shapes_and_ancestors`] — collect a root shape and all of its
//!   sub-shape labels into an [`IndexedMapOfShape`].

use std::collections::{HashMap, HashSet};

// ---------------------------------------------------------------------------
// IndexedMapOfShape
// ---------------------------------------------------------------------------

/// An ordered, duplicate-free map from shape labels to 1-based integer indices.
///
/// Mirrors the OCCT `TopTools_IndexedMapOfShape` class.  Shapes are stored in
/// insertion order; the index assigned to a shape never changes after it has
/// been added (indices are **not** compacted on removal because OCCT's API does
/// not expose removal either).
// occt: TopTools_IndexedMapOfShape
#[derive(Debug, Clone)]
pub struct IndexedMapOfShape {
    /// Shapes in insertion order (0-based internally; the public API is 1-based).
    pub shapes: Vec<String>,
    /// Maps each shape label to its 1-based index.
    pub index: HashMap<String, usize>,
}

impl IndexedMapOfShape {
    /// Create an empty map.
    ///
    /// # Example
    /// ```
    /// # use ironstream::topo_shape_iter::IndexedMapOfShape;
    /// let m = IndexedMapOfShape::new();
    /// assert_eq!(m.size(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            index: HashMap::new(),
        }
    }

    /// Add `s` to the map.
    ///
    /// If `s` is already present the existing 1-based index is returned and
    /// the map is **not** modified (mirrors `TopTools_IndexedMapOfShape::Add`).
    /// Otherwise `s` is appended and the new 1-based index is returned.
    pub fn add(&mut self, s: &str) -> usize {
        if let Some(&i) = self.index.get(s) {
            return i;
        }
        let idx = self.shapes.len() + 1; // 1-based
        self.shapes.push(s.to_string());
        self.index.insert(s.to_string(), idx);
        idx
    }

    /// Return the 1-based index of `s`, or `None` if `s` is not in the map.
    ///
    /// Mirrors `TopTools_IndexedMapOfShape::FindIndex`.
    pub fn find_index(&self, s: &str) -> Option<usize> {
        self.index.get(s).copied()
    }

    /// Return the shape label at 1-based index `i`, or `None` if out of range.
    ///
    /// Mirrors `TopTools_IndexedMapOfShape::FindKey`.
    pub fn find_key(&self, i: usize) -> Option<&String> {
        if i == 0 || i > self.shapes.len() {
            return None;
        }
        self.shapes.get(i - 1)
    }

    /// Number of shapes currently stored.
    ///
    /// Mirrors `TopTools_IndexedMapOfShape::Extent`.
    pub fn size(&self) -> usize {
        self.shapes.len()
    }

    /// Return `true` if `s` is present in the map.
    ///
    /// Mirrors `TopTools_IndexedMapOfShape::Contains`.
    pub fn contains(&self, s: &str) -> bool {
        self.index.contains_key(s)
    }

    /// Remove all entries, resetting the map to its initial empty state.
    ///
    /// Mirrors `TopTools_IndexedMapOfShape::Clear`.
    pub fn clear(&mut self) {
        self.shapes.clear();
        self.index.clear();
    }
}

impl Default for IndexedMapOfShape {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// MapOfShape
// ---------------------------------------------------------------------------

/// An unordered, duplicate-free set of shape labels.
///
/// Mirrors the OCCT `TopTools_MapOfShape` class.  Unlike [`IndexedMapOfShape`]
/// this container does not assign integer indices; it is suitable for fast
/// membership tests and de-duplication passes.
// occt: TopTools_MapOfShape
#[derive(Debug, Clone)]
pub struct MapOfShape {
    /// The underlying set of shape labels.
    pub set: HashSet<String>,
}

impl MapOfShape {
    /// Create an empty map.
    ///
    /// # Example
    /// ```
    /// # use ironstream::topo_shape_iter::MapOfShape;
    /// let m = MapOfShape::new();
    /// assert_eq!(m.size(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }

    /// Add `s` to the set.
    ///
    /// Returns `true` if `s` was newly inserted, or `false` if it was already
    /// present.  Mirrors `TopTools_MapOfShape::Add`.
    pub fn add(&mut self, s: &str) -> bool {
        self.set.insert(s.to_string())
    }

    /// Return `true` if `s` is present.
    ///
    /// Mirrors `TopTools_MapOfShape::Contains`.
    pub fn contains(&self, s: &str) -> bool {
        self.set.contains(s)
    }

    /// Number of shapes currently stored.
    ///
    /// Mirrors `TopTools_MapOfShape::Extent`.
    pub fn size(&self) -> usize {
        self.set.len()
    }

    /// Remove all entries, resetting the set to empty.
    ///
    /// Mirrors `TopTools_MapOfShape::Clear`.
    pub fn clear(&mut self) {
        self.set.clear();
    }
}

impl Default for MapOfShape {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// map_shapes_and_ancestors
// ---------------------------------------------------------------------------

/// Collect `root` and all of its sub-shape labels into an [`IndexedMapOfShape`].
///
/// In a full OCCT reimplementation this function would walk the topological
/// graph stored in `BRep_TShape` structures and call
/// `TopExp::MapShapesAndAncestors`.  Here the root label is added as the sole
/// entry; sub-shape discovery is left as a hook for callers that maintain their
/// own topology graph.
///
/// # Arguments
///
/// * `root` — opaque shape label (e.g. `"Solid_1"`, a hash, or any unique key).
///
/// # Returns
///
/// An [`IndexedMapOfShape`] containing at least `root` at index 1.
///
/// # Example
/// ```
/// # use ironstream::topo_shape_iter::map_shapes_and_ancestors;
/// let m = map_shapes_and_ancestors("Solid_1");
/// assert_eq!(m.find_key(1).map(String::as_str), Some("Solid_1"));
/// ```
// occt: TopExp::MapShapesAndAncestors
pub fn map_shapes_and_ancestors(root: &str) -> IndexedMapOfShape {
    let mut map = IndexedMapOfShape::new();
    map.add(root);
    map
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- IndexedMapOfShape ---------------------------------------------------

    #[test]
    fn indexed_map_new_is_empty() {
        let m = IndexedMapOfShape::new();
        assert_eq!(m.size(), 0);
    }

    #[test]
    fn indexed_map_add_returns_one_based_index() {
        let mut m = IndexedMapOfShape::new();
        let i1 = m.add("Face_1");
        let i2 = m.add("Face_2");
        assert_eq!(i1, 1);
        assert_eq!(i2, 2);
    }

    #[test]
    fn indexed_map_add_duplicate_returns_existing_index() {
        let mut m = IndexedMapOfShape::new();
        let i1 = m.add("Edge_1");
        let i2 = m.add("Edge_1");
        assert_eq!(i1, i2);
        assert_eq!(m.size(), 1);
    }

    #[test]
    fn indexed_map_find_index_hit_and_miss() {
        let mut m = IndexedMapOfShape::new();
        m.add("Wire_1");
        assert_eq!(m.find_index("Wire_1"), Some(1));
        assert_eq!(m.find_index("Wire_99"), None);
    }

    #[test]
    fn indexed_map_find_key_valid_and_out_of_range() {
        let mut m = IndexedMapOfShape::new();
        m.add("Vertex_A");
        assert_eq!(m.find_key(1).map(String::as_str), Some("Vertex_A"));
        assert!(m.find_key(0).is_none());
        assert!(m.find_key(2).is_none());
    }

    #[test]
    fn indexed_map_contains() {
        let mut m = IndexedMapOfShape::new();
        m.add("Shell_1");
        assert!(m.contains("Shell_1"));
        assert!(!m.contains("Shell_2"));
    }

    #[test]
    fn indexed_map_clear() {
        let mut m = IndexedMapOfShape::new();
        m.add("Solid_1");
        m.clear();
        assert_eq!(m.size(), 0);
        assert!(!m.contains("Solid_1"));
    }

    // -- MapOfShape ----------------------------------------------------------

    #[test]
    fn map_of_shape_new_is_empty() {
        let m = MapOfShape::new();
        assert_eq!(m.size(), 0);
    }

    #[test]
    fn map_of_shape_add_returns_true_on_new_false_on_dup() {
        let mut m = MapOfShape::new();
        assert!(m.add("Face_A"));
        assert!(!m.add("Face_A"));
        assert_eq!(m.size(), 1);
    }

    #[test]
    fn map_of_shape_contains() {
        let mut m = MapOfShape::new();
        m.add("Edge_X");
        assert!(m.contains("Edge_X"));
        assert!(!m.contains("Edge_Y"));
    }

    #[test]
    fn map_of_shape_clear() {
        let mut m = MapOfShape::new();
        m.add("Compound_1");
        m.clear();
        assert_eq!(m.size(), 0);
        assert!(!m.contains("Compound_1"));
    }

    // -- map_shapes_and_ancestors --------------------------------------------

    #[test]
    fn map_shapes_and_ancestors_contains_root() {
        let m = map_shapes_and_ancestors("Solid_1");
        assert_eq!(m.size(), 1);
        assert!(m.contains("Solid_1"));
        assert_eq!(m.find_key(1).map(String::as_str), Some("Solid_1"));
    }

    #[test]
    fn map_shapes_and_ancestors_empty_root() {
        let m = map_shapes_and_ancestors("");
        assert_eq!(m.size(), 1);
        assert!(m.contains(""));
    }
}

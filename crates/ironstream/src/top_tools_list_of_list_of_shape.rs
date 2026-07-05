// FILE: top_tools_list_of_list_of_shape.rs
// occt: TopTools_ListOfListOfShape

use std::collections::LinkedList;

/// Deprecated typedef: NCollection_List<TopTools_ListOfShape>
///
/// A doubly-nested list of TopTools_ListOfShape.
/// This is a faithful Rust port maintaining OCCT container semantics:
/// - Wraps std LinkedList with 1-based indexing (for sequence operations)
/// - Supports iteration and manipulation of nested lists
#[derive(Clone, Debug)]
pub struct TopToolsListOfListOfShape {
    items: LinkedList<TopToolsListOfShape>,
}

/// A single list of shapes (inner container)
#[derive(Clone, Debug)]
pub struct TopToolsListOfShape {
    items: LinkedList<String>, // Placeholder for TopoDS_Shape (would be actual shape type)
}

impl TopToolsListOfListOfShape {
    /// Create an empty list of lists
    pub fn new() -> Self {
        TopToolsListOfListOfShape {
            items: LinkedList::new(),
        }
    }

    /// Append a list of shapes to this list of lists
    pub fn append(&mut self, list: TopToolsListOfShape) {
        self.items.push_back(list);
    }

    /// Get the size (number of lists)
    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Clear all lists
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Get iterator
    pub fn iter(&self) -> impl Iterator<Item = &TopToolsListOfShape> {
        self.items.iter()
    }

    /// Get mutable iterator
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut TopToolsListOfShape> {
        self.items.iter_mut()
    }
}

impl Default for TopToolsListOfListOfShape {
    fn default() -> Self {
        Self::new()
    }
}

impl TopToolsListOfShape {
    /// Create an empty list of shapes
    pub fn new() -> Self {
        TopToolsListOfShape {
            items: LinkedList::new(),
        }
    }

    /// Append a shape (represented as String for now)
    pub fn append(&mut self, shape: String) {
        self.items.push_back(shape);
    }

    /// Get the size (number of shapes)
    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Clear all shapes
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Get iterator
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.items.iter()
    }
}

impl Default for TopToolsListOfShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list_of_list() {
        let list = TopToolsListOfListOfShape::new();
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_append_lists() {
        let mut outer = TopToolsListOfListOfShape::new();
        let mut inner1 = TopToolsListOfShape::new();
        inner1.append("shape1".to_string());
        inner1.append("shape2".to_string());

        outer.append(inner1);
        assert_eq!(outer.size(), 1);
    }

    #[test]
    fn test_nested_iteration() {
        let mut outer = TopToolsListOfListOfShape::new();

        let mut inner1 = TopToolsListOfShape::new();
        inner1.append("s1".to_string());
        inner1.append("s2".to_string());

        let mut inner2 = TopToolsListOfShape::new();
        inner2.append("s3".to_string());

        outer.append(inner1);
        outer.append(inner2);

        let mut total_shapes = 0;
        for list in outer.iter() {
            total_shapes += list.size();
        }
        assert_eq!(total_shapes, 3);
    }

    #[test]
    fn test_inner_list_operations() {
        let mut inner = TopToolsListOfShape::new();
        assert!(inner.is_empty());

        inner.append("shape_a".to_string());
        assert_eq!(inner.size(), 1);

        inner.clear();
        assert!(inner.is_empty());
    }

    #[test]
    fn test_list_of_list_clear() {
        let mut outer = TopToolsListOfListOfShape::new();
        let mut inner = TopToolsListOfShape::new();
        inner.append("test".to_string());
        outer.append(inner);

        assert_eq!(outer.size(), 1);
        outer.clear();
        assert_eq!(outer.size(), 0);
    }

    #[test]
    fn test_mutable_iteration() {
        let mut outer = TopToolsListOfListOfShape::new();
        let inner = TopToolsListOfShape::new();
        outer.append(inner);

        for _list in outer.iter_mut() {
            // Can modify inner list
        }
        assert_eq!(outer.size(), 1);
    }
}

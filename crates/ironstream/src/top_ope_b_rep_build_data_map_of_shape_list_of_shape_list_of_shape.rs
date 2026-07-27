// FILE: top_ope_b_rep_build_data_map_of_shape_list_of_shape_list_of_shape.rs
// occt: TopOpeBRepBuild_DataMapOfShapeListOfShapeListOfShape
// occt-ref: TopOpeBRepBuild_DataMapIteratorOfDataMapOfShapeListOfShapeListOfShape

use std::collections::HashMap;

/// ShapeSimplified: A simplified representation of TopoDS_Shape for hashing.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShapeSimplified {
    id: usize,
}

impl ShapeSimplified {
    pub fn new(id: usize) -> Self {
        ShapeSimplified { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// ListOfShapeSimplified: A list of shapes (simplified).
#[derive(Clone, Debug)]
pub struct ListOfShapeSimplified {
    shapes: Vec<ShapeSimplified>,
}

impl ListOfShapeSimplified {
    pub fn new() -> Self {
        ListOfShapeSimplified {
            shapes: Vec::new(),
        }
    }

    pub fn append(&mut self, shape: ShapeSimplified) {
        self.shapes.push(shape);
    }

    pub fn size(&self) -> usize {
        self.shapes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ShapeSimplified> {
        self.shapes.iter()
    }
}

impl Default for ListOfShapeSimplified {
    fn default() -> Self {
        Self::new()
    }
}

/// ListOfShapeListOfShape: A list of lists of shapes.
#[derive(Clone, Debug)]
pub struct ListOfShapeListOfShape {
    lists: Vec<ListOfShapeSimplified>,
}

impl ListOfShapeListOfShape {
    pub fn new() -> Self {
        ListOfShapeListOfShape {
            lists: Vec::new(),
        }
    }

    pub fn append(&mut self, list: ListOfShapeSimplified) {
        self.lists.push(list);
    }

    pub fn size(&self) -> usize {
        self.lists.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ListOfShapeSimplified> {
        self.lists.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut ListOfShapeSimplified> {
        self.lists.iter_mut()
    }

    pub fn get(&self, index: usize) -> Option<&ListOfShapeSimplified> {
        self.lists.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut ListOfShapeSimplified> {
        self.lists.get_mut(index)
    }
}

impl Default for ListOfShapeListOfShape {
    fn default() -> Self {
        Self::new()
    }
}

/// DataMapOfShapeListOfShapeListOfShape: Maps Shape -> ListOfShapeListOfShape.
///
/// This is a deprecated typedef for:
/// NCollection_DataMap<TopoDS_Shape, TopOpeBRepBuild_ListOfShapeListOfShape, TopTools_ShapeMapHasher>
#[derive(Clone, Debug)]
pub struct DataMapOfShapeListOfShapeListOfShape {
    data: HashMap<ShapeSimplified, ListOfShapeListOfShape>,
}

impl DataMapOfShapeListOfShapeListOfShape {
    /// Creates a new empty map.
    pub fn new() -> Self {
        DataMapOfShapeListOfShapeListOfShape {
            data: HashMap::new(),
        }
    }

    /// Binds a shape to a list of shape lists.
    /// Returns true if the shape was newly inserted, false if updated.
    pub fn bind(&mut self, shape: ShapeSimplified, value: ListOfShapeListOfShape) -> bool {
        self.data.insert(shape, value).is_none()
    }

    /// Returns true if the map contains the given shape.
    pub fn contains(&self, shape: &ShapeSimplified) -> bool {
        self.data.contains_key(shape)
    }

    /// Returns a reference to the value associated with the shape.
    pub fn find(&self, shape: &ShapeSimplified) -> Option<&ListOfShapeListOfShape> {
        self.data.get(shape)
    }

    /// Returns a mutable reference to the value associated with the shape.
    pub fn find_mut(&mut self, shape: &ShapeSimplified) -> Option<&mut ListOfShapeListOfShape> {
        self.data.get_mut(shape)
    }

    /// Removes a shape from the map.
    pub fn remove(&mut self, shape: &ShapeSimplified) -> bool {
        self.data.remove(shape).is_some()
    }

    /// Returns the number of entries in the map.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Clears the map.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Returns an iterator over the entries.
    pub fn iter(&self) -> impl Iterator<Item = (&ShapeSimplified, &ListOfShapeListOfShape)> {
        self.data.iter()
    }

    /// Returns a mutable iterator over the entries.
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (&ShapeSimplified, &mut ListOfShapeListOfShape)> {
        self.data.iter_mut()
    }
}

impl Default for DataMapOfShapeListOfShapeListOfShape {
    fn default() -> Self {
        Self::new()
    }
}

/// DataMapIterator: Iterator for the data map.
pub struct DataMapIterator {
    entries: Vec<(ShapeSimplified, ListOfShapeListOfShape)>,
    index: usize,
}

impl DataMapIterator {
    /// Creates a new iterator.
    pub fn new(map: &DataMapOfShapeListOfShapeListOfShape) -> Self {
        DataMapIterator {
            entries: map.data.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            index: 0,
        }
    }

    /// Returns true if there is a next element.
    pub fn is_more(&self) -> bool {
        self.index < self.entries.len()
    }

    /// Advances to the next element.
    pub fn next(&mut self) {
        self.index += 1;
    }

    /// Returns the current key.
    pub fn key(&self) -> Option<&ShapeSimplified> {
        self.entries.get(self.index).map(|(k, _)| k)
    }

    /// Returns the current value.
    pub fn value(&self) -> Option<&ListOfShapeListOfShape> {
        self.entries.get(self.index).map(|(_, v)| v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_simplified() {
        let shape = ShapeSimplified::new(42);
        assert_eq!(shape.id(), 42);
    }

    #[test]
    fn test_list_of_shape_simplified() {
        let mut list = ListOfShapeSimplified::new();
        list.append(ShapeSimplified::new(1));
        list.append(ShapeSimplified::new(2));
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_list_of_shape_list_of_shape() {
        let mut outer = ListOfShapeListOfShape::new();
        let mut inner1 = ListOfShapeSimplified::new();
        inner1.append(ShapeSimplified::new(1));
        outer.append(inner1);
        assert_eq!(outer.size(), 1);
    }

    #[test]
    fn test_data_map_bind() {
        let mut map = DataMapOfShapeListOfShapeListOfShape::new();
        let shape = ShapeSimplified::new(10);
        let list = ListOfShapeListOfShape::new();

        assert!(map.bind(shape.clone(), list.clone()));
        assert!(!map.bind(shape.clone(), list)); // Already inserted
    }

    #[test]
    fn test_data_map_contains() {
        let mut map = DataMapOfShapeListOfShapeListOfShape::new();
        let shape = ShapeSimplified::new(5);

        assert!(!map.contains(&shape));
        map.bind(shape.clone(), ListOfShapeListOfShape::new());
        assert!(map.contains(&shape));
    }

    #[test]
    fn test_data_map_find() {
        let mut map = DataMapOfShapeListOfShapeListOfShape::new();
        let shape = ShapeSimplified::new(3);
        let mut list = ListOfShapeListOfShape::new();
        let inner = ListOfShapeSimplified::new();
        list.append(inner);

        map.bind(shape.clone(), list);
        let found = map.find(&shape).unwrap();
        assert_eq!(found.size(), 1);
    }

    #[test]
    fn test_data_map_remove() {
        let mut map = DataMapOfShapeListOfShapeListOfShape::new();
        let shape = ShapeSimplified::new(7);
        map.bind(shape.clone(), ListOfShapeListOfShape::new());

        assert_eq!(map.size(), 1);
        assert!(map.remove(&shape));
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_data_map_iterator() {
        let mut map = DataMapOfShapeListOfShapeListOfShape::new();
        map.bind(ShapeSimplified::new(1), ListOfShapeListOfShape::new());
        map.bind(ShapeSimplified::new(2), ListOfShapeListOfShape::new());

        let mut iter = DataMapIterator::new(&map);
        let mut count = 0;
        while iter.is_more() {
            assert!(iter.key().is_some());
            assert!(iter.value().is_some());
            iter.next();
            count += 1;
        }
        assert_eq!(count, 2);
    }
}

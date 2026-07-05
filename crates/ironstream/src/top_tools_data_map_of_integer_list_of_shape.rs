// FILE: top_tools_data_map_of_integer_list_of_shape.rs
// occt: TopTools_DataMapOfIntegerListOfShape

use std::collections::HashMap;

/// Shape: Simple shape.
#[derive(Clone, Debug)]
pub struct Shape {
    id: usize,
}

impl Shape {
    pub fn new(id: usize) -> Self {
        Shape { id }
    }
}

/// ListOfShape: List of shapes.
#[derive(Clone, Debug)]
pub struct ListOfShape {
    shapes: Vec<Shape>,
}

impl ListOfShape {
    pub fn new() -> Self {
        ListOfShape {
            shapes: Vec::new(),
        }
    }

    pub fn append(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }

    pub fn size(&self) -> usize {
        self.shapes.len()
    }
}

impl Default for ListOfShape {
    fn default() -> Self {
        Self::new()
    }
}

/// DataMapOfIntegerListOfShape: Maps integer to list of shapes.
#[derive(Clone, Debug)]
pub struct DataMapOfIntegerListOfShape {
    data: HashMap<i32, ListOfShape>,
}

impl DataMapOfIntegerListOfShape {
    pub fn new() -> Self {
        DataMapOfIntegerListOfShape {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, key: i32, list: ListOfShape) -> bool {
        self.data.insert(key, list).is_none()
    }

    pub fn contains(&self, key: i32) -> bool {
        self.data.contains_key(&key)
    }

    pub fn find(&self, key: i32) -> Option<&ListOfShape> {
        self.data.get(&key)
    }

    pub fn remove(&mut self, key: i32) -> bool {
        self.data.remove(&key).is_some()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for DataMapOfIntegerListOfShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_of_shape() {
        let mut list = ListOfShape::new();
        list.append(Shape::new(1));
        assert_eq!(list.size(), 1);
    }

    #[test]
    fn test_data_map_bind() {
        let mut map = DataMapOfIntegerListOfShape::new();
        let mut list = ListOfShape::new();
        list.append(Shape::new(10));
        assert!(map.bind(5, list));
        assert!(!map.bind(5, ListOfShape::new()));
    }

    #[test]
    fn test_data_map_find() {
        let mut map = DataMapOfIntegerListOfShape::new();
        let mut list = ListOfShape::new();
        list.append(Shape::new(20));
        map.bind(3, list);
        let found = map.find(3).unwrap();
        assert_eq!(found.size(), 1);
    }
}

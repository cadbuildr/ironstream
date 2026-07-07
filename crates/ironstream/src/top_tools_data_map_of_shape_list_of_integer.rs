// FILE: top_tools_data_map_of_shape_list_of_integer.rs
// occt: TopTools_DataMapOfShapeListOfInteger

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Shape {
    id: usize,
}

impl Shape {
    pub fn new(id: usize) -> Self {
        Shape { id }
    }
}

#[derive(Clone, Debug)]
pub struct ListOfInteger {
    items: Vec<i32>,
}

impl ListOfInteger {
    pub fn new() -> Self {
        ListOfInteger { items: Vec::new() }
    }

    pub fn append(&mut self, val: i32) {
        self.items.push(val);
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}

impl Default for ListOfInteger {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct DataMapOfShapeListOfInteger {
    data: HashMap<Shape, ListOfInteger>,
}

impl DataMapOfShapeListOfInteger {
    pub fn new() -> Self {
        DataMapOfShapeListOfInteger {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, shape: Shape, list: ListOfInteger) -> bool {
        self.data.insert(shape, list).is_none()
    }

    pub fn find(&self, shape: &Shape) -> Option<&ListOfInteger> {
        self.data.get(shape)
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl Default for DataMapOfShapeListOfInteger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let mut map = DataMapOfShapeListOfInteger::new();
        let mut list = ListOfInteger::new();
        list.append(42);
        map.bind(Shape::new(1), list);
        assert!(map.find(&Shape::new(1)).is_some());
    }
}

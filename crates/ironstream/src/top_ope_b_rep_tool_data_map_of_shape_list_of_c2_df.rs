// FILE: top_ope_b_rep_tool_data_map_of_shape_list_of_c2_df.rs
// occt: TopOpeBRepTool_DataMapOfShapeListOfC2DF
// occt-ref: TopOpeBRepTool_C2DF

use std::collections::HashMap;

/// C2DF: Curve 2D-Face data.
#[derive(Clone, Debug)]
pub struct C2DF {
    curve_id: usize,
    param: f64,
}

impl C2DF {
    pub fn new(curve_id: usize, param: f64) -> Self {
        C2DF { curve_id, param }
    }

    pub fn curve_id(&self) -> usize {
        self.curve_id
    }

    pub fn param(&self) -> f64 {
        self.param
    }
}

/// ListOfC2DF: List of C2DF objects.
#[derive(Clone, Debug)]
pub struct ListOfC2DF {
    items: Vec<C2DF>,
}

impl ListOfC2DF {
    pub fn new() -> Self {
        ListOfC2DF { items: Vec::new() }
    }

    pub fn append(&mut self, item: C2DF) {
        self.items.push(item);
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &C2DF> {
        self.items.iter()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Default for ListOfC2DF {
    fn default() -> Self {
        Self::new()
    }
}

/// ShapeKey: Shape identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShapeKey {
    id: usize,
}

impl ShapeKey {
    pub fn new(id: usize) -> Self {
        ShapeKey { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// DataMapOfShapeListOfC2DF: Maps shape to list of C2DF.
#[derive(Clone, Debug)]
pub struct DataMapOfShapeListOfC2DF {
    data: HashMap<ShapeKey, ListOfC2DF>,
}

impl DataMapOfShapeListOfC2DF {
    pub fn new() -> Self {
        DataMapOfShapeListOfC2DF {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, shape: ShapeKey, list: ListOfC2DF) -> bool {
        self.data.insert(shape, list).is_none()
    }

    pub fn contains(&self, shape: &ShapeKey) -> bool {
        self.data.contains_key(shape)
    }

    pub fn find(&self, shape: &ShapeKey) -> Option<&ListOfC2DF> {
        self.data.get(shape)
    }

    pub fn find_mut(&mut self, shape: &ShapeKey) -> Option<&mut ListOfC2DF> {
        self.data.get_mut(shape)
    }

    pub fn remove(&mut self, shape: &ShapeKey) -> bool {
        self.data.remove(shape).is_some()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ShapeKey, &ListOfC2DF)> {
        self.data.iter()
    }
}

impl Default for DataMapOfShapeListOfC2DF {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c2df() {
        let c2df = C2DF::new(10, 0.5);
        assert_eq!(c2df.curve_id(), 10);
    }

    #[test]
    fn test_list_of_c2df() {
        let mut list = ListOfC2DF::new();
        list.append(C2DF::new(1, 0.1));
        list.append(C2DF::new(2, 0.2));
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_data_map_bind() {
        let mut map = DataMapOfShapeListOfC2DF::new();
        let shape = ShapeKey::new(5);
        let mut list = ListOfC2DF::new();
        list.append(C2DF::new(50, 1.5));
        assert!(map.bind(shape.clone(), list));
        assert!(!map.bind(shape, ListOfC2DF::new()));
    }

    #[test]
    fn test_data_map_find() {
        let mut map = DataMapOfShapeListOfC2DF::new();
        let shape = ShapeKey::new(3);
        let mut list = ListOfC2DF::new();
        list.append(C2DF::new(30, 0.3));
        map.bind(shape.clone(), list);

        let found = map.find(&shape).unwrap();
        assert_eq!(found.size(), 1);
    }
}

// FILE: top_ope_b_rep_tool_data_map_of_oriented_shape_c2_df.rs
// occt: TopOpeBRepTool_DataMapOfOrientedShapeC2DF

use std::collections::HashMap;

/// ShapeKey: Oriented shape identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShapeKey {
    id: usize,
    oriented: bool,
}

impl ShapeKey {
    pub fn new(id: usize, oriented: bool) -> Self {
        ShapeKey { id, oriented }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn is_oriented(&self) -> bool {
        self.oriented
    }
}

/// C2DF: Curve and parameter pair.
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

/// DataMapOfOrientedShapeC2DF: Maps oriented shape to C2DF.
#[derive(Clone, Debug)]
pub struct DataMapOfOrientedShapeC2DF {
    data: HashMap<ShapeKey, C2DF>,
}

impl DataMapOfOrientedShapeC2DF {
    pub fn new() -> Self {
        DataMapOfOrientedShapeC2DF {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, shape: ShapeKey, c2df: C2DF) -> bool {
        self.data.insert(shape, c2df).is_none()
    }

    pub fn contains(&self, shape: &ShapeKey) -> bool {
        self.data.contains_key(shape)
    }

    pub fn find(&self, shape: &ShapeKey) -> Option<&C2DF> {
        self.data.get(shape)
    }

    pub fn find_mut(&mut self, shape: &ShapeKey) -> Option<&mut C2DF> {
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

    pub fn iter(&self) -> impl Iterator<Item = (&ShapeKey, &C2DF)> {
        self.data.iter()
    }
}

impl Default for DataMapOfOrientedShapeC2DF {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_key() {
        let key = ShapeKey::new(42, true);
        assert_eq!(key.id(), 42);
        assert!(key.is_oriented());
    }

    #[test]
    fn test_c2df() {
        let c2df = C2DF::new(10, 0.5);
        assert_eq!(c2df.curve_id(), 10);
        assert_eq!(c2df.param(), 0.5);
    }

    #[test]
    fn test_data_map_bind() {
        let mut map = DataMapOfOrientedShapeC2DF::new();
        let shape = ShapeKey::new(5, true);
        let c2df = C2DF::new(50, 1.5);
        assert!(map.bind(shape.clone(), c2df));
        assert!(!map.bind(shape, C2DF::new(51, 2.5)));
    }

    #[test]
    fn test_data_map_find() {
        let mut map = DataMapOfOrientedShapeC2DF::new();
        let shape = ShapeKey::new(3, false);
        let c2df = C2DF::new(30, 0.3);
        map.bind(shape.clone(), c2df);

        let found = map.find(&shape).unwrap();
        assert_eq!(found.curve_id(), 30);
        assert_eq!(found.param(), 0.3);
    }

    #[test]
    fn test_data_map_remove() {
        let mut map = DataMapOfOrientedShapeC2DF::new();
        let shape = ShapeKey::new(7, true);
        map.bind(shape.clone(), C2DF::new(70, 0.7));

        assert_eq!(map.size(), 1);
        assert!(map.remove(&shape));
        assert_eq!(map.size(), 0);
    }
}

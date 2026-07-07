// FILE: top_ope_b_rep_ds_double_map_of_integer_shape.rs
// occt: TopOpeBRepDS_DoubleMapOfIntegerShape

/// ShapeKey: Simplified shape.
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

/// DoubleMapOfIntegerShape: Bidirectional mapping between integers and shapes.
///
/// Unlike a regular DataMap, DoubleMap allows lookup in both directions.
#[derive(Clone, Debug)]
pub struct DoubleMapOfIntegerShape {
    int_to_shape: std::collections::HashMap<i32, ShapeKey>,
    shape_to_int: std::collections::HashMap<ShapeKey, i32>,
}

impl DoubleMapOfIntegerShape {
    pub fn new() -> Self {
        DoubleMapOfIntegerShape {
            int_to_shape: std::collections::HashMap::new(),
            shape_to_int: std::collections::HashMap::new(),
        }
    }

    /// Binds an integer to a shape (both directions).
    pub fn bind(&mut self, key: i32, shape: ShapeKey) -> bool {
        if self.int_to_shape.contains_key(&key) || self.shape_to_int.contains_key(&shape) {
            false
        } else {
            self.int_to_shape.insert(key, shape.clone());
            self.shape_to_int.insert(shape, key);
            true
        }
    }

    /// Returns the shape associated with an integer.
    pub fn find_from_int(&self, key: i32) -> Option<&ShapeKey> {
        self.int_to_shape.get(&key)
    }

    /// Returns the integer associated with a shape.
    pub fn find_from_shape(&self, shape: &ShapeKey) -> Option<i32> {
        self.shape_to_int.get(shape).copied()
    }

    /// Checks if an integer key exists.
    pub fn contains_int(&self, key: i32) -> bool {
        self.int_to_shape.contains_key(&key)
    }

    /// Checks if a shape key exists.
    pub fn contains_shape(&self, shape: &ShapeKey) -> bool {
        self.shape_to_int.contains_key(shape)
    }

    /// Removes a mapping by integer.
    pub fn remove_by_int(&mut self, key: i32) -> bool {
        if let Some(shape) = self.int_to_shape.remove(&key) {
            self.shape_to_int.remove(&shape);
            true
        } else {
            false
        }
    }

    /// Removes a mapping by shape.
    pub fn remove_by_shape(&mut self, shape: &ShapeKey) -> bool {
        if let Some(key) = self.shape_to_int.remove(shape) {
            self.int_to_shape.remove(&key);
            true
        } else {
            false
        }
    }

    pub fn size(&self) -> usize {
        self.int_to_shape.len()
    }

    pub fn clear(&mut self) {
        self.int_to_shape.clear();
        self.shape_to_int.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&i32, &ShapeKey)> {
        self.int_to_shape.iter()
    }
}

impl Default for DoubleMapOfIntegerShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_key() {
        let shape = ShapeKey::new(42);
        assert_eq!(shape.id(), 42);
    }

    #[test]
    fn test_double_map_bind() {
        let mut map = DoubleMapOfIntegerShape::new();
        let shape = ShapeKey::new(10);
        assert!(map.bind(5, shape));
    }

    #[test]
    fn test_double_map_duplicate_int() {
        let mut map = DoubleMapOfIntegerShape::new();
        let shape1 = ShapeKey::new(10);
        let shape2 = ShapeKey::new(20);
        map.bind(5, shape1);
        assert!(!map.bind(5, shape2)); // Key 5 already exists
    }

    #[test]
    fn test_double_map_duplicate_shape() {
        let mut map = DoubleMapOfIntegerShape::new();
        let shape = ShapeKey::new(10);
        map.bind(5, shape.clone());
        assert!(!map.bind(6, shape)); // Shape already mapped
    }

    #[test]
    fn test_double_map_find_from_int() {
        let mut map = DoubleMapOfIntegerShape::new();
        let shape = ShapeKey::new(10);
        map.bind(5, shape.clone());

        let found = map.find_from_int(5).unwrap();
        assert_eq!(found.id(), 10);
    }

    #[test]
    fn test_double_map_find_from_shape() {
        let mut map = DoubleMapOfIntegerShape::new();
        let shape = ShapeKey::new(10);
        map.bind(5, shape.clone());

        let found = map.find_from_shape(&shape).unwrap();
        assert_eq!(found, 5);
    }

    #[test]
    fn test_double_map_contains() {
        let mut map = DoubleMapOfIntegerShape::new();
        let shape = ShapeKey::new(10);
        map.bind(5, shape.clone());

        assert!(map.contains_int(5));
        assert!(map.contains_shape(&shape));
        assert!(!map.contains_int(6));
    }

    #[test]
    fn test_double_map_remove_by_int() {
        let mut map = DoubleMapOfIntegerShape::new();
        let shape = ShapeKey::new(10);
        map.bind(5, shape.clone());

        assert_eq!(map.size(), 1);
        assert!(map.remove_by_int(5));
        assert_eq!(map.size(), 0);
        assert!(!map.contains_shape(&shape));
    }

    #[test]
    fn test_double_map_remove_by_shape() {
        let mut map = DoubleMapOfIntegerShape::new();
        let shape = ShapeKey::new(10);
        map.bind(5, shape.clone());

        assert_eq!(map.size(), 1);
        assert!(map.remove_by_shape(&shape));
        assert_eq!(map.size(), 0);
        assert!(!map.contains_int(5));
    }

    #[test]
    fn test_double_map_clear() {
        let mut map = DoubleMapOfIntegerShape::new();
        map.bind(1, ShapeKey::new(10));
        map.bind(2, ShapeKey::new(20));
        map.clear();
        assert_eq!(map.size(), 0);
    }
}

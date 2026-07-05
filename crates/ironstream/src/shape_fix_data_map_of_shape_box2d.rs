// FILE: shape_fix_data_map_of_shape_box2d.rs
// occt: ShapeFix_DataMapOfShapeBox2d

use std::collections::BTreeMap;

pub struct ShapeFixDataMapOfShapeBox2d {
    data: BTreeMap<String, (f64, f64, f64, f64)>,
}

impl ShapeFixDataMapOfShapeBox2d {
    pub fn new() -> Self {
        ShapeFixDataMapOfShapeBox2d {
            data: BTreeMap::new(),
        }
    }

    pub fn bind(&mut self, key: String, value: (f64, f64, f64, f64)) {
        self.data.insert(key, value);
    }

    pub fn find(&self, key: &str) -> Option<(f64, f64, f64, f64)> {
        self.data.get(key).copied()
    }

    pub fn remove(&mut self, key: &str) -> Option<(f64, f64, f64, f64)> {
        self.data.remove(key)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for ShapeFixDataMapOfShapeBox2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut map = ShapeFixDataMapOfShapeBox2d::new();
        map.bind("box".to_string(), (0.0, 1.0, 2.0, 3.0));
        assert_eq!(map.find("box"), Some((0.0, 1.0, 2.0, 3.0)));
    }
}

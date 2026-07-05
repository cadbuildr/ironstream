// FILE: top_ope_b_rep_tool_indexed_data_map_of_shape_box.rs
// occt: TopOpeBRepTool_IndexedDataMapOfShapeBox

/// ShapeKey: Shape identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
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

/// Box: 3D bounding box.
#[derive(Clone, Debug)]
pub struct Box {
    xmin: f64,
    ymin: f64,
    zmin: f64,
    xmax: f64,
    ymax: f64,
    zmax: f64,
}

impl Box {
    pub fn new(xmin: f64, ymin: f64, zmin: f64, xmax: f64, ymax: f64, zmax: f64) -> Self {
        Box {
            xmin,
            ymin,
            zmin,
            xmax,
            ymax,
            zmax,
        }
    }

    pub fn bounds(&self) -> (f64, f64, f64, f64, f64, f64) {
        (self.xmin, self.ymin, self.zmin, self.xmax, self.ymax, self.zmax)
    }
}

/// IndexedDataMapOfShapeBox: 1-based indexed map from Shape to Box.
#[derive(Clone, Debug)]
pub struct IndexedDataMapOfShapeBox {
    entries: Vec<(ShapeKey, Box)>,
}

impl IndexedDataMapOfShapeBox {
    pub fn new() -> Self {
        IndexedDataMapOfShapeBox {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, shape: ShapeKey, box_val: Box) -> usize {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == &shape) {
            self.entries[pos] = (shape, box_val);
            pos + 1
        } else {
            self.entries.push((shape, box_val));
            self.entries.len()
        }
    }

    pub fn bind(&mut self, shape: ShapeKey, box_val: Box) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|(k, _)| k == &shape) {
            entry.1 = box_val;
            false
        } else {
            self.entries.push((shape, box_val));
            true
        }
    }

    pub fn contains(&self, shape: &ShapeKey) -> bool {
        self.entries.iter().any(|(k, _)| k == shape)
    }

    pub fn find(&self, shape: &ShapeKey) -> Option<&Box> {
        self.entries.iter().find(|(k, _)| k == shape).map(|(_, v)| v)
    }

    pub fn value_at(&self, index_1based: usize) -> Option<&Box> {
        if index_1based == 0 {
            None
        } else {
            self.entries.get(index_1based - 1).map(|(_, v)| v)
        }
    }

    pub fn remove(&mut self, shape: &ShapeKey) -> bool {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == shape) {
            self.entries.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }

    pub fn lower(&self) -> usize {
        1
    }

    pub fn upper(&self) -> usize {
        self.entries.len()
    }
}

impl Default for IndexedDataMapOfShapeBox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_new() {
        let b = Box::new(0.0, 0.0, 0.0, 10.0, 10.0, 10.0);
        assert_eq!(b.bounds(), (0.0, 0.0, 0.0, 10.0, 10.0, 10.0));
    }

    #[test]
    fn test_indexed_map_add() {
        let mut map = IndexedDataMapOfShapeBox::new();
        let shape = ShapeKey::new(1);
        let b = Box::new(0.0, 0.0, 0.0, 5.0, 5.0, 5.0);
        let idx = map.add(shape, b);
        assert_eq!(idx, 1);
    }

    #[test]
    fn test_indexed_map_bind() {
        let mut map = IndexedDataMapOfShapeBox::new();
        let shape = ShapeKey::new(5);
        let b = Box::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        assert!(map.bind(shape.clone(), b));
        assert!(!map.bind(shape, Box::new(1.0, 1.0, 1.0, 2.0, 2.0, 2.0)));
    }

    #[test]
    fn test_indexed_map_find() {
        let mut map = IndexedDataMapOfShapeBox::new();
        let shape = ShapeKey::new(3);
        let b = Box::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        map.bind(shape.clone(), b);

        let found = map.find(&shape).unwrap();
        assert_eq!(found.bounds(), (1.0, 2.0, 3.0, 4.0, 5.0, 6.0));
    }

    #[test]
    fn test_indexed_map_value_at() {
        let mut map = IndexedDataMapOfShapeBox::new();
        map.add(ShapeKey::new(1), Box::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0));

        assert!(map.value_at(0).is_none());
        assert!(map.value_at(1).is_some());
    }
}

// FILE: top_ope_b_rep_tool_indexed_data_map_of_shape_box2d.rs
// occt: TopOpeBRepTool_IndexedDataMapOfShapeBox2d

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

/// Box2d: 2D bounding box.
#[derive(Clone, Debug)]
pub struct Box2d {
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
}

impl Box2d {
    pub fn new(xmin: f64, ymin: f64, xmax: f64, ymax: f64) -> Self {
        Box2d { xmin, ymin, xmax, ymax }
    }

    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        (self.xmin, self.ymin, self.xmax, self.ymax)
    }
}

/// IndexedDataMapOfShapeBox2d: 1-based indexed map from Shape to Box2d.
#[derive(Clone, Debug)]
pub struct IndexedDataMapOfShapeBox2d {
    entries: Vec<(ShapeKey, Box2d)>,
}

impl IndexedDataMapOfShapeBox2d {
    pub fn new() -> Self {
        IndexedDataMapOfShapeBox2d {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, shape: ShapeKey, box_val: Box2d) -> usize {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == &shape) {
            self.entries[pos] = (shape, box_val);
            pos + 1
        } else {
            self.entries.push((shape, box_val));
            self.entries.len()
        }
    }

    pub fn bind(&mut self, shape: ShapeKey, box_val: Box2d) -> bool {
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

    pub fn find(&self, shape: &ShapeKey) -> Option<&Box2d> {
        self.entries.iter().find(|(k, _)| k == shape).map(|(_, v)| v)
    }

    pub fn value_at(&self, index_1based: usize) -> Option<&Box2d> {
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

impl Default for IndexedDataMapOfShapeBox2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box2d_new() {
        let b = Box2d::new(0.0, 0.0, 10.0, 10.0);
        assert_eq!(b.bounds(), (0.0, 0.0, 10.0, 10.0));
    }

    #[test]
    fn test_indexed_map_add() {
        let mut map = IndexedDataMapOfShapeBox2d::new();
        let shape = ShapeKey::new(1);
        let b = Box2d::new(0.0, 0.0, 5.0, 5.0);
        let idx = map.add(shape, b);
        assert_eq!(idx, 1);
    }

    #[test]
    fn test_indexed_map_bind() {
        let mut map = IndexedDataMapOfShapeBox2d::new();
        let shape = ShapeKey::new(5);
        let b = Box2d::new(0.0, 0.0, 1.0, 1.0);
        assert!(map.bind(shape.clone(), b));
        assert!(!map.bind(shape, Box2d::new(1.0, 1.0, 2.0, 2.0)));
    }

    #[test]
    fn test_indexed_map_find() {
        let mut map = IndexedDataMapOfShapeBox2d::new();
        let shape = ShapeKey::new(3);
        let b = Box2d::new(1.0, 2.0, 3.0, 4.0);
        map.bind(shape.clone(), b);

        let found = map.find(&shape).unwrap();
        assert_eq!(found.bounds(), (1.0, 2.0, 3.0, 4.0));
    }
}

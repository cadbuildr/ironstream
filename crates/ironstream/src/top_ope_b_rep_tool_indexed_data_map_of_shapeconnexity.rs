// FILE: top_ope_b_rep_tool_indexed_data_map_of_shapeconnexity.rs
// occt: TopOpeBRepTool_IndexedDataMapOfShapeconnexity
// occt-ref: TopOpeBRepTool_Connexity

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

/// Connexity: Connectivity data for a shape.
#[derive(Clone, Debug)]
pub struct Connexity {
    degree: i32,
    connected_shapes: Vec<usize>,
}

impl Connexity {
    pub fn new() -> Self {
        Connexity {
            degree: 0,
            connected_shapes: Vec::new(),
        }
    }

    pub fn with_degree(degree: i32) -> Self {
        Connexity {
            degree,
            connected_shapes: Vec::new(),
        }
    }

    pub fn degree(&self) -> i32 {
        self.degree
    }

    pub fn set_degree(&mut self, degree: i32) {
        self.degree = degree;
    }

    pub fn add_connected_shape(&mut self, shape_id: usize) {
        self.connected_shapes.push(shape_id);
    }

    pub fn connected_shapes(&self) -> &[usize] {
        &self.connected_shapes
    }

    pub fn clear(&mut self) {
        self.connected_shapes.clear();
    }
}

impl Default for Connexity {
    fn default() -> Self {
        Self::new()
    }
}

/// IndexedDataMapOfShapeconnexity: 1-based indexed map.
#[derive(Clone, Debug)]
pub struct IndexedDataMapOfShapeconnexity {
    entries: Vec<(ShapeKey, Connexity)>,
}

impl IndexedDataMapOfShapeconnexity {
    pub fn new() -> Self {
        IndexedDataMapOfShapeconnexity {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, shape: ShapeKey, connexity: Connexity) -> usize {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == &shape) {
            self.entries[pos] = (shape, connexity);
            pos + 1
        } else {
            self.entries.push((shape, connexity));
            self.entries.len()
        }
    }

    pub fn bind(&mut self, shape: ShapeKey, connexity: Connexity) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|(k, _)| k == &shape) {
            entry.1 = connexity;
            false
        } else {
            self.entries.push((shape, connexity));
            true
        }
    }

    pub fn contains(&self, shape: &ShapeKey) -> bool {
        self.entries.iter().any(|(k, _)| k == shape)
    }

    pub fn find(&self, shape: &ShapeKey) -> Option<&Connexity> {
        self.entries.iter().find(|(k, _)| k == shape).map(|(_, v)| v)
    }

    pub fn find_mut(&mut self, shape: &ShapeKey) -> Option<&mut Connexity> {
        self.entries
            .iter_mut()
            .find(|(k, _)| k == shape)
            .map(|(_, v)| v)
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

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn lower(&self) -> usize {
        1
    }

    pub fn upper(&self) -> usize {
        self.entries.len()
    }
}

impl Default for IndexedDataMapOfShapeconnexity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connexity_new() {
        let conn = Connexity::new();
        assert_eq!(conn.degree(), 0);
    }

    #[test]
    fn test_connexity_with_degree() {
        let conn = Connexity::with_degree(5);
        assert_eq!(conn.degree(), 5);
    }

    #[test]
    fn test_connexity_add_connected_shape() {
        let mut conn = Connexity::new();
        conn.add_connected_shape(1);
        conn.add_connected_shape(2);
        assert_eq!(conn.connected_shapes().len(), 2);
    }

    #[test]
    fn test_indexed_map_add() {
        let mut map = IndexedDataMapOfShapeconnexity::new();
        let shape = ShapeKey::new(1);
        let conn = Connexity::with_degree(3);
        let idx = map.add(shape, conn);
        assert_eq!(idx, 1);
    }

    #[test]
    fn test_indexed_map_find() {
        let mut map = IndexedDataMapOfShapeconnexity::new();
        let shape = ShapeKey::new(3);
        let mut conn = Connexity::with_degree(2);
        conn.add_connected_shape(5);
        map.bind(shape.clone(), conn);

        let found = map.find(&shape).unwrap();
        assert_eq!(found.degree(), 2);
        assert_eq!(found.connected_shapes().len(), 1);
    }
}

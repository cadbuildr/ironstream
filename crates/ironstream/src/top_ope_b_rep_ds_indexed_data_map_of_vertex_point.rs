// FILE: top_ope_b_rep_ds_indexed_data_map_of_vertex_point.rs
// occt: TopOpeBRepDS_IndexedDataMapOfVertexPoint, TopOpeBRepDS_Point, TopOpeBRepDS_Vertex

/// Point: 3D point representation.
#[derive(Clone, Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    pub fn coordinates(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn set_coordinates(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

impl Default for Point {
    fn default() -> Self {
        Point::new(0.0, 0.0, 0.0)
    }
}

/// Vertex: Vertex identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vertex {
    id: usize,
}

impl Vertex {
    pub fn new(id: usize) -> Self {
        Vertex { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// IndexedDataMapOfVertexPoint: 1-based indexed map from Vertex to Point.
#[derive(Clone, Debug)]
pub struct IndexedDataMapOfVertexPoint {
    entries: Vec<(Vertex, Point)>,
}

impl IndexedDataMapOfVertexPoint {
    pub fn new() -> Self {
        IndexedDataMapOfVertexPoint {
            entries: Vec::new(),
        }
    }

    /// Adds or updates an entry, returns 1-based index.
    pub fn add(&mut self, vertex: Vertex, point: Point) -> usize {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == &vertex) {
            self.entries[pos] = (vertex, point);
            pos + 1
        } else {
            self.entries.push((vertex, point));
            self.entries.len()
        }
    }

    pub fn bind(&mut self, vertex: Vertex, point: Point) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|(k, _)| k == &vertex) {
            entry.1 = point;
            false
        } else {
            self.entries.push((vertex, point));
            true
        }
    }

    pub fn contains(&self, vertex: &Vertex) -> bool {
        self.entries.iter().any(|(k, _)| k == vertex)
    }

    pub fn find(&self, vertex: &Vertex) -> Option<&Point> {
        self.entries.iter().find(|(k, _)| k == vertex).map(|(_, v)| v)
    }

    pub fn find_mut(&mut self, vertex: &Vertex) -> Option<&mut Point> {
        self.entries
            .iter_mut()
            .find(|(k, _)| k == vertex)
            .map(|(_, v)| v)
    }

    pub fn value_at(&self, index_1based: usize) -> Option<&Point> {
        if index_1based == 0 {
            None
        } else {
            self.entries.get(index_1based - 1).map(|(_, v)| v)
        }
    }

    pub fn vertex_at(&self, index_1based: usize) -> Option<&Vertex> {
        if index_1based == 0 {
            None
        } else {
            self.entries.get(index_1based - 1).map(|(k, _)| k)
        }
    }

    pub fn value_at_mut(&mut self, index_1based: usize) -> Option<&mut Point> {
        if index_1based == 0 {
            None
        } else {
            self.entries.get_mut(index_1based - 1).map(|(_, v)| v)
        }
    }

    pub fn remove(&mut self, vertex: &Vertex) -> bool {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == vertex) {
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

    pub fn iter(&self) -> impl Iterator<Item = (&Vertex, &Point)> {
        self.entries.iter().map(|(k, v)| (k, v))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Vertex, &mut Point)> {
        self.entries.iter_mut().map(|(k, v)| (&*k, v))
    }
}

impl Default for IndexedDataMapOfVertexPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_new() {
        let pt = Point::new(1.5, 2.5, 3.5);
        assert_eq!(pt.coordinates(), (1.5, 2.5, 3.5));
        assert_eq!(pt.x(), 1.5);
        assert_eq!(pt.y(), 2.5);
        assert_eq!(pt.z(), 3.5);
    }

    #[test]
    fn test_point_default() {
        let pt = Point::default();
        assert_eq!(pt.coordinates(), (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_point_set_coordinates() {
        let mut pt = Point::new(0.0, 0.0, 0.0);
        pt.set_coordinates(10.0, 20.0, 30.0);
        assert_eq!(pt.coordinates(), (10.0, 20.0, 30.0));
    }

    #[test]
    fn test_vertex_new() {
        let vertex = Vertex::new(42);
        assert_eq!(vertex.id(), 42);
    }

    #[test]
    fn test_indexed_map_add() {
        let mut map = IndexedDataMapOfVertexPoint::new();
        let vertex1 = Vertex::new(1);
        let point1 = Point::new(1.0, 2.0, 3.0);
        let idx1 = map.add(vertex1, point1);
        assert_eq!(idx1, 1);

        let vertex2 = Vertex::new(2);
        let point2 = Point::new(4.0, 5.0, 6.0);
        let idx2 = map.add(vertex2, point2);
        assert_eq!(idx2, 2);
    }

    #[test]
    fn test_indexed_map_bind() {
        let mut map = IndexedDataMapOfVertexPoint::new();
        let vertex = Vertex::new(5);
        let point = Point::new(1.0, 2.0, 3.0);

        assert!(map.bind(vertex.clone(), point));
        assert!(!map.bind(vertex, Point::new(4.0, 5.0, 6.0)));
    }

    #[test]
    fn test_indexed_map_contains() {
        let mut map = IndexedDataMapOfVertexPoint::new();
        let vertex = Vertex::new(5);
        assert!(!map.contains(&vertex));

        map.bind(vertex.clone(), Point::new(0.0, 0.0, 0.0));
        assert!(map.contains(&vertex));
    }

    #[test]
    fn test_indexed_map_find() {
        let mut map = IndexedDataMapOfVertexPoint::new();
        let vertex = Vertex::new(3);
        let point = Point::new(10.0, 20.0, 30.0);
        map.bind(vertex.clone(), point);

        let found = map.find(&vertex).unwrap();
        assert_eq!(found.coordinates(), (10.0, 20.0, 30.0));
    }

    #[test]
    fn test_indexed_map_value_at() {
        let mut map = IndexedDataMapOfVertexPoint::new();
        let vertex1 = Vertex::new(1);
        let vertex2 = Vertex::new(2);
        map.add(vertex1, Point::new(1.0, 1.0, 1.0));
        map.add(vertex2, Point::new(2.0, 2.0, 2.0));

        assert!(map.value_at(0).is_none());
        assert_eq!(map.value_at(1).unwrap().coordinates(), (1.0, 1.0, 1.0));
        assert_eq!(map.value_at(2).unwrap().coordinates(), (2.0, 2.0, 2.0));
    }

    #[test]
    fn test_indexed_map_remove() {
        let mut map = IndexedDataMapOfVertexPoint::new();
        let vertex = Vertex::new(7);
        map.bind(vertex.clone(), Point::new(0.0, 0.0, 0.0));

        assert_eq!(map.size(), 1);
        assert!(map.remove(&vertex));
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_indexed_map_bounds() {
        let mut map = IndexedDataMapOfVertexPoint::new();
        map.add(Vertex::new(1), Point::new(0.0, 0.0, 0.0));
        map.add(Vertex::new(2), Point::new(0.0, 0.0, 0.0));

        assert_eq!(map.lower(), 1);
        assert_eq!(map.upper(), 2);
    }
}

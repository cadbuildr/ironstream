// FILE: topods_vertex.rs
// occt: TopoDS_Vertex, TopExp_Vertex, TopoDS_Builder for vertices,
//       TopoDsBuilder make_vertex

/// A topological vertex (0-dimensional shape).
/// occt: TopoDS_Vertex
#[derive(Clone, Debug)]
pub struct TopoDsVertex {
    pub shape_id: u32,
    pub location_id: u32,
    pub orientation: u8,  // 0=Forward 1=Reversed 2=Internal 3=External
    pub point: [f64; 3],
    pub tolerance: f64,
}

impl TopoDsVertex {
    pub fn new(shape_id: u32, point: [f64; 3]) -> Self {
        Self {
            shape_id,
            location_id: 0,
            orientation: 0,
            point,
            tolerance: 1e-7,
        }
    }

    pub fn set_location(&mut self, loc_id: u32) { self.location_id = loc_id; }
    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t.max(1e-15); }
    pub fn set_orientation(&mut self, o: u8) { self.orientation = o.min(3); }

    pub fn x(&self) -> f64 { self.point[0] }
    pub fn y(&self) -> f64 { self.point[1] }
    pub fn z(&self) -> f64 { self.point[2] }

    pub fn is_forward(&self) -> bool { self.orientation == 0 }
    pub fn is_reversed(&self) -> bool { self.orientation == 1 }

    pub fn distance_to(&self, other: &TopoDsVertex) -> f64 {
        let dx = self.point[0] - other.point[0];
        let dy = self.point[1] - other.point[1];
        let dz = self.point[2] - other.point[2];
        (dx*dx + dy*dy + dz*dz).sqrt()
    }

    pub fn same_point(&self, other: &TopoDsVertex) -> bool {
        self.distance_to(other) <= (self.tolerance + other.tolerance)
    }

    pub fn equals(&self, other: &TopoDsVertex) -> bool {
        self.shape_id == other.shape_id && self.same_point(other)
    }
}

/// Vertex builder: creates and manages vertices.
/// occt: TopoDS_Builder / BRepBuilder make_vertex functions
#[derive(Clone, Debug, Default)]
pub struct TopoDsVertexBuilder {
    pub vertices: Vec<TopoDsVertex>,
    pub next_id: u32,
}

impl TopoDsVertexBuilder {
    pub fn new() -> Self { Self { next_id: 1, ..Self::default() } }

    pub fn make_vertex(&mut self, point: [f64; 3]) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.vertices.push(TopoDsVertex::new(id, point));
        id
    }

    pub fn make_vertex_with_tolerance(&mut self, point: [f64; 3], tol: f64) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let mut v = TopoDsVertex::new(id, point);
        v.set_tolerance(tol);
        self.vertices.push(v);
        id
    }

    pub fn find_vertex(&self, id: u32) -> Option<&TopoDsVertex> {
        self.vertices.iter().find(|v| v.shape_id == id)
    }

    pub fn find_vertex_mut(&mut self, id: u32) -> Option<&mut TopoDsVertex> {
        self.vertices.iter_mut().find(|v| v.shape_id == id)
    }

    pub fn find_or_create(&mut self, point: [f64; 3], tol: f64) -> u32 {
        for v in &self.vertices {
            let dx = v.point[0] - point[0];
            let dy = v.point[1] - point[1];
            let dz = v.point[2] - point[2];
            let dist = (dx*dx + dy*dy + dz*dz).sqrt();
            if dist <= (v.tolerance + tol) { return v.shape_id; }
        }
        self.make_vertex_with_tolerance(point, tol)
    }

    pub fn nb_vertices(&self) -> usize { self.vertices.len() }
}

/// Vertex iterator: traverse all vertices.
/// occt: TopExp_Vertex (simplified)
#[derive(Clone, Debug)]
pub struct TopoDsVertexIterator {
    vertices: Vec<u32>,
    index: usize,
}

impl TopoDsVertexIterator {
    pub fn new(vertex_ids: Vec<u32>) -> Self {
        Self { vertices: vertex_ids, index: 0 }
    }

    pub fn more(&self) -> bool { self.index < self.vertices.len() }

    pub fn next(&mut self) -> Option<u32> {
        if self.more() {
            let id = self.vertices[self.index];
            self.index += 1;
            Some(id)
        } else {
            None
        }
    }

    pub fn reset(&mut self) { self.index = 0; }
    pub fn nb_vertices(&self) -> usize { self.vertices.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertex_creation() {
        let v = TopoDsVertex::new(1, [1.0, 2.0, 3.0]);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
        assert!(v.is_forward());
    }

    #[test]
    fn vertex_distance() {
        let v1 = TopoDsVertex::new(1, [0.0, 0.0, 0.0]);
        let v2 = TopoDsVertex::new(2, [3.0, 4.0, 0.0]);
        assert!((v1.distance_to(&v2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn vertex_same_point() {
        let mut v1 = TopoDsVertex::new(1, [0.0, 0.0, 0.0]);
        let mut v2 = TopoDsVertex::new(2, [1e-8, 1e-8, 1e-8]);
        v1.set_tolerance(1e-7);
        v2.set_tolerance(1e-7);
        assert!(v1.same_point(&v2));
    }

    #[test]
    fn vertex_builder() {
        let mut b = TopoDsVertexBuilder::new();
        let id1 = b.make_vertex([0.0, 0.0, 0.0]);
        let id2 = b.make_vertex([1.0, 1.0, 1.0]);
        assert_ne!(id1, id2);
        assert_eq!(b.nb_vertices(), 2);
        assert!(b.find_vertex(id1).is_some());
    }

    #[test]
    fn vertex_builder_find_or_create() {
        let mut b = TopoDsVertexBuilder::new();
        let id1 = b.make_vertex_with_tolerance([0.0, 0.0, 0.0], 1e-7);
        let id2 = b.find_or_create([1e-8, 1e-8, 1e-8], 1e-7);
        assert_eq!(id1, id2); // should find the existing vertex
        assert_eq!(b.nb_vertices(), 1);
    }

    #[test]
    fn vertex_iterator() {
        let ids = vec![1, 2, 3];
        let mut it = TopoDsVertexIterator::new(ids);
        assert_eq!(it.nb_vertices(), 3);
        let mut collected = Vec::new();
        while it.more() { collected.push(it.next().unwrap()); }
        assert_eq!(collected, vec![1, 2, 3]);
    }
}

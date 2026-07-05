// FILE: b_rep_tools_map_of_vertex_pnt2d.rs
// occt: BRepTools_MapOfVertexPnt2d

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

pub struct BreptoolsMapOfVertexPnt2d {
    data: HashMap<usize, Point2D>,
}

impl BreptoolsMapOfVertexPnt2d {
    pub fn new() -> Self {
        BreptoolsMapOfVertexPnt2d {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, vertex_id: usize, point: Point2D) {
        self.data.insert(vertex_id, point);
    }

    pub fn get(&self, vertex_id: usize) -> Option<Point2D> {
        self.data.get(&vertex_id).copied()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn remove(&mut self, vertex_id: usize) -> Option<Point2D> {
        self.data.remove(&vertex_id)
    }
}

impl Default for BreptoolsMapOfVertexPnt2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let point = Point2D::new(0.5, 1.5);
        assert_eq!(point.x(), 0.5);
        assert_eq!(point.y(), 1.5);
    }

    #[test]
    fn test_map_add_get() {
        let mut map = BreptoolsMapOfVertexPnt2d::new();
        let point = Point2D::new(0.1, 0.2);
        map.add(1, point);
        assert_eq!(map.get(1).unwrap().x(), 0.1);
    }
}

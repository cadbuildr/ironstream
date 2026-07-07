// FILE: bopds_vector_of_point.rs
// occt: BOPDS_VectorOfPoint

use std::collections::VecDeque;

/// Represents a 3D/2D point for intersection data in Boolean operations.
/// Mirrors BOPDS_Point from OCCT.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    pnt: [f64; 3],       // 3D point coordinates
    pnt2d_1: [f64; 2],   // 2D point on first face
    pnt2d_2: [f64; 2],   // 2D point on second face
    index: i32,          // Vertex index
}

impl Point {
    /// Creates a new Point with default sentinel values.
    fn new() -> Self {
        Point {
            pnt: [99.0, 99.0, 99.0],
            pnt2d_1: [99.0, 99.0],
            pnt2d_2: [99.0, 99.0],
            index: -1,
        }
    }

    /// Sets the 3D point.
    fn set_pnt(&mut self, pnt: [f64; 3]) {
        self.pnt = pnt;
    }

    /// Returns the 3D point.
    fn pnt(&self) -> [f64; 3] {
        self.pnt
    }

    /// Sets the 2D point on the first face.
    fn set_pnt2d_1(&mut self, pnt: [f64; 2]) {
        self.pnt2d_1 = pnt;
    }

    /// Returns the 2D point on the first face.
    fn pnt2d_1(&self) -> [f64; 2] {
        self.pnt2d_1
    }

    /// Sets the 2D point on the second face.
    fn set_pnt2d_2(&mut self, pnt: [f64; 2]) {
        self.pnt2d_2 = pnt;
    }

    /// Returns the 2D point on the second face.
    fn pnt2d_2(&self) -> [f64; 2] {
        self.pnt2d_2
    }

    /// Sets the vertex index.
    fn set_index(&mut self, index: i32) {
        self.index = index;
    }

    /// Returns the vertex index.
    fn index(&self) -> i32 {
        self.index
    }
}

/// Deprecated type alias: vector of points using a dynamic array.
/// This is a newtype wrapping VecDeque<Point> to match OCCT's NCollection_DynamicArray semantics.
pub struct BopdsVectorOfPoint {
    data: VecDeque<Point>,
}

impl BopdsVectorOfPoint {
    /// Creates an empty vector.
    pub fn new() -> Self {
        BopdsVectorOfPoint {
            data: VecDeque::new(),
        }
    }

    /// Appends a point to the vector.
    pub fn push(&mut self, point: Point) {
        self.data.push_back(point);
    }

    /// Returns the number of points.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the vector is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Accesses a point by index.
    pub fn get(&self, index: usize) -> Option<&Point> {
        self.data.get(index)
    }

    /// Mutably accesses a point by index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Point> {
        self.data.get_mut(index)
    }

    /// Clears all points.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for BopdsVectorOfPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_default() {
        let p = Point::new();
        assert_eq!(p.pnt(), [99.0, 99.0, 99.0]);
        assert_eq!(p.pnt2d_1(), [99.0, 99.0]);
        assert_eq!(p.pnt2d_2(), [99.0, 99.0]);
        assert_eq!(p.index(), -1);
    }

    #[test]
    fn test_point_setters() {
        let mut p = Point::new();
        p.set_pnt([1.0, 2.0, 3.0]);
        p.set_pnt2d_1([4.0, 5.0]);
        p.set_pnt2d_2([6.0, 7.0]);
        p.set_index(42);

        assert_eq!(p.pnt(), [1.0, 2.0, 3.0]);
        assert_eq!(p.pnt2d_1(), [4.0, 5.0]);
        assert_eq!(p.pnt2d_2(), [6.0, 7.0]);
        assert_eq!(p.index(), 42);
    }

    #[test]
    fn test_vector_basic() {
        let mut vec = BopdsVectorOfPoint::new();
        assert!(vec.is_empty());
        assert_eq!(vec.len(), 0);

        let mut p1 = Point::new();
        p1.set_index(1);
        vec.push(p1);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.get(0).unwrap().index(), 1);

        let mut p2 = Point::new();
        p2.set_index(2);
        vec.push(p2);
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn test_vector_mutate() {
        let mut vec = BopdsVectorOfPoint::new();
        let mut p = Point::new();
        p.set_pnt([0.0, 0.0, 0.0]);
        vec.push(p);

        if let Some(pt) = vec.get_mut(0) {
            pt.set_index(99);
        }
        assert_eq!(vec.get(0).unwrap().index(), 99);
    }

    #[test]
    fn test_vector_clear() {
        let mut vec = BopdsVectorOfPoint::new();
        let p1 = Point::new();
        let p2 = Point::new();
        vec.push(p1);
        vec.push(p2);
        assert_eq!(vec.len(), 2);

        vec.clear();
        assert!(vec.is_empty());
    }
}

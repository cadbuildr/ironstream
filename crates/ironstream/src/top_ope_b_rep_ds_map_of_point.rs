// FILE: top_ope_b_rep_ds_map_of_point.rs
// occt: TopOpeBRepDS_MapOfPoint

use std::collections::HashSet;

/// Point: 3D point in a set.
#[derive(Clone, Debug, PartialEq)]
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
}

impl Eq for Point {}

impl std::hash::Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
        self.z.to_bits().hash(state);
    }
}

/// MapOfPoint: Set-like container for points.
#[derive(Clone, Debug)]
pub struct MapOfPoint {
    data: HashSet<Point>,
}

impl MapOfPoint {
    pub fn new() -> Self {
        MapOfPoint {
            data: HashSet::new(),
        }
    }

    pub fn add(&mut self, point: Point) -> bool {
        self.data.insert(point)
    }

    pub fn remove(&mut self, point: &Point) -> bool {
        self.data.remove(point)
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.data.contains(point)
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &Point> {
        self.data.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for MapOfPoint {
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
    }

    #[test]
    fn test_map_add() {
        let mut map = MapOfPoint::new();
        let pt = Point::new(1.0, 2.0, 3.0);
        assert!(map.add(pt.clone()));
        assert!(!map.add(pt));
    }

    #[test]
    fn test_map_contains() {
        let mut map = MapOfPoint::new();
        let pt = Point::new(5.0, 5.0, 5.0);
        assert!(!map.contains(&pt));
        map.add(pt.clone());
        assert!(map.contains(&pt));
    }

    #[test]
    fn test_map_remove() {
        let mut map = MapOfPoint::new();
        let pt = Point::new(1.0, 2.0, 3.0);
        map.add(pt.clone());
        assert_eq!(map.size(), 1);
        assert!(map.remove(&pt));
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_map_clear() {
        let mut map = MapOfPoint::new();
        map.add(Point::new(1.0, 1.0, 1.0));
        map.add(Point::new(2.0, 2.0, 2.0));
        map.clear();
        assert_eq!(map.size(), 0);
    }
}

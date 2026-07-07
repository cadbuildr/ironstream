// FILE: b_rep_offset_data_map_of_shape_list_of_interval.rs
// occt: BRepOffset_DataMapOfShapeListOfInterval

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    first: f64,
    second: f64,
}

impl Interval {
    pub fn new(first: f64, second: f64) -> Self {
        Interval { first, second }
    }

    pub fn first(&self) -> f64 {
        self.first
    }

    pub fn second(&self) -> f64 {
        self.second
    }
}

pub struct BrepoffsetDataMapOfShapeListOfInterval {
    data: HashMap<usize, Vec<Interval>>,
}

impl BrepoffsetDataMapOfShapeListOfInterval {
    pub fn new() -> Self {
        BrepoffsetDataMapOfShapeListOfInterval {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, shape_id: usize, interval: Interval) {
        self.data.entry(shape_id).or_insert_with(Vec::new).push(interval);
    }

    pub fn get(&self, shape_id: usize) -> Option<&[Interval]> {
        self.data.get(&shape_id).map(|v| v.as_slice())
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

    pub fn remove(&mut self, shape_id: usize) -> Option<Vec<Interval>> {
        self.data.remove(&shape_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &Vec<Interval>)> {
        self.data.iter().map(|(k, v)| (*k, v))
    }
}

impl Default for BrepoffsetDataMapOfShapeListOfInterval {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_creation() {
        let interval = Interval::new(0.0, 1.0);
        assert_eq!(interval.first(), 0.0);
        assert_eq!(interval.second(), 1.0);
    }

    #[test]
    fn test_map_add() {
        let mut map = BrepoffsetDataMapOfShapeListOfInterval::new();
        map.add(1, Interval::new(0.0, 1.0));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_map_get() {
        let mut map = BrepoffsetDataMapOfShapeListOfInterval::new();
        map.add(1, Interval::new(0.0, 1.0));
        map.add(1, Interval::new(2.0, 3.0));
        let intervals = map.get(1).unwrap();
        assert_eq!(intervals.len(), 2);
    }

    #[test]
    fn test_map_clear() {
        let mut map = BrepoffsetDataMapOfShapeListOfInterval::new();
        map.add(1, Interval::new(0.0, 1.0));
        map.clear();
        assert!(map.is_empty());
    }
}

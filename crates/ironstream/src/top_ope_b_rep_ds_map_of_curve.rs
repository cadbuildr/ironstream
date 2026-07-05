// FILE: top_ope_b_rep_ds_map_of_curve.rs
// occt: TopOpeBRepDS_MapOfCurve

use std::collections::HashSet;

/// Curve: Simplified curve representation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Curve {
    id: usize,
}

impl Curve {
    pub fn new(id: usize) -> Self {
        Curve { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// MapOfCurve: Set-like container for curves.
#[derive(Clone, Debug)]
pub struct MapOfCurve {
    data: HashSet<Curve>,
}

impl MapOfCurve {
    pub fn new() -> Self {
        MapOfCurve {
            data: HashSet::new(),
        }
    }

    pub fn add(&mut self, curve: Curve) -> bool {
        self.data.insert(curve)
    }

    pub fn remove(&mut self, curve: &Curve) -> bool {
        self.data.remove(curve)
    }

    pub fn contains(&self, curve: &Curve) -> bool {
        self.data.contains(curve)
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &Curve> {
        self.data.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for MapOfCurve {
    fn default() -> Self {
        Self::new()
    }
}

/// MapIterator: Iterator for MapOfCurve.
pub struct MapIterator {
    curves: Vec<Curve>,
    index: usize,
}

impl MapIterator {
    pub fn new(map: &MapOfCurve) -> Self {
        MapIterator {
            curves: map.data.iter().cloned().collect(),
            index: 0,
        }
    }

    pub fn is_more(&self) -> bool {
        self.index < self.curves.len()
    }

    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn current(&self) -> Option<&Curve> {
        self.curves.get(self.index)
    }

    pub fn value(&self) -> Option<&Curve> {
        self.current()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_new() {
        let curve = Curve::new(42);
        assert_eq!(curve.id(), 42);
    }

    #[test]
    fn test_map_add() {
        let mut map = MapOfCurve::new();
        let curve = Curve::new(5);
        assert!(map.add(curve.clone()));
        assert!(!map.add(curve));
    }

    #[test]
    fn test_map_contains() {
        let mut map = MapOfCurve::new();
        let curve = Curve::new(5);
        assert!(!map.contains(&curve));
        map.add(curve.clone());
        assert!(map.contains(&curve));
    }

    #[test]
    fn test_map_remove() {
        let mut map = MapOfCurve::new();
        let curve = Curve::new(3);
        map.add(curve.clone());
        assert_eq!(map.size(), 1);
        assert!(map.remove(&curve));
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_map_clear() {
        let mut map = MapOfCurve::new();
        map.add(Curve::new(1));
        map.add(Curve::new(2));
        map.clear();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_iterator() {
        let mut map = MapOfCurve::new();
        map.add(Curve::new(1));
        map.add(Curve::new(2));

        let mut iter = MapIterator::new(&map);
        let mut count = 0;
        while iter.is_more() {
            assert!(iter.current().is_some());
            iter.next();
            count += 1;
        }
        assert_eq!(count, 2);
    }
}

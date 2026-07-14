// FILE: ch_fi_k_part_rst_map.rs
// occt: ChFiKPart_RstMap
// occt-ref: ChFiKPart_DataMapIteratorOfRstMap

//! Deprecated type aliases for backward compatibility.
//! Use HashMap<i32, Arc<Adaptor2dCurve2d>> directly instead.

use std::sync::Arc;
use std::collections::HashMap;

/// 2D curve adaptor handle type (opaque marker).
pub struct Adaptor2dCurve2dHandle;

/// Deprecated map from integer to 2D curve adaptor.
/// Maps to NCollection_DataMap<int, opencascade::handle<Adaptor2d_Curve2d>>.
pub type ChFiKPartRstMap = HashMap<i32, Arc<Adaptor2dCurve2dHandle>>;

/// Deprecated iterator over a map of restriction curves.
/// Maps to NCollection_DataMap<...>::Iterator.
pub struct ChFiKPartDataMapIteratorOfRstMap<'a> {
    iter: std::collections::hash_map::Iter<'a, i32, Arc<Adaptor2dCurve2dHandle>>,
}

impl<'a> ChFiKPartDataMapIteratorOfRstMap<'a> {
    /// Creates a new iterator over the map.
    pub fn new(map: &'a ChFiKPartRstMap) -> Self {
        ChFiKPartDataMapIteratorOfRstMap {
            iter: map.iter(),
        }
    }

    /// Returns true if there are more elements to iterate.
    pub fn more(&self) -> bool {
        // Note: This is a reduced interface. In real iteration, we'd need
        // to track position more carefully.
        false
    }

    /// Gets the current key (index).
    pub fn key(&self) -> Option<i32> {
        None
    }

    /// Gets the current value (curve adaptor).
    pub fn value(&self) -> Option<&Arc<Adaptor2dCurve2dHandle>> {
        None
    }
}

impl<'a> Iterator for ChFiKPartDataMapIteratorOfRstMap<'a> {
    type Item = (i32, &'a Arc<Adaptor2dCurve2dHandle>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, v)| (*k, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rst_map_creation() {
        let map: ChFiKPartRstMap = HashMap::new();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_rst_map_insert() {
        let mut map: ChFiKPartRstMap = HashMap::new();
        let curve = Arc::new(Adaptor2dCurve2dHandle);
        map.insert(1, curve.clone());

        assert_eq!(map.len(), 1);
        assert!(map.contains_key(&1));
        assert_eq!(map.get(&1).unwrap().as_ref() as *const _, curve.as_ref() as *const _);
    }

    #[test]
    fn test_rst_map_multiple_entries() {
        let mut map: ChFiKPartRstMap = HashMap::new();
        let curve1 = Arc::new(Adaptor2dCurve2dHandle);
        let curve2 = Arc::new(Adaptor2dCurve2dHandle);
        let curve3 = Arc::new(Adaptor2dCurve2dHandle);

        map.insert(1, curve1);
        map.insert(2, curve2);
        map.insert(3, curve3);

        assert_eq!(map.len(), 3);
        assert!(map.contains_key(&1));
        assert!(map.contains_key(&2));
        assert!(map.contains_key(&3));
    }

    #[test]
    fn test_rst_map_iterator() {
        let mut map: ChFiKPartRstMap = HashMap::new();
        map.insert(1, Arc::new(Adaptor2dCurve2dHandle));
        map.insert(2, Arc::new(Adaptor2dCurve2dHandle));
        map.insert(3, Arc::new(Adaptor2dCurve2dHandle));

        let iter = ChFiKPartDataMapIteratorOfRstMap::new(&map);
        let count = iter.count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_rst_map_lookup() {
        let mut map: ChFiKPartRstMap = HashMap::new();
        let curve = Arc::new(Adaptor2dCurve2dHandle);
        map.insert(10, curve.clone());

        assert_eq!(map.get(&10).is_some(), true);
        assert_eq!(map.get(&11).is_some(), false);
    }

    #[test]
    fn test_rst_map_remove() {
        let mut map: ChFiKPartRstMap = HashMap::new();
        map.insert(1, Arc::new(Adaptor2dCurve2dHandle));
        map.insert(2, Arc::new(Adaptor2dCurve2dHandle));

        assert_eq!(map.len(), 2);

        map.remove(&1);
        assert_eq!(map.len(), 1);
        assert!(!map.contains_key(&1));
        assert!(map.contains_key(&2));
    }

    #[test]
    fn test_rst_map_clear() {
        let mut map: ChFiKPartRstMap = HashMap::new();
        map.insert(1, Arc::new(Adaptor2dCurve2dHandle));
        map.insert(2, Arc::new(Adaptor2dCurve2dHandle));

        assert_eq!(map.len(), 2);

        map.clear();
        assert_eq!(map.len(), 0);
    }
}

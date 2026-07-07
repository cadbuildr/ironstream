// FILE: int_tools_map_of_surface_sample.rs
// occt: IntTools_MapOfSurfaceSample, IntTools_MapIteratorOfMapOfSurfaceSample

use std::collections::BTreeSet;

/// Deprecated alias for NCollection_Map<IntTools_SurfaceRangeSample>.
/// Maintains backward compatibility. Use BTreeSet or HashMap directly in new code.
pub struct IntToolsMapOfSurfaceSample {
    items: BTreeSet<u32>, // Placeholder for IntTools_SurfaceRangeSample (opaque type)
}

impl IntToolsMapOfSurfaceSample {
    pub fn new() -> Self {
        Self {
            items: BTreeSet::new(),
        }
    }

    pub fn add(&mut self, item: u32) -> bool {
        self.items.insert(item)
    }

    pub fn remove(&mut self, item: u32) -> bool {
        self.items.remove(&item)
    }

    pub fn contains(&self, item: u32) -> bool {
        self.items.contains(&item)
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn iterator(&self) -> IntToolsMapIteratorOfMapOfSurfaceSample {
        IntToolsMapIteratorOfMapOfSurfaceSample {
            items: self.items.iter().cloned().collect::<Vec<_>>(),
            index: 0,
        }
    }
}

impl Default for IntToolsMapOfSurfaceSample {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for IntTools_MapOfSurfaceSample.
pub struct IntToolsMapIteratorOfMapOfSurfaceSample {
    items: Vec<u32>,
    index: usize,
}

impl IntToolsMapIteratorOfMapOfSurfaceSample {
    pub fn more(&self) -> bool {
        self.index < self.items.len()
    }

    pub fn next(&mut self) {
        if self.index < self.items.len() {
            self.index += 1;
        }
    }

    pub fn value(&self) -> Option<u32> {
        if self.index < self.items.len() {
            Some(self.items[self.index])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_add_and_remove() {
        let mut map = IntToolsMapOfSurfaceSample::new();
        assert!(map.is_empty());

        assert!(map.add(42));
        assert!(!map.add(42));
        assert_eq!(map.len(), 1);
        assert!(map.contains(42));

        assert!(map.remove(42));
        assert!(!map.contains(42));
        assert!(!map.remove(42));
    }

    #[test]
    fn test_map_clear() {
        let mut map = IntToolsMapOfSurfaceSample::new();
        map.add(1);
        map.add(2);
        map.add(3);
        assert_eq!(map.len(), 3);

        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_iterator() {
        let mut map = IntToolsMapOfSurfaceSample::new();
        map.add(10);
        map.add(20);
        map.add(30);

        let mut iter = map.iterator();
        let mut values = Vec::new();
        while iter.more() {
            if let Some(v) = iter.value() {
                values.push(v);
            }
            iter.next();
        }

        assert_eq!(values.len(), 3);
        assert!(values.contains(&10));
        assert!(values.contains(&20));
        assert!(values.contains(&30));
    }

    #[test]
    fn test_default() {
        let map = IntToolsMapOfSurfaceSample::default();
        assert!(map.is_empty());
    }
}

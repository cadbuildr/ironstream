// FILE: top_ope_b_rep_ds_map_of_surface.rs
// occt: TopOpeBRepDS_MapOfSurface

use std::collections::HashSet;

/// Surface: Simplified surface representation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Surface {
    id: usize,
}

impl Surface {
    pub fn new(id: usize) -> Self {
        Surface { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// MapOfSurface: Set-like container for surfaces.
#[derive(Clone, Debug)]
pub struct MapOfSurface {
    data: HashSet<Surface>,
}

impl MapOfSurface {
    pub fn new() -> Self {
        MapOfSurface {
            data: HashSet::new(),
        }
    }

    pub fn add(&mut self, surface: Surface) -> bool {
        self.data.insert(surface)
    }

    pub fn remove(&mut self, surface: &Surface) -> bool {
        self.data.remove(surface)
    }

    pub fn contains(&self, surface: &Surface) -> bool {
        self.data.contains(surface)
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &Surface> {
        self.data.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for MapOfSurface {
    fn default() -> Self {
        Self::new()
    }
}

/// MapIterator: Iterator for MapOfSurface.
pub struct MapIterator {
    surfaces: Vec<Surface>,
    index: usize,
}

impl MapIterator {
    pub fn new(map: &MapOfSurface) -> Self {
        MapIterator {
            surfaces: map.data.iter().cloned().collect(),
            index: 0,
        }
    }

    pub fn is_more(&self) -> bool {
        self.index < self.surfaces.len()
    }

    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn current(&self) -> Option<&Surface> {
        self.surfaces.get(self.index)
    }

    pub fn value(&self) -> Option<&Surface> {
        self.current()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_new() {
        let surf = Surface::new(42);
        assert_eq!(surf.id(), 42);
    }

    #[test]
    fn test_map_add() {
        let mut map = MapOfSurface::new();
        let surf = Surface::new(5);
        assert!(map.add(surf.clone()));
        assert!(!map.add(surf));
    }

    #[test]
    fn test_map_contains() {
        let mut map = MapOfSurface::new();
        let surf = Surface::new(5);
        assert!(!map.contains(&surf));
        map.add(surf.clone());
        assert!(map.contains(&surf));
    }

    #[test]
    fn test_map_remove() {
        let mut map = MapOfSurface::new();
        let surf = Surface::new(3);
        map.add(surf.clone());
        assert_eq!(map.size(), 1);
        assert!(map.remove(&surf));
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_map_clear() {
        let mut map = MapOfSurface::new();
        map.add(Surface::new(1));
        map.add(Surface::new(2));
        map.clear();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_iterator() {
        let mut map = MapOfSurface::new();
        map.add(Surface::new(1));
        map.add(Surface::new(2));

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

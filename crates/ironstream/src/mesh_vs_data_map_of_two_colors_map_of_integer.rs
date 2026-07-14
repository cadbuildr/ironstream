// FILE: mesh_vs_data_map_of_two_colors_map_of_integer.rs
// occt: MeshVS_DataMapOfTwoColorsMapOfInteger
// occt-ref: MeshVS_DataMapIteratorOfDataMapOfTwoColorsMapOfInteger

use std::collections::{HashMap, HashSet};

/// MeshVS_TwoColors represents a pair of colors (primary and secondary).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MeshVsTwoColors {
    pub r1: u8,
    pub g1: u8,
    pub b1: u8,
    pub r2: u8,
    pub g2: u8,
    pub b2: u8,
}

impl MeshVsTwoColors {
    pub fn new(
        r1: u8,
        g1: u8,
        b1: u8,
        r2: u8,
        g2: u8,
        b2: u8,
    ) -> Self {
        MeshVsTwoColors { r1, g1, b1, r2, g2, b2 }
    }

    pub fn uniform(r: u8, g: u8, b: u8) -> Self {
        MeshVsTwoColors { r1: r, g1: g, b1: b, r2: r, g2: g, b2: b }
    }
}

/// TColStd_MapOfInteger represents a set of integers.
pub type TColstdMapOfInteger = HashSet<i32>;

/// Deprecated typedef alias for backward compatibility.
/// Original OCCT: `NCollection_DataMap<MeshVS_TwoColors, TColStd_MapOfInteger>`
pub type MeshVsDataMapOfTwoColorsMapOfInteger = HashMap<MeshVsTwoColors, TColstdMapOfInteger>;

/// Deprecated typedef alias for the iterator.
/// Original OCCT: `NCollection_DataMap<MeshVS_TwoColors, TColStd_MapOfInteger>::Iterator`
pub type MeshVsDataMapIteratorOfDataMapOfTwoColorsMapOfInteger =
    std::collections::hash_map::IntoIter<MeshVsTwoColors, TColstdMapOfInteger>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_colors_creation() {
        let colors = MeshVsTwoColors::new(255, 0, 0, 0, 255, 0);
        assert_eq!(colors.r1, 255);
        assert_eq!(colors.g1, 0);
        assert_eq!(colors.b1, 0);
        assert_eq!(colors.r2, 0);
        assert_eq!(colors.g2, 255);
        assert_eq!(colors.b2, 0);
    }

    #[test]
    fn test_two_colors_uniform() {
        let colors = MeshVsTwoColors::uniform(128, 128, 128);
        assert_eq!(colors.r1, 128);
        assert_eq!(colors.g1, 128);
        assert_eq!(colors.b1, 128);
        assert_eq!(colors.r2, 128);
        assert_eq!(colors.g2, 128);
        assert_eq!(colors.b2, 128);
    }

    #[test]
    fn test_map_of_integer_creation() {
        let set: TColstdMapOfInteger = HashSet::new();
        assert!(set.is_empty());
    }

    #[test]
    fn test_map_of_integer_insert() {
        let mut set: TColstdMapOfInteger = HashSet::new();
        assert!(set.insert(1));
        assert!(set.insert(2));
        assert!(set.insert(3));
        assert!(!set.insert(1)); // already exists
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_data_map_creation() {
        let map: MeshVsDataMapOfTwoColorsMapOfInteger = HashMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_data_map_insert_and_retrieve() {
        let mut map: MeshVsDataMapOfTwoColorsMapOfInteger = HashMap::new();

        let colors1 = MeshVsTwoColors::new(255, 0, 0, 0, 255, 0);
        let colors2 = MeshVsTwoColors::new(0, 0, 255, 255, 255, 0);

        let mut set1: TColstdMapOfInteger = HashSet::new();
        set1.insert(1);
        set1.insert(2);
        set1.insert(3);

        let mut set2: TColstdMapOfInteger = HashSet::new();
        set2.insert(10);
        set2.insert(20);

        map.insert(colors1, set1.clone());
        map.insert(colors2, set2.clone());

        assert_eq!(map.get(&colors1), Some(&set1));
        assert_eq!(map.get(&colors2), Some(&set2));
    }

    #[test]
    fn test_data_map_size() {
        let mut map: MeshVsDataMapOfTwoColorsMapOfInteger = HashMap::new();
        assert_eq!(map.len(), 0);

        let colors1 = MeshVsTwoColors::uniform(100, 100, 100);
        let colors2 = MeshVsTwoColors::uniform(200, 200, 200);

        let mut set: TColstdMapOfInteger = HashSet::new();
        set.insert(1);

        map.insert(colors1, set.clone());
        assert_eq!(map.len(), 1);

        map.insert(colors2, set.clone());
        assert_eq!(map.len(), 2);

        map.remove(&colors1);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_data_map_iteration() {
        let mut map: MeshVsDataMapOfTwoColorsMapOfInteger = HashMap::new();

        let colors1 = MeshVsTwoColors::new(255, 0, 0, 0, 255, 0);
        let colors2 = MeshVsTwoColors::new(0, 0, 255, 255, 255, 0);

        let mut set1: TColstdMapOfInteger = HashSet::new();
        set1.insert(1);

        let mut set2: TColstdMapOfInteger = HashSet::new();
        set2.insert(10);

        map.insert(colors1, set1);
        map.insert(colors2, set2);

        let collected: Vec<(MeshVsTwoColors, TColstdMapOfInteger)> = map.into_iter().collect();
        assert_eq!(collected.len(), 2);
    }
}

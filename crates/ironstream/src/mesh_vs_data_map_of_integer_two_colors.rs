// FILE: mesh_vs_data_map_of_integer_two_colors.rs
// occt: MeshVS_DataMapOfIntegerTwoColors
// occt-ref: MeshVS_DataMapIteratorOfDataMapOfIntegerTwoColors

use std::collections::HashMap;

/// MeshVS_TwoColors represents a pair of colors (primary and secondary).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeshVsTwoColors {
    pub color1: (f32, f32, f32, f32),
    pub color2: (f32, f32, f32, f32),
}

impl MeshVsTwoColors {
    pub fn new(
        color1: (f32, f32, f32, f32),
        color2: (f32, f32, f32, f32),
    ) -> Self {
        MeshVsTwoColors { color1, color2 }
    }

    pub fn from_rgb(
        r1: f32,
        g1: f32,
        b1: f32,
        r2: f32,
        g2: f32,
        b2: f32,
    ) -> Self {
        MeshVsTwoColors {
            color1: (r1, g1, b1, 1.0),
            color2: (r2, g2, b2, 1.0),
        }
    }

    pub fn uniform(color: (f32, f32, f32, f32)) -> Self {
        MeshVsTwoColors {
            color1: color,
            color2: color,
        }
    }
}

/// Deprecated typedef alias for backward compatibility.
/// Original OCCT: `NCollection_DataMap<int, MeshVS_TwoColors>`
pub type MeshVsDataMapOfIntegerTwoColors = HashMap<i32, MeshVsTwoColors>;

/// Deprecated typedef alias for the iterator.
/// Original OCCT: `NCollection_DataMap<int, MeshVS_TwoColors>::Iterator`
pub type MeshVsDataMapIteratorOfDataMapOfIntegerTwoColors =
    std::collections::hash_map::IntoIter<i32, MeshVsTwoColors>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_colors_creation() {
        let colors = MeshVsTwoColors::new(
            (1.0, 0.0, 0.0, 1.0),
            (0.0, 1.0, 0.0, 1.0),
        );
        assert_eq!(colors.color1, (1.0, 0.0, 0.0, 1.0));
        assert_eq!(colors.color2, (0.0, 1.0, 0.0, 1.0));
    }

    #[test]
    fn test_two_colors_from_rgb() {
        let colors = MeshVsTwoColors::from_rgb(1.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(colors.color1, (1.0, 0.0, 0.0, 1.0));
        assert_eq!(colors.color2, (0.0, 1.0, 0.0, 1.0));
    }

    #[test]
    fn test_two_colors_uniform() {
        let colors = MeshVsTwoColors::uniform((0.5, 0.5, 0.5, 1.0));
        assert_eq!(colors.color1, (0.5, 0.5, 0.5, 1.0));
        assert_eq!(colors.color2, (0.5, 0.5, 0.5, 1.0));
    }

    #[test]
    fn test_data_map_creation() {
        let map: MeshVsDataMapOfIntegerTwoColors = HashMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_data_map_insert_and_retrieve() {
        let mut map: MeshVsDataMapOfIntegerTwoColors = HashMap::new();
        let colors1 = MeshVsTwoColors::new(
            (1.0, 0.0, 0.0, 1.0),
            (0.0, 1.0, 0.0, 1.0),
        );
        let colors2 = MeshVsTwoColors::new(
            (0.0, 0.0, 1.0, 1.0),
            (1.0, 1.0, 0.0, 1.0),
        );

        map.insert(1, colors1);
        map.insert(2, colors2);

        assert_eq!(map.get(&1), Some(&colors1));
        assert_eq!(map.get(&2), Some(&colors2));
        assert_eq!(map.get(&3), None);
    }

    #[test]
    fn test_data_map_size() {
        let mut map: MeshVsDataMapOfIntegerTwoColors = HashMap::new();
        assert_eq!(map.len(), 0);

        let colors = MeshVsTwoColors::uniform((0.5, 0.5, 0.5, 1.0));
        map.insert(10, colors);
        map.insert(20, colors);
        assert_eq!(map.len(), 2);

        map.remove(&10);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_data_map_iteration() {
        let mut map: MeshVsDataMapOfIntegerTwoColors = HashMap::new();
        let colors = MeshVsTwoColors::from_rgb(1.0, 0.0, 0.0, 0.0, 1.0, 0.0);

        map.insert(1, colors);
        map.insert(2, colors);

        let collected: Vec<(i32, MeshVsTwoColors)> = map.into_iter().collect();
        assert_eq!(collected.len(), 2);
    }
}

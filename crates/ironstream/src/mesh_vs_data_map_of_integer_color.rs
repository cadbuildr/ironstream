// FILE: mesh_vs_data_map_of_integer_color.rs
// occt: MeshVS_DataMapOfIntegerColor, MeshVS_DataMapIteratorOfDataMapOfIntegerColor

use std::collections::HashMap;

/// Deprecated typedef alias for backward compatibility.
/// A data map from integer to Quantity_Color values.
/// Quantity_Color is represented as an RGBA color with values 0.0 to 1.0.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuantityColor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl QuantityColor {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        QuantityColor { red, green, blue, alpha }
    }

    pub fn white() -> Self {
        QuantityColor { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }
    }

    pub fn black() -> Self {
        QuantityColor { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }
    }

    pub fn red() -> Self {
        QuantityColor { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }
    }

    pub fn green() -> Self {
        QuantityColor { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }
    }

    pub fn blue() -> Self {
        QuantityColor { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }
    }
}

/// Deprecated typedef alias for backward compatibility.
/// Original OCCT: `NCollection_DataMap<int, Quantity_Color>`
pub type MeshVsDataMapOfIntegerColor = HashMap<i32, QuantityColor>;

/// Deprecated typedef alias for the iterator.
/// Original OCCT: `NCollection_DataMap<int, Quantity_Color>::Iterator`
pub type MeshVsDataMapIteratorOfDataMapOfIntegerColor =
    std::collections::hash_map::IntoIter<i32, QuantityColor>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = QuantityColor::new(0.5, 0.6, 0.7, 1.0);
        assert_eq!(color.red, 0.5);
        assert_eq!(color.green, 0.6);
        assert_eq!(color.blue, 0.7);
        assert_eq!(color.alpha, 1.0);
    }

    #[test]
    fn test_color_constants() {
        let white = QuantityColor::white();
        assert_eq!(white, QuantityColor::new(1.0, 1.0, 1.0, 1.0));

        let black = QuantityColor::black();
        assert_eq!(black, QuantityColor::new(0.0, 0.0, 0.0, 1.0));

        let red = QuantityColor::red();
        assert_eq!(red, QuantityColor::new(1.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn test_data_map_creation() {
        let map: MeshVsDataMapOfIntegerColor = HashMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_data_map_insert_and_retrieve() {
        let mut map: MeshVsDataMapOfIntegerColor = HashMap::new();
        let color1 = QuantityColor::red();
        let color2 = QuantityColor::blue();

        map.insert(1, color1);
        map.insert(2, color2);

        assert_eq!(map.get(&1), Some(&color1));
        assert_eq!(map.get(&2), Some(&color2));
        assert_eq!(map.get(&3), None);
    }

    #[test]
    fn test_data_map_size() {
        let mut map: MeshVsDataMapOfIntegerColor = HashMap::new();
        assert_eq!(map.len(), 0);

        map.insert(10, QuantityColor::white());
        map.insert(20, QuantityColor::black());
        assert_eq!(map.len(), 2);

        map.remove(&10);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_data_map_iteration() {
        let mut map: MeshVsDataMapOfIntegerColor = HashMap::new();
        let color1 = QuantityColor::new(1.0, 0.0, 0.0, 1.0);
        let color2 = QuantityColor::new(0.0, 1.0, 0.0, 1.0);

        map.insert(1, color1);
        map.insert(2, color2);

        let collected: Vec<(i32, QuantityColor)> = map.into_iter().collect();
        assert_eq!(collected.len(), 2);
    }
}

// FILE: ais_data_map_of_shape_drawer.rs
// occt: AIS_DataMapOfShapeDrawer

//! Deprecated NCollection alias: DataMap<TopoDS_Shape, Drawer>

use std::collections::HashMap;

/// Shape drawer (simplified stub).
#[derive(Clone, Debug)]
pub struct Drawer {
    pub id: u32,
}

/// DataMap from shape to drawer.
pub type AisDataMapOfShapeDrawer = HashMap<u32, Drawer>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_drawer_map() {
        let mut map: AisDataMapOfShapeDrawer = HashMap::new();
        map.insert(1, Drawer { id: 100 });
        assert_eq!(map.get(&1).map(|d| d.id), Some(100));
    }
}

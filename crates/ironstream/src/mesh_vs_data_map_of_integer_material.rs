// FILE: mesh_vs_data_map_of_integer_material.rs
// occt: MeshVS_DataMapOfIntegerMaterial, MeshVS_DataMapIteratorOfDataMapOfIntegerMaterial

use std::collections::HashMap;

/// Graphic3d_MaterialAspect represents material properties for 3D rendering.
#[derive(Clone, Debug, PartialEq)]
pub struct Graphic3dMaterialAspect {
    pub ambient: (f32, f32, f32),
    pub diffuse: (f32, f32, f32),
    pub specular: (f32, f32, f32),
    pub emission: (f32, f32, f32),
    pub shininess: f32,
    pub transparency: f32,
}

impl Graphic3dMaterialAspect {
    pub fn new(
        ambient: (f32, f32, f32),
        diffuse: (f32, f32, f32),
        specular: (f32, f32, f32),
        emission: (f32, f32, f32),
        shininess: f32,
        transparency: f32,
    ) -> Self {
        Graphic3dMaterialAspect {
            ambient,
            diffuse,
            specular,
            emission,
            shininess,
            transparency,
        }
    }

    pub fn default_material() -> Self {
        Graphic3dMaterialAspect {
            ambient: (0.2, 0.2, 0.2),
            diffuse: (0.8, 0.8, 0.8),
            specular: (1.0, 1.0, 1.0),
            emission: (0.0, 0.0, 0.0),
            shininess: 32.0,
            transparency: 0.0,
        }
    }
}

/// Deprecated typedef alias for backward compatibility.
/// Original OCCT: `NCollection_DataMap<int, Graphic3d_MaterialAspect>`
pub type MeshVsDataMapOfIntegerMaterial = HashMap<i32, Graphic3dMaterialAspect>;

/// Deprecated typedef alias for the iterator.
/// Original OCCT: `NCollection_DataMap<int, Graphic3d_MaterialAspect>::Iterator`
pub type MeshVsDataMapIteratorOfDataMapOfIntegerMaterial =
    std::collections::hash_map::IntoIter<i32, Graphic3dMaterialAspect>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_creation() {
        let material = Graphic3dMaterialAspect::new(
            (0.2, 0.2, 0.2),
            (0.8, 0.8, 0.8),
            (1.0, 1.0, 1.0),
            (0.0, 0.0, 0.0),
            32.0,
            0.0,
        );
        assert_eq!(material.ambient, (0.2, 0.2, 0.2));
        assert_eq!(material.diffuse, (0.8, 0.8, 0.8));
        assert_eq!(material.shininess, 32.0);
    }

    #[test]
    fn test_default_material() {
        let material = Graphic3dMaterialAspect::default_material();
        assert_eq!(material.ambient, (0.2, 0.2, 0.2));
        assert_eq!(material.transparency, 0.0);
    }

    #[test]
    fn test_data_map_creation() {
        let map: MeshVsDataMapOfIntegerMaterial = HashMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_data_map_insert_and_retrieve() {
        let mut map: MeshVsDataMapOfIntegerMaterial = HashMap::new();
        let material1 = Graphic3dMaterialAspect::default_material();
        let material2 = Graphic3dMaterialAspect::new(
            (0.1, 0.1, 0.1),
            (0.9, 0.9, 0.9),
            (1.0, 1.0, 1.0),
            (0.0, 0.0, 0.0),
            64.0,
            0.1,
        );

        map.insert(1, material1.clone());
        map.insert(2, material2.clone());

        assert_eq!(map.get(&1), Some(&material1));
        assert_eq!(map.get(&2), Some(&material2));
        assert_eq!(map.get(&3), None);
    }

    #[test]
    fn test_data_map_size() {
        let mut map: MeshVsDataMapOfIntegerMaterial = HashMap::new();
        assert_eq!(map.len(), 0);

        map.insert(10, Graphic3dMaterialAspect::default_material());
        map.insert(20, Graphic3dMaterialAspect::default_material());
        assert_eq!(map.len(), 2);

        map.remove(&10);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_data_map_iteration() {
        let mut map: MeshVsDataMapOfIntegerMaterial = HashMap::new();
        let material = Graphic3dMaterialAspect::default_material();

        map.insert(1, material.clone());
        map.insert(2, material.clone());

        let collected: Vec<(i32, Graphic3dMaterialAspect)> = map.into_iter().collect();
        assert_eq!(collected.len(), 2);
    }
}

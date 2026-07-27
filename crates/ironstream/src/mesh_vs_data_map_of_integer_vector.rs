// FILE: mesh_vs_data_map_of_integer_vector.rs
// occt: MeshVS_DataMapOfIntegerVector
// occt-ref: MeshVS_DataMapIteratorOfDataMapOfIntegerVector

use std::collections::HashMap;

/// gp_Vec represents a 3D vector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpVec {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GpVec {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        GpVec { x, y, z }
    }

    pub fn zero() -> Self {
        GpVec { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn unit_x() -> Self {
        GpVec { x: 1.0, y: 0.0, z: 0.0 }
    }

    pub fn unit_y() -> Self {
        GpVec { x: 0.0, y: 1.0, z: 0.0 }
    }

    pub fn unit_z() -> Self {
        GpVec { x: 0.0, y: 0.0, z: 1.0 }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, other: &GpVec) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &GpVec) -> GpVec {
        GpVec {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(&self) -> GpVec {
        let mag = self.magnitude();
        if mag > 0.0 {
            GpVec {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        } else {
            GpVec::zero()
        }
    }
}

/// Deprecated typedef alias for backward compatibility.
/// Original OCCT: `NCollection_DataMap<int, gp_Vec>`
pub type MeshVsDataMapOfIntegerVector = HashMap<i32, GpVec>;

/// Deprecated typedef alias for the iterator.
/// Original OCCT: `NCollection_DataMap<int, gp_Vec>::Iterator`
pub type MeshVsDataMapIteratorOfDataMapOfIntegerVector =
    std::collections::hash_map::IntoIter<i32, GpVec>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vec = GpVec::new(1.0, 2.0, 3.0);
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 3.0);
    }

    #[test]
    fn test_vector_zero() {
        let vec = GpVec::zero();
        assert_eq!(vec, GpVec::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_vector_unit_vectors() {
        assert_eq!(GpVec::unit_x(), GpVec::new(1.0, 0.0, 0.0));
        assert_eq!(GpVec::unit_y(), GpVec::new(0.0, 1.0, 0.0));
        assert_eq!(GpVec::unit_z(), GpVec::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_vector_magnitude() {
        let vec = GpVec::new(3.0, 4.0, 0.0);
        assert!((vec.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector_dot_product() {
        let vec1 = GpVec::new(1.0, 2.0, 3.0);
        let vec2 = GpVec::new(4.0, 5.0, 6.0);
        let dot = vec1.dot(&vec2);
        assert!((dot - 32.0).abs() < 1e-10); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_vector_cross_product() {
        let vec1 = GpVec::unit_x();
        let vec2 = GpVec::unit_y();
        let cross = vec1.cross(&vec2);
        assert_eq!(cross, GpVec::unit_z());
    }

    #[test]
    fn test_vector_normalize() {
        let vec = GpVec::new(3.0, 4.0, 0.0);
        let normalized = vec.normalize();
        assert!((normalized.magnitude() - 1.0).abs() < 1e-10);
        assert!((normalized.x - 0.6).abs() < 1e-10);
        assert!((normalized.y - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_data_map_creation() {
        let map: MeshVsDataMapOfIntegerVector = HashMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_data_map_insert_and_retrieve() {
        let mut map: MeshVsDataMapOfIntegerVector = HashMap::new();
        let vec1 = GpVec::new(1.0, 2.0, 3.0);
        let vec2 = GpVec::new(4.0, 5.0, 6.0);

        map.insert(1, vec1);
        map.insert(2, vec2);

        assert_eq!(map.get(&1), Some(&vec1));
        assert_eq!(map.get(&2), Some(&vec2));
        assert_eq!(map.get(&3), None);
    }

    #[test]
    fn test_data_map_size() {
        let mut map: MeshVsDataMapOfIntegerVector = HashMap::new();
        assert_eq!(map.len(), 0);

        let vec = GpVec::unit_x();
        map.insert(10, vec);
        map.insert(20, vec);
        assert_eq!(map.len(), 2);

        map.remove(&10);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_data_map_iteration() {
        let mut map: MeshVsDataMapOfIntegerVector = HashMap::new();
        let vec1 = GpVec::unit_x();
        let vec2 = GpVec::unit_y();

        map.insert(1, vec1);
        map.insert(2, vec2);

        let collected: Vec<(i32, GpVec)> = map.into_iter().collect();
        assert_eq!(collected.len(), 2);
    }
}

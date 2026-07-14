// FILE: bvh_set.rs
// occt: BVH_Set // - Abstract set of entities bounded by BVH boxes

use crate::bvh_types::Vec3;

/// Represents an axis-aligned bounding box (AABB).
/// Contains minimum and maximum corner points.
/// occt-note: BVH_Box<T, N> (specialized for 3D with T=f64)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BBox<T: Clone + Copy> {
    pub min: Vec3<T>,
    pub max: Vec3<T>,
}

impl<T: Clone + Copy> BBox<T> {
    /// Creates a new bounding box from min and max corners.
    pub fn new(min: Vec3<T>, max: Vec3<T>) -> Self {
        BBox { min, max }
    }
}

/// Concrete implementation of a BVH set for 3D objects with f64 coordinates.
/// occt-note: BVH_Set<f64, 3> - template specialization
/// Stores a collection of objects with their bounding boxes and centroids.
pub struct SimpleBVHSet {
    objects: Vec<(BBox<f64>, Vec3<f64>)>, // (bounding box, centroid)
}

impl SimpleBVHSet {
    /// Creates a new empty BVH set.
    pub fn new() -> Self {
        SimpleBVHSet {
            objects: Vec::new(),
        }
    }

    /// Adds an object with its bounding box and centroid.
    pub fn add_object(&mut self, bbox: BBox<f64>, centroid: Vec3<f64>) {
        self.objects.push((bbox, centroid));
    }

    /// Returns the total number of objects in the set.
    pub fn size(&self) -> usize {
        self.objects.len()
    }

    /// Returns the AABB of the object at the given index.
    pub fn box_at(&self, index: usize) -> BBox<f64> {
        self.objects[index].0
    }

    /// Returns the centroid position of the object at the given index along the specified axis.
    /// axis: 0 for X, 1 for Y, 2 for Z.
    pub fn center(&self, index: usize, axis: usize) -> f64 {
        match axis {
            0 => self.objects[index].1.x,
            1 => self.objects[index].1.y,
            2 => self.objects[index].1.z,
            _ => panic!("Invalid axis"),
        }
    }

    /// Swaps the positions of two objects in the set.
    /// This is used during sorting/partitioning operations.
    pub fn swap(&mut self, index1: usize, index2: usize) {
        self.objects.swap(index1, index2);
    }

    /// Returns the AABB of the entire set of objects by combining all individual boxes.
    pub fn bounding_box(&self) -> BBox<f64> {
        if self.objects.is_empty() {
            return BBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        }

        let mut result = self.objects[0].0;
        for index in 1..self.objects.len() {
            result = self.combine_boxes(result, self.objects[index].0);
        }
        result
    }

    /// Helper method to combine two bounding boxes.
    fn combine_boxes(&self, box1: BBox<f64>, box2: BBox<f64>) -> BBox<f64> {
        let min_x = if box1.min.x < box2.min.x {
            box1.min.x
        } else {
            box2.min.x
        };
        let min_y = if box1.min.y < box2.min.y {
            box1.min.y
        } else {
            box2.min.y
        };
        let min_z = if box1.min.z < box2.min.z {
            box1.min.z
        } else {
            box2.min.z
        };

        let max_x = if box1.max.x > box2.max.x {
            box1.max.x
        } else {
            box2.max.x
        };
        let max_y = if box1.max.y > box2.max.y {
            box1.max.y
        } else {
            box2.max.y
        };
        let max_z = if box1.max.z > box2.max.z {
            box1.max.z
        } else {
            box2.max.z
        };

        BBox::new(Vec3::new(min_x, min_y, min_z), Vec3::new(max_x, max_y, max_z))
    }
}

impl Default for SimpleBVHSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bbox_creation() {
        let min = Vec3::new(0.0, 0.0, 0.0);
        let max = Vec3::new(1.0, 1.0, 1.0);
        let bbox = BBox::new(min, max);
        assert_eq!(bbox.min.x, 0.0);
        assert_eq!(bbox.max.z, 1.0);
    }

    #[test]
    fn test_simple_bvh_set_empty() {
        let set = SimpleBVHSet::new();
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn test_simple_bvh_set_add_object() {
        let mut set = SimpleBVHSet::new();
        let bbox = BBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let centroid = Vec3::new(0.5, 0.5, 0.5);
        set.add_object(bbox, centroid);
        assert_eq!(set.size(), 1);
    }

    #[test]
    fn test_simple_bvh_set_box_at() {
        let mut set = SimpleBVHSet::new();
        let bbox = BBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let centroid = Vec3::new(0.5, 0.5, 0.5);
        set.add_object(bbox, centroid);

        let retrieved_bbox = set.box_at(0);
        assert_eq!(retrieved_bbox.min.x, 0.0);
        assert_eq!(retrieved_bbox.max.z, 1.0);
    }

    #[test]
    fn test_simple_bvh_set_center() {
        let mut set = SimpleBVHSet::new();
        let bbox = BBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 2.0, 2.0));
        let centroid = Vec3::new(1.0, 1.5, 2.0);
        set.add_object(bbox, centroid);

        assert_eq!(set.center(0, 0), 1.0);
        assert_eq!(set.center(0, 1), 1.5);
        assert_eq!(set.center(0, 2), 2.0);
    }

    #[test]
    fn test_simple_bvh_set_swap() {
        let mut set = SimpleBVHSet::new();
        let bbox1 = BBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let centroid1 = Vec3::new(0.5, 0.5, 0.5);
        let bbox2 = BBox::new(Vec3::new(2.0, 2.0, 2.0), Vec3::new(3.0, 3.0, 3.0));
        let centroid2 = Vec3::new(2.5, 2.5, 2.5);

        set.add_object(bbox1, centroid1);
        set.add_object(bbox2, centroid2);

        set.swap(0, 1);

        let box_after_swap = set.box_at(0);
        assert_eq!(box_after_swap.min.x, 2.0);
        assert_eq!(box_after_swap.max.x, 3.0);
    }

    #[test]
    fn test_bounding_box_combination() {
        let mut set = SimpleBVHSet::new();
        let bbox1 = BBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let centroid1 = Vec3::new(0.5, 0.5, 0.5);
        let bbox2 = BBox::new(Vec3::new(2.0, 2.0, 2.0), Vec3::new(3.0, 3.0, 3.0));
        let centroid2 = Vec3::new(2.5, 2.5, 2.5);

        set.add_object(bbox1, centroid1);
        set.add_object(bbox2, centroid2);

        let combined = set.bounding_box();
        assert_eq!(combined.min.x, 0.0);
        assert_eq!(combined.min.y, 0.0);
        assert_eq!(combined.min.z, 0.0);
        assert_eq!(combined.max.x, 3.0);
        assert_eq!(combined.max.y, 3.0);
        assert_eq!(combined.max.z, 3.0);
    }
}

// FILE: bvh_object_set.rs
//! Array of abstract entities (bounded by BVH boxes) to build BVH.
//! Templated on numeric data type (T) and vector dimension (N).

// occt: BVH_ObjectSet

/// Represents a single BVH object with an AABB.
pub trait BVHObject<T: Copy, const N: usize> {
    /// Returns the AABB (bounding box) of this object.
    fn box_aabb(&self) -> BVHBox<T, N>;
}

/// Simple N-dimensional axis-aligned bounding box.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BVHBox<T: Copy, const N: usize> {
    pub corner_min: [T; N],
    pub corner_max: [T; N],
}

impl<T: Copy + Default, const N: usize> BVHBox<T, N> {
    pub fn new() -> Self {
        BVHBox {
            corner_min: [T::default(); N],
            corner_max: [T::default(); N],
        }
    }

    pub fn with_corners(min: [T; N], max: [T; N]) -> Self {
        BVHBox {
            corner_min: min,
            corner_max: max,
        }
    }

    /// Combines two boxes into their union.
    pub fn combine(&mut self, other: &BVHBox<T, N>)
    where
        T: PartialOrd,
    {
        for i in 0..N {
            if other.corner_min[i] < self.corner_min[i] {
                self.corner_min[i] = other.corner_min[i];
            }
            if other.corner_max[i] > self.corner_max[i] {
                self.corner_max[i] = other.corner_max[i];
            }
        }
    }
}

/// Abstract set interface for BVH construction.
/// This is the minimal geometry interface needed to construct BVH.
pub trait BVHSet<T: Copy, const N: usize> {
    /// Returns the total number of objects in the set.
    fn size(&self) -> usize;

    /// Returns the AABB of the entire set of objects.
    fn r#box(&self) -> BVHBox<T, N>
    where
        T: PartialOrd + Default,
    {
        let mut bbox = BVHBox::new();
        for i in 0..self.size() {
            bbox.combine(&self.box_at(i));
        }
        bbox
    }

    /// Returns the AABB of the object at the given index.
    fn box_at(&self, index: usize) -> BVHBox<T, N>;

    /// Returns the centroid position along the given axis (0..N-1).
    fn center(&self, index: usize, axis: usize) -> T;

    /// Swaps two objects in the set.
    fn swap(&mut self, index1: usize, index2: usize);
}

/// Array of abstract entities (bounded by BVH boxes) to build BVH.
/// This is a collection of BVH objects organized for BVH tree construction.
///
/// Type parameters:
/// - T: Numeric data type (f32, f64, etc.)
/// - N: Vector dimension
pub struct BVHObjectSet<T: Copy, const N: usize> {
    objects: Vec<Box<dyn BVHObject<T, N>>>,
}

impl<T: Copy + Default, const N: usize> BVHObjectSet<T, N> {
    /// Creates a new empty set of geometric objects.
    pub fn new() -> Self {
        BVHObjectSet {
            objects: Vec::new(),
        }
    }

    /// Creates a new set with pre-allocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        BVHObjectSet {
            objects: Vec::with_capacity(capacity),
        }
    }

    /// Removes all geometric objects from the set.
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// Returns a reference to the array of geometric objects.
    pub fn objects(&self) -> &[Box<dyn BVHObject<T, N>>] {
        &self.objects
    }

    /// Returns a mutable reference to the array of geometric objects.
    pub fn objects_mut(&mut self) -> &mut Vec<Box<dyn BVHObject<T, N>>> {
        &mut self.objects
    }

    /// Adds an object to the set.
    pub fn add_object(&mut self, obj: Box<dyn BVHObject<T, N>>) {
        self.objects.push(obj);
    }
}

impl<T: Copy + Default + PartialOrd, const N: usize> Default for BVHObjectSet<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy + Default + PartialOrd + core::ops::Add<Output = T>, const N: usize> BVHSet<T, N>
    for BVHObjectSet<T, N>
{
    fn size(&self) -> usize {
        self.objects.len()
    }

    fn box_at(&self, index: usize) -> BVHBox<T, N> {
        if index < self.objects.len() {
            self.objects[index].box_aabb()
        } else {
            BVHBox::new()
        }
    }

    fn center(&self, index: usize, axis: usize) -> T {
        if index < self.objects.len() && axis < N {
            let bbox = self.objects[index].box_aabb();
            // Return the sum of min and max (algorithm handles division externally)
            // This matches OCCT's CenterAxis implementation
            bbox.corner_min[axis] + bbox.corner_max[axis]
        } else {
            T::default()
        }
    }

    fn swap(&mut self, index1: usize, index2: usize) {
        if index1 < self.objects.len() && index2 < self.objects.len() && index1 != index2 {
            self.objects.swap(index1, index2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Simple test object implementing BVHObject
    #[derive(Clone, Copy)]
    struct TestObject {
        min: [f64; 3],
        max: [f64; 3],
    }

    impl BVHObject<f64, 3> for TestObject {
        fn box_aabb(&self) -> BVHBox<f64, 3> {
            BVHBox {
                corner_min: self.min,
                corner_max: self.max,
            }
        }
    }

    #[test]
    fn test_object_set_creation() {
        let set: BVHObjectSet<f64, 3> = BVHObjectSet::new();
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn test_object_set_with_capacity() {
        let set: BVHObjectSet<f64, 3> = BVHObjectSet::with_capacity(10);
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn test_clear() {
        let mut set: BVHObjectSet<f64, 3> = BVHObjectSet::new();
        let obj = TestObject {
            min: [0.0, 0.0, 0.0],
            max: [1.0, 1.0, 1.0],
        };
        set.add_object(Box::new(obj));
        assert_eq!(set.size(), 1);
        set.clear();
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn test_box_at() {
        let mut set: BVHObjectSet<f64, 3> = BVHObjectSet::new();
        let obj = TestObject {
            min: [1.0, 2.0, 3.0],
            max: [4.0, 5.0, 6.0],
        };
        set.add_object(Box::new(obj));

        let bbox = set.box_at(0);
        assert_eq!(bbox.corner_min[0], 1.0);
        assert_eq!(bbox.corner_min[1], 2.0);
        assert_eq!(bbox.corner_min[2], 3.0);
        assert_eq!(bbox.corner_max[0], 4.0);
        assert_eq!(bbox.corner_max[1], 5.0);
        assert_eq!(bbox.corner_max[2], 6.0);
    }

    #[test]
    fn test_center() {
        let mut set: BVHObjectSet<f64, 3> = BVHObjectSet::new();
        let obj = TestObject {
            min: [0.0, 2.0, 4.0],
            max: [2.0, 4.0, 6.0],
        };
        set.add_object(Box::new(obj));

        // Center returns (min + max) for use in algorithms
        assert_eq!(set.center(0, 0), 2.0);
        assert_eq!(set.center(0, 1), 6.0);
        assert_eq!(set.center(0, 2), 10.0);
    }

    #[test]
    fn test_swap() {
        let mut set: BVHObjectSet<f64, 3> = BVHObjectSet::new();
        let obj1 = TestObject {
            min: [0.0, 0.0, 0.0],
            max: [1.0, 1.0, 1.0],
        };
        let obj2 = TestObject {
            min: [2.0, 2.0, 2.0],
            max: [3.0, 3.0, 3.0],
        };
        set.add_object(Box::new(obj1));
        set.add_object(Box::new(obj2));

        let box_before_0 = set.box_at(0);
        let box_before_1 = set.box_at(1);

        set.swap(0, 1);

        let box_after_0 = set.box_at(0);
        let box_after_1 = set.box_at(1);

        // After swap, indices should reference swapped objects
        assert_eq!(box_after_0.corner_min[0], box_before_1.corner_min[0]);
        assert_eq!(box_after_1.corner_min[0], box_before_0.corner_min[0]);
    }

    #[test]
    fn test_combined_box() {
        let mut set: BVHObjectSet<f64, 3> = BVHObjectSet::new();
        let obj1 = TestObject {
            min: [0.0, 0.0, 0.0],
            max: [1.0, 1.0, 1.0],
        };
        let obj2 = TestObject {
            min: [2.0, 2.0, 2.0],
            max: [3.0, 3.0, 3.0],
        };
        set.add_object(Box::new(obj1));
        set.add_object(Box::new(obj2));

        let combined = set.r#box();
        assert_eq!(combined.corner_min[0], 0.0);
        assert_eq!(combined.corner_min[1], 0.0);
        assert_eq!(combined.corner_min[2], 0.0);
        assert_eq!(combined.corner_max[0], 3.0);
        assert_eq!(combined.corner_max[1], 3.0);
        assert_eq!(combined.corner_max[2], 3.0);
    }
}

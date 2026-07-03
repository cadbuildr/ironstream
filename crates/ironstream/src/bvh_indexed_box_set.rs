// FILE: bvh_indexed_box_set.rs
//! Port of OCCT's `BVH_IndexedBoxSet` — an efficient indexed container for
//! building a BVH tree over elements with precomputed bounding boxes.
//!
//! Classes implemented:
//! - `BvhIndexedBoxSet<NumType, Dimension, DataType>` — templated indexed box set

use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;

// ────────────────────────────────────────────────────────────────────────────
// Minimal supporting types for standalone operation
// ────────────────────────────────────────────────────────────────────────────

/// A generic N-dimensional vector using array backing.
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
pub struct VecN<T: Copy, const N: usize> {
    data: [T; N],
}

impl<T: Copy, const N: usize> VecN<T, N> {
    /// Create a vector with uniform value.
    pub fn new(val: T) -> Self {
        VecN {
            data: [val; N],
        }
    }

    /// Access element by index.
    pub fn get(&self, i: usize) -> T {
        self.data[i]
    }

    /// Mutable access to data array.
    pub fn data_mut(&mut self) -> &mut [T; N] {
        &mut self.data
    }
}

impl<T: Copy, const N: usize> std::ops::Index<usize> for VecN<T, N> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        &self.data[i]
    }
}

/// An axis-aligned bounding box: min and max corners.
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct BvhBox<T: Copy + PartialOrd, const N: usize> {
    min: VecN<T, N>,
    max: VecN<T, N>,
}

impl<T: Copy + PartialOrd, const N: usize> BvhBox<T, N> {
    /// Create a box from corner points.
    pub fn new(min: VecN<T, N>, max: VecN<T, N>) -> Self {
        BvhBox { min, max }
    }

    /// Minimum corner.
    pub fn min(&self) -> VecN<T, N> {
        self.min
    }

    /// Maximum corner.
    pub fn max(&self) -> VecN<T, N> {
        self.max
    }
}

impl<T: Copy + PartialOrd + Default, const N: usize> Default for BvhBox<T, N> {
    fn default() -> Self {
        BvhBox::new(VecN::new(T::default()), VecN::new(T::default()))
    }
}

/// Abstract builder trait for BVH construction.
pub trait BvhBuilder<T: Copy + PartialOrd, const N: usize> {
    /// Build the BVH tree (simplified signature).
    fn build(&self);
}

/// Default binned SAH-based builder (simplified).
pub struct BvhBinnedBuilder<T: Copy + PartialOrd, const N: usize>(PhantomData<T>);

impl<T: Copy + PartialOrd, const N: usize> BvhBuilder<T, N> for BvhBinnedBuilder<T, N> {
    fn build(&self) {
        // Minimal implementation
    }
}

// ────────────────────────────────────────────────────────────────────────────
// BVH_IndexedBoxSet<NumType, Dimension, DataType>
// ────────────────────────────────────────────────────────────────────────────

/// An indexed container for BVH tree construction.
///
/// This class implements an efficient interface for adding elements and their
/// bounding boxes to a BVH tree. It uses indirect indexing (a separate index
/// vector) to access both the elements and their boxes, which allows heavy
/// data types to be stored with better efficiency during BVH construction
/// while only slightly slowing down selection time.
///
/// The typical usage pattern is:
/// 1. Create the set
/// 2. Call `set_size()` with the expected number of elements
/// 3. Call `add()` for each element with its bounding box
/// 4. Call the builder to construct the BVH tree
///
/// Generic parameters:
/// - `NumType`: numeric data type (f32, f64, etc.)
/// - `Dimension`: vector dimension (typically 3)
/// - `DataType`: type of elements (default: i32, can be any type)
// occt: BVH_IndexedBoxSet
pub struct BvhIndexedBoxSet<NumType: Copy + Default + PartialOrd, const Dimension: usize, DataType: Copy = i32> {
    /// Vector of elements (arbitrary data type, typically indices).
    elements: RefCell<std::vec::Vec<DataType>>,

    /// Vector of bounding boxes corresponding to elements.
    boxes: RefCell<std::vec::Vec<BvhBox<NumType, Dimension>>>,

    /// Vector of indices for indirect access (supports reordering without moving elements).
    indices: RefCell<std::vec::Vec<i32>>,

    /// The builder strategy.
    builder: Rc<dyn BvhBuilder<NumType, Dimension>>,

    /// Dirty flag indicating the BVH needs rebuilding.
    is_dirty: RefCell<bool>,
}

impl<NumType: Copy + Default + PartialOrd + 'static, const Dimension: usize, DataType: Copy + Default>
    BvhIndexedBoxSet<NumType, Dimension, DataType>
{
    /// Create an empty indexed box set with the default builder.
    pub fn new() -> Self {
        BvhIndexedBoxSet {
            elements: RefCell::new(std::vec::Vec::new()),
            boxes: RefCell::new(std::vec::Vec::new()),
            indices: RefCell::new(std::vec::Vec::new()),
            builder: Rc::new(BvhBinnedBuilder::<NumType, Dimension>(PhantomData)),
            is_dirty: RefCell::new(true),
        }
    }

    /// Create an indexed box set with a custom builder.
    pub fn with_builder(builder: Rc<dyn BvhBuilder<NumType, Dimension>>) -> Self {
        BvhIndexedBoxSet {
            elements: RefCell::new(std::vec::Vec::new()),
            boxes: RefCell::new(std::vec::Vec::new()),
            indices: RefCell::new(std::vec::Vec::new()),
            builder,
            is_dirty: RefCell::new(true),
        }
    }

    /// Set the expected size of the BVH tree.
    ///
    /// This pre-allocates space for elements, boxes, and indices to avoid
    /// repeated allocations during the adding phase.
    pub fn set_size(&self, size: usize) {
        self.indices.borrow_mut().reserve(size);
        self.elements.borrow_mut().reserve(size);
        self.boxes.borrow_mut().reserve(size);
    }

    /// Add an element with its bounding box to the set.
    ///
    /// Each call appends a new index (equal to the number of elements before
    /// the call), then appends the element and its box.
    pub fn add(&self, element: DataType, box_3d: BvhBox<NumType, Dimension>) {
        let mut indices = self.indices.borrow_mut();
        let mut elements = self.elements.borrow_mut();
        let mut boxes = self.boxes.borrow_mut();

        // Append a new index equal to the current size
        let new_index = indices.len() as i32;
        indices.push(new_index);

        // Append the element and its box
        elements.push(element);
        boxes.push(box_3d);

        // Mark as dirty since the set has changed
        *self.is_dirty.borrow_mut() = true;
    }

    /// Clear all elements, boxes, and indices.
    pub fn clear(&self) {
        self.elements.borrow_mut().clear();
        self.boxes.borrow_mut().clear();
        self.indices.borrow_mut().clear();
        *self.is_dirty.borrow_mut() = true;
    }

    /// Get the bounding box at the given logical index.
    ///
    /// This performs indirect access: it looks up the real index in the
    /// `indices` vector, then retrieves the box at that real index.
    pub fn box_at(&self, logical_index: usize) -> Option<BvhBox<NumType, Dimension>> {
        let indices = self.indices.borrow();
        if logical_index >= indices.len() {
            return None;
        }
        let real_index = indices[logical_index] as usize;
        let boxes = self.boxes.borrow();
        if real_index >= boxes.len() {
            return None;
        }
        Some(boxes[real_index])
    }

    /// Swap the indices of two logical elements.
    ///
    /// This allows the builder to reorder elements without moving the actual
    /// element or box data.
    pub fn swap(&self, logical_idx1: usize, logical_idx2: usize) {
        let mut indices = self.indices.borrow_mut();
        if logical_idx1 < indices.len() && logical_idx2 < indices.len() {
            indices.swap(logical_idx1, logical_idx2);
        }
    }

    /// Get the element at the given logical index.
    ///
    /// This performs indirect access, similar to `box_at()`.
    pub fn element_at(&self, logical_index: usize) -> Option<DataType> {
        let indices = self.indices.borrow();
        if logical_index >= indices.len() {
            return None;
        }
        let real_index = indices[logical_index] as usize;
        let elements = self.elements.borrow();
        if real_index >= elements.len() {
            return None;
        }
        Some(elements[real_index])
    }

    /// Returns the number of elements in the set.
    pub fn size(&self) -> usize {
        self.indices.borrow().len()
    }

    /// Check if the set is marked dirty.
    pub fn is_dirty(&self) -> bool {
        *self.is_dirty.borrow()
    }

    /// Mark the set as dirty.
    pub fn mark_dirty(&self) {
        *self.is_dirty.borrow_mut() = true;
    }

    /// Build the BVH tree from the current set.
    pub fn build(&self) {
        self.builder.build();
        *self.is_dirty.borrow_mut() = false;
    }

    /// Get reference to underlying elements (for advanced use).
    pub fn elements(&self) -> std::cell::Ref<std::vec::Vec<DataType>> {
        self.elements.borrow()
    }

    /// Get reference to underlying boxes (for advanced use).
    pub fn boxes(&self) -> std::cell::Ref<std::vec::Vec<BvhBox<NumType, Dimension>>> {
        self.boxes.borrow()
    }

    /// Get reference to underlying indices (for advanced use).
    pub fn indices(&self) -> std::cell::Ref<std::vec::Vec<i32>> {
        self.indices.borrow()
    }
}

impl<NumType: Copy + Default + PartialOrd + 'static, const Dimension: usize, DataType: Copy + Default> Default
    for BvhIndexedBoxSet<NumType, Dimension, DataType>
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexed_box_set_creation() {
        let set = BvhIndexedBoxSet::<f64, 3, i32>::new();
        assert_eq!(set.size(), 0);
        assert!(set.is_dirty());
    }

    #[test]
    fn test_indexed_box_set_reserve() {
        let set = BvhIndexedBoxSet::<f64, 3, i32>::new();
        set.set_size(10);
        assert_eq!(set.size(), 0); // Size doesn't change, just reserves capacity
    }

    #[test]
    fn test_indexed_box_set_add_single() {
        let set = BvhIndexedBoxSet::<f64, 3, i32>::new();

        let min = VecN::<f64, 3>::new(0.0);
        let max = VecN::<f64, 3>::new(1.0);
        let box_3d = BvhBox::new(min, max);

        set.add(42, box_3d);
        assert_eq!(set.size(), 1);

        // Retrieve the element and box
        assert_eq!(set.element_at(0), Some(42));
        assert_eq!(set.box_at(0), Some(box_3d));
    }

    #[test]
    fn test_indexed_box_set_add_multiple() {
        let set = BvhIndexedBoxSet::<f64, 3, i32>::new();
        set.set_size(3);

        let min1 = VecN::<f64, 3>::new(0.0);
        let max1 = VecN::<f64, 3>::new(1.0);
        let box1 = BvhBox::new(min1, max1);

        let min2 = VecN::<f64, 3>::new(2.0);
        let max2 = VecN::<f64, 3>::new(3.0);
        let box2 = BvhBox::new(min2, max2);

        let min3 = VecN::<f64, 3>::new(4.0);
        let max3 = VecN::<f64, 3>::new(5.0);
        let box3 = BvhBox::new(min3, max3);

        set.add(10, box1);
        set.add(20, box2);
        set.add(30, box3);

        assert_eq!(set.size(), 3);
        assert_eq!(set.element_at(0), Some(10));
        assert_eq!(set.element_at(1), Some(20));
        assert_eq!(set.element_at(2), Some(30));
    }

    #[test]
    fn test_indexed_box_set_swap() {
        let set = BvhIndexedBoxSet::<f64, 3, i32>::new();

        let min1 = VecN::<f64, 3>::new(0.0);
        let max1 = VecN::<f64, 3>::new(1.0);
        let box1 = BvhBox::new(min1, max1);

        let min2 = VecN::<f64, 3>::new(2.0);
        let max2 = VecN::<f64, 3>::new(3.0);
        let box2 = BvhBox::new(min2, max2);

        set.add(10, box1);
        set.add(20, box2);

        // After swap, indices should be reordered
        set.swap(0, 1);

        // Elements now appear in swapped order due to indirect indexing
        assert_eq!(set.element_at(0), Some(20));
        assert_eq!(set.element_at(1), Some(10));
    }

    #[test]
    fn test_indexed_box_set_clear() {
        let set = BvhIndexedBoxSet::<f64, 3, i32>::new();

        let min = VecN::<f64, 3>::new(0.0);
        let max = VecN::<f64, 3>::new(1.0);
        let box_3d = BvhBox::new(min, max);

        set.add(42, box_3d);
        assert_eq!(set.size(), 1);

        set.clear();
        assert_eq!(set.size(), 0);
        assert!(set.is_dirty());
    }

    #[test]
    fn test_indexed_box_set_out_of_bounds() {
        let set = BvhIndexedBoxSet::<f64, 3, i32>::new();

        // Accessing beyond size should return None
        assert_eq!(set.element_at(0), None);
        assert_eq!(set.box_at(0), None);
    }

    #[test]
    fn test_indexed_box_set_build() {
        let set = BvhIndexedBoxSet::<f64, 3, i32>::new();

        let min = VecN::<f64, 3>::new(0.0);
        let max = VecN::<f64, 3>::new(1.0);
        let box_3d = BvhBox::new(min, max);

        set.add(1, box_3d);
        assert!(set.is_dirty());

        set.build();
        assert!(!set.is_dirty());
    }
}

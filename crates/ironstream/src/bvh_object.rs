// FILE: bvh_object.rs
//! Port of OCCT's `BVH_Object` — abstract geometric object bounded by an AABB.
//!
//! Classes implemented:
//! - `BvhObjectTransient` — non-template base with properties and dirty flag
//! - `BvhObject<T, N>` — template for any numeric type and dimension

use std::cell::RefCell;
use std::rc::Rc;

// ────────────────────────────────────────────────────────────────────────────
// Simple vector/box types for standalone compilation
// ────────────────────────────────────────────────────────────────────────────

/// A generic N-dimensional vector using array backing.
#[derive(Clone, Debug, Copy)]
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
#[derive(Clone, Debug, Copy)]
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

// ────────────────────────────────────────────────────────────────────────────
// BVH_ObjectTransient
// ────────────────────────────────────────────────────────────────────────────

/// Abstract properties of a geometric object (generic handle).
/// In a full implementation, this would hold transform matrices, etc.
pub struct BvhProperties;

/// Non-template base class for `BvhObject<T, N>`.
/// Holds the dirty flag and generic properties.
// occt: BVH_ObjectTransient
pub struct BvhObjectTransient {
    /// Properties handle (in Rust, simplified as a marker type).
    properties: Option<Rc<BvhProperties>>,
    /// Is object state outdated?
    is_dirty: RefCell<bool>,
}

impl BvhObjectTransient {
    /// Create a new object in clean state.
    pub fn new() -> Self {
        BvhObjectTransient {
            properties: None,
            is_dirty: RefCell::new(false),
        }
    }

    /// Returns the object's properties.
    pub fn properties(&self) -> Option<&Rc<BvhProperties>> {
        self.properties.as_ref()
    }

    /// Sets the object's properties.
    pub fn set_properties(&mut self, props: Option<Rc<BvhProperties>>) {
        self.properties = props;
    }

    /// Check if object state is outdated.
    pub fn is_dirty(&self) -> bool {
        *self.is_dirty.borrow()
    }

    /// Mark object state as outdated (needs BVH rebuilding).
    pub fn mark_dirty(&self) {
        *self.is_dirty.borrow_mut() = true;
    }
}

impl Default for BvhObjectTransient {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────────────────────
// BVH_Object<T, N>
// ────────────────────────────────────────────────────────────────────────────

/// Abstract geometric object bounded by a bounding volume hierarchy box.
/// Generic over numeric type `T` and dimension `N`.
///
/// Implementors must provide a `Box()` method returning the axis-aligned
/// bounding box of the geometric object.
// occt: BVH_Object
pub trait BvhObject<T: Copy + PartialOrd, const N: usize> {
    /// Returns the axis-aligned bounding box of this geometric object.
    fn box_3d(&self) -> BvhBox<T, N>;

    /// Inherited from `BvhObjectTransient`: check if object state is outdated.
    fn is_dirty(&self) -> bool;

    /// Inherited from `BvhObjectTransient`: mark object state as outdated.
    fn mark_dirty(&self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bvh_object_transient_creation() {
        let obj = BvhObjectTransient::new();
        assert!(!obj.is_dirty());
    }

    #[test]
    fn test_bvh_object_transient_mark_dirty() {
        let obj = BvhObjectTransient::new();
        obj.mark_dirty();
        assert!(obj.is_dirty());
    }

    #[test]
    fn test_bvh_object_transient_properties() {
        let mut obj = BvhObjectTransient::new();
        assert!(obj.properties().is_none());

        let props = Rc::new(BvhProperties);
        obj.set_properties(Some(props.clone()));
        assert!(obj.properties().is_some());
    }
}

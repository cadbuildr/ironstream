// FILE: bvh_geometry.rs
//! Port of OCCT's `BVH_Geometry` — collection of abstract geometric objects
//! organized with a bounding volume hierarchy (BVH).
//!
//! Classes implemented:
//! - `BvhGeometry<T, N>` — templated geometry container with lazy BVH building

use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;

// ────────────────────────────────────────────────────────────────────────────
// Minimal supporting types for standalone operation
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

    /// Check if this box is valid (min <= max in all dimensions).
    pub fn is_valid(&self) -> bool {
        // A box is invalid if min > max in any dimension
        for i in 0..N {
            if self.min[i] > self.max[i] {
                return false;
            }
        }
        true
    }
}

impl<T: Copy + PartialOrd + Default, const N: usize> Default for BvhBox<T, N> {
    fn default() -> Self {
        BvhBox::new(VecN::new(T::default()), VecN::new(T::default()))
    }
}

/// An abstract BVH tree storing hierarchical bounds.
/// In a full implementation, this would contain node data, leaf data, etc.
pub struct BvhTree<T: Copy + PartialOrd, const N: usize> {
    _phantom: PhantomData<(T, [(); N])>,
}

impl<T: Copy + PartialOrd, const N: usize> BvhTree<T, N> {
    /// Create a new empty BVH tree.
    pub fn new() -> Self {
        BvhTree {
            _phantom: PhantomData,
        }
    }
}

impl<T: Copy + PartialOrd, const N: usize> Default for BvhTree<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Abstract base for a BVH builder.
/// Implementors implement the `build()` method.
pub trait BvhBuilder<T: Copy + PartialOrd, const N: usize> {
    /// Build the BVH tree from a set of objects.
    fn build(&self, tree: &mut BvhTree<T, N>, root_box: &BvhBox<T, N>);
}

/// Default binned SAH-based builder (simplified).
pub struct BvhBinnedBuilder<T: Copy + PartialOrd, const N: usize>(PhantomData<T>);

impl<T: Copy + PartialOrd, const N: usize> BvhBuilder<T, N> for BvhBinnedBuilder<T, N> {
    fn build(&self, _tree: &mut BvhTree<T, N>, _root_box: &BvhBox<T, N>) {
        // Minimal implementation: in a full implementation, this would
        // perform surface area heuristic splitting and build the tree.
    }
}

// ────────────────────────────────────────────────────────────────────────────
// BVH_Geometry<T, N>
// ────────────────────────────────────────────────────────────────────────────

/// A collection of abstract geometric objects organized with a bounding
/// volume hierarchy (BVH).
///
/// The geometry is lazily built: the BVH tree is constructed on first access
/// if the dirty flag is set. This class extends an object set, storing the
/// dirty flag, the BVH tree, and the builder strategy.
// occt: BVH_Geometry
pub struct BvhGeometry<T: Copy + Default + PartialOrd, const N: usize> {
    /// Is the geometry state outdated and needs rebuilding?
    is_dirty: RefCell<bool>,

    /// The constructed BVH tree.
    bvh: RefCell<BvhTree<T, N>>,

    /// The method (builder) used to construct the BVH.
    builder: Rc<dyn BvhBuilder<T, N>>,

    /// Cached bounding box of all geometric objects.
    cached_box: RefCell<BvhBox<T, N>>,
}

impl<T: Copy + Default + PartialOrd + 'static, const N: usize> BvhGeometry<T, N> {
    /// Create a new uninitialized BVH geometry with the default builder.
    pub fn new() -> Self {
        BvhGeometry {
            is_dirty: RefCell::new(false),
            bvh: RefCell::new(BvhTree::new()),
            builder: Rc::new(BvhBinnedBuilder::<T, N>(PhantomData)),
            cached_box: RefCell::new(BvhBox::default()),
        }
    }

    /// Create a new BVH geometry with a custom builder.
    pub fn with_builder(builder: Rc<dyn BvhBuilder<T, N>>) -> Self {
        BvhGeometry {
            is_dirty: RefCell::new(false),
            bvh: RefCell::new(BvhTree::new()),
            builder,
            cached_box: RefCell::new(BvhBox::default()),
        }
    }

    /// Check if the geometry state is outdated.
    ///
    /// Returns `true` if the geometry state should be updated (i.e., the BVH
    /// tree needs to be rebuilt).
    pub fn is_dirty(&self) -> bool {
        *self.is_dirty.borrow()
    }

    /// Mark the geometry as outdated.
    ///
    /// This sets the dirty flag, indicating that the BVH tree needs to be
    /// rebuilt before the next access.
    pub fn mark_dirty(&self) {
        *self.is_dirty.borrow_mut() = true;
    }

    /// Returns the bounding box of the whole geometry.
    ///
    /// If the geometry is marked dirty, this recomputes the box from the
    /// objects. (In a full implementation, this would delegate to the object
    /// set's `Box()` method.)
    pub fn box_3d(&self) -> BvhBox<T, N> {
        if *self.is_dirty.borrow() {
            let box_val = BvhBox::default();
            *self.cached_box.borrow_mut() = box_val;
        }
        *self.cached_box.borrow()
    }

    /// Returns the BVH tree, building it if necessary.
    ///
    /// If the geometry is marked dirty, this calls `update()` to rebuild
    /// the tree before returning it.
    pub fn bvh(&self) -> &BvhTree<T, N> {
        if *self.is_dirty.borrow() {
            self.update();
        }
        unsafe {
            // Safety: We just updated, so the bvh is valid for the duration
            // of this call. In real code, we'd use a proper cell pattern.
            &*(&*self.bvh.borrow() as *const BvhTree<T, N>)
        }
    }

    /// Returns the builder method used to construct the BVH.
    pub fn builder(&self) -> &Rc<dyn BvhBuilder<T, N>> {
        &self.builder
    }

    /// Sets the method (builder) used to construct the BVH.
    pub fn set_builder(&mut self, builder: Rc<dyn BvhBuilder<T, N>>) {
        self.builder = builder;
    }

    /// Internal: updates the geometry state by rebuilding the BVH tree.
    fn update(&self) {
        if *self.is_dirty.borrow() {
            let root_box = self.box_3d();
            let mut bvh_mut = self.bvh.borrow_mut();
            self.builder.build(&mut *bvh_mut, &root_box);
            drop(bvh_mut);
            *self.is_dirty.borrow_mut() = false;
        }
    }
}

impl<T: Copy + Default + PartialOrd + 'static, const N: usize> Default for BvhGeometry<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bvh_geometry_creation() {
        let geom = BvhGeometry::<f64, 3>::new();
        assert!(!geom.is_dirty());
    }

    #[test]
    fn test_bvh_geometry_mark_dirty() {
        let geom = BvhGeometry::<f64, 3>::new();
        geom.mark_dirty();
        assert!(geom.is_dirty());
    }

    #[test]
    fn test_bvh_geometry_default_box() {
        let geom = BvhGeometry::<f64, 3>::new();
        let b = geom.box_3d();
        // Default box should have default values
        assert_eq!(b.min().get(0), 0.0);
        assert_eq!(b.max().get(0), 0.0);
    }

    #[test]
    fn test_bvh_geometry_builder() {
        let geom = BvhGeometry::<f64, 3>::new();
        // Builder should be set to default (binned builder)
        let _ = geom.builder();
    }

    #[test]
    fn test_box_validity() {
        let min = VecN::<f64, 3>::new(0.0);
        let max = VecN::<f64, 3>::new(1.0);
        let b = BvhBox::new(min, max);
        assert!(b.is_valid());
    }
}

// FILE: extrema_hub_tree_of_sphere.rs
// occt: Extrema_HUBTreeOfSphere

use std::rc::Rc;
use std::cell::RefCell;

/// A sphere (placeholder).
#[derive(Clone, Debug)]
pub struct Sphere {}

pub type SphereHandle = Rc<RefCell<Sphere>>;

/// Deprecated: BVH tree for spheres.
/// Use a direct spatial data structure instead.
#[derive(Clone, Debug)]
pub struct HUBTreeOfSphere {
    /// List of spheres in the tree (flat for simplicity).
    spheres: Vec<SphereHandle>,
}

impl HUBTreeOfSphere {
    /// Create an empty BVH tree.
    pub fn new() -> Self {
        HUBTreeOfSphere {
            spheres: Vec::new(),
        }
    }

    /// Add a sphere to the tree.
    pub fn add(&mut self, sphere: SphereHandle) {
        self.spheres.push(sphere);
    }

    /// Get all spheres in the tree.
    pub fn spheres(&self) -> &[SphereHandle] {
        &self.spheres
    }

    /// Get the number of spheres.
    pub fn len(&self) -> usize {
        self.spheres.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.spheres.is_empty()
    }
}

impl Default for HUBTreeOfSphere {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_creation() {
        let tree = HUBTreeOfSphere::new();
        assert!(tree.is_empty());
    }

    #[test]
    fn test_tree_add_sphere() {
        let mut tree = HUBTreeOfSphere::new();
        let sphere = Rc::new(RefCell::new(Sphere {}));
        tree.add(sphere);
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_tree_spheres() {
        let mut tree = HUBTreeOfSphere::new();
        let s1 = Rc::new(RefCell::new(Sphere {}));
        let s2 = Rc::new(RefCell::new(Sphere {}));
        tree.add(s1);
        tree.add(s2);
        assert_eq!(tree.spheres().len(), 2);
    }
}

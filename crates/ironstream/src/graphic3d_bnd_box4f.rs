// FILE: graphic3d_bnd_box4f.rs
// occt: Graphic3d_BndBox4f

//! 4D Axis-aligned bounding box using single precision floats.
//!
//! This is a typedef for BVH_Box<float, 4>, representing a 4-dimensional
//! bounding box for AABB (Axis-Aligned Bounding Box) representation in rendering.

/// A 4-dimensional bounding box using single precision.
/// Stores minimum and maximum coordinates for 4 dimensions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BndBox4f {
    min: [f32; 4],
    max: [f32; 4],
}

impl BndBox4f {
    /// Creates an empty bounding box.
    pub fn new() -> Self {
        BndBox4f {
            min: [f32::INFINITY; 4],
            max: [f32::NEG_INFINITY; 4],
        }
    }

    /// Creates a bounding box with specified min and max coordinates.
    pub fn with_bounds(
        min_x: f32,
        min_y: f32,
        min_z: f32,
        min_w: f32,
        max_x: f32,
        max_y: f32,
        max_z: f32,
        max_w: f32,
    ) -> Self {
        BndBox4f {
            min: [min_x, min_y, min_z, min_w],
            max: [max_x, max_y, max_z, max_w],
        }
    }

    /// Returns the minimum coordinates.
    pub fn min(&self) -> &[f32; 4] {
        &self.min
    }

    /// Returns the maximum coordinates.
    pub fn max(&self) -> &[f32; 4] {
        &self.max
    }

    /// Returns the minimum coordinate for the given axis (0-3).
    pub fn corner_min(&self, axis: usize) -> f32 {
        if axis < 4 {
            self.min[axis]
        } else {
            f32::NAN
        }
    }

    /// Returns the maximum coordinate for the given axis (0-3).
    pub fn corner_max(&self, axis: usize) -> f32 {
        if axis < 4 {
            self.max[axis]
        } else {
            f32::NAN
        }
    }

    /// Returns true if the bounding box is empty (not initialized).
    pub fn is_empty(&self) -> bool {
        self.min[0] > self.max[0]
            || self.min[1] > self.max[1]
            || self.min[2] > self.max[2]
            || self.min[3] > self.max[3]
    }

    /// Expands the bounding box to include a point.
    pub fn add_point(&mut self, x: f32, y: f32, z: f32, w: f32) {
        self.min[0] = self.min[0].min(x);
        self.min[1] = self.min[1].min(y);
        self.min[2] = self.min[2].min(z);
        self.min[3] = self.min[3].min(w);
        self.max[0] = self.max[0].max(x);
        self.max[1] = self.max[1].max(y);
        self.max[2] = self.max[2].max(z);
        self.max[3] = self.max[3].max(w);
    }

    /// Expands the bounding box to include another bounding box.
    pub fn add_box(&mut self, other: &BndBox4f) {
        if !other.is_empty() {
            for i in 0..4 {
                self.min[i] = self.min[i].min(other.min[i]);
                self.max[i] = self.max[i].max(other.max[i]);
            }
        }
    }

    /// Returns the size (extent) along a specific axis.
    pub fn size(&self, axis: usize) -> f32 {
        if axis < 4 && !self.is_empty() {
            self.max[axis] - self.min[axis]
        } else {
            0.0
        }
    }

    /// Returns the center coordinate for a given axis.
    pub fn center(&self, axis: usize) -> f32 {
        if axis < 4 && !self.is_empty() {
            (self.min[axis] + self.max[axis]) * 0.5
        } else {
            0.0
        }
    }
}

impl Default for BndBox4f {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bnd_box4f_new() {
        let bbox = BndBox4f::new();
        assert!(bbox.is_empty());
    }

    #[test]
    fn test_bnd_box4f_with_bounds() {
        let bbox = BndBox4f::with_bounds(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert!(!bbox.is_empty());
        assert_eq!(bbox.corner_min(0), 0.0);
        assert_eq!(bbox.corner_max(0), 4.0);
        assert_eq!(bbox.corner_min(3), 3.0);
        assert_eq!(bbox.corner_max(3), 7.0);
    }

    #[test]
    fn test_bnd_box4f_add_point() {
        let mut bbox = BndBox4f::new();
        bbox.add_point(1.0, 2.0, 3.0, 4.0);
        assert!(!bbox.is_empty());
        assert_eq!(bbox.corner_min(0), 1.0);
        assert_eq!(bbox.corner_max(0), 1.0);
        assert_eq!(bbox.corner_min(3), 4.0);
    }

    #[test]
    fn test_bnd_box4f_add_box() {
        let mut bbox1 = BndBox4f::with_bounds(0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0);
        let bbox2 = BndBox4f::with_bounds(0.5, 0.5, 0.5, 0.5, 2.0, 2.0, 2.0, 2.0);
        bbox1.add_box(&bbox2);
        assert_eq!(bbox1.corner_min(0), 0.0);
        assert_eq!(bbox1.corner_max(0), 2.0);
    }

    #[test]
    fn test_bnd_box4f_size() {
        let bbox = BndBox4f::with_bounds(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq!(bbox.size(0), 4.0);
        assert_eq!(bbox.size(1), 4.0);
        assert_eq!(bbox.size(2), 4.0);
        assert_eq!(bbox.size(3), 4.0);
    }

    #[test]
    fn test_bnd_box4f_center() {
        let bbox = BndBox4f::with_bounds(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq!(bbox.center(0), 2.0);
        assert_eq!(bbox.center(1), 3.0);
        assert_eq!(bbox.center(2), 4.0);
        assert_eq!(bbox.center(3), 5.0);
    }

    #[test]
    fn test_bnd_box4f_invalid_axis() {
        let bbox = BndBox4f::with_bounds(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert!(bbox.corner_min(4).is_nan());
        assert!(bbox.corner_max(10).is_nan());
    }
}

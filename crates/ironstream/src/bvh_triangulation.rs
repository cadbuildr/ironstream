// FILE: bvh_triangulation.rs
//! Triangulation as an example of BVH primitive set.
//! Ported from OpenCascade BVH_Triangulation template class.

// occt: BVH_Triangulation

use std::cmp::Ordering;

/// A generic N-dimensional vector type
type VecN<T, const N: usize> = [T; N];

/// A 4-element integer vector for triangle indices
#[derive(Debug, Clone, Copy)]
struct Vec4i(i32, i32, i32, i32);

impl Vec4i {
    fn x(&self) -> i32 {
        self.0
    }
    fn y(&self) -> i32 {
        self.1
    }
    fn z(&self) -> i32 {
        self.2
    }
    fn w(&self) -> i32 {
        self.3
    }
}

/// A 3-dimensional box bounding vertices
#[derive(Debug, Clone, Copy)]
struct Box<T> {
    min: [T; 3],
    max: [T; 3],
}

impl<T: Copy + PartialOrd> Box<T> {
    fn new(min: [T; 3], max: [T; 3]) -> Self {
        Box { min, max }
    }

    fn corner_min(&self) -> [T; 3] {
        self.min
    }

    fn corner_max(&self) -> [T; 3] {
        self.max
    }
}

/// Triangulation storing vertices and element indices
///
/// Generic over numeric type T and dimension N.
/// Provides methods to compute bounding boxes and centroids for triangles.
#[derive(Debug, Clone)]
pub struct BvhTriangulation<T, const N: usize> {
    /// Array of vertex coordinates (N-dimensional vectors)
    pub vertices: std::vec::Vec<VecN<T, N>>,
    /// Array of triangle vertex indices (4 integers per triangle: i0, i1, i2, unused)
    pub elements: std::vec::Vec<Vec4i>,
}

impl<T: Default + Copy, const N: usize> BvhTriangulation<T, N> {
    /// Creates an empty triangulation
    pub fn new() -> Self {
        BvhTriangulation {
            vertices: std::vec::Vec::new(),
            elements: std::vec::Vec::new(),
        }
    }

    /// Returns the total number of triangles
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    /// Appends a vertex to the triangulation
    pub fn append_vertex(&mut self, vertex: VecN<T, N>) {
        self.vertices.push(vertex);
    }

    /// Appends a triangle element (4 indices: i0, i1, i2, unused)
    pub fn append_element(&mut self, element: Vec4i) {
        self.elements.push(element);
    }

    /// Swaps two triangles in the set
    pub fn swap(&mut self, index1: usize, index2: usize) {
        self.elements.swap(index1, index2);
    }
}

impl BvhTriangulation<f64, 3> {
    /// Returns the axis-aligned bounding box of the given triangle (3D, f64)
    pub fn box_3d(&self, triangle_index: usize) -> Box<f64> {
        let indices = self.elements[triangle_index];
        let p0 = self.vertices[indices.x() as usize];
        let p1 = self.vertices[indices.y() as usize];
        let p2 = self.vertices[indices.z() as usize];

        let mut min = p0;
        let mut max = p0;

        // Component-wise min
        for k in 0..3 {
            if p1[k] < min[k] {
                min[k] = p1[k];
            }
            if p2[k] < min[k] {
                min[k] = p2[k];
            }
        }

        // Component-wise max
        for k in 0..3 {
            if p1[k] > max[k] {
                max[k] = p1[k];
            }
            if p2[k] > max[k] {
                max[k] = p2[k];
            }
        }

        Box::new(min, max)
    }

    /// Returns centroid position along the given axis (0=X, 1=Y, 2=Z)
    pub fn center(&self, triangle_index: usize, axis: usize) -> f64 {
        let indices = self.elements[triangle_index];
        let p0 = self.vertices[indices.x() as usize];
        let p1 = self.vertices[indices.y() as usize];
        let p2 = self.vertices[indices.z() as usize];

        (p0[axis] + p1[axis] + p2[axis]) * (1.0 / 3.0)
    }
}

impl BvhTriangulation<f32, 3> {
    /// Returns the axis-aligned bounding box of the given triangle (3D, f32)
    pub fn box_3d(&self, triangle_index: usize) -> Box<f32> {
        let indices = self.elements[triangle_index];
        let p0 = self.vertices[indices.x() as usize];
        let p1 = self.vertices[indices.y() as usize];
        let p2 = self.vertices[indices.z() as usize];

        let mut min = p0;
        let mut max = p0;

        // Component-wise min
        for k in 0..3 {
            if p1[k] < min[k] {
                min[k] = p1[k];
            }
            if p2[k] < min[k] {
                min[k] = p2[k];
            }
        }

        // Component-wise max
        for k in 0..3 {
            if p1[k] > max[k] {
                max[k] = p1[k];
            }
            if p2[k] > max[k] {
                max[k] = p2[k];
            }
        }

        Box::new(min, max)
    }

    /// Returns centroid position along the given axis
    pub fn center(&self, triangle_index: usize, axis: usize) -> f32 {
        let indices = self.elements[triangle_index];
        let p0 = self.vertices[indices.x() as usize];
        let p1 = self.vertices[indices.y() as usize];
        let p2 = self.vertices[indices.z() as usize];

        (p0[axis] + p1[axis] + p2[axis]) * (1.0 / 3.0)
    }
}

impl<T: Default + Copy, const N: usize> Default for BvhTriangulation<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_constructor() {
        let tri: BvhTriangulation<f64, 3> = BvhTriangulation::new();
        assert_eq!(tri.size(), 0);
        assert_eq!(tri.vertices.len(), 0);
        assert_eq!(tri.elements.len(), 0);
    }

    #[test]
    fn test_add_single_triangle() {
        let mut tri: BvhTriangulation<f64, 3> = BvhTriangulation::new();

        tri.append_vertex([0.0, 0.0, 0.0]);
        tri.append_vertex([1.0, 0.0, 0.0]);
        tri.append_vertex([0.0, 1.0, 0.0]);
        tri.append_element(Vec4i(0, 1, 2, 0));

        assert_eq!(tri.size(), 1);
        assert_eq!(tri.vertices.len(), 3);
    }

    #[test]
    fn test_add_multiple_triangles() {
        let mut tri: BvhTriangulation<f64, 3> = BvhTriangulation::new();

        for i in 0..5 {
            let x = i as f64;
            tri.append_vertex([x, 0.0, 0.0]);
            tri.append_vertex([x + 1.0, 0.0, 0.0]);
            tri.append_vertex([x + 0.5, 1.0, 0.0]);
            tri.append_element(Vec4i(i * 3, i * 3 + 1, i * 3 + 2, 0));
        }

        assert_eq!(tri.size(), 5);
        assert_eq!(tri.vertices.len(), 15);
    }

    #[test]
    fn test_box_for_single_triangle() {
        let mut tri: BvhTriangulation<f64, 3> = BvhTriangulation::new();

        tri.append_vertex([1.0, 2.0, 3.0]);
        tri.append_vertex([4.0, 1.0, 2.0]);
        tri.append_vertex([2.0, 3.0, 1.0]);
        tri.append_element(Vec4i(0, 1, 2, 0));

        let bbox = tri.box_3d(0);

        assert!((bbox.corner_min()[0] - 1.0).abs() < 1e-10);
        assert!((bbox.corner_min()[1] - 1.0).abs() < 1e-10);
        assert!((bbox.corner_min()[2] - 1.0).abs() < 1e-10);

        assert!((bbox.corner_max()[0] - 4.0).abs() < 1e-10);
        assert!((bbox.corner_max()[1] - 3.0).abs() < 1e-10);
        assert!((bbox.corner_max()[2] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_box_for_degenerate_triangle() {
        let mut tri: BvhTriangulation<f64, 3> = BvhTriangulation::new();

        // All vertices at the same point
        tri.append_vertex([1.0, 1.0, 1.0]);
        tri.append_vertex([1.0, 1.0, 1.0]);
        tri.append_vertex([1.0, 1.0, 1.0]);
        tri.append_element(Vec4i(0, 1, 2, 0));

        let bbox = tri.box_3d(0);

        assert!((bbox.corner_min()[0] - 1.0).abs() < 1e-10);
        assert!((bbox.corner_min()[1] - 1.0).abs() < 1e-10);
        assert!((bbox.corner_min()[2] - 1.0).abs() < 1e-10);
        assert!((bbox.corner_max()[0] - 1.0).abs() < 1e-10);
        assert!((bbox.corner_max()[1] - 1.0).abs() < 1e-10);
        assert!((bbox.corner_max()[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_box_for_flat_triangle() {
        let mut tri: BvhTriangulation<f64, 3> = BvhTriangulation::new();

        tri.append_vertex([0.0, 0.0, 0.0]);
        tri.append_vertex([2.0, 0.0, 0.0]);
        tri.append_vertex([1.0, 3.0, 0.0]);
        tri.append_element(Vec4i(0, 1, 2, 0));

        let bbox = tri.box_3d(0);

        assert!((bbox.corner_min()[2] - 0.0).abs() < 1e-10);
        assert!((bbox.corner_max()[2] - 0.0).abs() < 1e-10);
        assert!((bbox.corner_max()[0] - 2.0).abs() < 1e-10);
        assert!((bbox.corner_max()[1] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_center_computation() {
        let mut tri: BvhTriangulation<f64, 3> = BvhTriangulation::new();

        tri.append_vertex([0.0, 0.0, 0.0]);
        tri.append_vertex([3.0, 0.0, 0.0]);
        tri.append_vertex([0.0, 3.0, 0.0]);
        tri.append_element(Vec4i(0, 1, 2, 0));

        assert!((tri.center(0, 0) - 1.0).abs() < 1e-10);
        assert!((tri.center(0, 1) - 1.0).abs() < 1e-10);
        assert!((tri.center(0, 2) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_center_along_x_axis() {
        let mut tri: BvhTriangulation<f64, 3> = BvhTriangulation::new();

        tri.append_vertex([1.0, 0.0, 0.0]);
        tri.append_vertex([2.0, 0.0, 0.0]);
        tri.append_vertex([3.0, 0.0, 0.0]);
        tri.append_element(Vec4i(0, 1, 2, 0));

        assert!((tri.center(0, 0) - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_center_along_y_axis() {
        let mut tri: BvhTriangulation<f64, 3> = BvhTriangulation::new();

        tri.append_vertex([0.0, 2.0, 0.0]);
        tri.append_vertex([0.0, 4.0, 0.0]);
        tri.append_vertex([0.0, 6.0, 0.0]);
        tri.append_element(Vec4i(0, 1, 2, 0));

        assert!((tri.center(0, 1) - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_center_along_z_axis() {
        let mut tri: BvhTriangulation<f64, 3> = BvhTriangulation::new();

        tri.append_vertex([0.0, 0.0, 1.0]);
        tri.append_vertex([0.0, 0.0, 4.0]);
        tri.append_vertex([0.0, 0.0, 7.0]);
        tri.append_element(Vec4i(0, 1, 2, 0));

        assert!((tri.center(0, 2) - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_swap_two_triangles() {
        let mut tri: BvhTriangulation<f64, 3> = BvhTriangulation::new();

        // Triangle 0
        tri.append_vertex([0.0, 0.0, 0.0]);
        tri.append_vertex([1.0, 0.0, 0.0]);
        tri.append_vertex([0.0, 1.0, 0.0]);
        tri.append_element(Vec4i(0, 1, 2, 0));

        // Triangle 1
        tri.append_vertex([5.0, 5.0, 5.0]);
        tri.append_vertex([6.0, 5.0, 5.0]);
        tri.append_vertex([5.0, 6.0, 5.0]);
        tri.append_element(Vec4i(3, 4, 5, 0));

        let centroid0_before = tri.center(0, 0);
        let centroid1_before = tri.center(1, 0);

        tri.swap(0, 1);

        assert!((tri.center(0, 0) - centroid1_before).abs() < 1e-10);
        assert!((tri.center(1, 0) - centroid0_before).abs() < 1e-10);
    }

    #[test]
    fn test_swap_preserves_vertices() {
        let mut tri: BvhTriangulation<f64, 3> = BvhTriangulation::new();

        for i in 0..3 {
            let x = i as f64 * 10.0;
            tri.append_vertex([x, 0.0, 0.0]);
            tri.append_vertex([x + 1.0, 0.0, 0.0]);
            tri.append_vertex([x + 0.5, 1.0, 0.0]);
            tri.append_element(Vec4i(i * 3, i * 3 + 1, i * 3 + 2, 0));
        }

        let vertex_count = tri.vertices.len();
        tri.swap(0, 2);

        assert_eq!(tri.vertices.len(), vertex_count);
    }

    #[test]
    fn test_triangulation_2d() {
        let mut tri: BvhTriangulation<f64, 2> = BvhTriangulation::new();

        tri.append_vertex([0.0, 0.0]);
        tri.append_vertex([1.0, 0.0]);
        tri.append_vertex([0.5, 1.0]);
        tri.append_element(Vec4i(0, 1, 2, 0));

        assert_eq!(tri.size(), 1);
    }

    #[test]
    fn test_shared_vertices() {
        let mut tri: BvhTriangulation<f64, 3> = BvhTriangulation::new();

        tri.append_vertex([0.0, 0.0, 0.0]);
        tri.append_vertex([1.0, 0.0, 0.0]);
        tri.append_vertex([1.0, 1.0, 0.0]);
        tri.append_vertex([0.0, 1.0, 0.0]);

        tri.append_element(Vec4i(0, 1, 2, 0));
        tri.append_element(Vec4i(0, 2, 3, 0));

        assert_eq!(tri.size(), 2);
        assert_eq!(tri.vertices.len(), 4);
    }

    #[test]
    fn test_float_precision() {
        let mut tri: BvhTriangulation<f32, 3> = BvhTriangulation::new();

        tri.append_vertex([0.0f32, 0.0f32, 0.0f32]);
        tri.append_vertex([1.0f32, 0.0f32, 0.0f32]);
        tri.append_vertex([0.0f32, 1.0f32, 0.0f32]);
        tri.append_element(Vec4i(0, 1, 2, 0));

        assert_eq!(tri.size(), 1);

        let bbox = tri.box_3d(0);
        assert!((bbox.corner_max()[0] - 1.0f32).abs() < 1e-6f32);
        assert!((bbox.corner_max()[1] - 1.0f32).abs() < 1e-6f32);
    }

    #[test]
    fn test_negative_coordinates() {
        let mut tri: BvhTriangulation<f64, 3> = BvhTriangulation::new();

        tri.append_vertex([-1.0, -2.0, -3.0]);
        tri.append_vertex([1.0, -1.0, -2.0]);
        tri.append_vertex([0.0, 0.0, -1.0]);
        tri.append_element(Vec4i(0, 1, 2, 0));

        let bbox = tri.box_3d(0);

        assert!((bbox.corner_min()[0] - (-1.0)).abs() < 1e-10);
        assert!((bbox.corner_min()[1] - (-2.0)).abs() < 1e-10);
        assert!((bbox.corner_min()[2] - (-3.0)).abs() < 1e-10);
        assert!((bbox.corner_max()[0] - 1.0).abs() < 1e-10);
        assert!((bbox.corner_max()[1] - 0.0).abs() < 1e-10);
        assert!((bbox.corner_max()[2] - (-1.0)).abs() < 1e-10);
    }
}

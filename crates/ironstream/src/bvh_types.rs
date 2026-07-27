// FILE: bvh_types.rs
// occt: BVH_Types // - Namespace and type aliases for BVH operations

//! Tool classes for selecting appropriate vector types and providing operations on arrays.
//! This module provides trait-like structures to handle 1D, 2D, 3D, and 4D vectors and matrices,
//! along with utilities for vector component access and array operations.

use std::ops::{Add, Sub, Mul, Index, IndexMut};

/// Represents 2D vector of scalar values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Clone + Copy> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub fn get_data(&self) -> [T; 2] {
        [self.x, self.y]
    }
}

impl<T: Clone + Copy> Index<usize> for Vec2<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// Represents 3D vector of scalar values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Clone + Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }

    pub fn get_data(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    pub fn change_data(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.x, &mut self.y, &mut self.z)
    }
}

impl<T: Clone + Copy> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T: Clone + Copy> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// Represents 4D vector of scalar values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Clone + Copy> Vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Vec4 { x, y, z, w }
    }

    pub fn get_data(&self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl<T: Clone + Copy> Index<usize> for Vec4<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds"),
        }
    }
}

// Vector arithmetic implementations
impl<T: Add<Output = T> + Clone + Copy> Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, other: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: Add<Output = T> + Clone + Copy> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, other: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T: Add<Output = T> + Clone + Copy> Add for Vec4<T> {
    type Output = Vec4<T>;

    fn add(self, other: Vec4<T>) -> Vec4<T> {
        Vec4::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl<T: Sub<Output = T> + Clone + Copy> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, other: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T: Mul<Output = T> + Clone + Copy> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, scalar: T) -> Vec3<T> {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

// occt-note: VectorType - Trait-like struct for selecting vector types by dimension
/// Selects the appropriate vector type based on dimension.
/// For N=1, uses scalar T. For N=2, uses Vec2<T>. For N=3, uses Vec3<T>. For N=4, uses Vec4<T>.
pub struct VectorType;

// occt-note: VecComp - Tool class for accessing vector components by index
/// Provides component access for vectors by index.
pub struct VecComp;

impl VecComp {
    /// Gets a component from a 2D vector by axis index (0=x, 1=y).
    pub fn get_2d<T: Clone + Copy>(vec: &Vec2<T>, axis: usize) -> T {
        match axis {
            0 => vec.x,
            1 => vec.y,
            _ => panic!("Axis out of bounds for 2D vector"),
        }
    }

    /// Gets a component from a 3D vector by axis index (0=x, 1=y, 2=z).
    pub fn get_3d<T: Clone + Copy>(vec: &Vec3<T>, axis: usize) -> T {
        match axis {
            0 => vec.x,
            1 => vec.y,
            2 => vec.z,
            _ => panic!("Axis out of bounds for 3D vector"),
        }
    }

    /// Gets a component from a 4D vector by axis index (0=x, 1=y, 2=z, 3=w).
    pub fn get_4d<T: Clone + Copy>(vec: &Vec4<T>, axis: usize) -> T {
        match axis {
            0 => vec.x,
            1 => vec.y,
            2 => vec.z,
            3 => vec.w,
            _ => panic!("Axis out of bounds for 4D vector"),
        }
    }
}

// occt-note: Array - Utility struct providing operations on arrays of vectors
/// Provides array operations for storing and manipulating collections of vectors.
pub struct Array;

impl Array {
    /// Returns value at the given index. Wrapper for Vec<T>::get().
    pub fn value<T: Clone>(array: &[T], index: usize) -> T {
        array[index].clone()
    }

    /// Returns reference to the value at the given index.
    pub fn value_ref<T>(array: &[T], index: usize) -> &T {
        &array[index]
    }

    /// Returns mutable reference to the value at the given index.
    pub fn value_mut<T>(array: &mut [T], index: usize) -> &mut T {
        &mut array[index]
    }

    /// Returns the number of elements in the array.
    pub fn size<T>(array: &[T]) -> usize {
        array.len()
    }

    /// Clears the array.
    pub fn clear<T>(array: &mut Vec<T>) {
        array.clear();
    }

    /// Reserves capacity for at least the given number of elements.
    pub fn reserve<T>(array: &mut Vec<T>, count: usize) {
        array.reserve(count);
    }
}

// occt-note: IntFloor - Utility function for integer floor operation
/// Computes integer floor of a floating point value.
/// Returns the largest integer less than or equal to the input value.
pub fn int_floor_f64(value: f64) -> i32 {
    let res = value as i32;
    if (res as f64) > value {
        res - 1
    } else {
        res
    }
}

pub fn int_floor_f32(value: f32) -> i32 {
    let res = value as i32;
    if (res as f32) > value {
        res - 1
    } else {
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_creation() {
        let v = Vec2::new(1.0f64, 2.0f64);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
    }

    #[test]
    fn test_vec3_creation() {
        let v = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vec3_indexing() {
        let v = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    fn test_vec3_addition() {
        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = Vec3::new(4.0f64, 5.0f64, 6.0f64);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 5.0);
        assert_eq!(v3.y, 7.0);
        assert_eq!(v3.z, 9.0);
    }

    #[test]
    fn test_vec3_subtraction() {
        let v1 = Vec3::new(4.0f64, 5.0f64, 6.0f64);
        let v2 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v3 = v1 - v2;
        assert_eq!(v3.x, 3.0);
        assert_eq!(v3.y, 3.0);
        assert_eq!(v3.z, 3.0);
    }

    #[test]
    fn test_vec3_scalar_multiply() {
        let v1 = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        let v2 = v1 * 2.0f64;
        assert_eq!(v2.x, 2.0);
        assert_eq!(v2.y, 4.0);
        assert_eq!(v2.z, 6.0);
    }

    #[test]
    fn test_vec_comp_get_3d() {
        let v = Vec3::new(1.0f64, 2.0f64, 3.0f64);
        assert_eq!(VecComp::get_3d(&v, 0), 1.0);
        assert_eq!(VecComp::get_3d(&v, 1), 2.0);
        assert_eq!(VecComp::get_3d(&v, 2), 3.0);
    }

    #[test]
    fn test_array_size() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(Array::size(&arr), 5);
    }
}

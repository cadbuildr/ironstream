// FILE: int_polyh_array_of_triangles.rs
// occt: IntPolyh_ArrayOfTriangles

use crate::int_polyh_triangle::Triangle;

/// Dynamic array of triangles.
/// Represents a growable array using a dynamic allocation strategy.
#[derive(Clone, Debug)]
pub struct ArrayOfTriangles {
    data: Vec<Triangle>,
    nb_allocated: i32,
    nb_items: i32,
    increment: i32,
}

impl ArrayOfTriangles {
    /// Create a new array with optional increment size
    pub fn new(increment: i32) -> Self {
        ArrayOfTriangles {
            data: Vec::new(),
            nb_allocated: 0,
            nb_items: 0,
            increment: if increment > 0 { increment } else { 256 },
        }
    }

    /// Create an array with initial capacity
    pub fn with_capacity(capacity: i32, increment: i32) -> Self {
        let inc = if increment > 0 { increment } else { 256 };
        let cap = if capacity > 0 { capacity as usize } else { 0 };
        ArrayOfTriangles {
            data: vec![Triangle::new(); cap],
            nb_allocated: capacity,
            nb_items: 0,
            increment: inc,
        }
    }

    /// Initialize array with capacity for N items
    pub fn init(&mut self, n: i32) {
        let size = if n > 0 { n as usize } else { 0 };
        self.data.clear();
        self.data.resize(size, Triangle::new());
        self.nb_allocated = n;
    }

    /// Increment the number of items
    pub fn increment_nb_items(&mut self) {
        self.nb_items += 1;
        if self.nb_items >= self.nb_allocated {
            let new_cap = self.nb_allocated + self.increment;
            self.init(new_cap);
        }
    }

    /// Get number of allocated items
    pub fn get_n(&self) -> i32 {
        self.nb_allocated
    }

    /// Get number of stored items
    pub fn nb_items(&self) -> i32 {
        self.nb_items
    }

    /// Set number of stored items
    pub fn set_nb_items(&mut self, n: i32) {
        self.nb_items = n;
    }

    /// Get value at index
    pub fn value(&self, index: i32) -> Option<Triangle> {
        if index >= 0 && index < self.nb_allocated {
            Some(self.data[index as usize])
        } else {
            None
        }
    }

    /// Get mutable value at index
    pub fn change_value(&mut self, index: i32) -> Option<&mut Triangle> {
        if index >= 0 && index < self.nb_allocated {
            Some(&mut self.data[index as usize])
        } else {
            None
        }
    }

    /// Access via bracket operator (immutable)
    pub fn get(&self, index: i32) -> Option<Triangle> {
        self.value(index)
    }

    /// Access via bracket operator (mutable)
    pub fn get_mut(&mut self, index: i32) -> Option<&mut Triangle> {
        self.change_value(index)
    }
}

impl Default for ArrayOfTriangles {
    fn default() -> Self {
        Self::new(256)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let arr = ArrayOfTriangles::new(256);
        assert_eq!(arr.get_n(), 0);
        assert_eq!(arr.nb_items(), 0);
        assert_eq!(arr.increment, 256);
    }

    #[test]
    fn test_with_capacity() {
        let arr = ArrayOfTriangles::with_capacity(10, 128);
        assert_eq!(arr.get_n(), 10);
        assert_eq!(arr.nb_items(), 0);
        assert_eq!(arr.increment, 128);
    }

    #[test]
    fn test_init() {
        let mut arr = ArrayOfTriangles::new(256);
        arr.init(20);
        assert_eq!(arr.get_n(), 20);
        assert_eq!(arr.nb_items(), 0);
    }

    #[test]
    fn test_access() {
        let mut arr = ArrayOfTriangles::with_capacity(10, 256);
        if let Some(tri) = arr.get_mut(0) {
            tri.set_first_point(5);
        }
        if let Some(tri) = arr.get(0) {
            assert_eq!(tri.first_point(), 5);
        }
    }

    #[test]
    fn test_increment_nb_items() {
        let mut arr = ArrayOfTriangles::with_capacity(5, 10);
        arr.increment_nb_items();
        assert_eq!(arr.nb_items(), 1);
        arr.increment_nb_items();
        assert_eq!(arr.nb_items(), 2);
    }
}

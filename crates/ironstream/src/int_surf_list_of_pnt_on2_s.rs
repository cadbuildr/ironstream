// FILE: int_surf_list_of_pnt_on2_s.rs
// occt: IntSurf_ListOfPntOn2S

use std::vec::Vec;

/// Deprecated alias for a list of points on two surfaces.
#[derive(Clone, Debug)]
pub struct IntSurf_ListOfPntOn2S {
    points: Vec<([f64; 3], [f64; 3])>,
}

impl IntSurf_ListOfPntOn2S {
    /// Create a new list.
    pub fn new() -> Self {
        IntSurf_ListOfPntOn2S {
            points: Vec::new(),
        }
    }

    /// Add a point pair to the list.
    pub fn append(&mut self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) {
        self.points.push(([x1, y1, z1], [x2, y2, z2]));
    }

    /// Get the number of points.
    pub fn length(&self) -> usize {
        self.points.len()
    }

    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Get a point pair by index.
    pub fn point(&self, index: usize) -> Option<([f64; 3], [f64; 3])> {
        self.points.get(index).copied()
    }

    /// Clear the list.
    pub fn clear(&mut self) {
        self.points.clear();
    }
}

impl Default for IntSurf_ListOfPntOn2S {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list() {
        let list = IntSurf_ListOfPntOn2S::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_append() {
        let mut list = IntSurf_ListOfPntOn2S::new();
        list.append(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        assert_eq!(list.length(), 1);
        assert_eq!(list.point(0), Some(([0.0, 0.0, 0.0], [1.0, 1.0, 1.0])));
    }

    #[test]
    fn test_clear() {
        let mut list = IntSurf_ListOfPntOn2S::new();
        list.append(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        list.clear();
        assert!(list.is_empty());
    }
}

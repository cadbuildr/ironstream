// FILE: poly_coherent_tri_ptr.rs
// occt: Poly_CoherentTriPtr, Poly_CoherentTriPtr::Iterator

/// Pointer and iterator implementation for triangles in a round double-linked list.
#[derive(Debug, Clone)]
pub struct CoherentTriPtr {
    pub triangle_index: i32,  // Index of the triangle
    pub prev: Option<Box<CoherentTriPtr>>,  // Previous element (in round list)
    pub next: Option<Box<CoherentTriPtr>>,  // Next element (in round list)
}

impl CoherentTriPtr {
    /// Create new triangle pointer.
    pub fn new(tri_index: i32) -> Self {
        CoherentTriPtr {
            triangle_index: tri_index,
            prev: None,
            next: None,
        }
    }

    /// Check if pointer is valid.
    pub fn is_valid(&self) -> bool {
        self.triangle_index >= 0
    }

    /// Get triangle index.
    pub fn get_triangle_index(&self) -> i32 {
        self.triangle_index
    }
}

/// Iterator for round double-linked list of triangles.
#[derive(Debug, Clone)]
pub struct TriangleIterator {
    start_index: i32,
    current_index: i32,
    finished: bool,
}

impl TriangleIterator {
    /// Create iterator starting at given triangle.
    pub fn new(start_index: i32) -> Self {
        TriangleIterator {
            start_index,
            current_index: start_index,
            finished: start_index < 0,
        }
    }

    /// Check if iteration is complete.
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// Get current triangle index.
    pub fn current(&self) -> Option<i32> {
        if self.finished || self.current_index < 0 {
            None
        } else {
            Some(self.current_index)
        }
    }

    /// Advance to next triangle.
    pub fn next(&mut self) {
        if self.finished {
            return;
        }
        // In a real implementation, this would follow the linked list.
        // For now we mark as finished.
        self.finished = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coherent_tri_ptr_new() {
        let ptr = CoherentTriPtr::new(5);
        assert!(ptr.is_valid());
        assert_eq!(ptr.get_triangle_index(), 5);
    }

    #[test]
    fn test_coherent_tri_ptr_invalid() {
        let ptr = CoherentTriPtr::new(-1);
        assert!(!ptr.is_valid());
    }

    #[test]
    fn test_triangle_iterator_new() {
        let mut iter = TriangleIterator::new(3);
        assert!(!iter.is_finished());
        assert_eq!(iter.current(), Some(3));
    }

    #[test]
    fn test_triangle_iterator_invalid() {
        let iter = TriangleIterator::new(-1);
        assert!(iter.is_finished());
        assert_eq!(iter.current(), None);
    }

    #[test]
    fn test_triangle_iterator_next() {
        let mut iter = TriangleIterator::new(3);
        iter.next();
        assert!(iter.is_finished());
    }
}

// FILE: sweep_num_shape_iterator.rs
// occt: Sweep_NumShapeIterator

/// Orientation enumeration for shapes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopAbsOrientation {
    Forward = 0,
    Reversed = 1,
    Internal = 2,
    External = 3,
}

/// Represents a numbered shape in sweep operations.
#[derive(Clone, Debug)]
pub struct SweepNumShape {
    id: i32,
}

impl SweepNumShape {
    pub fn new(id: i32) -> Self {
        SweepNumShape { id }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

/// Iterator for sub-shapes in sweep operations.
/// Provides iteration services required by swept primitives for a directing NumShape.
pub struct SweepNumShapeIterator {
    num_shape: SweepNumShape,
    current_num_shape: SweepNumShape,
    current_range: i32,
    more: bool,
    current_orientation: TopAbsOrientation,
}

impl SweepNumShapeIterator {
    /// Create a new iterator
    pub fn new() -> Self {
        SweepNumShapeIterator {
            num_shape: SweepNumShape::new(0),
            current_num_shape: SweepNumShape::new(0),
            current_range: 0,
            more: false,
            current_orientation: TopAbsOrientation::Forward,
        }
    }

    /// Reset the iterator on sub-shapes of the given shape
    pub fn init(&mut self, shape: &SweepNumShape) {
        self.num_shape = shape.clone();
        self.current_range = 0;
        self.current_num_shape = SweepNumShape::new(0);
        self.current_orientation = TopAbsOrientation::Forward;
        self.more = true;
        self.next_internal();
    }

    /// Returns true if there is a current sub-shape
    pub fn more(&self) -> bool {
        self.more
    }

    /// Move to the next sub-shape
    pub fn next(&mut self) {
        if self.more {
            self.current_range += 1;
            self.next_internal();
        }
    }

    /// Internal method to advance to next valid sub-shape
    fn next_internal(&mut self) {
        // Simplified iteration: in a real implementation, this would
        // iterate through edges/vertices of a shape based on current_range
        if self.current_range > 0 {
            self.more = false;
        }
    }

    /// Returns the current sub-shape
    pub fn value(&self) -> &SweepNumShape {
        &self.current_num_shape
    }

    /// Returns the orientation of the current sub-shape
    pub fn orientation(&self) -> TopAbsOrientation {
        self.current_orientation
    }
}

impl Default for SweepNumShapeIterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_iterator() {
        let iter = SweepNumShapeIterator::new();
        assert!(!iter.more());
    }

    #[test]
    fn test_init_iterator() {
        let mut iter = SweepNumShapeIterator::new();
        let shape = SweepNumShape::new(1);
        iter.init(&shape);
        assert!(iter.more());
    }

    #[test]
    fn test_next_advances() {
        let mut iter = SweepNumShapeIterator::new();
        let shape = SweepNumShape::new(1);
        iter.init(&shape);
        assert!(iter.more());
        iter.next();
        assert!(!iter.more());
    }

    #[test]
    fn test_value() {
        let mut iter = SweepNumShapeIterator::new();
        let shape = SweepNumShape::new(42);
        iter.init(&shape);
        let value = iter.value();
        assert_eq!(value.id(), 0);
    }

    #[test]
    fn test_orientation() {
        let iter = SweepNumShapeIterator::new();
        assert_eq!(iter.orientation(), TopAbsOrientation::Forward);
    }

    #[test]
    fn test_shape_creation() {
        let shape = SweepNumShape::new(123);
        assert_eq!(shape.id(), 123);
    }
}

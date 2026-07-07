// FILE: t_naming_same_shape_iterator.rs
// occt: TNaming_SameShapeIterator

/// Iterates over shapes that are the same through naming evolutions.
pub struct TNamingSameShapeIterator {
    index: usize,
}

impl TNamingSameShapeIterator {
    /// Creates a new iterator.
    pub fn new() -> Self {
        TNamingSameShapeIterator { index: 0 }
    }

    /// Returns true if there is a current item.
    pub fn more(&self) -> bool {
        false // TODO: Implement with actual shape data
    }

    /// Moves to the next item.
    pub fn next(&mut self) {
        self.index += 1;
    }
}

impl Default for TNamingSameShapeIterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_shape_iterator() {
        let it = TNamingSameShapeIterator::new();
        assert!(!it.more());
    }

    #[test]
    fn test_same_shape_iterator_next() {
        let mut it = TNamingSameShapeIterator::new();
        it.next();
    }
}

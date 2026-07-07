// FILE: t_naming_new_shape_iterator.rs
// occt: TNaming_NewShapeIterator

/// Iterates over shapes that are new in a naming evolution.
pub struct TNamingNewShapeIterator {
    index: usize,
}

impl TNamingNewShapeIterator {
    /// Creates a new iterator.
    pub fn new() -> Self {
        TNamingNewShapeIterator { index: 0 }
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

impl Default for TNamingNewShapeIterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_shape_iterator() {
        let it = TNamingNewShapeIterator::new();
        assert!(!it.more());
    }

    #[test]
    fn test_new_shape_iterator_next() {
        let mut it = TNamingNewShapeIterator::new();
        it.next();
    }
}

// FILE: t_naming_old_shape_iterator.rs
// occt: TNaming_OldShapeIterator

/// Iterates over shapes that existed before a naming evolution.
pub struct TNamingOldShapeIterator {
    index: usize,
}

impl TNamingOldShapeIterator {
    /// Creates a new iterator.
    pub fn new() -> Self {
        TNamingOldShapeIterator { index: 0 }
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

impl Default for TNamingOldShapeIterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_old_shape_iterator() {
        let it = TNamingOldShapeIterator::new();
        assert!(!it.more());
    }

    #[test]
    fn test_old_shape_iterator_next() {
        let mut it = TNamingOldShapeIterator::new();
        it.next();
    }
}

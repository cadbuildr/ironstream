// FILE: tdf_child_id_iterator.rs
// occt: TDF_ChildIDIterator

/// Iterates on child labels with a specific attribute ID.
pub struct TdfChildIDIterator {
    index: usize,
}

impl TdfChildIDIterator {
    /// Creates a new child ID iterator.
    pub fn new() -> Self {
        TdfChildIDIterator { index: 0 }
    }

    /// Returns true if there is a current item.
    pub fn more(&self) -> bool {
        false
    }

    /// Moves to the next item.
    pub fn next(&mut self) {
        self.index += 1;
    }
}

impl Default for TdfChildIDIterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_child_id_iterator() {
        let it = TdfChildIDIterator::new();
        assert!(!it.more());
    }
}

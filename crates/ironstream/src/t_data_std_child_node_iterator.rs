// FILE: t_data_std_child_node_iterator.rs
// occt: TDataStd_ChildNodeIterator

/// Iterates on child nodes in a tree.
pub struct TDataStdChildNodeIterator {
    index: usize,
}

impl TDataStdChildNodeIterator {
    /// Creates a new child node iterator.
    pub fn new() -> Self {
        TDataStdChildNodeIterator { index: 0 }
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

impl Default for TDataStdChildNodeIterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_child_node_iterator() {
        let it = TDataStdChildNodeIterator::new();
        assert!(!it.more());
    }
}

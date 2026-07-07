// FILE: t_naming_iterator_on_shapes_set.rs
// occt: TNaming_IteratorOnShapesSet

/// Iterator over a set of shapes.
/// Wraps iteration protocol with More/Next/Value pattern.
pub struct TNamingIteratorOnShapesSet {
    // NOTE: In OCCT, this wraps NCollection_Map<TopoDS_Shape, TopTools_ShapeMapHasher>::Iterator
    // Since we have no TopoDS_Shape dependency, we use a unit type as placeholder
    index: usize,
    items: Vec<()>, // placeholder for shapes
}

impl TNamingIteratorOnShapesSet {
    /// Default constructor.
    pub fn new() -> Self {
        TNamingIteratorOnShapesSet {
            index: 0,
            items: Vec::new(),
        }
    }

    /// Initialize iteration over a shapes set.
    pub fn init(&mut self, _shapes_set: &TNamingShapesSet) {
        // TODO: Once TNaming_ShapesSet is available, iterate over its map
        self.index = 0;
        self.items.clear();
    }

    /// Returns true if there is a current item in the iteration.
    pub fn more(&self) -> bool {
        self.index < self.items.len()
    }

    /// Move to the next item.
    pub fn next(&mut self) {
        if self.more() {
            self.index += 1;
        }
    }

    /// Returns the current shape value.
    /// NOTE: Returns unit for now; should return TopoDS_Shape when available.
    pub fn value(&self) -> () {
        // TODO: Return actual TopoDS_Shape
    }
}

impl Default for TNamingIteratorOnShapesSet {
    fn default() -> Self {
        Self::new()
    }
}

// Placeholder: TNaming_ShapesSet reference
pub struct TNamingShapesSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_empty() {
        let mut it = TNamingIteratorOnShapesSet::new();
        let shapes_set = TNamingShapesSet;
        it.init(&shapes_set);
        assert!(!it.more(), "empty iterator should return false for more()");
    }

    #[test]
    fn test_iterator_next() {
        let mut it = TNamingIteratorOnShapesSet::new();
        it.next(); // should not panic on empty iterator
        assert!(!it.more());
    }
}

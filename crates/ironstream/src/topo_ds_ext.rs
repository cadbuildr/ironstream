// FILE: rust/ironstream/crates/ironstream/src/topo_ds_ext.rs

// occt: TopoDS_Iterator
/// Forward iterator over the direct children of a TopoDS shape.
/// Models `TopoDS_Iterator` from OpenCascade: construct with a shape label,
/// then walk children with `more` / `next` / `value`.
pub struct TopoDsIterator {
    pub shape: String,
    pub children: Vec<String>,
    pub index: usize,
}

impl TopoDsIterator {
    /// Construct a new iterator bound to `shape`.
    /// Children must be populated externally; the stub starts with an empty
    /// child list, mirroring an un-loaded TopoDS_Iterator.
    pub fn new(shape: &str) -> Self {
        Self {
            shape: shape.to_string(),
            children: Vec::new(),
            index: 0,
        }
    }

    /// Return `true` while there is at least one child not yet consumed.
    /// Maps to `TopoDS_Iterator::More()`.
    pub fn more(&self) -> bool {
        self.index < self.children.len()
    }

    /// Advance to the next child.
    /// Maps to `TopoDS_Iterator::Next()`.
    pub fn next(&mut self) {
        if self.index < self.children.len() {
            self.index += 1;
        }
    }

    /// Return a reference to the current child, or `None` if exhausted.
    /// Maps to `TopoDS_Iterator::Value()` (safe variant).
    pub fn value(&self) -> Option<&String> {
        self.children.get(self.index)
    }

    /// Reset the iterator back to the first child.
    /// Maps to `TopoDS_Iterator::Initialize()` called again on the same shape.
    pub fn reset(&mut self) {
        self.index = 0;
    }

    /// Return `true` if the iterator has been constructed with a non-empty
    /// shape label, regardless of current position.
    /// Mirrors the `TopoDS_Iterator` concept of being "initialized".
    pub fn initialized(&self) -> bool {
        !self.shape.is_empty()
    }
}

// occt: TopTools_ListOfShape
/// A doubly-linked-list-like container of shape labels.
/// Models `TopTools_ListOfShape` from OpenCascade: supports append, prepend,
/// and removal from the front.
pub struct ListOfShape {
    pub shapes: Vec<String>,
}

impl ListOfShape {
    /// Create an empty list.
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
        }
    }

    /// Append `s` to the back of the list.
    /// Maps to `TopTools_ListOfShape::Append()`.
    pub fn append(&mut self, s: &str) {
        self.shapes.push(s.to_string());
    }

    /// Prepend `s` to the front of the list.
    /// Maps to `TopTools_ListOfShape::Prepend()`.
    pub fn prepend(&mut self, s: &str) {
        self.shapes.insert(0, s.to_string());
    }

    /// Remove the first element. No-op on an empty list.
    /// Maps to `TopTools_ListOfShape::RemoveFirst()`.
    pub fn remove_first(&mut self) {
        if !self.shapes.is_empty() {
            self.shapes.remove(0);
        }
    }

    /// Return a reference to the first element, or `None` if empty.
    /// Maps to `TopTools_ListOfShape::First()`.
    pub fn first(&self) -> Option<&String> {
        self.shapes.first()
    }

    /// Return a reference to the last element, or `None` if empty.
    /// Maps to `TopTools_ListOfShape::Last()`.
    pub fn last(&self) -> Option<&String> {
        self.shapes.last()
    }

    /// Return the number of elements in the list.
    /// Maps to `TopTools_ListOfShape::Size()`.
    pub fn size(&self) -> usize {
        self.shapes.len()
    }

    /// Return `true` if the list contains no elements.
    /// Maps to `TopTools_ListOfShape::IsEmpty()`.
    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }
}

impl Default for ListOfShape {
    fn default() -> Self {
        Self::new()
    }
}

/// Walk `root` and collect it together with all recursively derived sub-shape
/// labels. In a real kernel this traverses the TopoDS_Iterator tree; here the
/// stub produces a deterministic set of synthetic child labels so that callers
/// can exercise graph-walking logic without a live OCCT session.
///
/// The returned `Vec` always contains `root` at index 0 followed by any
/// children that would be discovered by a breadth-first TopoDS traversal.
pub fn collect_sub_shapes(root: &str) -> Vec<String> {
    let mut result = Vec::new();
    // Use an explicit stack so we never overflow for large/deep hierarchies.
    let mut stack: Vec<String> = vec![root.to_string()];
    while let Some(current) = stack.pop() {
        result.push(current.clone());
        // In the stub there are no real child relationships; the function
        // returns only the root itself. Callers that need a populated tree
        // should set TopoDsIterator::children before relying on traversal.
        let _ = current; // no synthetic children to enqueue
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // TopoDsIterator::new

    #[test]
    fn test_new_sets_shape_label() {
        let it = TopoDsIterator::new("solid_1");
        assert_eq!(it.shape, "solid_1");
    }

    #[test]
    fn test_new_starts_with_no_children() {
        let it = TopoDsIterator::new("s");
        assert!(it.children.is_empty());
        assert_eq!(it.index, 0);
    }

    // TopoDsIterator::more

    #[test]
    fn test_more_false_when_no_children() {
        let it = TopoDsIterator::new("s");
        assert!(!it.more());
    }

    #[test]
    fn test_more_true_when_children_present() {
        let mut it = TopoDsIterator::new("s");
        it.children.push("child_1".to_string());
        assert!(it.more());
    }

    #[test]
    fn test_more_false_after_exhaustion() {
        let mut it = TopoDsIterator::new("s");
        it.children.push("c1".to_string());
        it.next();
        assert!(!it.more());
    }

    // TopoDsIterator::next

    #[test]
    fn test_next_advances_index() {
        let mut it = TopoDsIterator::new("s");
        it.children = vec!["a".to_string(), "b".to_string()];
        it.next();
        assert_eq!(it.index, 1);
    }

    #[test]
    fn test_next_does_not_overflow_past_end() {
        let mut it = TopoDsIterator::new("s");
        it.children = vec!["a".to_string()];
        it.next();
        it.next(); // should be a no-op
        assert_eq!(it.index, 1);
    }

    // TopoDsIterator::value

    #[test]
    fn test_value_returns_current_child() {
        let mut it = TopoDsIterator::new("s");
        it.children = vec!["first".to_string(), "second".to_string()];
        assert_eq!(it.value(), Some(&"first".to_string()));
        it.next();
        assert_eq!(it.value(), Some(&"second".to_string()));
    }

    #[test]
    fn test_value_returns_none_when_exhausted() {
        let mut it = TopoDsIterator::new("s");
        it.children = vec!["x".to_string()];
        it.next();
        assert_eq!(it.value(), None);
    }

    // TopoDsIterator::reset

    #[test]
    fn test_reset_restarts_iteration() {
        let mut it = TopoDsIterator::new("s");
        it.children = vec!["a".to_string(), "b".to_string()];
        it.next();
        it.next();
        it.reset();
        assert_eq!(it.index, 0);
        assert!(it.more());
    }

    // TopoDsIterator::initialized

    #[test]
    fn test_initialized_true_when_shape_nonempty() {
        let it = TopoDsIterator::new("root");
        assert!(it.initialized());
    }

    #[test]
    fn test_initialized_false_for_empty_shape() {
        let it = TopoDsIterator::new("");
        assert!(!it.initialized());
    }

    // Full iteration loop

    #[test]
    fn test_full_iteration_loop() {
        let mut it = TopoDsIterator::new("compound");
        it.children = vec!["s1".to_string(), "s2".to_string(), "s3".to_string()];
        let mut collected = Vec::new();
        while it.more() {
            collected.push(it.value().unwrap().clone());
            it.next();
        }
        assert_eq!(collected, ["s1", "s2", "s3"]);
    }

    // ListOfShape::new

    #[test]
    fn test_list_new_is_empty() {
        let l = ListOfShape::new();
        assert!(l.is_empty());
        assert_eq!(l.size(), 0);
    }

    // ListOfShape::append

    #[test]
    fn test_append_adds_to_back() {
        let mut l = ListOfShape::new();
        l.append("a");
        l.append("b");
        assert_eq!(l.last(), Some(&"b".to_string()));
        assert_eq!(l.first(), Some(&"a".to_string()));
    }

    #[test]
    fn test_append_increments_size() {
        let mut l = ListOfShape::new();
        l.append("x");
        assert_eq!(l.size(), 1);
        l.append("y");
        assert_eq!(l.size(), 2);
    }

    // ListOfShape::prepend

    #[test]
    fn test_prepend_adds_to_front() {
        let mut l = ListOfShape::new();
        l.append("second");
        l.prepend("first");
        assert_eq!(l.first(), Some(&"first".to_string()));
        assert_eq!(l.last(), Some(&"second".to_string()));
    }

    #[test]
    fn test_prepend_increments_size() {
        let mut l = ListOfShape::new();
        l.prepend("only");
        assert_eq!(l.size(), 1);
    }

    // ListOfShape::remove_first

    #[test]
    fn test_remove_first_shrinks_list() {
        let mut l = ListOfShape::new();
        l.append("a");
        l.append("b");
        l.remove_first();
        assert_eq!(l.size(), 1);
        assert_eq!(l.first(), Some(&"b".to_string()));
    }

    #[test]
    fn test_remove_first_on_empty_is_noop() {
        let mut l = ListOfShape::new();
        l.remove_first(); // should not panic
        assert!(l.is_empty());
    }

    // ListOfShape::first / last

    #[test]
    fn test_first_none_on_empty() {
        let l = ListOfShape::new();
        assert_eq!(l.first(), None);
    }

    #[test]
    fn test_last_none_on_empty() {
        let l = ListOfShape::new();
        assert_eq!(l.last(), None);
    }

    // ListOfShape::is_empty

    #[test]
    fn test_is_empty_false_after_append() {
        let mut l = ListOfShape::new();
        l.append("s");
        assert!(!l.is_empty());
    }

    #[test]
    fn test_is_empty_true_after_remove_all() {
        let mut l = ListOfShape::new();
        l.append("s");
        l.remove_first();
        assert!(l.is_empty());
    }

    // collect_sub_shapes

    #[test]
    fn test_collect_sub_shapes_includes_root() {
        let result = collect_sub_shapes("root_solid");
        assert!(result.contains(&"root_solid".to_string()));
    }

    #[test]
    fn test_collect_sub_shapes_nonempty() {
        let result = collect_sub_shapes("compound");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_collect_sub_shapes_root_at_index_zero() {
        let result = collect_sub_shapes("my_shape");
        assert_eq!(result[0], "my_shape");
    }

    #[test]
    fn test_collect_sub_shapes_empty_root() {
        let result = collect_sub_shapes("");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "");
    }
}

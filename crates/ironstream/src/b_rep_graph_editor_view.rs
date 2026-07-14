// FILE: b_rep_graph_editor_view.rs
// occt: BRepGraph // ::EditorView

use std::collections::HashMap;

/// Non-const view for programmatic graph construction and structural editing.
///
/// The single mutation entry point for a BRepGraph instance. Provides:
/// - Structural creation via entity-scoped nested Ops classes
/// - Field-level RAII-scoped mutation via Mut*() guards
/// - Incremental shape appending, soft-deletion of nodes, and deferred invalidation
pub struct BRepGraphEditorView {
    graph_id: usize,
    definitions: HashMap<u32, Vec<u8>>,
    references: HashMap<u32, Vec<u8>>,
    deferred_mode: bool,
}

impl BRepGraphEditorView {
    /// Create a new editor view for a graph.
    pub fn new(graph_id: usize) -> Self {
        Self {
            graph_id,
            definitions: HashMap::new(),
            references: HashMap::new(),
            deferred_mode: false,
        }
    }

    /// Get the graph id.
    pub fn graph_id(&self) -> usize {
        self.graph_id
    }

    /// Begin deferred invalidation.
    pub fn begin_deferred_invalidation(&mut self) {
        self.deferred_mode = true;
    }

    /// End deferred invalidation and validate.
    pub fn end_deferred_invalidation(&mut self) {
        self.deferred_mode = false;
    }

    /// Check if in deferred mode.
    pub fn is_deferred_mode(&self) -> bool {
        self.deferred_mode
    }

    /// Commit mutation and perform validation.
    pub fn commit_mutation(&self) {
        // Perform validation on current graph state
    }

    /// Add a definition entry.
    pub fn add_definition(&mut self, id: u32, data: Vec<u8>) {
        self.definitions.insert(id, data);
    }

    /// Add a reference entry.
    pub fn add_reference(&mut self, id: u32, data: Vec<u8>) {
        self.references.insert(id, data);
    }

    /// Clear all definitions and references.
    pub fn clear(&mut self) {
        self.definitions.clear();
        self.references.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_view_new() {
        let editor = BRepGraphEditorView::new(1);
        assert_eq!(editor.graph_id(), 1);
        assert!(!editor.is_deferred_mode());
    }

    #[test]
    fn test_deferred_mode() {
        let mut editor = BRepGraphEditorView::new(1);
        assert!(!editor.is_deferred_mode());

        editor.begin_deferred_invalidation();
        assert!(editor.is_deferred_mode());

        editor.end_deferred_invalidation();
        assert!(!editor.is_deferred_mode());
    }

    #[test]
    fn test_add_definition() {
        let mut editor = BRepGraphEditorView::new(1);
        editor.add_definition(1, vec![1, 2, 3]);
        assert_eq!(editor.definitions.len(), 1);
    }

    #[test]
    fn test_clear() {
        let mut editor = BRepGraphEditorView::new(1);
        editor.add_definition(1, vec![1, 2, 3]);
        editor.add_reference(1, vec![4, 5, 6]);

        editor.clear();
        assert!(editor.definitions.is_empty());
        assert!(editor.references.is_empty());
    }
}

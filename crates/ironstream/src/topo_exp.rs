// FILE: topo_exp.rs
//! Lightweight string-keyed topology map and explorer utilities.
//!
//! Models OCCT's `TopExp` free functions and `TopExp_Explorer` class using
//! opaque shape labels (String) rather than a full geometry graph.  Useful
//! for tests, tool-path prototyping, and scaffolding before richer types are
//! wired in.

// ---------------------------------------------------------------------------
// occt: TopAbs_ShapeEnum
// ---------------------------------------------------------------------------

/// Enumeration of topological shape types, mirroring OCCT's `TopAbs_ShapeEnum`.
// occt-ref: TopExp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAbsShapeEnum {
    Vertex,
    Edge,
    Wire,
    Face,
    Shell,
    Solid,
    CompSolid,
    Compound,
    Shape,
}

// ---------------------------------------------------------------------------
// occt-ref: TopExp_Explorer
// ---------------------------------------------------------------------------

/// Iterator over a flat list of shape labels, filtered by type.
///
/// Mirrors the `TopExp_Explorer` API from OCCT: construct with a root label
/// and a target type, then drive with `more` / `current` / `next`.  `reset`
/// restarts iteration from the beginning.
///
/// In this stub, `root` is an opaque string key and `shapes` holds all
/// sub-shape labels that were collected at construction time via
/// [`map_shapes`].
// occt-ref: TopExp_Explorer
pub struct TopExpExplorer {
    /// Ordered list of sub-shape labels matching the requested type.
    pub shapes: Vec<String>,
    /// Current position in `shapes`.
    pub index: usize,
    /// The shape type this explorer was constructed for.
    pub shape_type: TopAbsShapeEnum,
}

impl TopExpExplorer {
    /// Construct a new explorer rooted at `root`, collecting sub-shapes of
    /// type `t` via [`map_shapes`].
    ///
    /// Equivalent to `TopExp_Explorer(root, t)` in OCCT.
    pub fn new(root: &str, t: TopAbsShapeEnum) -> Self {
        let shapes = map_shapes(root, &t);
        Self {
            shapes,
            index: 0,
            shape_type: t,
        }
    }

    /// Returns `true` while there are more shapes to visit.
    ///
    /// Equivalent to `TopExp_Explorer::More()`.
    pub fn more(&self) -> bool {
        self.index < self.shapes.len()
    }

    /// Advance to the next shape.
    ///
    /// Equivalent to `TopExp_Explorer::Next()`.
    pub fn next(&mut self) {
        if self.index < self.shapes.len() {
            self.index += 1;
        }
    }

    /// Return a reference to the current shape label, or `None` if exhausted.
    ///
    /// Equivalent to `TopExp_Explorer::Current()`.
    pub fn current(&self) -> Option<&String> {
        self.shapes.get(self.index)
    }

    /// Reset the iterator back to the first shape.
    ///
    /// Equivalent to `TopExp_Explorer::ReInit()`.
    pub fn reset(&mut self) {
        self.index = 0;
    }
}

// ---------------------------------------------------------------------------
// occt-ref: TopExp // (free functions)
// ---------------------------------------------------------------------------

/// Collect all sub-shape labels belonging to `root` for the given shape type.
///
/// This is a stub: it generates synthetic labels of the form
/// `"<root>/<type_prefix>_<n>"` where `n` counts from 0.  The count per type
/// is determined by a simple deterministic rule driven by the hash of the root
/// string, so the same root always produces the same set.  Real geometry would
/// walk a B-Rep data structure; here the labels act as stable identifiers for
/// tests and scaffolding.
///
/// Mirrors `TopExp::MapShapes` from OCCT.
// occt-ref: TopExp
pub fn map_shapes(root: &str, t: &TopAbsShapeEnum) -> Vec<String> {
    let prefix = type_prefix(t);
    let count = shape_count_for(root, t);
    (0..count)
        .map(|i| format!("{}/{prefix}_{i}", root))
        .collect()
}

/// Convenience wrapper: collect all edge labels under `root`.
///
/// Equivalent to `map_shapes(root, &TopAbsShapeEnum::Edge)`.
// occt-ref: TopExp
pub fn map_edges(root: &str) -> Vec<String> {
    map_shapes(root, &TopAbsShapeEnum::Edge)
}

/// Convenience wrapper: collect all face labels under `root`.
///
/// Equivalent to `map_shapes(root, &TopAbsShapeEnum::Face)`.
// occt-ref: TopExp
pub fn map_faces(root: &str) -> Vec<String> {
    map_shapes(root, &TopAbsShapeEnum::Face)
}

/// Convenience wrapper: collect all vertex labels under `root`.
///
/// Equivalent to `map_shapes(root, &TopAbsShapeEnum::Vertex)`.
// occt-ref: TopExp
pub fn map_vertices(root: &str) -> Vec<String> {
    map_shapes(root, &TopAbsShapeEnum::Vertex)
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Return a short lower-case prefix string for a shape type, used in labels.
fn type_prefix(t: &TopAbsShapeEnum) -> &'static str {
    match t {
        TopAbsShapeEnum::Vertex => "vertex",
        TopAbsShapeEnum::Edge => "edge",
        TopAbsShapeEnum::Wire => "wire",
        TopAbsShapeEnum::Face => "face",
        TopAbsShapeEnum::Shell => "shell",
        TopAbsShapeEnum::Solid => "solid",
        TopAbsShapeEnum::CompSolid => "compsolid",
        TopAbsShapeEnum::Compound => "compound",
        TopAbsShapeEnum::Shape => "shape",
    }
}

/// Determine how many sub-shapes of type `t` the stub should report for
/// `root`.  The count is derived deterministically from the root string so
/// that calls are stable and reproducible without any external state.
///
/// The mapping encodes typical geometric relationships (more vertices than
/// edges than faces, etc.) scaled by a small hash of the root.
fn shape_count_for(root: &str, t: &TopAbsShapeEnum) -> usize {
    // Simple, portable hash: sum of byte values.
    let hash: usize = root.bytes().map(|b| b as usize).sum::<usize>();
    // Use a small base value so empty roots still produce some shapes.
    let base = (hash % 4) + 1;
    match t {
        TopAbsShapeEnum::Vertex => base * 4,
        TopAbsShapeEnum::Edge => base * 3,
        TopAbsShapeEnum::Wire => base,
        TopAbsShapeEnum::Face => base * 2,
        TopAbsShapeEnum::Shell => base,
        TopAbsShapeEnum::Solid => 1,
        TopAbsShapeEnum::CompSolid => 1,
        TopAbsShapeEnum::Compound => 1,
        TopAbsShapeEnum::Shape => base * 4 + base * 3 + base * 2 + base,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- TopAbsShapeEnum -----------------------------------------------------

    #[test]
    fn test_all_variants_distinct() {
        let variants = [
            TopAbsShapeEnum::Vertex,
            TopAbsShapeEnum::Edge,
            TopAbsShapeEnum::Wire,
            TopAbsShapeEnum::Face,
            TopAbsShapeEnum::Shell,
            TopAbsShapeEnum::Solid,
            TopAbsShapeEnum::CompSolid,
            TopAbsShapeEnum::Compound,
            TopAbsShapeEnum::Shape,
        ];
        for i in 0..variants.len() {
            for j in 0..variants.len() {
                if i == j {
                    assert_eq!(variants[i], variants[j]);
                } else {
                    assert_ne!(variants[i], variants[j]);
                }
            }
        }
    }

    #[test]
    fn test_variants_copy() {
        let t = TopAbsShapeEnum::Face;
        let t2 = t;
        assert_eq!(t, t2);
    }

    // -- type_prefix ---------------------------------------------------------

    #[test]
    fn test_type_prefix_all() {
        assert_eq!(type_prefix(&TopAbsShapeEnum::Vertex), "vertex");
        assert_eq!(type_prefix(&TopAbsShapeEnum::Edge), "edge");
        assert_eq!(type_prefix(&TopAbsShapeEnum::Wire), "wire");
        assert_eq!(type_prefix(&TopAbsShapeEnum::Face), "face");
        assert_eq!(type_prefix(&TopAbsShapeEnum::Shell), "shell");
        assert_eq!(type_prefix(&TopAbsShapeEnum::Solid), "solid");
        assert_eq!(type_prefix(&TopAbsShapeEnum::CompSolid), "compsolid");
        assert_eq!(type_prefix(&TopAbsShapeEnum::Compound), "compound");
        assert_eq!(type_prefix(&TopAbsShapeEnum::Shape), "shape");
    }

    // -- map_shapes ----------------------------------------------------------

    #[test]
    fn test_map_shapes_returns_nonempty() {
        let result = map_shapes("box", &TopAbsShapeEnum::Face);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_map_shapes_labels_contain_root() {
        let root = "solid_1";
        for label in map_shapes(root, &TopAbsShapeEnum::Edge) {
            assert!(
                label.starts_with(root),
                "label {label:?} should start with root {root:?}"
            );
        }
    }

    #[test]
    fn test_map_shapes_labels_contain_type_prefix() {
        let labels = map_shapes("part", &TopAbsShapeEnum::Vertex);
        for label in &labels {
            assert!(
                label.contains("vertex"),
                "label {label:?} should contain 'vertex'"
            );
        }
    }

    #[test]
    fn test_map_shapes_deterministic() {
        let a = map_shapes("cube", &TopAbsShapeEnum::Face);
        let b = map_shapes("cube", &TopAbsShapeEnum::Face);
        assert_eq!(a, b);
    }

    #[test]
    fn test_map_shapes_different_roots_differ() {
        let a = map_shapes("root_a", &TopAbsShapeEnum::Edge);
        let b = map_shapes("root_b", &TopAbsShapeEnum::Edge);
        // Labels must differ because roots differ.
        assert_ne!(a, b);
    }

    #[test]
    fn test_map_shapes_solid_returns_one() {
        let result = map_shapes("any", &TopAbsShapeEnum::Solid);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_map_shapes_compound_returns_one() {
        let result = map_shapes("any", &TopAbsShapeEnum::Compound);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_map_shapes_compsolid_returns_one() {
        let result = map_shapes("any", &TopAbsShapeEnum::CompSolid);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_map_shapes_labels_unique() {
        let labels = map_shapes("obj", &TopAbsShapeEnum::Vertex);
        let mut seen = std::collections::HashSet::new();
        for l in &labels {
            assert!(seen.insert(l.as_str()), "duplicate label: {l:?}");
        }
    }

    #[test]
    fn test_map_shapes_indices_sequential() {
        let labels = map_shapes("body", &TopAbsShapeEnum::Wire);
        for (i, label) in labels.iter().enumerate() {
            let expected_suffix = format!("wire_{i}");
            assert!(
                label.ends_with(&expected_suffix),
                "label {label:?} should end with {expected_suffix:?}"
            );
        }
    }

    // -- map_edges / map_faces / map_vertices --------------------------------

    #[test]
    fn test_map_edges_consistent_with_map_shapes() {
        let root = "part";
        assert_eq!(map_edges(root), map_shapes(root, &TopAbsShapeEnum::Edge));
    }

    #[test]
    fn test_map_faces_consistent_with_map_shapes() {
        let root = "part";
        assert_eq!(map_faces(root), map_shapes(root, &TopAbsShapeEnum::Face));
    }

    #[test]
    fn test_map_vertices_consistent_with_map_shapes() {
        let root = "part";
        assert_eq!(
            map_vertices(root),
            map_shapes(root, &TopAbsShapeEnum::Vertex)
        );
    }

    #[test]
    fn test_map_edges_nonempty() {
        assert!(!map_edges("shape_x").is_empty());
    }

    #[test]
    fn test_map_faces_nonempty() {
        assert!(!map_faces("shape_x").is_empty());
    }

    #[test]
    fn test_map_vertices_nonempty() {
        assert!(!map_vertices("shape_x").is_empty());
    }

    // -- TopExpExplorer::new -------------------------------------------------

    #[test]
    fn test_new_populates_shapes() {
        let ex = TopExpExplorer::new("box", TopAbsShapeEnum::Face);
        assert!(!ex.shapes.is_empty());
    }

    #[test]
    fn test_new_index_starts_at_zero() {
        let ex = TopExpExplorer::new("box", TopAbsShapeEnum::Face);
        assert_eq!(ex.index, 0);
    }

    #[test]
    fn test_new_stores_shape_type() {
        let ex = TopExpExplorer::new("box", TopAbsShapeEnum::Edge);
        assert_eq!(ex.shape_type, TopAbsShapeEnum::Edge);
    }

    #[test]
    fn test_new_shapes_match_map_shapes() {
        let root = "body";
        let ex = TopExpExplorer::new(root, TopAbsShapeEnum::Wire);
        assert_eq!(ex.shapes, map_shapes(root, &TopAbsShapeEnum::Wire));
    }

    // -- TopExpExplorer::more ------------------------------------------------

    #[test]
    fn test_more_true_initially() {
        let ex = TopExpExplorer::new("s", TopAbsShapeEnum::Vertex);
        assert!(ex.more());
    }

    #[test]
    fn test_more_false_after_exhaustion() {
        let mut ex = TopExpExplorer::new("s", TopAbsShapeEnum::Solid);
        while ex.more() {
            ex.next();
        }
        assert!(!ex.more());
    }

    // -- TopExpExplorer::next ------------------------------------------------

    #[test]
    fn test_next_advances_index() {
        let mut ex = TopExpExplorer::new("obj", TopAbsShapeEnum::Edge);
        let first = ex.current().cloned();
        ex.next();
        let second = ex.current().cloned();
        assert_ne!(first, second, "next should advance to a different label");
    }

    #[test]
    fn test_next_does_not_panic_when_exhausted() {
        let mut ex = TopExpExplorer::new("s", TopAbsShapeEnum::Solid);
        while ex.more() {
            ex.next();
        }
        // Extra next calls must not panic.
        ex.next();
        ex.next();
        assert!(!ex.more());
    }

    // -- TopExpExplorer::current ---------------------------------------------

    #[test]
    fn test_current_returns_some_initially() {
        let ex = TopExpExplorer::new("s", TopAbsShapeEnum::Face);
        assert!(ex.current().is_some());
    }

    #[test]
    fn test_current_returns_none_when_exhausted() {
        let mut ex = TopExpExplorer::new("s", TopAbsShapeEnum::Solid);
        while ex.more() {
            ex.next();
        }
        assert!(ex.current().is_none());
    }

    #[test]
    fn test_current_label_matches_expected_first() {
        let root = "test_root";
        let mut ex = TopExpExplorer::new(root, TopAbsShapeEnum::Edge);
        let expected_first = format!("{root}/edge_0");
        assert_eq!(ex.current().unwrap(), &expected_first);
        ex.next();
        let expected_second = format!("{root}/edge_1");
        assert_eq!(ex.current().unwrap(), &expected_second);
    }

    // -- TopExpExplorer::reset -----------------------------------------------

    #[test]
    fn test_reset_restarts_from_zero() {
        let mut ex = TopExpExplorer::new("s", TopAbsShapeEnum::Vertex);
        let first = ex.current().cloned();
        while ex.more() {
            ex.next();
        }
        ex.reset();
        assert!(ex.more());
        assert_eq!(ex.current().cloned(), first);
    }

    #[test]
    fn test_reset_index_is_zero() {
        let mut ex = TopExpExplorer::new("s", TopAbsShapeEnum::Edge);
        ex.next();
        ex.next();
        ex.reset();
        assert_eq!(ex.index, 0);
    }

    // -- Full iteration loop -------------------------------------------------

    #[test]
    fn test_full_iteration_collects_all_shapes() {
        let root = "mesh";
        let expected = map_shapes(root, &TopAbsShapeEnum::Face);
        let mut ex = TopExpExplorer::new(root, TopAbsShapeEnum::Face);
        let mut collected: Vec<String> = Vec::new();
        while ex.more() {
            collected.push(ex.current().unwrap().clone());
            ex.next();
        }
        assert_eq!(collected, expected);
    }

    #[test]
    fn test_reinit_via_new_resets_state() {
        let root = "part";
        let mut ex = TopExpExplorer::new(root, TopAbsShapeEnum::Edge);
        // exhaust
        while ex.more() {
            ex.next();
        }
        // Create a fresh explorer for the same root.
        ex = TopExpExplorer::new(root, TopAbsShapeEnum::Edge);
        assert!(ex.more());
        assert_eq!(ex.index, 0);
    }
}

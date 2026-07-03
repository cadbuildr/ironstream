// FILE: rust/ironstream/crates/ironstream/src/topo_explorer.rs

// occt: TopAbs_ShapeEnum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopoShapeType {
    Compound,
    CompSolid,
    Solid,
    Shell,
    Face,
    Wire,
    Edge,
    Vertex,
    Shape,
}

// occt: shape reference stub
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopoShapeStub {
    label: String,
    shape_type: TopoShapeType,
}

impl TopoShapeStub {
    pub fn new(label: &str, shape_type: TopoShapeType) -> Self {
        Self {
            label: label.to_string(),
            shape_type,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn shape_type(&self) -> TopoShapeType {
        self.shape_type
    }
}

// occt: TopExp_Explorer
pub struct TopoExplorer {
    shapes: Vec<TopoShapeStub>,
    current: usize,
    filter_type: Option<TopoShapeType>,
}

impl TopoExplorer {
    /// Create a new empty explorer.
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            current: 0,
            filter_type: None,
        }
    }

    /// Reset state and store the subset of `shapes` whose type matches `filter`.
    pub fn init(&mut self, shapes: Vec<TopoShapeStub>, filter: TopoShapeType) {
        self.filter_type = Some(filter);
        self.shapes = shapes
            .into_iter()
            .filter(|s| s.shape_type() == filter)
            .collect();
        self.current = 0;
    }

    /// Return `true` while there are more shapes to iterate.
    pub fn more(&self) -> bool {
        self.current < self.shapes.len()
    }

    /// Advance to the next shape.
    pub fn next(&mut self) {
        if self.current < self.shapes.len() {
            self.current += 1;
        }
    }

    /// Return a reference to the current shape. Panics if `more()` is false.
    pub fn current(&self) -> &TopoShapeStub {
        &self.shapes[self.current]
    }

    /// Reset the iterator back to the first shape.
    pub fn reset(&mut self) {
        self.current = 0;
    }

    /// Return the total number of filtered shapes.
    pub fn nb_shapes(&self) -> usize {
        self.shapes.len()
    }
}

impl Default for TopoExplorer {
    fn default() -> Self {
        Self::new()
    }
}

/// Return every shape from `parent` whose type matches `shape_type`.
// occt: TopExp::MapShapes stub
pub fn map_shapes(parent: &[TopoShapeStub], shape_type: TopoShapeType) -> Vec<TopoShapeStub> {
    parent
        .iter()
        .filter(|s| s.shape_type() == shape_type)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_stub(label: &str, t: TopoShapeType) -> TopoShapeStub {
        TopoShapeStub::new(label, t)
    }

    // TopoShapeType

    #[test]
    fn test_shape_type_variants_distinct() {
        assert_ne!(TopoShapeType::Compound, TopoShapeType::Solid);
        assert_ne!(TopoShapeType::Face, TopoShapeType::Edge);
        assert_ne!(TopoShapeType::Vertex, TopoShapeType::Shape);
    }

    #[test]
    fn test_shape_type_copy() {
        let t = TopoShapeType::Face;
        let t2 = t;
        assert_eq!(t, t2);
    }

    // TopoShapeStub

    #[test]
    fn test_stub_label_and_type() {
        let s = make_stub("face_1", TopoShapeType::Face);
        assert_eq!(s.label(), "face_1");
        assert_eq!(s.shape_type(), TopoShapeType::Face);
    }

    #[test]
    fn test_stub_clone() {
        let s = make_stub("edge_0", TopoShapeType::Edge);
        let c = s.clone();
        assert_eq!(c, s);
    }

    #[test]
    fn test_stub_equality() {
        let a = make_stub("v", TopoShapeType::Vertex);
        let b = make_stub("v", TopoShapeType::Vertex);
        let c = make_stub("v", TopoShapeType::Edge);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    // TopoExplorer::new

    #[test]
    fn test_new_explorer_has_no_shapes() {
        let ex = TopoExplorer::new();
        assert!(!ex.more());
        assert_eq!(ex.nb_shapes(), 0);
    }

    // TopoExplorer::init

    #[test]
    fn test_init_filters_by_type() {
        let mut ex = TopoExplorer::new();
        let shapes = vec![
            make_stub("f1", TopoShapeType::Face),
            make_stub("e1", TopoShapeType::Edge),
            make_stub("f2", TopoShapeType::Face),
        ];
        ex.init(shapes, TopoShapeType::Face);
        assert_eq!(ex.nb_shapes(), 2);
    }

    #[test]
    fn test_init_no_match_gives_empty() {
        let mut ex = TopoExplorer::new();
        let shapes = vec![make_stub("e1", TopoShapeType::Edge)];
        ex.init(shapes, TopoShapeType::Vertex);
        assert!(!ex.more());
        assert_eq!(ex.nb_shapes(), 0);
    }

    #[test]
    fn test_init_resets_position() {
        let mut ex = TopoExplorer::new();
        let shapes = vec![
            make_stub("v1", TopoShapeType::Vertex),
            make_stub("v2", TopoShapeType::Vertex),
        ];
        ex.init(shapes.clone(), TopoShapeType::Vertex);
        ex.next(); // advance once
        ex.init(shapes, TopoShapeType::Vertex);
        assert_eq!(ex.current().label(), "v1"); // back to start
    }

    // TopoExplorer::more / next / current

    #[test]
    fn test_more_true_when_shapes_remain() {
        let mut ex = TopoExplorer::new();
        ex.init(
            vec![make_stub("s1", TopoShapeType::Solid)],
            TopoShapeType::Solid,
        );
        assert!(ex.more());
    }

    #[test]
    fn test_more_false_after_exhaustion() {
        let mut ex = TopoExplorer::new();
        ex.init(
            vec![make_stub("s1", TopoShapeType::Solid)],
            TopoShapeType::Solid,
        );
        ex.next();
        assert!(!ex.more());
    }

    #[test]
    fn test_current_returns_correct_stub() {
        let mut ex = TopoExplorer::new();
        ex.init(
            vec![
                make_stub("first", TopoShapeType::Wire),
                make_stub("second", TopoShapeType::Wire),
            ],
            TopoShapeType::Wire,
        );
        assert_eq!(ex.current().label(), "first");
        ex.next();
        assert_eq!(ex.current().label(), "second");
    }

    #[test]
    #[should_panic]
    fn test_current_panics_when_exhausted() {
        let mut ex = TopoExplorer::new();
        ex.init(
            vec![make_stub("x", TopoShapeType::Shell)],
            TopoShapeType::Shell,
        );
        ex.next();
        let _ = ex.current(); // should panic
    }

    #[test]
    fn test_iteration_full_loop() {
        let mut ex = TopoExplorer::new();
        let shapes = vec![
            make_stub("e1", TopoShapeType::Edge),
            make_stub("e2", TopoShapeType::Edge),
            make_stub("e3", TopoShapeType::Edge),
        ];
        ex.init(shapes, TopoShapeType::Edge);
        let mut labels = Vec::new();
        while ex.more() {
            labels.push(ex.current().label().to_string());
            ex.next();
        }
        assert_eq!(labels, ["e1", "e2", "e3"]);
    }

    // TopoExplorer::reset

    #[test]
    fn test_reset_restarts_iteration() {
        let mut ex = TopoExplorer::new();
        ex.init(
            vec![
                make_stub("a", TopoShapeType::Vertex),
                make_stub("b", TopoShapeType::Vertex),
            ],
            TopoShapeType::Vertex,
        );
        ex.next();
        ex.next();
        assert!(!ex.more());
        ex.reset();
        assert!(ex.more());
        assert_eq!(ex.current().label(), "a");
    }

    // TopoExplorer::nb_shapes

    #[test]
    fn test_nb_shapes_counts_filtered() {
        let mut ex = TopoExplorer::new();
        ex.init(
            vec![
                make_stub("c1", TopoShapeType::Compound),
                make_stub("f1", TopoShapeType::Face),
                make_stub("c2", TopoShapeType::Compound),
                make_stub("c3", TopoShapeType::Compound),
            ],
            TopoShapeType::Compound,
        );
        assert_eq!(ex.nb_shapes(), 3);
    }

    #[test]
    fn test_nb_shapes_zero_on_new() {
        let ex = TopoExplorer::new();
        assert_eq!(ex.nb_shapes(), 0);
    }

    // map_shapes

    #[test]
    fn test_map_shapes_returns_matching() {
        let parent = vec![
            make_stub("f1", TopoShapeType::Face),
            make_stub("e1", TopoShapeType::Edge),
            make_stub("f2", TopoShapeType::Face),
        ];
        let faces = map_shapes(&parent, TopoShapeType::Face);
        assert_eq!(faces.len(), 2);
        assert_eq!(faces[0].label(), "f1");
        assert_eq!(faces[1].label(), "f2");
    }

    #[test]
    fn test_map_shapes_empty_parent() {
        let result = map_shapes(&[], TopoShapeType::Solid);
        assert!(result.is_empty());
    }

    #[test]
    fn test_map_shapes_no_match() {
        let parent = vec![make_stub("v1", TopoShapeType::Vertex)];
        let result = map_shapes(&parent, TopoShapeType::Face);
        assert!(result.is_empty());
    }

    #[test]
    fn test_map_shapes_all_match() {
        let parent = vec![
            make_stub("s1", TopoShapeType::Solid),
            make_stub("s2", TopoShapeType::Solid),
        ];
        let result = map_shapes(&parent, TopoShapeType::Solid);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_map_shapes_does_not_mutate_parent() {
        let parent = vec![
            make_stub("e1", TopoShapeType::Edge),
            make_stub("e2", TopoShapeType::Edge),
        ];
        let _ = map_shapes(&parent, TopoShapeType::Edge);
        assert_eq!(parent.len(), 2); // parent slice unchanged
    }

    #[test]
    fn test_map_shapes_clones_stubs() {
        let parent = vec![make_stub("w1", TopoShapeType::Wire)];
        let result = map_shapes(&parent, TopoShapeType::Wire);
        assert_eq!(result[0], parent[0]);
    }

    // all nine shape types exercised

    #[test]
    fn test_all_shape_types_in_explorer() {
        let all_types = [
            TopoShapeType::Compound,
            TopoShapeType::CompSolid,
            TopoShapeType::Solid,
            TopoShapeType::Shell,
            TopoShapeType::Face,
            TopoShapeType::Wire,
            TopoShapeType::Edge,
            TopoShapeType::Vertex,
            TopoShapeType::Shape,
        ];
        for t in all_types {
            let mut ex = TopoExplorer::new();
            ex.init(vec![make_stub("x", t)], t);
            assert!(ex.more(), "should have one shape for type {t:?}");
            assert_eq!(ex.nb_shapes(), 1);
        }
    }
}

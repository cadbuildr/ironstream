// FILE: rust/ironstream/crates/ironstream/tests/occt_topo_explorer.rs
extern crate ironstream;
use ironstream::topo_explorer::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn stub(label: &str, t: TopoShapeType) -> TopoShapeStub {
    TopoShapeStub::new(label, t)
}

fn faces(n: usize) -> Vec<TopoShapeStub> {
    (0..n).map(|i| stub(&format!("f{i}"), TopoShapeType::Face)).collect()
}

fn edges(n: usize) -> Vec<TopoShapeStub> {
    (0..n).map(|i| stub(&format!("e{i}"), TopoShapeType::Edge)).collect()
}

// ---------------------------------------------------------------------------
// TopoShapeType
// ---------------------------------------------------------------------------

#[test]
fn test_shape_type_eq() {
    assert_eq!(TopoShapeType::Face, TopoShapeType::Face);
    assert_ne!(TopoShapeType::Face, TopoShapeType::Edge);
}

#[test]
fn test_shape_type_copy_semantics() {
    let t = TopoShapeType::Solid;
    let u = t; // Copy
    assert_eq!(t, u);
}

#[test]
fn test_all_nine_variants_distinct() {
    let all = [
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
    for i in 0..all.len() {
        for j in 0..all.len() {
            if i == j {
                assert_eq!(all[i], all[j]);
            } else {
                assert_ne!(all[i], all[j]);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// TopoShapeStub
// ---------------------------------------------------------------------------

#[test]
fn test_stub_new_label() {
    let s = stub("my_face", TopoShapeType::Face);
    assert_eq!(s.label(), "my_face");
}

#[test]
fn test_stub_new_shape_type() {
    let s = stub("my_edge", TopoShapeType::Edge);
    assert_eq!(s.shape_type(), TopoShapeType::Edge);
}

#[test]
fn test_stub_clone_equals_original() {
    let s = stub("v0", TopoShapeType::Vertex);
    assert_eq!(s.clone(), s);
}

#[test]
fn test_stub_equality_by_label_and_type() {
    let a = stub("x", TopoShapeType::Wire);
    let b = stub("x", TopoShapeType::Wire);
    let c = stub("x", TopoShapeType::Face);
    let d = stub("y", TopoShapeType::Wire);
    assert_eq!(a, b);
    assert_ne!(a, c);
    assert_ne!(a, d);
}

#[test]
fn test_stub_empty_label() {
    let s = stub("", TopoShapeType::Shape);
    assert_eq!(s.label(), "");
    assert_eq!(s.shape_type(), TopoShapeType::Shape);
}

// ---------------------------------------------------------------------------
// TopoExplorer::new
// ---------------------------------------------------------------------------

#[test]
fn test_new_more_is_false() {
    let ex = TopoExplorer::new();
    assert!(!ex.more());
}

#[test]
fn test_new_nb_shapes_is_zero() {
    let ex = TopoExplorer::new();
    assert_eq!(ex.nb_shapes(), 0);
}

#[test]
fn test_default_equivalent_to_new() {
    let ex: TopoExplorer = Default::default();
    assert!(!ex.more());
    assert_eq!(ex.nb_shapes(), 0);
}

// ---------------------------------------------------------------------------
// TopoExplorer::init
// ---------------------------------------------------------------------------

#[test]
fn test_init_filters_matching_type() {
    let mut ex = TopoExplorer::new();
    let mut shapes = faces(3);
    shapes.extend(edges(2));
    ex.init(shapes, TopoShapeType::Face);
    assert_eq!(ex.nb_shapes(), 3);
}

#[test]
fn test_init_filters_out_non_matching() {
    let mut ex = TopoExplorer::new();
    ex.init(edges(4), TopoShapeType::Face);
    assert_eq!(ex.nb_shapes(), 0);
    assert!(!ex.more());
}

#[test]
fn test_init_all_match() {
    let mut ex = TopoExplorer::new();
    ex.init(edges(5), TopoShapeType::Edge);
    assert_eq!(ex.nb_shapes(), 5);
    assert!(ex.more());
}

#[test]
fn test_init_resets_cursor_to_start() {
    let mut ex = TopoExplorer::new();
    ex.init(faces(3), TopoShapeType::Face);
    ex.next();
    ex.next();
    // re-init should put cursor back at 0
    ex.init(faces(3), TopoShapeType::Face);
    assert_eq!(ex.current().label(), "f0");
}

#[test]
fn test_init_replaces_previous_shapes() {
    let mut ex = TopoExplorer::new();
    ex.init(faces(2), TopoShapeType::Face);
    assert_eq!(ex.nb_shapes(), 2);
    ex.init(edges(5), TopoShapeType::Edge);
    assert_eq!(ex.nb_shapes(), 5);
}

#[test]
fn test_init_empty_input() {
    let mut ex = TopoExplorer::new();
    ex.init(vec![], TopoShapeType::Solid);
    assert!(!ex.more());
    assert_eq!(ex.nb_shapes(), 0);
}

// ---------------------------------------------------------------------------
// TopoExplorer::more / next / current
// ---------------------------------------------------------------------------

#[test]
fn test_more_true_at_start() {
    let mut ex = TopoExplorer::new();
    ex.init(faces(1), TopoShapeType::Face);
    assert!(ex.more());
}

#[test]
fn test_more_false_after_last_next() {
    let mut ex = TopoExplorer::new();
    ex.init(faces(2), TopoShapeType::Face);
    ex.next();
    ex.next();
    assert!(!ex.more());
}

#[test]
fn test_next_does_not_panic_when_exhausted() {
    let mut ex = TopoExplorer::new();
    ex.init(faces(1), TopoShapeType::Face);
    ex.next(); // exhaust
    ex.next(); // extra call — must not panic
    assert!(!ex.more());
}

#[test]
fn test_current_label_first() {
    let mut ex = TopoExplorer::new();
    ex.init(
        vec![stub("first", TopoShapeType::Edge), stub("second", TopoShapeType::Edge)],
        TopoShapeType::Edge,
    );
    assert_eq!(ex.current().label(), "first");
}

#[test]
fn test_current_label_after_next() {
    let mut ex = TopoExplorer::new();
    ex.init(
        vec![stub("a", TopoShapeType::Wire), stub("b", TopoShapeType::Wire)],
        TopoShapeType::Wire,
    );
    ex.next();
    assert_eq!(ex.current().label(), "b");
}

#[test]
fn test_current_shape_type_matches_filter() {
    let mut ex = TopoExplorer::new();
    ex.init(faces(3), TopoShapeType::Face);
    while ex.more() {
        assert_eq!(ex.current().shape_type(), TopoShapeType::Face);
        ex.next();
    }
}

#[test]
#[should_panic]
fn test_current_panics_past_end() {
    let mut ex = TopoExplorer::new();
    ex.init(faces(1), TopoShapeType::Face);
    ex.next();
    let _ = ex.current();
}

#[test]
fn test_full_iteration_order_preserved() {
    let mut ex = TopoExplorer::new();
    let input = vec![
        stub("x0", TopoShapeType::Vertex),
        stub("x1", TopoShapeType::Vertex),
        stub("x2", TopoShapeType::Vertex),
    ];
    ex.init(input, TopoShapeType::Vertex);
    let mut labels = Vec::new();
    while ex.more() {
        labels.push(ex.current().label().to_string());
        ex.next();
    }
    assert_eq!(labels, ["x0", "x1", "x2"]);
}

// ---------------------------------------------------------------------------
// TopoExplorer::reset
// ---------------------------------------------------------------------------

#[test]
fn test_reset_after_exhaustion() {
    let mut ex = TopoExplorer::new();
    ex.init(faces(2), TopoShapeType::Face);
    ex.next();
    ex.next();
    assert!(!ex.more());
    ex.reset();
    assert!(ex.more());
    assert_eq!(ex.current().label(), "f0");
}

#[test]
fn test_reset_mid_iteration() {
    let mut ex = TopoExplorer::new();
    ex.init(
        vec![
            stub("s1", TopoShapeType::Shell),
            stub("s2", TopoShapeType::Shell),
            stub("s3", TopoShapeType::Shell),
        ],
        TopoShapeType::Shell,
    );
    ex.next();
    ex.reset();
    assert_eq!(ex.current().label(), "s1");
}

#[test]
fn test_reset_on_empty_explorer() {
    let mut ex = TopoExplorer::new();
    ex.reset(); // must not panic
    assert!(!ex.more());
}

// ---------------------------------------------------------------------------
// TopoExplorer::nb_shapes
// ---------------------------------------------------------------------------

#[test]
fn test_nb_shapes_reflects_filter() {
    let mut ex = TopoExplorer::new();
    let mut shapes = faces(4);
    shapes.extend(edges(6));
    ex.init(shapes, TopoShapeType::Edge);
    assert_eq!(ex.nb_shapes(), 6);
}

#[test]
fn test_nb_shapes_stable_during_iteration() {
    let mut ex = TopoExplorer::new();
    ex.init(faces(3), TopoShapeType::Face);
    ex.next();
    assert_eq!(ex.nb_shapes(), 3); // count does not change as we iterate
}

// ---------------------------------------------------------------------------
// map_shapes (free function)
// ---------------------------------------------------------------------------

#[test]
fn test_map_shapes_basic() {
    let parent = vec![
        stub("f0", TopoShapeType::Face),
        stub("e0", TopoShapeType::Edge),
        stub("f1", TopoShapeType::Face),
    ];
    let result = map_shapes(&parent, TopoShapeType::Face);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].label(), "f0");
    assert_eq!(result[1].label(), "f1");
}

#[test]
fn test_map_shapes_no_match_returns_empty() {
    let parent = vec![stub("v0", TopoShapeType::Vertex)];
    assert!(map_shapes(&parent, TopoShapeType::Solid).is_empty());
}

#[test]
fn test_map_shapes_empty_parent() {
    assert!(map_shapes(&[], TopoShapeType::Face).is_empty());
}

#[test]
fn test_map_shapes_all_match() {
    let parent: Vec<_> = (0..4).map(|i| stub(&format!("c{i}"), TopoShapeType::Compound)).collect();
    let result = map_shapes(&parent, TopoShapeType::Compound);
    assert_eq!(result.len(), 4);
}

#[test]
fn test_map_shapes_returns_clones_not_references() {
    let parent = vec![stub("w0", TopoShapeType::Wire)];
    let result = map_shapes(&parent, TopoShapeType::Wire);
    assert_eq!(result[0], parent[0]);
}

#[test]
fn test_map_shapes_does_not_mutate_parent() {
    let parent = vec![stub("e0", TopoShapeType::Edge), stub("e1", TopoShapeType::Edge)];
    let _ = map_shapes(&parent, TopoShapeType::Edge);
    assert_eq!(parent.len(), 2);
}

#[test]
fn test_map_shapes_preserves_order() {
    let parent = vec![
        stub("z", TopoShapeType::Solid),
        stub("a", TopoShapeType::Solid),
        stub("m", TopoShapeType::Solid),
    ];
    let result = map_shapes(&parent, TopoShapeType::Solid);
    let labels: Vec<_> = result.iter().map(|s| s.label()).collect();
    assert_eq!(labels, ["z", "a", "m"]);
}

// ---------------------------------------------------------------------------
// Integration: map_shapes -> TopoExplorer round-trip
// ---------------------------------------------------------------------------

#[test]
fn test_map_then_explore() {
    let parent = vec![
        stub("f0", TopoShapeType::Face),
        stub("e0", TopoShapeType::Edge),
        stub("f1", TopoShapeType::Face),
        stub("e1", TopoShapeType::Edge),
        stub("f2", TopoShapeType::Face),
    ];
    let mapped = map_shapes(&parent, TopoShapeType::Face);
    let mut ex = TopoExplorer::new();
    ex.init(mapped, TopoShapeType::Face);
    assert_eq!(ex.nb_shapes(), 3);
    let mut count = 0;
    while ex.more() {
        assert_eq!(ex.current().shape_type(), TopoShapeType::Face);
        count += 1;
        ex.next();
    }
    assert_eq!(count, 3);
}

#[test]
fn test_reinit_multiple_times() {
    let mut ex = TopoExplorer::new();
    for _ in 0..3 {
        ex.init(faces(2), TopoShapeType::Face);
        let mut n = 0;
        while ex.more() {
            n += 1;
            ex.next();
        }
        assert_eq!(n, 2);
    }
}

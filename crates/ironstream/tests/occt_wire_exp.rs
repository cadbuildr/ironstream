extern crate ironstream;
use ironstream::wire_exp::*;

#[test]
fn wire_explorer_init_with_empty_edges_more_returns_false() {
    let mut exp = WireExplorer::new();
    exp.init(1, None, vec![]);
    assert!(!exp.more());
    assert_eq!(exp.nb_edges(), 0);
    assert!(exp.current_edge().is_none());
    assert!(exp.current_vertex().is_none());
}

#[test]
fn wire_explorer_current_vertex_returns_first_vertex_id() {
    let edges = vec![
        ExploredEdge::new(1, EdgeOrientation::Forward, 100, 101),
        ExploredEdge::new(2, EdgeOrientation::Forward, 101, 102),
    ];
    let mut exp = WireExplorer::new();
    exp.init(10, None, edges);
    assert_eq!(exp.current_vertex(), Some(100));
    exp.next();
    assert_eq!(exp.current_vertex(), Some(101));
}

#[test]
fn wire_explorer_nb_edges_count() {
    let edges = vec![
        ExploredEdge::new(1, EdgeOrientation::Forward, 1, 2),
        ExploredEdge::new(2, EdgeOrientation::Reversed, 2, 3),
        ExploredEdge::new(3, EdgeOrientation::Forward, 3, 4),
        ExploredEdge::new(4, EdgeOrientation::Forward, 4, 1),
    ];
    let mut exp = WireExplorer::new();
    exp.init(99, Some(5), edges);
    assert_eq!(exp.nb_edges(), 4);
    assert!(exp.is_closed());
}

#[test]
fn reshape_replace_with_orientation_stores_orientation() {
    let mut rs = ReShape::new();
    rs.replace_with_orientation(10, 20, EdgeOrientation::Reversed);
    assert_eq!(rs.nb_substitutions(), 1);
    assert!(rs.has_replace(10));
    let sub = &rs.substitutions[0];
    assert_eq!(sub.orientation, EdgeOrientation::Reversed);
    assert_eq!(sub.new_id, Some(20));
}

#[test]
fn reshape_nb_substitutions_count() {
    let mut rs = ReShape::new();
    assert_eq!(rs.nb_substitutions(), 0);
    rs.replace(1, 2);
    rs.replace(3, 4);
    rs.remove(5);
    assert_eq!(rs.nb_substitutions(), 3);
    assert_eq!(rs.apply(1), ReShapeStatus::Replaced);
    assert_eq!(rs.apply(5), ReShapeStatus::Removed);
}

#[test]
fn reshape_clear_empties_substitutions() {
    let mut rs = ReShape::new();
    rs.replace(1, 2);
    rs.replace(3, 4);
    rs.remove(5);
    assert_eq!(rs.nb_substitutions(), 3);
    rs.clear();
    assert_eq!(rs.nb_substitutions(), 0);
    // After clearing, apply returns NoChange
    assert_eq!(rs.apply(1), ReShapeStatus::NoChange);
}

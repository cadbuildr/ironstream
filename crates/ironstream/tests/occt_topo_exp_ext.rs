// FILE: tests/occt_topo_exp_ext.rs
extern crate ironstream;

use ironstream::topo_exp_ext::{
    Orientation, ExploredShape, TopExpExplorerExt, WireExplorerExt, TopExp,
};

#[test]
fn orientation_reverse_and_predicates() {
    assert_eq!(Orientation::Forward.reverse(), Orientation::Reversed);
    assert_eq!(Orientation::Reversed.reverse(), Orientation::Forward);
    assert_eq!(Orientation::Internal.reverse(), Orientation::Internal);
    assert_eq!(Orientation::External.reverse(), Orientation::External);
    assert!(Orientation::Forward.is_forward());
    assert!(!Orientation::Forward.is_reversed());
    assert!(Orientation::Reversed.is_reversed());
}

#[test]
fn explorer_filters_by_seek_type() {
    let mut exp = TopExpExplorerExt::new(1, 3); // seek faces (type=3)
    exp.init(vec![
        (10, 3, Orientation::Forward),
        (11, 5, Orientation::Forward), // edge — filtered out
        (12, 3, Orientation::Reversed),
        (13, 6, Orientation::Forward), // vertex — filtered out
    ]);
    assert_eq!(exp.nb_entries(), 2);
    let ids = exp.collect_all();
    assert_eq!(ids, vec![10, 12]);
}

#[test]
fn explorer_more_next_reset() {
    let mut exp = TopExpExplorerExt::new(1, 5); // seek edges
    exp.init(vec![
        (20, 5, Orientation::Forward),
        (21, 5, Orientation::Reversed),
    ]);
    assert!(exp.more());
    assert_eq!(exp.current().map(|e| e.shape_id), Some(20));
    exp.next();
    assert_eq!(exp.current().map(|e| e.shape_id), Some(21));
    exp.next();
    assert!(!exp.more());
    exp.reset();
    assert!(exp.more());
    assert_eq!(exp.current().map(|e| e.shape_id), Some(20));
}

#[test]
fn wire_explorer_ordered_edges() {
    let mut we = WireExplorerExt::new(99);
    we.init(vec![
        (1, Orientation::Forward),
        (2, Orientation::Reversed),
        (3, Orientation::Forward),
    ]);
    assert_eq!(we.nb_edges(), 3);
    assert_eq!(we.current_edge(), Some(1));
    assert_eq!(we.current_orientation(), Some(Orientation::Forward));
    we.next();
    assert_eq!(we.current_edge(), Some(2));
    assert_eq!(we.current_orientation(), Some(Orientation::Reversed));
    we.next();
    we.next();
    assert!(!we.more());
    we.reset();
    assert!(we.more());
}

#[test]
fn top_exp_vertices_of_edge() {
    let (v1, v2) = TopExp::vertices_of_edge(5);
    assert_eq!(v1, 50);
    assert_eq!(v2, 51);
    let (v1b, v2b) = TopExp::vertices_of_edge(1);
    assert_eq!(v1b, 10);
    assert_eq!(v2b, 11);
}

#[test]
fn top_exp_map_shapes_and_ancestors() {
    let result = TopExp::map_shapes_and_ancestors(3, 5, 3); // child >= parent — empty
    assert!(result.is_empty());
    let result2 = TopExp::map_shapes_and_ancestors(3, 3, 5); // valid
    assert_eq!(result2.len(), 2);
    assert_eq!(result2[0].0, 3 * 10 + 1);
}

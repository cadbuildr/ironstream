extern crate ironstream;
use ironstream::topo_exp::*;

#[test]
fn test_topo_exp_basic() {
    // map_edges returns a non-empty list for any root.
    let edges = map_edges("my_box");
    assert!(!edges.is_empty());
    // Labels should contain the root and the type prefix.
    assert!(edges[0].contains("my_box"));
    assert!(edges[0].contains("edge"));

    // map_faces / map_vertices convenience wrappers.
    let faces = map_faces("my_box");
    let verts = map_vertices("my_box");
    assert!(!faces.is_empty());
    assert!(!verts.is_empty());
    // Typically more vertices than faces.
    assert!(verts.len() >= faces.len());

    // TopExpExplorer iteration.
    let mut exp = TopExpExplorer::new("solid1", TopAbsShapeEnum::Face);
    assert!(exp.more());
    let first = exp.current().unwrap().clone();
    assert!(first.contains("face"));
    exp.next();
    // reset goes back to the beginning.
    exp.reset();
    assert_eq!(exp.current().unwrap(), &first);

    // map_shapes for Solid type returns at least 1.
    let solids = map_shapes("compound", &TopAbsShapeEnum::Solid);
    assert!(!solids.is_empty());
}

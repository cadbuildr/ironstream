extern crate ironstream;
use ironstream::xcaf_assembly::*;

#[test]
fn test_xcaf_assembly_basic() {
    // Location: identity.
    let id = Location::identity();
    assert!(id.is_identity());

    // Translation is not identity.
    let t = Location::from_translation([1.0, 2.0, 3.0]);
    assert!(!t.is_identity());
    assert!((t.matrix[0][3] - 1.0).abs() < f64::EPSILON);
    assert!((t.matrix[1][3] - 2.0).abs() < f64::EPSILON);
    assert!((t.matrix[2][3] - 3.0).abs() < f64::EPSILON);

    // Compose two translations.
    let tx = Location::from_translation([1.0, 0.0, 0.0]);
    let ty = Location::from_translation([0.0, 2.0, 0.0]);
    let combined = tx.compose(&ty);
    assert!((combined.matrix[0][3] - 1.0).abs() < 1e-12);
    assert!((combined.matrix[1][3] - 2.0).abs() < 1e-12);

    // AssemblyNode: leaf and child management.
    let mut node = AssemblyNode::new("Part1");
    assert_eq!(node.label, "Part1");
    assert!(node.is_leaf());
    node.add_child(1);
    assert!(!node.is_leaf());

    // AssemblyGraph: add nodes and roots, check depth.
    let mut graph = AssemblyGraph::new();
    let leaf1 = graph.add_node(AssemblyNode::new("Leaf1"));
    let leaf2 = graph.add_node(AssemblyNode::new("Leaf2"));
    let mut root_node = AssemblyNode::new("Root");
    root_node.add_child(leaf1);
    root_node.add_child(leaf2);
    let root_idx = graph.add_node(root_node);
    graph.add_root(root_idx);

    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.depth(), 2);
}

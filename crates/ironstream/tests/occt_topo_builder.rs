extern crate ironstream;
use ironstream::topo_builder::*;

#[test]
fn test_topo_builder_basic() {
    let v1 = TopoBuilder::make_vertex([0.0, 0.0, 0.0]);
    let v2 = TopoBuilder::make_vertex([1.0, 0.0, 0.0]);
    assert!(v1.starts_with("vertex:"));

    let edge = TopoBuilder::make_edge(&v1, &v2);
    assert!(edge.starts_with("edge:"));

    let wire = TopoBuilder::make_wire(&[edge.clone()]);
    assert!(wire.starts_with("wire:"));

    let face = TopoBuilder::make_face(&wire);
    assert!(face.starts_with("face:"));

    let shell = TopoBuilder::make_shell(&[face.clone()]);
    assert!(shell.starts_with("shell:"));

    let solid = TopoBuilder::make_solid(&shell);
    assert!(solid.starts_with("solid:"));

    let compound = TopoBuilder::make_compound(&[solid.clone()]);
    assert!(compound.starts_with("compound:"));

    let mut bb = BrepBuilder::new();
    let i0 = bb.add_vertex([0.0, 0.0, 0.0]);
    let i1 = bb.add_vertex([1.0, 0.0, 0.0]);
    bb.add_edge(i0, i1);
    let built = bb.build();
    assert!(built.starts_with("brep:"));
}

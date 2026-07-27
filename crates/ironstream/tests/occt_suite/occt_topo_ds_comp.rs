use ironstream::topo_ds_comp::*;

#[test]
fn compound_add_remove_and_nb_children() {
    let mut c = TopoDsCompound::new();
    assert!(c.is_empty());
    c.add(TopoDsShapeEntry::new(1, TopAbsShapeEnum::Face));
    c.add(TopoDsShapeEntry::new(2, TopAbsShapeEnum::Edge));
    assert_eq!(c.nb_children(), 2);
    c.remove(1);
    assert_eq!(c.nb_children(), 1);
    assert!(!c.is_empty());
}

#[test]
fn compound_children_of_type() {
    let mut c = TopoDsCompound::new();
    c.add(TopoDsShapeEntry::new(1, TopAbsShapeEnum::Face));
    c.add(TopoDsShapeEntry::new(2, TopAbsShapeEnum::Face));
    c.add(TopoDsShapeEntry::new(3, TopAbsShapeEnum::Edge));
    let faces = c.children_of_type(TopAbsShapeEnum::Face);
    assert_eq!(faces.len(), 2);
    let edges = c.children_of_type(TopAbsShapeEnum::Edge);
    assert_eq!(edges.len(), 1);
}

#[test]
fn orientation_reversed_and_is_closed() {
    assert_eq!(TopAbsOrientation::Forward.reversed(), TopAbsOrientation::Reversed);
    assert_eq!(TopAbsOrientation::Reversed.reversed(), TopAbsOrientation::Forward);
    assert!(!TopAbsOrientation::Forward.is_closed());
    assert!(TopAbsOrientation::Internal.is_closed());
    assert!(TopAbsOrientation::External.is_closed());
}

#[test]
fn comp_solid_dedup() {
    let mut cs = TopoDsCompSolid::new();
    cs.add_solid(1);
    cs.add_solid(1); // dup
    cs.add_solid(2);
    assert_eq!(cs.nb_solids(), 2);
}

#[test]
fn brep_builder_make_compound_and_solid() {
    let mut b = BrepBuilder::new();
    let mut comp = b.make_compound();
    b.add(&mut comp, TopoDsShapeEntry::new(10, TopAbsShapeEnum::Solid));
    assert_eq!(comp.nb_children(), 1);
    b.remove(&mut comp, 10);
    assert_eq!(comp.nb_children(), 0);
    let solid_id = b.make_solid_from_shell(99);
    assert!(solid_id > 0);
}

#[test]
fn make_vertex_and_make_wire() {
    let mut b = BrepBuilder::new();

    let mut mv = BrepBuilderMakeVertex::new([1.0, 2.0, 3.0]);
    let id = mv.build(&mut b);
    assert!(mv.is_done());
    assert_eq!(mv.vertex_id(), id);
    assert!(id > 0);

    let mut mw = BrepBuilderMakeWire::new();
    // empty wire build should fail
    assert!(mw.build(&mut b).is_err());
    mw.add_edge(5);
    mw.add_edge(6);
    let res = mw.build(&mut b);
    assert!(res.is_ok());
    assert!(mw.is_done());
    assert_eq!(mw.nb_edges(), 2);
}

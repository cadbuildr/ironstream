use ironstream::brep_tool::*;

#[test]
fn orientation_basics() {
    assert_eq!(TopAbsOrientation::Forward.reverse(), TopAbsOrientation::Reversed);
    assert_eq!(TopAbsOrientation::Reversed.reverse(), TopAbsOrientation::Forward);
    assert!(TopAbsOrientation::Forward.is_forward());
    assert!(TopAbsOrientation::Reversed.is_reversed());
}

#[test]
fn orientation_compose() {
    let fwd = TopAbsOrientation::Forward;
    let rev = TopAbsOrientation::Reversed;
    assert_eq!(fwd.compose(rev), TopAbsOrientation::Reversed);
    assert_eq!(rev.compose(rev), TopAbsOrientation::Forward);
}

#[test]
fn shape_enum_order() {
    assert!(TopAbsShapeEnum::Compound < TopAbsShapeEnum::Solid);
    assert!(TopAbsShapeEnum::Solid.is_container());
    assert!(TopAbsShapeEnum::Edge.is_leaf());
    assert!(!TopAbsShapeEnum::Shell.is_leaf());
}

#[test]
fn topo_shape_lifecycle() {
    let n = TopoShape::null();
    assert!(n.is_null());
    let s = TopoShape::new(42, TopAbsShapeEnum::Face);
    assert!(!s.is_null());
    assert_eq!(s.shape_type(), TopAbsShapeEnum::Face);
    let r = s.reversed();
    assert_eq!(r.orientation(), TopAbsOrientation::Reversed);
    assert!(s.is_same(&r));
    assert!(!s.is_equal(&r));
}

#[test]
fn brep_tool_query() {
    let (cid, f, l) = BrepToolQuery::curve(5);
    assert!(cid > 0 && f < l);
    assert!(BrepToolQuery::surface(3) > 0);
    assert_eq!(BrepToolQuery::surface(0), 0);
    assert!(BrepToolQuery::degenerated(0));
    assert!(!BrepToolQuery::degenerated(1));
    assert!(BrepToolQuery::curve_on_surface(2, 3).is_some());
    assert!(BrepToolQuery::curve_on_surface(0, 1).is_none());
}

#[test]
fn brep_builder_shapes() {
    let mut b = BrepBuilder::new();
    let solid = b.make_solid();
    let face  = b.make_face(10);
    let _edge = b.make_edge(5);
    let _vx   = b.make_vertex([1.0, 2.0, 3.0]);
    assert_eq!(solid.shape_type(), TopAbsShapeEnum::Solid);
    assert_eq!(face.shape_type(), TopAbsShapeEnum::Face);
    assert_eq!(b.nb_shapes(), 4);
}

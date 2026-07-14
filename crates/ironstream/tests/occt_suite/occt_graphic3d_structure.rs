// FILE: rust/ironstream/crates/ironstream/tests/occt_graphic3d_structure.rs

extern crate ironstream;
use ironstream::graphic3d_structure::*;

#[test]
fn test_primitive_new_is_empty() {
    let p = Graphic3dPrimitive::new();
    assert_eq!(p.nb_vertices(), 0);
    assert_eq!(p.nb_indices(), 0);
    assert!(!p.is_indexed());
}

#[test]
fn test_primitive_add_vertex() {
    let mut p = Graphic3dPrimitive::new();
    p.add_vertex([1.0, 2.0, 3.0]);
    p.add_vertex([4.0, 5.0, 6.0]);
    assert_eq!(p.nb_vertices(), 2);
    assert_eq!(p.vertex(0), [1.0, 2.0, 3.0]);
    assert_eq!(p.vertex(1), [4.0, 5.0, 6.0]);
}

#[test]
fn test_primitive_add_normal_and_color() {
    let mut p = Graphic3dPrimitive::new();
    p.add_normal([0.0, 1.0, 0.0]);
    p.add_color([1.0, 0.0, 0.0, 1.0]);
    assert_eq!(p.normals.len(), 1);
    assert_eq!(p.colors.len(), 1);
    assert_eq!(p.normals[0], [0.0, 1.0, 0.0]);
    assert_eq!(p.colors[0], [1.0, 0.0, 0.0, 1.0]);
}

#[test]
fn test_primitive_indexed() {
    let mut p = Graphic3dPrimitive::new();
    assert!(!p.is_indexed());
    p.add_index(0);
    p.add_index(1);
    p.add_index(2);
    assert!(p.is_indexed());
    assert_eq!(p.nb_indices(), 3);
}

#[test]
fn test_primitive_default() {
    let p = Graphic3dPrimitive::default();
    assert_eq!(p.nb_vertices(), 0);
    assert!(!p.is_indexed());
}

#[test]
fn test_group_new_is_empty() {
    let g = Graphic3dGroup::new(Graphic3dGroupType::PrimitivesAspect);
    assert!(g.is_empty());
    assert_eq!(g.nb_primitives(), 0);
    assert_eq!(g.group_type(), Graphic3dGroupType::PrimitivesAspect);
}

#[test]
fn test_group_add_primitive() {
    let mut g = Graphic3dGroup::new(Graphic3dGroupType::LineAspect);
    assert!(g.is_empty());
    let mut p = Graphic3dPrimitive::new();
    p.add_vertex([0.0, 0.0, 0.0]);
    g.add_primitive(p);
    assert!(!g.is_empty());
    assert_eq!(g.nb_primitives(), 1);
}

#[test]
fn test_group_clear() {
    let mut g = Graphic3dGroup::new(Graphic3dGroupType::TextAspect);
    g.add_primitive(Graphic3dPrimitive::new());
    assert!(!g.is_empty());
    g.clear();
    assert!(g.is_empty());
    assert_eq!(g.nb_primitives(), 0);
}

#[test]
fn test_group_type_variants() {
    let g1 = Graphic3dGroup::new(Graphic3dGroupType::PrimitivesAspect);
    let g2 = Graphic3dGroup::new(Graphic3dGroupType::TextAspect);
    let g3 = Graphic3dGroup::new(Graphic3dGroupType::MarkerAspect);
    let g4 = Graphic3dGroup::new(Graphic3dGroupType::LineAspect);
    assert_eq!(g1.group_type(), Graphic3dGroupType::PrimitivesAspect);
    assert_eq!(g2.group_type(), Graphic3dGroupType::TextAspect);
    assert_eq!(g3.group_type(), Graphic3dGroupType::MarkerAspect);
    assert_eq!(g4.group_type(), Graphic3dGroupType::LineAspect);
}

#[test]
fn test_structure_new() {
    let s = Graphic3dStructure::new(42);
    assert_eq!(s.id(), 42);
    assert_eq!(s.nb_groups(), 0);
    assert!(s.is_visible());
    assert_eq!(s.priority(), 0);
}

#[test]
fn test_structure_identity_transform() {
    let s = Graphic3dStructure::new(1);
    let identity = [
        [1.0f32, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    assert_eq!(s.transform, identity);
}

#[test]
fn test_structure_new_group_and_access() {
    let mut s = Graphic3dStructure::new(1);
    let idx0 = s.new_group(Graphic3dGroupType::PrimitivesAspect);
    let idx1 = s.new_group(Graphic3dGroupType::LineAspect);
    assert_eq!(idx0, 0);
    assert_eq!(idx1, 1);
    assert_eq!(s.nb_groups(), 2);
    assert_eq!(s.group(0).group_type(), Graphic3dGroupType::PrimitivesAspect);
    assert_eq!(s.group(1).group_type(), Graphic3dGroupType::LineAspect);
}

#[test]
fn test_structure_set_visible() {
    let mut s = Graphic3dStructure::new(1);
    assert!(s.is_visible());
    s.set_visible(false);
    assert!(!s.is_visible());
    s.set_visible(true);
    assert!(s.is_visible());
}

#[test]
fn test_structure_set_priority() {
    let mut s = Graphic3dStructure::new(1);
    assert_eq!(s.priority(), 0);
    s.set_priority(5);
    assert_eq!(s.priority(), 5);
    s.set_priority(-3);
    assert_eq!(s.priority(), -3);
}

#[test]
fn test_structure_group_mutation() {
    let mut s = Graphic3dStructure::new(10);
    let idx = s.new_group(Graphic3dGroupType::MarkerAspect);
    let mut prim = Graphic3dPrimitive::new();
    prim.add_vertex([1.0, 2.0, 3.0]);
    prim.add_index(0);
    s.groups[idx].add_primitive(prim);
    assert_eq!(s.group(idx).nb_primitives(), 1);
    assert!(!s.group(idx).is_empty());
    assert!(s.group(idx).primitives[0].is_indexed());
}

#[test]
fn test_structure_multiple_groups_independent() {
    let mut s = Graphic3dStructure::new(7);
    let i0 = s.new_group(Graphic3dGroupType::PrimitivesAspect);
    let i1 = s.new_group(Graphic3dGroupType::TextAspect);
    s.groups[i0].add_primitive(Graphic3dPrimitive::new());
    assert!(!s.group(i0).is_empty());
    assert!(s.group(i1).is_empty());
}

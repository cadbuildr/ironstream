// FILE: tests/occt_graphic3d_group.rs
extern crate ironstream;

use ironstream::graphic3d_group::{
    ArrayOfPrimitives, Graphic3dGroup, Graphic3dStructure, PrimitiveType, Vertex3d,
};

#[test]
fn array_of_primitives_add_vertex_and_nb_primitives() {
    let mut arr = ArrayOfPrimitives::new(PrimitiveType::Triangles, 3);
    arr.add_vertex(Vertex3d::new([0.0, 0.0, 0.0]));
    arr.add_vertex(Vertex3d::new([1.0, 0.0, 0.0]));
    arr.add_vertex(Vertex3d::new([0.5, 1.0, 0.0]));
    arr.add_triangle(0, 1, 2);
    assert_eq!(arr.nb_vertices(), 3);
    assert_eq!(arr.nb_primitives(), 1);
    assert!(arr.is_indexed());
}

#[test]
fn graphic3d_group_total_vertices() {
    let mut grp = Graphic3dGroup::new(0);
    let mut arr = ArrayOfPrimitives::new(PrimitiveType::Points, 2);
    arr.add_vertex(Vertex3d::new([0.0, 0.0, 0.0]));
    arr.add_vertex(Vertex3d::new([1.0, 0.0, 0.0]));
    grp.add_primitive_array(arr);
    assert_eq!(grp.total_vertices(), 2);
    assert!(!grp.is_empty());
}

#[test]
fn graphic3d_structure_new_group_and_highlight() {
    let mut st = Graphic3dStructure::new(1);
    let gid = st.new_group();
    assert_eq!(st.nb_groups(), 1);
    assert_eq!(gid, 0);
    assert!(st.group(gid).is_some());

    st.highlight([1.0, 1.0, 0.0, 1.0]);
    assert!(st.is_highlighted());

    st.unhighlight();
    assert!(!st.is_highlighted());
}

#[test]
fn graphic3d_structure_erase_display() {
    let mut st = Graphic3dStructure::new(0);
    assert!(st.is_visible);
    st.erase();
    assert!(!st.is_visible);
    st.display();
    assert!(st.is_visible);
}

#[test]
fn array_bounding_box() {
    let mut arr = ArrayOfPrimitives::new(PrimitiveType::Points, 2);
    arr.add_vertex(Vertex3d::new([0.0, 0.0, 0.0]));
    arr.add_vertex(Vertex3d::new([3.0, 4.0, 5.0]));
    let (mn, mx) = arr.bounding_box();
    assert!((mn[0]).abs() < 1e-6);
    assert!((mx[0] - 3.0).abs() < 1e-6);
    assert!((mx[1] - 4.0).abs() < 1e-6);
}

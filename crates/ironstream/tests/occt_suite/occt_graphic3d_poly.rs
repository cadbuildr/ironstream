use ironstream::graphic3d_poly::*;

#[test]
fn array_of_primitives_add_vertex_add_edge_is_indexed() {
    let mut arr = Graphic3dArrayOfPrimitives::new(
        Graphic3dPrimitiveType::Triangles,
        VertexFlags::default(),
    );
    let i0 = arr.add_vertex(Graphic3dVertex::from_pos([0.0, 0.0, 0.0]));
    let i1 = arr.add_vertex(Graphic3dVertex::from_pos([1.0, 0.0, 0.0]));
    let i2 = arr.add_vertex(Graphic3dVertex::from_pos([0.5, 1.0, 0.0]));
    arr.add_edge(i0);
    arr.add_edge(i1);
    arr.add_edge(i2);
    assert_eq!(arr.nb_vertices(), 3);
    assert_eq!(arr.nb_edges(), 3);
    assert!(arr.is_indexed());
    assert_eq!(arr.vertex_position(1), Some([1.0, 0.0, 0.0]));
}

#[test]
fn array_of_polygons_add_vertex_and_bound() {
    let mut p = Graphic3dArrayOfPolygons::new(true, false);
    p.add_vertex([0.0, 0.0, 0.0]);
    p.add_vertex([2.0, 0.0, 0.0]);
    p.add_vertex([1.0, 2.0, 0.0]);
    p.add_bound(3);
    assert_eq!(p.nb_vertices(), 3);
    assert_eq!(p.nb_bounds(), 1);
}

#[test]
fn array_of_polylines_add_vertices_and_bound() {
    let mut pl = Graphic3dArrayOfPolylines::new(false);
    pl.add_vertex([0.0, 0.0, 0.0]);
    pl.add_vertex([1.0, 0.0, 0.0]);
    pl.add_vertex([2.0, 0.0, 0.0]);
    pl.add_bound(3);
    assert_eq!(pl.nb_vertices(), 3);
    assert_eq!(pl.nb_bounds(), 1);
}

#[test]
fn array_of_points_add_multiple_vertices() {
    let mut pts = Graphic3dArrayOfPoints::new(false);
    for i in 0..5 {
        pts.add_vertex([i as f32, 0.0, 0.0]);
    }
    assert_eq!(pts.nb_vertices(), 5);
}

#[test]
fn graphic3d_vertex_builder_with_normal_and_tex() {
    let v = Graphic3dVertex::from_pos([1.0, 2.0, 3.0])
        .with_normal([0.0, 0.0, 1.0])
        .with_tex([0.25, 0.75]);
    assert_eq!(v.position, [1.0, 2.0, 3.0]);
    assert_eq!(v.normal, [0.0, 0.0, 1.0]);
    assert_eq!(v.tex_coord, [0.25, 0.75]);
}

#[test]
fn empty_primitive_array_not_indexed() {
    let arr = Graphic3dArrayOfPrimitives::new(
        Graphic3dPrimitiveType::Points,
        VertexFlags::default(),
    );
    assert_eq!(arr.nb_vertices(), 0);
    assert_eq!(arr.nb_edges(), 0);
    assert!(!arr.is_indexed());
}

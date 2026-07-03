// FILE: tests/occt_poly_triangulation.rs
extern crate ironstream;
use ironstream::poly_triangulation::*;

#[test]
fn triangle_indices_and_contains_node() {
    let t = Triangle::new(0, 1, 2);
    assert_eq!(t.indices(), [0, 1, 2]);
    assert!(t.contains_node(1));
    assert!(!t.contains_node(5));
}

#[test]
fn triangulation_add_nodes_and_triangles() {
    let mut tri = Triangulation::new();
    let i0 = tri.add_node([0.0, 0.0, 0.0]);
    let i1 = tri.add_node([1.0, 0.0, 0.0]);
    let i2 = tri.add_node([0.0, 1.0, 0.0]);
    tri.add_triangle(Triangle::new(i0, i1, i2));
    assert_eq!(tri.nb_nodes(), 3);
    assert_eq!(tri.nb_triangles(), 1);
}

#[test]
fn triangulation_triangle_area() {
    let mut tri = Triangulation::new();
    tri.add_node([0.0, 0.0, 0.0]);
    tri.add_node([2.0, 0.0, 0.0]);
    tri.add_node([0.0, 2.0, 0.0]);
    let t = Triangle::new(0, 1, 2);
    assert!((tri.triangle_area(&t) - 2.0).abs() < 1e-10);
}

#[test]
fn triangulation_from_quad_two_triangles() {
    let t = Triangulation::from_quad(
        [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]
    );
    assert_eq!(t.nb_nodes(), 4);
    assert_eq!(t.nb_triangles(), 2);
    assert!((t.total_area() - 1.0).abs() < 1e-10);
}

#[test]
fn triangulation_bounding_box() {
    let t = Triangulation::from_quad(
        [0.0, 0.0, 0.0], [5.0, 0.0, 0.0], [5.0, 3.0, 0.0], [0.0, 3.0, 0.0]
    );
    let (mn, mx) = t.bounding_box().unwrap();
    assert!((mx[0] - 5.0).abs() < 1e-10);
    assert!((mx[1] - 3.0).abs() < 1e-10);
    assert!(mn[0].abs() < 1e-10);
}

#[test]
fn triangulation_flip_normals() {
    let mut tri = Triangulation::new();
    tri.add_node([0.0, 0.0, 0.0]);
    tri.add_node([1.0, 0.0, 0.0]);
    tri.add_node([0.0, 1.0, 0.0]);
    tri.add_normal([0.0, 0.0, 1.0]);
    tri.add_normal([0.0, 0.0, 1.0]);
    tri.add_normal([0.0, 0.0, 1.0]);
    tri.add_triangle(Triangle::new(0, 1, 2));
    tri.flip_normals();
    // After flip, normal z should be -1
    assert!((tri.normals[0][2] + 1.0).abs() < 1e-10);
    // Triangle winding reversed
    assert_eq!(tri.triangles[0].n2, 2);
    assert_eq!(tri.triangles[0].n3, 1);
}

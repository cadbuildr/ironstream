extern crate ironstream;

use ironstream::ppoly::{PPolyPolygon3D, PPolyTriangle, PPolyTriangulation};

#[test]
fn triangle_has_vertex() {
    let t = PPolyTriangle::new(3, 7, 11);
    assert!(t.has_vertex(7));
    assert!(!t.has_vertex(0));
    assert_eq!(t.indices(), [3, 7, 11]);
}

#[test]
fn polygon3d_length_and_params() {
    let nodes = vec![[0.0,0.0,0.0],[3.0,4.0,0.0]];
    let poly = PPolyPolygon3D::new(nodes, 0.005).with_parameters(vec![0.0, 5.0]);
    assert_eq!(poly.nb_nodes(), 2);
    assert!((poly.length() - 5.0).abs() < 1e-10);
    assert_eq!(poly.parameters.len(), 2);
}

#[test]
fn triangulation_basic() {
    let nodes = vec![
        [0.0,0.0,0.0],[1.0,0.0,0.0],[0.5,1.0,0.0],[1.5,1.0,0.0],
    ];
    let tris = vec![PPolyTriangle::new(0,1,2), PPolyTriangle::new(1,3,2)];
    let mesh = PPolyTriangulation::new(nodes, tris);
    assert_eq!(mesh.nb_nodes(), 4);
    assert_eq!(mesh.nb_triangles(), 2);
    assert!(!mesh.has_normals());
    assert!(!mesh.has_uv());
}

#[test]
fn triangulation_compute_normals() {
    let nodes = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[0.0,1.0,0.0]];
    let tris = vec![PPolyTriangle::new(0,1,2)];
    let mut mesh = PPolyTriangulation::new(nodes, tris);
    mesh.compute_normals();
    assert!(mesh.has_normals());
    // Normal should point in +Z
    assert!(mesh.normals[0][2] > 0.5);
}

#[test]
fn triangulation_deflection() {
    let mut mesh = PPolyTriangulation::new(vec![], vec![]);
    mesh.set_deflection(0.01);
    assert!((mesh.deflection - 0.01).abs() < 1e-14);
}

extern crate ironstream;
use ironstream::geom_triangle_mesh::*;

#[test]
fn test_geom_triangle_mesh_basic() {
    // Triangle construction.
    let t = Triangle::new(0, 1, 2);
    assert!(t.contains(0));
    assert!(t.contains(2));
    assert!(!t.contains(3));

    // Mesh construction.
    let mut m = TriangulationMesh::new();
    assert_eq!(m.node_count(), 0);
    assert_eq!(m.triangle_count(), 0);
    assert!(!m.has_normals());
    assert!(!m.has_uvs());

    let i0 = m.add_node([0.0, 0.0, 0.0]);
    let i1 = m.add_node([1.0, 0.0, 0.0]);
    let i2 = m.add_node([0.0, 1.0, 0.0]);
    assert_eq!(i0, 0);
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
    assert_eq!(m.node_count(), 3);

    m.add_triangle(i0, i1, i2);
    assert_eq!(m.triangle_count(), 1);

    // Normal computation for flat XY triangle → all normals along +Z.
    m.compute_normals();
    assert!(m.has_normals());
    for n in &m.normals {
        assert!((n[2] - 1.0).abs() < 1e-12, "nz={}", n[2]);
    }
}

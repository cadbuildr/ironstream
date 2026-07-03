// FILE: tests/occt_meshvs_mesh.rs
extern crate ironstream;
use ironstream::meshvs_mesh::*;

#[test]
fn mesh_data_source_add_nodes() {
    let mut ds = MeshDataSource::new();
    ds.add_node(1, [0.0, 0.0, 0.0]);
    ds.add_node(2, [1.0, 0.0, 0.0]);
    ds.add_node(3, [0.5, 1.0, 0.0]);
    assert_eq!(ds.nb_nodes(), 3);
}

#[test]
fn mesh_data_source_find_node() {
    let mut ds = MeshDataSource::new();
    ds.add_node(42, [1.0, 2.0, 3.0]);
    let n = ds.find_node(42).unwrap();
    assert!((n.coords[1] - 2.0).abs() < 1e-10);
    assert!(ds.find_node(99).is_none());
}

#[test]
fn mesh_data_source_elements_of_type() {
    let mut ds = MeshDataSource::new();
    ds.add_element(MeshElement::new(1, vec![1, 2, 3], MeshElementType::Face));
    ds.add_element(MeshElement::new(2, vec![1, 2], MeshElementType::Edge));
    ds.add_element(MeshElement::new(3, vec![4, 5, 6], MeshElementType::Face));
    assert_eq!(ds.elements_of_type(MeshElementType::Face).len(), 2);
    assert_eq!(ds.elements_of_type(MeshElementType::Edge).len(), 1);
}

#[test]
fn mesh_data_source_bounding_box() {
    let mut ds = MeshDataSource::new();
    ds.add_node(1, [0.0, 0.0, 0.0]);
    ds.add_node(2, [5.0, 3.0, -1.0]);
    let (mn, mx) = ds.bounding_box().unwrap();
    assert!((mx[0] - 5.0).abs() < 1e-10);
    assert!((mn[2] + 1.0).abs() < 1e-10);
}

#[test]
fn meshvs_mesh_wraps_data_source() {
    let mut ds = MeshDataSource::new();
    ds.add_node(1, [0.0; 3]);
    ds.add_node(2, [1.0, 0.0, 0.0]);
    ds.add_element(MeshElement::new(1, vec![1, 2], MeshElementType::Edge));
    let m = MeshVsMesh::new("my_mesh", ds);
    assert!(m.is_visible);
    assert_eq!(m.nb_nodes(), 2);
    assert_eq!(m.nb_elements(), 1);
}

#[test]
fn mesh_drawer_builder() {
    let d = MeshDrawer::new().with_color(1.0, 0.0, 0.0).with_line_width(2.5);
    assert!((d.line_width - 2.5).abs() < 1e-6);
    assert!((d.color[0] - 1.0).abs() < 1e-6);
}

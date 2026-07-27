extern crate ironstream;
use ironstream::xcaf_mesh::*;

#[test]
fn test_vis_material_pbr_properties() {
    let pbr = VisMaterialPBR::new()
        .with_base_color(1.0, 0.0, 0.0, 0.5)
        .with_metallic(0.8)
        .with_roughness(0.3);
    assert!(pbr.is_transparent()); // alpha=0.5 < 1.0
    assert!((pbr.metallic_roughness[0] - 0.8).abs() < 1e-6);
    assert!((pbr.metallic_roughness[1] - 0.3).abs() < 1e-6);
}

#[test]
fn test_vis_material_with_pbr() {
    let pbr = VisMaterialPBR::new().with_base_color(0.0, 0.0, 1.0, 1.0);
    let m = VisMaterial::new("Blue").with_pbr(pbr);
    assert!(m.has_pbr());
    assert!(m.is_defined);
    assert!((m.base_color()[2] - 1.0).abs() < 1e-6); // blue channel
}

#[test]
fn test_document_explorer_nodes_and_leaves() {
    let mut ex = DocumentExplorer::new();
    let mut root = DocumentNode::new(1, "root");
    root.children.push(2);
    root.children.push(3);
    ex.add_node(root);
    let mut leaf1 = DocumentNode::new(2, "leaf1");
    leaf1.parent = Some(1);
    leaf1.mesh_id = Some(100);
    ex.add_node(leaf1);
    let mut leaf2 = DocumentNode::new(3, "leaf2");
    leaf2.parent = Some(1);
    ex.add_node(leaf2);
    assert_eq!(ex.nb_roots(), 1);
    assert_eq!(ex.nb_nodes(), 3);
    assert_eq!(ex.leaves().len(), 2);
    assert_eq!(ex.nodes_with_mesh().len(), 1);
}

#[test]
fn test_document_explorer_find() {
    let mut ex = DocumentExplorer::new();
    ex.add_node(DocumentNode::new(42, "mynode"));
    let n = ex.find(42).unwrap();
    assert_eq!(n.name, "mynode");
    assert!(ex.find(999).is_none());
}

#[test]
fn test_material_map() {
    let mut mm = MaterialMap::new();
    mm.add(VisMaterial::new("opaque"));
    let mut transparent = VisMaterial::new("glass");
    transparent.common_transparency = 0.5;
    mm.add(transparent);
    assert_eq!(mm.nb_materials(), 2);
    assert!(mm.find("opaque").is_some());
    assert!(mm.find("missing").is_none());
    assert_eq!(mm.transparent_count(), 1);
}

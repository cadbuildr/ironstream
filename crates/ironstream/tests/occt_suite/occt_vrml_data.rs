// FILE: rust/ironstream/crates/ironstream/tests/occt_vrml_data.rs

extern crate ironstream;
use ironstream::vrml_data::*;

// --- VrmlDataNodeType ---

#[test]
fn test_node_type_variants_are_distinct() {
    assert_ne!(VrmlDataNodeType::Shape, VrmlDataNodeType::Material);
    assert_ne!(VrmlDataNodeType::IndexedFaceSet, VrmlDataNodeType::Group);
    assert_ne!(VrmlDataNodeType::Unknown, VrmlDataNodeType::Transform);
}

#[test]
fn test_node_type_copy() {
    let t = VrmlDataNodeType::Coordinate;
    let t2 = t;
    assert_eq!(t, t2);
}

#[test]
fn test_node_type_all_variants_reachable() {
    let variants = [
        VrmlDataNodeType::Shape,
        VrmlDataNodeType::Appearance,
        VrmlDataNodeType::Material,
        VrmlDataNodeType::IndexedFaceSet,
        VrmlDataNodeType::Coordinate,
        VrmlDataNodeType::Normal,
        VrmlDataNodeType::TextureCoordinate,
        VrmlDataNodeType::Group,
        VrmlDataNodeType::Transform,
        VrmlDataNodeType::Unknown,
    ];
    assert_eq!(variants.len(), 10);
}

// --- VrmlDataMaterial ---

#[test]
fn test_material_new_stores_name() {
    let m = VrmlDataMaterial::new("mat1");
    assert_eq!(m.name(), "mat1");
}

#[test]
fn test_material_new_diffuse_zero() {
    let m = VrmlDataMaterial::new("m");
    assert_eq!(m.diffuse(), [0.0_f32; 3]);
}

#[test]
fn test_material_new_specular_zero() {
    let m = VrmlDataMaterial::new("m");
    assert_eq!(m.specular(), [0.0_f32; 3]);
}

#[test]
fn test_material_new_transparency_zero() {
    let m = VrmlDataMaterial::new("m");
    assert_eq!(m.transparency(), 0.0_f32);
}

#[test]
fn test_material_new_shininess_default() {
    let m = VrmlDataMaterial::new("m");
    assert!((m.shininess() - 0.2_f32).abs() < 1e-6);
}

#[test]
fn test_material_set_diffuse_roundtrip() {
    let mut m = VrmlDataMaterial::new("m");
    m.set_diffuse([0.8, 0.2, 0.0]);
    let d = m.diffuse();
    assert!((d[0] - 0.8).abs() < 1e-6);
    assert!((d[1] - 0.2).abs() < 1e-6);
    assert!((d[2] - 0.0).abs() < 1e-6);
}

#[test]
fn test_material_set_specular_roundtrip() {
    let mut m = VrmlDataMaterial::new("m");
    m.set_specular([1.0, 0.5, 0.25]);
    let s = m.specular();
    assert!((s[0] - 1.0).abs() < 1e-6);
    assert!((s[1] - 0.5).abs() < 1e-6);
    assert!((s[2] - 0.25).abs() < 1e-6);
}

#[test]
fn test_material_set_transparency_roundtrip() {
    let mut m = VrmlDataMaterial::new("m");
    m.set_transparency(0.5);
    assert!((m.transparency() - 0.5).abs() < 1e-6);
}

#[test]
fn test_material_set_transparency_zero() {
    let mut m = VrmlDataMaterial::new("m");
    m.set_transparency(0.9);
    m.set_transparency(0.0);
    assert_eq!(m.transparency(), 0.0);
}

#[test]
fn test_material_set_shininess_roundtrip() {
    let mut m = VrmlDataMaterial::new("m");
    m.set_shininess(0.75);
    assert!((m.shininess() - 0.75).abs() < 1e-6);
}

#[test]
fn test_material_set_shininess_zero() {
    let mut m = VrmlDataMaterial::new("m");
    m.set_shininess(0.5);
    m.set_shininess(0.0);
    assert_eq!(m.shininess(), 0.0);
}

#[test]
fn test_material_clone_is_independent() {
    let mut m = VrmlDataMaterial::new("orig");
    m.set_diffuse([0.1, 0.2, 0.3]);
    m.set_transparency(0.4);
    let m2 = m.clone();
    assert_eq!(m2.name(), "orig");
    let d = m2.diffuse();
    assert!((d[0] - 0.1).abs() < 1e-6);
    assert!((m2.transparency() - 0.4).abs() < 1e-6);
}

// --- VrmlDataIndexedFaceSet ---

#[test]
fn test_face_set_new_is_empty() {
    let fs = VrmlDataIndexedFaceSet::new();
    assert_eq!(fs.nb_coords(), 0);
    assert_eq!(fs.nb_faces(), 0);
}

#[test]
fn test_face_set_default_is_empty() {
    let fs = VrmlDataIndexedFaceSet::default();
    assert_eq!(fs.nb_coords(), 0);
    assert_eq!(fs.nb_faces(), 0);
}

#[test]
fn test_face_set_add_coord_increments_nb_coords() {
    let mut fs = VrmlDataIndexedFaceSet::new();
    fs.add_coord([1.0, 2.0, 3.0]);
    assert_eq!(fs.nb_coords(), 1);
    fs.add_coord([4.0, 5.0, 6.0]);
    assert_eq!(fs.nb_coords(), 2);
}

#[test]
fn test_face_set_add_coord_stores_values() {
    let mut fs = VrmlDataIndexedFaceSet::new();
    fs.add_coord([7.0, 8.0, 9.0]);
    let c = fs.coords[0];
    assert!((c[0] - 7.0).abs() < 1e-6);
    assert!((c[1] - 8.0).abs() < 1e-6);
    assert!((c[2] - 9.0).abs() < 1e-6);
}

#[test]
fn test_face_set_add_face_increments_nb_faces() {
    let mut fs = VrmlDataIndexedFaceSet::new();
    fs.add_face(vec![0, 1, 2]);
    assert_eq!(fs.nb_faces(), 1);
    fs.add_face(vec![0, 2, 3]);
    assert_eq!(fs.nb_faces(), 2);
}

#[test]
fn test_face_set_face_returns_indices() {
    let mut fs = VrmlDataIndexedFaceSet::new();
    fs.add_face(vec![10, 20, 30]);
    let f = fs.face(0);
    assert_eq!(f, &[10usize, 20, 30]);
}

#[test]
fn test_face_set_face_multiple() {
    let mut fs = VrmlDataIndexedFaceSet::new();
    fs.add_face(vec![0, 1, 2]);
    fs.add_face(vec![3, 4, 5, 6]);
    assert_eq!(fs.face(0), &[0usize, 1, 2]);
    assert_eq!(fs.face(1), &[3usize, 4, 5, 6]);
}

#[test]
fn test_face_set_normal_per_vertex_default_true() {
    let fs = VrmlDataIndexedFaceSet::new();
    assert!(fs.normal_per_vertex);
}

#[test]
fn test_face_set_normals_initially_empty() {
    let fs = VrmlDataIndexedFaceSet::new();
    assert!(fs.normals.is_empty());
}

#[test]
fn test_face_set_normals_stored() {
    let mut fs = VrmlDataIndexedFaceSet::new();
    fs.normals.push([0.0, 0.0, 1.0]);
    assert_eq!(fs.normals.len(), 1);
    let n = fs.normals[0];
    assert!((n[2] - 1.0).abs() < 1e-6);
}

#[test]
fn test_face_set_triangle_geometry() {
    let mut fs = VrmlDataIndexedFaceSet::new();
    fs.add_coord([0.0, 0.0, 0.0]);
    fs.add_coord([1.0, 0.0, 0.0]);
    fs.add_coord([0.0, 1.0, 0.0]);
    fs.add_face(vec![0, 1, 2]);
    assert_eq!(fs.nb_coords(), 3);
    assert_eq!(fs.nb_faces(), 1);
    assert_eq!(fs.face(0), &[0usize, 1, 2]);
}

// --- VrmlDataScene ---

#[test]
fn test_scene_new_stores_name() {
    let sc = VrmlDataScene::new("MyScene");
    assert_eq!(sc.name, "MyScene");
}

#[test]
fn test_scene_new_is_empty() {
    let sc = VrmlDataScene::new("s");
    assert_eq!(sc.nb_face_sets(), 0);
    assert_eq!(sc.nb_materials(), 0);
}

#[test]
fn test_scene_add_face_set_increments_count() {
    let mut sc = VrmlDataScene::new("s");
    sc.add_face_set(VrmlDataIndexedFaceSet::new());
    assert_eq!(sc.nb_face_sets(), 1);
    sc.add_face_set(VrmlDataIndexedFaceSet::new());
    assert_eq!(sc.nb_face_sets(), 2);
}

#[test]
fn test_scene_add_material_increments_count() {
    let mut sc = VrmlDataScene::new("s");
    sc.add_material(VrmlDataMaterial::new("m1"));
    assert_eq!(sc.nb_materials(), 1);
    sc.add_material(VrmlDataMaterial::new("m2"));
    assert_eq!(sc.nb_materials(), 2);
}

#[test]
fn test_scene_face_sets_accessible() {
    let mut sc = VrmlDataScene::new("s");
    let mut fs = VrmlDataIndexedFaceSet::new();
    fs.add_coord([1.0, 2.0, 3.0]);
    fs.add_face(vec![0]);
    sc.add_face_set(fs);
    assert_eq!(sc.face_sets[0].nb_coords(), 1);
    assert_eq!(sc.face_sets[0].nb_faces(), 1);
}

#[test]
fn test_scene_materials_accessible() {
    let mut sc = VrmlDataScene::new("s");
    let mut m = VrmlDataMaterial::new("red");
    m.set_diffuse([1.0, 0.0, 0.0]);
    sc.add_material(m);
    assert_eq!(sc.materials[0].name(), "red");
    let d = sc.materials[0].diffuse();
    assert!((d[0] - 1.0).abs() < 1e-6);
}

#[test]
fn test_scene_round_trip() {
    let mut sc = VrmlDataScene::new("scene");

    let mut m = VrmlDataMaterial::new("plastic");
    m.set_diffuse([0.0, 0.5, 1.0]);
    m.set_transparency(0.1);
    m.set_shininess(0.6);
    sc.add_material(m);

    let mut fs = VrmlDataIndexedFaceSet::new();
    fs.add_coord([0.0, 0.0, 0.0]);
    fs.add_coord([1.0, 0.0, 0.0]);
    fs.add_coord([0.0, 1.0, 0.0]);
    fs.add_coord([0.0, 0.0, 1.0]);
    fs.add_face(vec![0, 1, 2]);
    fs.add_face(vec![0, 1, 3]);
    fs.add_face(vec![0, 2, 3]);
    fs.add_face(vec![1, 2, 3]);
    sc.add_face_set(fs);

    assert_eq!(sc.nb_materials(), 1);
    assert_eq!(sc.nb_face_sets(), 1);
    assert_eq!(sc.materials[0].name(), "plastic");
    assert!((sc.materials[0].transparency() - 0.1).abs() < 1e-6);
    assert_eq!(sc.face_sets[0].nb_coords(), 4);
    assert_eq!(sc.face_sets[0].nb_faces(), 4);
    assert_eq!(sc.face_sets[0].face(3), &[1usize, 2, 3]);
}
